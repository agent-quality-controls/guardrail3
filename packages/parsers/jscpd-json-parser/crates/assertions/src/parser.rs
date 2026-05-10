//! Assertion helpers that surface typed `.jscpd.json` document state to runtime tests.

use jscpd_json_parser_runtime::types::JscpdDocument;

/// Asserts that `document` was parsed into a typed snapshot (not invalid).
///
/// # Panics
/// Panics when `document` is in the invalid state.
pub fn assert_parsed_document(document: &JscpdDocument) {
    assert!(
        jscpd_json_parser_runtime::typed(document).is_some(),
        "expected parsed .jscpd document, got: {document:#?}"
    );
}

/// Asserts that `document` is in the invalid state and that its reason contains `expected_reason_fragment`.
///
/// # Panics
/// Panics when `document` is in the parsed state or the invalid reason does not contain `expected_reason_fragment`.
pub fn assert_invalid_document(document: &JscpdDocument, expected_reason_fragment: &str) {
    let Some(reason) = jscpd_json_parser_runtime::parse_error_reason(document) else {
        unreachable!("expected invalid .jscpd document, got parsed: {document:#?}");
    };

    assert!(
        reason.contains(expected_reason_fragment),
        "expected invalid reason to contain {expected_reason_fragment:?}, got {reason:?}"
    );
}

/// Asserts the parsed snapshot fields match the expected values.
///
/// # Panics
/// Panics when `document` is invalid or any snapshot field does not equal its expected value.
#[allow(
    clippy::too_many_arguments,
    reason = "snapshot proof site asserts each .jscpd.json field independently per g3rs-test/runtime-assertions-split"
)]
pub fn assert_snapshot(
    document: &JscpdDocument,
    threshold: Option<i64>,
    min_tokens: Option<u64>,
    absolute: Option<bool>,
    format: &[&str],
    ignore: &[&str],
) {
    let Some(snapshot) = jscpd_json_parser_runtime::typed(document) else {
        unreachable!("expected parsed .jscpd document, got: {document:#?}");
    };

    assert_eq!(snapshot.threshold, threshold, "threshold mismatch");
    assert_eq!(snapshot.min_tokens, min_tokens, "minTokens mismatch");
    assert_eq!(snapshot.absolute, absolute, "absolute mismatch");
    assert_eq!(
        snapshot.format,
        format
            .iter()
            .map(|item| (*item).to_owned())
            .collect::<Vec<_>>(),
        "format mismatch"
    );
    assert_eq!(
        snapshot.ignore,
        ignore
            .iter()
            .map(|item| (*item).to_owned())
            .collect::<Vec<_>>(),
        "ignore mismatch"
    );
}

/// Asserts that the parsed snapshot's `extra_keys` set equals `expected`.
///
/// # Panics
/// Panics when `document` is invalid or its `extra_keys` set does not equal `expected`.
pub fn assert_extra_keys(document: &JscpdDocument, expected: &[&str]) {
    let Some(snapshot) = jscpd_json_parser_runtime::typed(document) else {
        unreachable!("expected parsed .jscpd document, got: {document:#?}");
    };
    assert_eq!(
        snapshot.extra_keys,
        expected
            .iter()
            .map(|item| (*item).to_owned())
            .collect::<Vec<_>>(),
        "extra key mismatch"
    );
}
