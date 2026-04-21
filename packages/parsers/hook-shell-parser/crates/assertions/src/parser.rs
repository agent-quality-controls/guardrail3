use hook_shell_parser_runtime::parse_script;
use hook_shell_parser_runtime::types::{FailOpenWrapper, ParsedShellScript};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedFailOpen {
    None,
    True,
    NoOp,
    Echo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandExpectation<'a> {
    name: &'a str,
    text: Option<&'a str>,
    dispatcher: Option<bool>,
    fail_open: Option<ExpectedFailOpen>,
    exit_zero: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionExpectation<'a> {
    name: &'a str,
    body_contains: Option<&'a str>,
    body_command_names: Option<&'a [&'a str]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScriptExpectation<'a> {
    shebang: Option<&'a str>,
    commands: &'a [CommandExpectation<'a>],
    functions: &'a [FunctionExpectation<'a>],
}

impl<'a> CommandExpectation<'a> {
    #[must_use]
    pub const fn new(
        name: &'a str,
        text: Option<&'a str>,
        dispatcher: Option<bool>,
        fail_open: Option<ExpectedFailOpen>,
        exit_zero: Option<bool>,
    ) -> Self {
        Self {
            name,
            text,
            dispatcher,
            fail_open,
            exit_zero,
        }
    }
}

impl<'a> FunctionExpectation<'a> {
    #[must_use]
    pub const fn new(
        name: &'a str,
        body_contains: Option<&'a str>,
        body_command_names: Option<&'a [&'a str]>,
    ) -> Self {
        Self {
            name,
            body_contains,
            body_command_names,
        }
    }
}

impl<'a> ScriptExpectation<'a> {
    #[must_use]
    pub const fn new(
        shebang: Option<&'a str>,
        commands: &'a [CommandExpectation<'a>],
        functions: &'a [FunctionExpectation<'a>],
    ) -> Self {
        Self {
            shebang,
            commands,
            functions,
        }
    }
}

pub fn assert_script_matches(parsed: &ParsedShellScript, expected: ScriptExpectation<'_>) {
    assert_eq!(
        parsed.shebang.as_deref(),
        expected.shebang,
        "shebang mismatch"
    );
    assert_eq!(
        parsed.executable_lines.len(),
        expected.commands.len(),
        "executable line count mismatch",
    );
    assert_eq!(
        parsed.functions.len(),
        expected.functions.len(),
        "function count mismatch",
    );

    for (index, command) in expected.commands.iter().enumerate() {
        let actual = &parsed.executable_lines[index];
        assert_eq!(
            actual.command_name.as_str(),
            command.name,
            "command name mismatch at index {index}",
        );
        if let Some(text) = command.text {
            assert_eq!(
                actual.command_text.as_str(),
                text,
                "command text mismatch at index {index}",
            );
        }
        if let Some(dispatcher) = command.dispatcher {
            assert_eq!(
                actual.is_dispatcher_syntax, dispatcher,
                "dispatcher flag mismatch at index {index}",
            );
        }
        if let Some(exit_zero) = command.exit_zero {
            assert_eq!(
                actual.is_exit_zero, exit_zero,
                "exit-zero flag mismatch at index {index}",
            );
        }
        if let Some(fail_open) = command.fail_open {
            let actual_fail_open = actual.softened_by.as_ref();
            match fail_open {
                ExpectedFailOpen::None => {
                    assert!(
                        actual_fail_open.is_none(),
                        "expected no fail-open wrapper at index {index}",
                    );
                }
                ExpectedFailOpen::True => {
                    assert_eq!(
                        actual_fail_open,
                        Some(&FailOpenWrapper::True),
                        "expected || true wrapper at index {index}",
                    );
                }
                ExpectedFailOpen::NoOp => {
                    assert_eq!(
                        actual_fail_open,
                        Some(&FailOpenWrapper::NoOp),
                        "expected || : wrapper at index {index}",
                    );
                }
                ExpectedFailOpen::Echo => {
                    assert!(
                        matches!(actual_fail_open, Some(FailOpenWrapper::Echo(_))),
                        "expected || echo wrapper at index {index}",
                    );
                }
            }
        }
    }

    for (index, function) in expected.functions.iter().enumerate() {
        let actual = &parsed.functions[index];
        assert_eq!(
            actual.name.as_str(),
            function.name,
            "function name mismatch at index {index}",
        );
        if let Some(body_contains) = function.body_contains {
            assert!(
                actual.body.contains(body_contains),
                "function body at index {index} should contain {body_contains:?}",
            );
        }
        if let Some(body_command_names) = function.body_command_names {
            let nested = parse_script(&actual.body);
            let nested_names = nested
                .executable_lines
                .iter()
                .map(|line| line.command_name.as_str())
                .collect::<Vec<_>>();
            assert_eq!(
                nested_names, body_command_names,
                "function body command names mismatch at index {index}",
            );
        }
    }
}
