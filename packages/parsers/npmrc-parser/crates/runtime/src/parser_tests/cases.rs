use npmrc_parser_runtime_assertions::parser::{
    assert_duplicate_keys, assert_effective_value, assert_invalid_document, assert_parsed_document,
};

#[test]
fn parses_narrow_npmrc_surface() {
    let document = super::super::parse_document(
        r"
        strict-peer-dependencies=true
        engine-strict=true
        minimum-release-age=1440
        minimum-release-age-exclude=@base-ui/react
        //registry.npmjs.org/:_authToken=${NPM_TOKEN}
        ",
    );

    assert_parsed_document(&document);
    assert_effective_value(&document, "strict-peer-dependencies", Some("true"));
    assert_effective_value(&document, "engine-strict", Some("true"));
    assert_effective_value(&document, "minimum-release-age", Some("1440"));
    assert_effective_value(
        &document,
        "minimum-release-age-exclude",
        Some("@base-ui/react"),
    );
    assert_effective_value(
        &document,
        "//registry.npmjs.org/:_authToken",
        Some("${NPM_TOKEN}"),
    );
    assert_duplicate_keys(&document, &[]);
}

#[test]
fn strips_inline_comments_and_quotes() {
    let document = super::super::parse_document(
        r#"
        strict-peer-dependencies=true # comment
        save-prefix=""
        public-hoist-pattern='' ; comment
        "#,
    );

    assert_parsed_document(&document);
    assert_effective_value(&document, "strict-peer-dependencies", Some("true"));
    assert_effective_value(&document, "save-prefix", Some(""));
    assert_effective_value(&document, "public-hoist-pattern", Some(""));
}

#[test]
fn preserves_duplicate_keys_but_reports_them() {
    let document = super::super::parse_document(
        r"
        strict-peer-dependencies=true
        strict-peer-dependencies=false
        ",
    );

    assert_parsed_document(&document);
    assert_effective_value(&document, "strict-peer-dependencies", Some("false"));
    assert_duplicate_keys(&document, &["strict-peer-dependencies"]);
}

#[test]
fn rejects_lines_without_key_value_syntax() {
    let document = super::super::parse_document("not-a-setting");

    assert_invalid_document(&document, "must use key=value syntax");
}
