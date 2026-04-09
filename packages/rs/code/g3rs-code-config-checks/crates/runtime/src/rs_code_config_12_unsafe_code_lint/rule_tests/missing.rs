use g3rs_code_config_checks_types::G3RsCodeUnsafeCodeLintFact;

use super::helpers::run_check;

#[test]
fn stays_clean_for_missing_or_other_levels() {
    let results = run_check(vec![
        G3RsCodeUnsafeCodeLintFact {
            cargo_rel_path: "Cargo.toml".to_owned(),
            lint_level: None,
        },
        G3RsCodeUnsafeCodeLintFact {
            cargo_rel_path: "nested/Cargo.toml".to_owned(),
            lint_level: Some("warn".to_owned()),
        },
    ]);

    assert!(results.is_empty(), "unexpected results: {results:#?}");
}
