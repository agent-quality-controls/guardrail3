use tsconfig_json_parser_runtime::types::TsconfigBoolFieldState;

pub fn assert_parsed_document(document: &tsconfig_json_parser_runtime::types::TsconfigDocument) {
    assert!(
        tsconfig_json_parser_runtime::typed(document).is_some(),
        "expected parsed tsconfig document, got: {document:#?}"
    );
}

pub fn assert_invalid_document(
    document: &tsconfig_json_parser_runtime::types::TsconfigDocument,
    expected_reason_fragment: &str,
) {
    let Some(reason) = tsconfig_json_parser_runtime::parse_error_reason(document) else {
        assert!(
            false,
            "expected invalid tsconfig document, got parsed: {document:#?}"
        );
        return;
    };
    assert!(
        reason.contains(expected_reason_fragment),
        "expected invalid reason to contain {expected_reason_fragment:?}, got {reason:?}"
    );
}

pub fn assert_extends_entries(
    document: &tsconfig_json_parser_runtime::types::TsconfigDocument,
    expected: &[&str],
) {
    let actual = tsconfig_json_parser_runtime::extends_entries(document);
    let expected_vec = expected
        .iter()
        .map(|entry| (*entry).to_owned())
        .collect::<Vec<_>>();
    assert_eq!(actual, expected_vec.as_slice(), "tsconfig extends mismatch");
}

pub fn assert_bool_field_state(
    document: &tsconfig_json_parser_runtime::types::TsconfigDocument,
    field: &str,
    expected: Option<bool>,
) {
    match (
        tsconfig_json_parser_runtime::bool_field_state(document, field),
        expected,
    ) {
        (TsconfigBoolFieldState::Missing, None) => {}
        (TsconfigBoolFieldState::Value(actual), Some(expected)) => {
            assert_eq!(actual, expected, "bool field mismatch for {field}");
        }
        (actual, expected) => assert!(
            false,
            "unexpected bool field state for {field}; actual: {actual:?}, expected: {expected:?}"
        ),
    }
}
