use package_json_parser_runtime_assertions::parser::{
    assert_bool_field_state, assert_invalid_document, assert_parsed_document,
    assert_snapshot_fields,
};

#[test]
fn parses_root_manifest_snapshot() {
    let document = super::super::parse_document(
        r#"
        {
          "private": true,
          "packageManager": "pnpm@10.0.0",
          "engines": {
            "node": ">=24",
            "pnpm": "10"
          },
          "scripts": {
            "lint": "eslint .",
            "typecheck": "tsc --noEmit"
          },
          "pnpm": {
            "overrides": {
              "@eslint/js": "^9.0.0",
              "zod": "^4.0.0"
            },
            "onlyBuiltDependencies": ["esbuild"]
          },
          "dependencies": {
            "react": "^19.0.0"
          },
          "devDependencies": {
            "typescript": "^5.7.0"
          }
        }
        "#,
    )
    .expect("package.json document should parse");

    assert_parsed_document(&document);
    assert_bool_field_state(&document, "private", Some(true));

    assert_snapshot_fields(
        &document,
        Some("pnpm@10.0.0"),
        Some(">=24"),
        Some("10"),
        Some("eslint ."),
        &["@eslint/js", "zod"],
        &["esbuild"],
        &["react"],
        &["typescript"],
    );
}

#[test]
fn rejects_non_object_root() {
    let document = super::super::parse_document(r#""not-an-object""#)
        .expect("json parser should still produce a document");

    assert_invalid_document(&document, "package.json root must be a JSON object");
}

#[test]
fn rejects_non_object_scripts() {
    let document = super::super::parse_document(
        r#"
        {
          "scripts": true
        }
        "#,
    )
    .expect("json parser should still produce a document");

    assert_invalid_document(&document, "package.json field `scripts` must be an object");
}

#[test]
fn rejects_non_string_dependency_versions() {
    let document = super::super::parse_document(
        r#"
        {
          "dependencies": {
            "react": true
          }
        }
        "#,
    )
    .expect("json parser should still produce a document");

    assert_invalid_document(
        &document,
        "package.json field `dependencies.react` must be a string",
    );
}
