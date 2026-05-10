/// Fails the calling test when `parse_document` accepts `input` instead of rejecting it.
///
/// # Panics
/// Panics when `parse_document` returns `Ok`, which the assertion intentionally treats as a test failure.
pub fn assert_parse_error(input: &str) {
    assert!(
        cspell_config_parser_runtime::parse_document(input).is_err(),
        "cspell config parser should reject invalid JSON"
    );
}
