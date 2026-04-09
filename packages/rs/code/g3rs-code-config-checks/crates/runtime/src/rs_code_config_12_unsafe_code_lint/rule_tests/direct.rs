use g3rs_code_config_checks_assertions::common::require_single_result;
use g3rs_code_config_checks_assertions::rs_code_config_12_unsafe_code_lint::{
    assert_deny_error, assert_forbid_inventory_info,
};
use g3rs_code_config_checks_types::{G3RsCodeConfigChecksInput, G3RsCodeUnsafeCodeLintFact};

#[test]
fn emits_inventory_info_for_forbid() {
    let input = G3RsCodeConfigChecksInput {
        exception_comments: Vec::new(),
        unsafe_code_lints: vec![G3RsCodeUnsafeCodeLintFact {
            cargo_rel_path: "Cargo.toml".to_owned(),
            lint_level: Some("forbid".to_owned()),
        }],
    };

    let results = crate::run::check(&input);
    let result = require_single_result(&results);
    assert_forbid_inventory_info(result, "Cargo.toml");
}

#[test]
fn emits_error_for_deny() {
    let input = G3RsCodeConfigChecksInput {
        exception_comments: Vec::new(),
        unsafe_code_lints: vec![G3RsCodeUnsafeCodeLintFact {
            cargo_rel_path: "Cargo.toml".to_owned(),
            lint_level: Some("deny".to_owned()),
        }],
    };

    let results = crate::run::check(&input);
    let result = require_single_result(&results);
    assert_deny_error(result, "Cargo.toml");
}

#[test]
fn stays_clean_for_missing_or_other_levels() {
    let input = G3RsCodeConfigChecksInput {
        exception_comments: Vec::new(),
        unsafe_code_lints: vec![
            G3RsCodeUnsafeCodeLintFact {
                cargo_rel_path: "Cargo.toml".to_owned(),
                lint_level: None,
            },
            G3RsCodeUnsafeCodeLintFact {
                cargo_rel_path: "nested/Cargo.toml".to_owned(),
                lint_level: Some("warn".to_owned()),
            },
        ],
    };

    let results = crate::run::check(&input);
    assert!(results.is_empty(), "unexpected results: {results:#?}");
}
