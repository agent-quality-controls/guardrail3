#![allow(
    clippy::indexing_slicing,
    clippy::panic,
    reason = "parser tests use direct exact-shape assertions for concise contract proofs"
)]

use clippy_toml_parser_runtime_assertions::parser as assertions;
use helpers::{parse_fixture, parse_from_tempfile};

use super::super::{
    InherentImplLintScope, MatchLintBehaviour, PubUnderscoreFieldsBehaviour,
    SourceItemOrderingWithinModuleItemGroupings,
};
use super::helpers;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse_fixture("");

    assertions::assert_thresholds_empty(&cfg);
    assertions::assert_ban_lists_empty(&cfg);
}

#[test]
fn thresholds_parse() {
    let cfg = parse_fixture(
        r"
max-struct-bools = 3
max-fn-params-bools = 3
too-many-lines-threshold = 75
too-many-arguments-threshold = 7
excessive-nesting-threshold = 4
cognitive-complexity-threshold = 15
type-complexity-threshold = 75
",
    );

    assertions::assert_thresholds(
        &cfg,
        Some(3),
        Some(3),
        Some(75),
        Some(7),
        Some(4),
        Some(15),
        Some(75),
    );
}

#[test]
fn simple_ban_entries() {
    let cfg = parse_fixture(
        r#"
disallowed-methods = ["std::env::var", "std::process::exit"]
disallowed-types = ["std::collections::HashMap"]
disallowed-macros = ["println!", "dbg!"]
"#,
    );

    assertions::assert_ban_entry(cfg.disallowed_methods.first(), "std::env::var", None);
    assertions::assert_list_len(&cfg.disallowed_methods, 2, "disallowed_methods");
    assertions::assert_list_len(&cfg.disallowed_types, 1, "disallowed_types");
    assertions::assert_list_len(&cfg.disallowed_macros, 2, "disallowed_macros");
}

#[test]
fn detailed_ban_entries_with_reason() {
    let cfg = parse_fixture(
        r#"
disallowed-methods = [
    { path = "std::env::var", reason = "Use config module", replacement = "crate::config::var", allow-invalid = true },
    "std::process::exit",
]
"#,
    );

    assertions::assert_list_len(&cfg.disallowed_methods, 2, "disallowed_methods");
    assertions::assert_first_detailed_method(
        &cfg,
        "std::env::var",
        Some("Use config module"),
        Some("crate::config::var"),
        Some(true),
    );
    assertions::assert_ban_entry(cfg.disallowed_methods.get(1), "std::process::exit", None);
}

#[test]
fn disallowed_fields_do_not_accept_replacement() {
    let err = super::super::parse(
        r#"
disallowed-fields = [
    { path = "crate::Thing::field", replacement = "crate::Thing::field()" },
]
"#,
    )
    .expect_err("replacement is not allowed for disallowed-fields");

    assertions::assert_parse_error(err);
}

#[test]
fn test_relaxations_parse() {
    let cfg = parse_fixture(
        r"
allow-dbg-in-tests = false
allow-print-in-tests = false
allow-expect-in-tests = true
allow-panic-in-tests = false
allow-unwrap-in-tests = false
",
    );

    assertions::assert_bool_field(cfg.allow_dbg_in_tests, Some(false), "allow_dbg_in_tests");
    assertions::assert_bool_field(
        cfg.allow_print_in_tests,
        Some(false),
        "allow_print_in_tests",
    );
    assertions::assert_bool_field(
        cfg.allow_expect_in_tests,
        Some(true),
        "allow_expect_in_tests",
    );
    assertions::assert_bool_field(
        cfg.allow_panic_in_tests,
        Some(false),
        "allow_panic_in_tests",
    );
    assertions::assert_bool_field(
        cfg.allow_unwrap_in_tests,
        Some(false),
        "allow_unwrap_in_tests",
    );
}

#[test]
fn exact_structured_fields_parse() {
    let cfg = parse_fixture(
        r#"
accept-comment-above-attributes = false
allow-comparison-to-zero = false
allowed-prefixes = ["to", "into"]
allowed-duplicate-crates = ["syn"]
array-size-threshold = 4096
check-private-items = true
doc-valid-idents = ["OpenAI", "GitHub"]
warn-on-all-wildcard-imports = true
matches-for-let-else = "Never"
inherent-impl-lint-scope = "file"
pub-underscore-fields-behavior = "PubliclyExported"
module-items-ordered-within-groupings = "none"
source-item-ordering = ["enum", "impl", "module", "struct", "trait"]
trait-assoc-item-kinds-order = ["const", "fn", "type"]
standard-macro-braces = [
    { name = "println", brace = "(" },
]
disallowed-fields = [
    { path = "crate::Thing::field", reason = "use accessor", allow-invalid = true },
]
enforced-import-renames = [
    { path = "std::collections::HashMap", rename = "Map" },
]
"#,
    );

    assertions::assert_bool_field(
        cfg.accept_comment_above_attributes,
        Some(false),
        "accept_comment_above_attributes",
    );
    assertions::assert_bool_field(
        cfg.allow_comparison_to_zero,
        Some(false),
        "allow_comparison_to_zero",
    );
    assertions::assert_string_list(&cfg.allowed_prefixes, &["to", "into"], "allowed_prefixes");
    assertions::assert_string_list(
        &cfg.allowed_duplicate_crates,
        &["syn"],
        "allowed_duplicate_crates",
    );
    assert_eq!(cfg.array_size_threshold, Some(4096));
    assertions::assert_bool_field(cfg.check_private_items, Some(true), "check_private_items");
    assertions::assert_string_list(
        &cfg.doc_valid_idents,
        &["OpenAI", "GitHub"],
        "doc_valid_idents",
    );
    assertions::assert_bool_field(
        cfg.warn_on_all_wildcard_imports,
        Some(true),
        "warn_on_all_wildcard_imports",
    );
    assertions::assert_matches_for_let_else(&cfg, MatchLintBehaviour::Never);
    assertions::assert_inherent_impl_lint_scope(&cfg, InherentImplLintScope::File);
    assertions::assert_pub_underscore_fields_behavior(
        &cfg,
        PubUnderscoreFieldsBehaviour::PubliclyExported,
    );
    assertions::assert_module_items_ordered_within_groupings(
        &cfg,
        SourceItemOrderingWithinModuleItemGroupings::None,
    );
    assert_eq!(cfg.standard_macro_braces.len(), 1, "standard_macro_braces");
    assert_eq!(cfg.standard_macro_braces[0].name, "println");
    assert_eq!(cfg.standard_macro_braces[0].brace, '(');
    assert_eq!(
        cfg.enforced_import_renames.len(),
        1,
        "enforced_import_renames"
    );
    assert_eq!(
        cfg.enforced_import_renames[0].path,
        "std::collections::HashMap"
    );
    assert_eq!(cfg.enforced_import_renames[0].rename, "Map");
    assertions::assert_first_detailed_field(
        &cfg,
        "crate::Thing::field",
        Some("use accessor"),
        Some(true),
    );
}

#[test]
fn unknown_keys_are_rejected() {
    let err = super::super::parse(
        r#"
max-struct-bools = 3
some-future-clippy-option = "yes"
"#,
    )
    .expect_err("unknown clippy keys should be rejected");

    assertions::assert_parse_error(err);
}

#[test]
fn invalid_macro_brace_is_rejected() {
    let err = super::super::parse(
        r#"
standard-macro-braces = [
    { name = "println", brace = "<" },
]
"#,
    )
    .expect_err("invalid brace should be rejected");

    assertions::assert_parse_error(err);
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile("max-struct-bools = 4\n");
    assertions::assert_thresholds(&cfg, Some(4), None, None, None, None, None, None);
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = super::super::parse("this is not [[[valid toml");
    assertions::assert_parse_error(err.expect_err("invalid TOML should produce an error"));
}
