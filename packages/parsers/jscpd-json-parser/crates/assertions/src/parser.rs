use jscpd_json_parser_runtime::types::JscpdDocument;

pub fn assert_parsed_document(document: &JscpdDocument) {
    assert!(
        jscpd_json_parser_runtime::typed(document).is_some(),
        "expected parsed .jscpd document, got: {document:#?}"
    );
}

pub fn assert_invalid_document(document: &JscpdDocument, expected_reason_fragment: &str) {
    let reason = match jscpd_json_parser_runtime::parse_error_reason(document) {
        Some(reason) => reason,
        None => {
            assert!(
                false,
                "expected invalid .jscpd document, got parsed: {document:#?}"
            );
            return;
        }
    };

    assert!(
        reason.contains(expected_reason_fragment),
        "expected invalid reason to contain {expected_reason_fragment:?}, got {reason:?}"
    );
}

pub fn assert_snapshot(
    document: &JscpdDocument,
    threshold: Option<i64>,
    min_tokens: Option<u64>,
    absolute: Option<bool>,
    format: &[&str],
    ignore: &[&str],
) {
    let snapshot = match jscpd_json_parser_runtime::typed(document) {
        Some(snapshot) => snapshot,
        None => {
            assert!(false, "expected parsed .jscpd document, got: {document:#?}");
            return;
        }
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

pub fn assert_extra_keys(document: &JscpdDocument, expected: &[&str]) {
    let snapshot = match jscpd_json_parser_runtime::typed(document) {
        Some(snapshot) => snapshot,
        None => {
            assert!(false, "expected parsed .jscpd document, got: {document:#?}");
            return;
        }
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
