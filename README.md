# Text Inject

## Use case

When writing documentation, it's useful to reference and inline code snippets.

`text_inject` helps you do that.

## Features

Replace text in a file, taking content from another file.

A simple API:

```rs
pub fn inject(source_text: &str) -> Result<String, InjectError>;
```

### Content Marker

Any occurrence of the tag `<load path='test/to_inject1.txt' marker='ToInject1_1' />`
will load the file `test/to_inject1.txt`, search for the text between `// DOCUSAURUS: ToInject1_1: start`
and the next `// DOCUSAURUS: ToInject1_1: stop`, and replace the tag with the text between those markers.

## Removes extraneous spaces

Useful code is sometimes indented, `text_inject` removes out-of-context indentation.

## Recommendations

We advise to setup your own diff tooling to verify no unintended modifications were introduced
after the injection took place.

## Alternatives

- `rustdoc` checks the whole inlined code, and allows to hide specific lines
  - Good:
    - Easy to use
  - Bad:
    - Not ideal for code snippets which need a lot of setup boilerplate.
    - It is impractical to inline code with a more advanced setup.
    - Less customizable, `text_inject` is context agnostic.
- using custom script with sed / awk / perl / whatever
  - Good:
    - it can work.
  - Bad:
    - cross-platform support is not ideal.
    - not trivial to write, and even worse to read.
- Tera/Askama/rinja/handlebars/
  - Good:
    - powerful
  - Bad:
    - A lot of features
    - difficult to know if it's possible to search specific data from somewhere else.
- manual copy pasting
  - Good:
    - Quick solution for simple cases
  - Bad:
    - error-prone
    - difficult to verify code correctness
