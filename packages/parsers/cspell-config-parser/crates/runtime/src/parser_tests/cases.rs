#[test]
fn invalid_json_returns_error() {
    let error = super::super::parse_document("{")
        .expect_err("invalid JSON should fail before typed normalization");

    assert!(
        error.to_string().contains("EOF"),
        "error should preserve serde_json parse detail: {error}"
    );
}

#[test]
fn non_object_json_returns_invalid_document() {
    let document =
        super::super::parse_document("[]").expect("valid JSON should produce a document");

    assert_eq!(
        crate::parse_error_reason(&document),
        Some("cspell config root must be a JSON object")
    );
}
