#![allow(
    clippy::assertions_on_constants,
    clippy::expect_used,
    clippy::missing_panics_doc,
    reason = "assertion helpers are panic-based proof sites for parser tests"
)]

pub use eslint_directive_parser_runtime::types::{
    EslintDirectiveKind, EslintDisabledRuleSet,
};

use eslint_directive_parser_runtime::types::{
    EslintDirectiveDocument, EslintDirectiveParseState,
};

pub fn assert_parsed_document(document: &EslintDirectiveDocument) {
    assert!(
        matches!(
            eslint_directive_parser_runtime::typed(document).state,
            EslintDirectiveParseState::Parsed { .. }
        ),
        "expected parsed ESLint directive document, got: {document:#?}"
    );
}

pub fn assert_state_reason_contains(document: &EslintDirectiveDocument, expected: &str) {
    let Some(reason) = eslint_directive_parser_runtime::parse_error_reason(document) else {
        assert!(false, "expected non-parsed directive state");
        return;
    };
    assert!(
        reason.contains(expected),
        "expected reason to contain {expected:?}, got {reason:?}"
    );
}

pub fn assert_ambiguous_document(document: &EslintDirectiveDocument) {
    assert!(
        matches!(
            eslint_directive_parser_runtime::typed(document).state,
            EslintDirectiveParseState::Ambiguous { .. }
        ),
        "expected ambiguous ESLint directive document, got: {document:#?}"
    );
}

pub fn assert_unsupported_document(document: &EslintDirectiveDocument) {
    assert!(
        matches!(
            eslint_directive_parser_runtime::typed(document).state,
            EslintDirectiveParseState::Unsupported { .. }
        ),
        "expected unsupported ESLint directive document, got: {document:#?}"
    );
}

pub fn assert_parse_error_document(document: &EslintDirectiveDocument) {
    assert!(
        matches!(
            eslint_directive_parser_runtime::typed(document).state,
            EslintDirectiveParseState::ParseError { .. }
        ),
        "expected parse-error ESLint directive document, got: {document:#?}"
    );
}

pub fn assert_directive_count(document: &EslintDirectiveDocument, expected: usize) {
    let EslintDirectiveParseState::Parsed { findings } =
        &eslint_directive_parser_runtime::typed(document).state
    else {
        assert!(false, "document should parse");
        return;
    };
    assert_eq!(findings.len(), expected, "directive count mismatch");
}

pub fn assert_directive(
    document: &EslintDirectiveDocument,
    index: usize,
    expected_kind: EslintDirectiveKind,
    expected_line: u32,
    expected_target_line: Option<u32>,
    expected_rules: &EslintDisabledRuleSet,
) {
    let EslintDirectiveParseState::Parsed { findings } =
        &eslint_directive_parser_runtime::typed(document).state
    else {
        assert!(false, "document should parse");
        return;
    };
    let directive = findings.get(index).expect("directive should exist");
    assert_eq!(
        directive.directive_kind, expected_kind,
        "directive kind mismatch"
    );
    assert_eq!(directive.line, expected_line, "directive line mismatch");
    assert_eq!(
        directive.target_line, expected_target_line,
        "directive target line mismatch"
    );
    assert_eq!(
        &directive.disabled_rules, expected_rules,
        "directive disabled rules mismatch"
    );
}
