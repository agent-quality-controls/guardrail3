use eslint_directive_parser_runtime_assertions::parser::{
    assert_ambiguous_document, assert_directive, assert_directive_count, assert_parse_error_document,
    assert_parsed_document, assert_state_reason_contains, assert_unsupported_document,
    EslintDirectiveKind, EslintDisabledRuleSet,
};

#[test]
fn parses_eslint_directives_from_supported_comment_forms() {
    let document = super::super::parse_document(
        r"
const a = 1; // eslint-disable-line no-console
// eslint-disable-next-line @typescript-eslint/no-explicit-any, no-alert -- migration
const b = 2;
/*
 * eslint-disable astro-pipeline/no-inline-public-content
 */
<!--
eslint-enable astro-pipeline/no-inline-public-content
-->
",
        "src/index.ts",
    )
    .expect("directive document should parse");

    assert_parsed_document(&document);
    assert_directive_count(&document, 4);
    assert_directive(
        &document,
        0,
        EslintDirectiveKind::DisableLine,
        2,
        Some(2),
        &EslintDisabledRuleSet::Rules(vec!["no-console".to_owned()]),
    );
    assert_directive(
        &document,
        1,
        EslintDirectiveKind::DisableNextLine,
        3,
        Some(4),
        &EslintDisabledRuleSet::Rules(vec![
            "@typescript-eslint/no-explicit-any".to_owned(),
            "no-alert".to_owned(),
        ]),
    );
    assert_directive(
        &document,
        2,
        EslintDirectiveKind::Disable,
        6,
        None,
        &EslintDisabledRuleSet::Rules(vec![
            "astro-pipeline/no-inline-public-content".to_owned(),
        ]),
    );
    assert_directive(
        &document,
        3,
        EslintDirectiveKind::Enable,
        9,
        None,
        &EslintDisabledRuleSet::Rules(vec![
            "astro-pipeline/no-inline-public-content".to_owned(),
        ]),
    );
}

#[test]
fn empty_rule_list_targets_all_rules() {
    let document = super::super::parse_document("// eslint-disable\n", "src/index.ts")
        .expect("directive document should parse");

    assert_directive(
        &document,
        0,
        EslintDirectiveKind::Disable,
        1,
        None,
        &EslintDisabledRuleSet::AllRules,
    );
}

#[test]
fn ignores_directives_inside_strings() {
    let document = super::super::parse_document(
        r#"
const a = "// eslint-disable no-console";
const b = '/* eslint-disable no-alert */';
"#,
        "src/index.ts",
    )
    .expect("directive document should parse");

    assert_parsed_document(&document);
    assert_directive_count(&document, 0);
}

#[test]
fn skips_directives_inside_template_literal_bodies() {
    let document = super::super::parse_document(
        "const a = `// eslint-disable no-console`;\n",
        "src/index.ts",
    )
    .expect("directive document should parse");

    assert_parsed_document(&document);
    assert_directive_count(&document, 0);
}

#[test]
fn parses_directives_inside_template_expression_comments() {
    let document = super::super::parse_document(
        "const a = `${/* eslint-disable astro-pipeline/no-inline-public-content */ value}`;\n",
        "src/index.ts",
    )
    .expect("directive document should parse");

    assert_parsed_document(&document);
    assert_directive(
        &document,
        0,
        EslintDirectiveKind::Disable,
        1,
        None,
        &EslintDisabledRuleSet::Rules(vec![
            "astro-pipeline/no-inline-public-content".to_owned(),
        ]),
    );
}

#[test]
fn ignores_marker_mentions_that_are_not_directive_prefixes() {
    let document = super::super::parse_document(
        "// TODO: eslint-disable was removed\n// not-eslint-disable no-console\n",
        "src/index.ts",
    )
    .expect("directive document should parse");

    assert_parsed_document(&document);
    assert_directive_count(&document, 0);
}

#[test]
fn parses_inline_eslint_config_directives() {
    let document = super::super::parse_document(
        "/* eslint astro-pipeline/no-inline-public-content: \"off\", no-console: \"warn\" */\n",
        "src/index.ts",
    )
    .expect("directive document should parse");

    assert_directive(
        &document,
        0,
        EslintDirectiveKind::InlineConfig,
        1,
        None,
        &EslintDisabledRuleSet::Rules(vec![
            "astro-pipeline/no-inline-public-content".to_owned(),
            "no-console".to_owned(),
        ]),
    );
}

#[test]
fn multiline_disable_next_line_targets_after_comment_end() {
    let document = super::super::parse_document(
        "/*\n * eslint-disable-next-line astro-pipeline/no-inline-public-content\n */\nconst value = 1;\n",
        "src/index.ts",
    )
    .expect("directive document should parse");

    assert_directive(
        &document,
        0,
        EslintDirectiveKind::DisableNextLine,
        2,
        Some(4),
        &EslintDisabledRuleSet::Rules(vec![
            "astro-pipeline/no-inline-public-content".to_owned(),
        ]),
    );
}

#[test]
fn astro_template_text_is_not_scanned_as_line_comment() {
    let document = super::super::parse_document(
        "---\nconst value = 1;\n---\n<div>// eslint-disable no-console</div>\n<!-- eslint-disable astro-pipeline/no-inline-public-content -->\n",
        "src/pages/index.astro",
    )
    .expect("directive document should parse");

    assert_directive_count(&document, 1);
    assert_directive(
        &document,
        0,
        EslintDirectiveKind::Disable,
        5,
        None,
        &EslintDisabledRuleSet::Rules(vec![
            "astro-pipeline/no-inline-public-content".to_owned(),
        ]),
    );
}

#[test]
fn regex_escaped_comment_text_is_not_scanned_as_comment() {
    let document = super::super::parse_document(
        r"const pattern = /\/\/ eslint-disable astro-pipeline\/x/;",
        "src/index.ts",
    )
    .expect("directive document should parse");

    assert_directive_count(&document, 0);
}

#[test]
fn unsupported_file_extension_is_retained() {
    let document = super::super::parse_document("// eslint-disable no-console\n", "README.md")
        .expect("directive document should parse");

    assert_unsupported_document(&document);
    assert_state_reason_contains(&document, "unsupported");
}

#[test]
fn mdx_directive_is_ambiguous() {
    let document = super::super::parse_document(
        "{/* eslint-disable no-console */}\n",
        "src/content/post.mdx",
    )
    .expect("directive document should parse");

    assert_ambiguous_document(&document);
    assert_state_reason_contains(&document, "MDX");
}

#[test]
fn mdx_prose_mention_without_comment_is_not_ambiguous() {
    let document = super::super::parse_document(
        "A paragraph mentioning eslint-disable in prose.\n",
        "src/content/post.mdx",
    )
    .expect("directive document should parse");

    assert_parsed_document(&document);
    assert_directive_count(&document, 0);
}

#[test]
fn malformed_directive_fails_closed() {
    for source in [
        "// eslint-disable no-console,\n",
        "// eslint-disable , astro-pipeline/no-inline-public-content\n",
        "// eslint-disable no-console, , astro-pipeline/no-inline-public-content\n",
        "// eslint-disable no-console astro-pipeline/no-inline-public-content\n",
    ] {
        let document = super::super::parse_document(source, "src/index.ts")
            .expect("directive document should parse");

        assert_parse_error_document(&document);
        assert_state_reason_contains(&document, "malformed");
    }
}

#[test]
fn unterminated_block_comment_fails_closed() {
    let document = super::super::parse_document("/* eslint-disable no-console\n", "src/index.ts")
        .expect("directive document should parse");

    assert_parse_error_document(&document);
    assert_state_reason_contains(&document, "unterminated");
}
