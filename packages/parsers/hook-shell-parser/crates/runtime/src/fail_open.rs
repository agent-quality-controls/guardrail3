use crate::shell_ast;
use crate::types::FailOpenWrapper;

pub(super) fn detect_fail_open_wrapper(line: &str) -> Option<FailOpenWrapper> {
    shell_ast::command_segments(line)
        .into_iter()
        .find(|segment| matches!(segment.operator_before, Some("||")))
        .and_then(|segment| fail_open_wrapper_from_command(&segment.text))
}

pub(super) fn command_substitution_assignment_wrapper(line: &str) -> Option<FailOpenWrapper> {
    let words = shell_ast::shell_words(line);
    let command = words.first()?;
    matches!(
        command.as_str(),
        "export" | "local" | "declare" | "readonly"
    )
    .then_some(FailOpenWrapper::CommandSubstitutionAssignment(
        line.to_owned(),
    ))
}

pub(super) fn is_fail_open_wrapper_command(command_text: &str) -> bool {
    fail_open_wrapper_from_command(command_text).is_some()
}

fn fail_open_wrapper_from_command(command_text: &str) -> Option<FailOpenWrapper> {
    let words = shell_ast::shell_words(command_text);
    let command = words.first()?;
    match command.as_str() {
        "true" => Some(FailOpenWrapper::True),
        ":" => Some(FailOpenWrapper::NoOp),
        "echo" => Some(FailOpenWrapper::Echo(command_text.to_owned())),
        "printf" => Some(FailOpenWrapper::Printf(command_text.to_owned())),
        "exit" if words.get(1).is_some_and(|argument| argument == "0") => {
            Some(FailOpenWrapper::ExitZero)
        }
        "return" if words.get(1).is_some_and(|argument| argument == "0") => {
            Some(FailOpenWrapper::ReturnZero)
        }
        _ => None,
    }
}
