use npmrc_parser_runtime::types::NpmrcDocument;

pub fn assert_parsed_document(document: &NpmrcDocument) {
    assert!(
        npmrc_parser_runtime::typed(document).is_some(),
        "expected parsed .npmrc document, got: {document:#?}"
    );
}

pub fn assert_invalid_document(document: &NpmrcDocument, expected_reason_fragment: &str) {
    let Some(reason) = npmrc_parser_runtime::parse_error_reason(document) else {
        assert!(
            false,
            "expected invalid .npmrc document, got parsed: {document:#?}"
        );
        return;
    };

    assert!(
        reason.contains(expected_reason_fragment),
        "expected invalid reason to contain {expected_reason_fragment:?}, got {reason:?}"
    );
}

pub fn assert_effective_value(document: &NpmrcDocument, key: &str, expected: Option<&str>) {
    assert_eq!(
        npmrc_parser_runtime::effective_value(document, key),
        expected,
        "effective .npmrc value mismatch for `{key}`"
    );
}

pub fn assert_duplicate_keys(document: &NpmrcDocument, expected: &[&str]) {
    let Some(snapshot) = npmrc_parser_runtime::typed(document) else {
        assert!(false, "expected parsed .npmrc document, got: {document:#?}");
        return;
    };

    let expected = expected
        .iter()
        .map(|item| (*item).to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        snapshot.duplicate_keys, expected,
        "duplicate .npmrc key mismatch"
    );
}
