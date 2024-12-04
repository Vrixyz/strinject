#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use regex::{Captures, Regex};
use std::{borrow::Cow, fs::read_to_string};

/// Something went wrong while looking for the markers
#[derive(Debug, PartialEq)]
pub struct IncorrectMarker {
    /// name of the marker.
    pub marker: String,
    /// file where the marker should have been found.
    pub filepath: String,
}

/// An error while injecting text.
#[derive(Debug, PartialEq)]
pub enum ErrorType {
    /// a load tag was incorrectly formed in source text.
    IncorrectTag,
    /// Could not read the file at given path:
    IncorrectPath(String),
    /// An error while injecting text.
    IncorrectMarker(IncorrectMarker),
}

/// Something went wrong during text injection
#[derive(Debug)]
pub struct InjectError<'a> {
    /// Contains the best result we could do.
    pub result: Option<Cow<'a, str>>,
    /// Errors encountered
    pub errors: Vec<ErrorType>,
}

/// Reads the parameter and returns a new string with injected text.
///
/// This assumes paths are from current directory. See [`inject_with_path`] for more options.
pub fn inject(source_text: &str) -> Result<String, InjectError> {
    inject_with_path(source_text, |path| path.to_string())
}

/// Reads the parameter and returns a new string with injected text.
///
/// This gives you a chance to customize any loaded path.
pub fn inject_with_path(
    source_text: &str,
    get_path: fn(&str) -> String,
) -> Result<String, InjectError> {
    let source_text = &source_text.replace("\r\n", "\n");
    let re = Regex::new(r"<load.*>").unwrap();
    let total_to_inject = re.find_iter(source_text).count();
    let mut injected_count = 0;
    // Regex to find "<load" tags and capture their info (path + marker)
    let re = Regex::new(r"<load path='(.*)'.*marker='(.*)'.*>\n?").unwrap();

    let mut error = InjectError {
        result: None,
        errors: Vec::new(),
    };
    let result = re.replace_all(source_text, |caps: &Captures| {
        let infos = &caps.extract::<2>().1;
        assert!(
            infos.len() == 2,
            "load tag encountered without a path or marker"
        );
        let path = get_path(infos[0]);
        // Reading file from the path of the tag of input file
        let Ok(to_inject) = read_to_string(&path) else {
            error.errors.push(ErrorType::IncorrectPath(path));
            return "".to_string();
        };
        let to_inject = to_inject.replace("\r\n", "\n");
        // Regex to find the markers inside comments, and only print what's inside
        // FIXME: I think we should just paste all the inside,
        // and then remove all "// DOCUSAURUS*"" lines, to allow reuse of a same file.
        let regex = format!(
            r"// DOCUSAURUS: {} start\n((?:\s|.)*)\s+\/\/ DOCUSAURUS: {} stop",
            infos[1], infos[1]
        );
        let re = Regex::new(&regex).unwrap();
        let to_keep = re
            .captures_iter(&to_inject)
            .map(|c| {
                let to_keep = c.extract::<1>();
                injected_count += 1;
                let injected_string = to_keep.1[0];
                let injected_string = remove_indent(injected_string)
                    .unwrap_or(injected_string.to_string())
                    .trim_end()
                    .to_string();
                injected_string
            })
            .collect::<Vec<_>>();
        if to_keep.is_empty() {
            error
                .errors
                .push(ErrorType::IncorrectMarker(IncorrectMarker {
                    marker: infos[1].to_string(),
                    filepath: infos[0].to_string(),
                }));
        }
        let mut result = to_keep.join("");
        result.push('\n');
        result
    });
    if (injected_count + error.errors.len()) != total_to_inject {
        error.errors.push(ErrorType::IncorrectTag);
    }
    if !error.errors.is_empty() {
        return Err(error);
    }
    let re = Regex::new(r"(.*\/\/ DOCUSAURUS:.*\n)").unwrap();
    let result = re.replace_all(&result, |_: &Captures| "").to_string();
    Ok(result)
}

/// Returns a new string with the same content but with the minimum indent removed.
///
/// Also normalizes line endings to `\n`.
pub fn remove_indent(source: &str) -> Option<String> {
    let source = source.replace("\r\n", "\n");
    let min_indent = source
        .lines()
        .filter_map(|l| {
            // Don't count empty lines
            if !l.chars().any(|c| !c.is_whitespace()) {
                return None;
            }
            Some(l.chars().take_while(|c| c.is_whitespace()).count())
        })
        .min()?;
    if min_indent == 0 {
        return Some(source.to_string());
    }
    let unindented_lines = source
        .lines()
        .map(|l| l.chars().skip(min_indent).collect::<String>());
    let mut result = String::new();
    for unindented_line in unindented_lines {
        result.push_str(&unindented_line);
        result.push('\n');
    }
    Some(result)
}
