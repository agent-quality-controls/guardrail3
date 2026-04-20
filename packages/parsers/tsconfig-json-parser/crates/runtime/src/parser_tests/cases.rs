use tsconfig_json_parser_runtime_assertions::parser::{
    assert_bool_field_state, assert_extends_entries, assert_invalid_document,
    assert_parsed_document,
};

#[test]
fn parses_jsonc_with_comments_and_trailing_commas() {
    let document = super::super::parse_document(
        r#"
        {
          // shared base
          "extends": ["../../tsconfig.base.json",],
          "compilerOptions": {
            "strict": true,
            "allowUnusedLabels": false,
          },
        }
        "#,
    )
    .expect("tsconfig document should parse");

    assert_parsed_document(&document);
    assert_extends_entries(&document, &["../../tsconfig.base.json"]);
    assert_bool_field_state(&document, "strict", Some(true));
    assert_bool_field_state(&document, "allowUnusedLabels", Some(false));
}

#[test]
fn rejects_non_object_root() {
    let document = super::super::parse_document(r#""not-an-object""#)
        .expect("jsonc parser should still produce a document");

    assert_invalid_document(&document, "tsconfig root must be a JSON object");
}

#[test]
fn rejects_non_string_extends_entries() {
    let document = super::super::parse_document(
        r#"
        {
          "extends": [true]
        }
        "#,
    )
    .expect("jsonc parser should still produce a document");

    assert_invalid_document(
        &document,
        "tsconfig extends array must contain only strings",
    );
}

#[test]
fn rejects_non_object_compiler_options() {
    let document = super::super::parse_document(
        r#"
        {
          "compilerOptions": true
        }
        "#,
    )
    .expect("jsonc parser should still produce a document");

    assert_invalid_document(&document, "tsconfig compilerOptions must be a JSON object");
}
