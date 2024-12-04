use strinject::*;

#[test]
fn simple_injection() {
    let result = inject("<load path='tests/to_inject1.txt' marker='ToInject1_1' />");

    assert_eq!(
        result.expect("This should not error out"),
        "correct data1 1
"
    );
}

#[test]
fn marker_short() {
    use crate::*;

    let result = inject("<load path='tests/to_inject1.txt' marker='ToInject' />");

    assert_eq!(
        result.expect("This should not error out"),
        "correct data short
"
    );
}

#[test]
fn nest_removal() {
    use crate::*;

    let result = inject("<load path='tests/to_inject1.txt' marker='ToInject1_nest' />");

    assert_eq!(
        result.expect("This should not error out"),
        "correct data nest1
correct data nested
correct data nest2
"
    );
}

#[test]
fn whole_file() {
    use crate::*;

    let result = inject("<load path='tests/to_inject1.txt' />");

    assert_eq!(
        result.expect("This should not error out"),
        "incorrect data1 1
correct data1 1
incorrect data1 2
correct data1 2
incorrect data1 3


// Short marker to verify it's not taking the longer ones.
correct data short

correct data nest1
correct data nested
correct data nest2

some outside data
    correct data not indented
        empty line without space next
            empty line without space next

    correct data not indented again

outside data again
"
    );
}

#[test]
fn indent_removal_simple() {
    use crate::*;

    let result = remove_indent(
        "    correct data indented 1
        correct data indented more
    correct data indented 2
",
    );

    assert_eq!(
        result.expect("This should not error out"),
        "correct data indented 1
    correct data indented more
correct data indented 2
"
    );
}

#[test]
fn indent_removal() {
    use crate::*;

    let result = inject("<load path='tests/to_inject1.txt' marker='ToInject1_indent' />");

    assert_eq!(
        result.expect("This should not error out"),
        "correct data not indented
    empty line without space next
        empty line without space next

correct data not indented again
"
    );
}

#[test]
fn simple_tag_error() {
    use crate::*;

    let result = inject("<load path='tests/to_inject1.txt' marker=\"ToInject1_1\" />");

    assert!(matches!(
        result.expect_err("This should error out").errors[0],
        ErrorType::IncorrectTag
    ));
}

#[test]
fn simple_path_error() {
    use crate::*;

    let result =
        inject("<load path='this_path_does_not_exist_obviously' marker='ToInject1_wrong' />");

    assert_eq!(
        result.expect_err("This should error out").errors[0],
        ErrorType::IncorrectPath("this_path_does_not_exist_obviously".to_string())
    );
}

#[test]
fn simple_marker_error() {
    use crate::*;

    let result = inject("<load path='tests/to_inject1.txt' marker='ToInject1_wrong' />");

    match &result.expect_err("This should error out").errors[0] {
        ErrorType::IncorrectMarker(_) => {}
        invalid_value => {
            panic!("unexpected error type: {:?}", invalid_value);
        }
    }
}
