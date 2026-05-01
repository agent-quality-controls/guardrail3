pub fn assert_parse_error(input: &str) {
    assert!(
        cspell_config_parser_runtime::parse_document(input).is_err(),
        "cspell config parser should reject invalid JSON"
    );
}
