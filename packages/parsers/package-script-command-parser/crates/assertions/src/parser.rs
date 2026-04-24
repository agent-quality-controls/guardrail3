#![allow(
    clippy::assertions_on_constants,
    clippy::expect_used,
    clippy::missing_panics_doc,
    reason = "assertion helpers are panic-based proof sites for parser tests"
)]

use package_script_command_parser_runtime::types::{
    PackageScriptCommandDocument, PackageScriptCommandSeparator, PackageScriptParseState,
};

pub fn assert_parsed_document(document: &PackageScriptCommandDocument) {
    assert!(
        matches!(
            package_script_command_parser_runtime::typed(document).state,
            PackageScriptParseState::Parsed { .. }
        ),
        "expected parsed package script command document, got: {document:#?}"
    );
}

pub fn assert_no_eslint_invocation(document: &PackageScriptCommandDocument) {
    assert!(
        matches!(
            package_script_command_parser_runtime::typed(document).state,
            PackageScriptParseState::NoEslintInvocation
        ),
        "expected no ESLint invocation, got: {document:#?}"
    );
}

pub fn assert_unsupported_document(document: &PackageScriptCommandDocument) {
    assert!(
        matches!(
            package_script_command_parser_runtime::typed(document).state,
            PackageScriptParseState::Unsupported { .. }
        ),
        "expected unsupported package script document, got: {document:#?}"
    );
}

pub fn assert_parse_error_document(document: &PackageScriptCommandDocument) {
    assert!(
        matches!(
            package_script_command_parser_runtime::typed(document).state,
            PackageScriptParseState::ParseError { .. }
        ),
        "expected parse-error package script document, got: {document:#?}"
    );
}

pub fn assert_state_reason_contains(
    document: &PackageScriptCommandDocument,
    expected_reason_fragment: &str,
) {
    let Some(reason) = package_script_command_parser_runtime::parse_error_reason(document) else {
        assert!(false, "expected non-parsed package script state");
        return;
    };
    assert!(
        reason.contains(expected_reason_fragment),
        "expected invalid reason to contain {expected_reason_fragment:?}, got {reason:?}"
    );
}

pub fn assert_command_count(document: &PackageScriptCommandDocument, expected: usize) {
    let PackageScriptParseState::Parsed { commands, .. } =
        &package_script_command_parser_runtime::typed(document).state
    else {
        assert!(false, "document should parse");
        return;
    };
    assert_eq!(commands.len(), expected, "command count mismatch");
}

pub fn assert_command(
    document: &PackageScriptCommandDocument,
    index: usize,
    expected_invocation: &str,
    expected_executable: &str,
    expected_args: &[&str],
    expected_preceded_by: Option<PackageScriptCommandSeparator>,
) {
    let PackageScriptParseState::Parsed { commands, .. } =
        &package_script_command_parser_runtime::typed(document).state
    else {
        assert!(false, "document should parse");
        return;
    };
    let command = commands.get(index).expect("command should exist");
    assert_eq!(
        command.invocation, expected_invocation,
        "command invocation mismatch"
    );
    assert_eq!(
        command.executable, expected_executable,
        "command executable mismatch"
    );
    assert_eq!(
        command.args,
        expected_args
            .iter()
            .map(|item| (*item).to_owned())
            .collect::<Vec<_>>(),
        "command args mismatch"
    );
    assert_eq!(
        command.preceded_by, expected_preceded_by,
        "command separator mismatch"
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectedEslintInvocation<'a> {
    pub script_name: &'a str,
    pub command_index: usize,
    pub invocation: &'a str,
    pub args: &'a [&'a str],
    pub ignore_patterns: &'a [&'a str],
    pub ignore_path: Option<&'a str>,
    pub config_path: Option<&'a str>,
}

pub fn assert_eslint_invocation(
    document: &PackageScriptCommandDocument,
    index: usize,
    expected: ExpectedEslintInvocation<'_>,
) {
    let PackageScriptParseState::Parsed {
        eslint_invocations,
        ..
    } = &package_script_command_parser_runtime::typed(document).state
    else {
        assert!(false, "document should parse");
        return;
    };
    let invocation = eslint_invocations
        .get(index)
        .expect("ESLint invocation should exist");
    assert_eq!(
        invocation.script_name, expected.script_name,
        "ESLint script name mismatch"
    );
    assert_eq!(
        invocation.command_index, expected.command_index,
        "ESLint command index mismatch"
    );
    assert_eq!(
        invocation.invocation, expected.invocation,
        "ESLint raw invocation mismatch"
    );
    assert_eq!(
        invocation.args,
        expected
            .args
            .iter()
            .map(|item| (*item).to_owned())
            .collect::<Vec<_>>(),
        "ESLint args mismatch"
    );
    assert_eq!(
        invocation.ignore_patterns,
        expected
            .ignore_patterns
            .iter()
            .map(|item| (*item).to_owned())
            .collect::<Vec<_>>(),
        "ESLint ignore patterns mismatch"
    );
    assert_eq!(
        invocation.ignore_path.as_deref(),
        expected.ignore_path,
        "ESLint ignore path mismatch"
    );
    assert_eq!(
        invocation.config_path.as_deref(),
        expected.config_path,
        "ESLint config path mismatch"
    );
}
