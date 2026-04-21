use jscpd_json_parser_runtime_assertions::parser::{
    assert_extra_keys, assert_invalid_document, assert_parsed_document, assert_snapshot,
};

#[test]
fn parses_minimal_jscpd_surface() {
    let document = super::super::parse_document(
        r#"
        {
          "threshold": 0,
          "minTokens": 50,
          "absolute": true,
          "format": ["typescript"],
          "ignore": ["**/node_modules/**"]
        }
        "#,
    )
    .expect(".jscpd document should parse");

    assert_parsed_document(&document);
    assert_snapshot(
        &document,
        Some(0),
        Some(50),
        Some(true),
        &["typescript"],
        &["**/node_modules/**"],
    );
    assert_extra_keys(&document, &[]);
}

#[test]
fn preserves_unknown_top_level_keys_as_inventory_surface() {
    let document = super::super::parse_document(
        r#"
        {
          "threshold": 0,
          "minTokens": 50,
          "absolute": true,
          "format": ["typescript", "rust"],
          "ignore": ["**/node_modules/**"],
          "reporters": ["consoleFull"],
          "$schema": "https://json.schemastore.org/jscpd.json",
          "gitignore": true
        }
        "#,
    )
    .expect(".jscpd document should parse");

    assert_parsed_document(&document);
    assert_extra_keys(&document, &["gitignore"]);
}

#[test]
fn rejects_non_object_roots() {
    let document = super::super::parse_document(r#"["not","an","object"]"#)
        .expect(".jscpd parse should return document");

    assert_invalid_document(&document, "must be a JSON object");
}

#[test]
fn reports_invalid_json() {
    let error = super::super::parse_document("{ invalid ").expect_err("invalid JSON should error");
    assert!(
        error.to_string().starts_with("failed to parse .jscpd:"),
        "unexpected parse error: {error}"
    );
}
