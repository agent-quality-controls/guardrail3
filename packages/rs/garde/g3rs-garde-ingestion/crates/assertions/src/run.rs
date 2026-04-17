use guardrail3_check_types::G3CheckResult;

pub fn assert_missing_garde_dependency(results: &[G3CheckResult]) {
    g3rs_garde_config_checks_assertions::rs_garde_config_01_dependency_present::rule::assert_contains(
        results,
        g3rs_garde_config_checks_assertions::rs_garde_config_01_dependency_present::rule::error(
            "garde dependency missing",
            "Missing `garde` dependency in `Cargo.toml`. Add `garde` to `[dependencies]` or `[workspace.dependencies]` in this Cargo.toml.",
            "Cargo.toml",
        ),
    );
}

pub fn assert_missing_clippy_config_warnings(results: &[G3CheckResult]) {
    g3rs_garde_config_checks_assertions::run::assert_missing_clippy_config_warnings(results);
}

pub fn assert_invalid_clippy_config_warnings(results: &[G3CheckResult]) {
    for (id, title) in [
        ("RS-GARDE-CONFIG-02", "cannot verify core garde method bans"),
        ("RS-GARDE-CONFIG-03", "cannot verify garde extractor bans"),
        ("RS-GARDE-CONFIG-04", "cannot verify reqwest garde ban"),
        ("RS-GARDE-CONFIG-05", "cannot verify additional garde method bans"),
    ] {
        assert!(
            results.iter().any(|result| {
                result.id() == id
                    && result.title() == title
                    && result.file() == Some("clippy.toml")
                    && result
                        .message()
                        .contains("Failed to parse `clippy.toml` for garde clippy-ban validation:")
            }),
            "{results:#?}"
        );
    }
}

pub fn assert_no_results(results: &[G3CheckResult]) {
    assert!(results.is_empty(), "{results:#?}");
}

pub fn assert_rule_present(results: &[G3CheckResult], id: &str, file: &str) {
    assert!(
        results
            .iter()
            .any(|result| result.id() == id && result.file() == Some(file)),
        "{results:#?}"
    );
}

pub fn assert_rule_absent(results: &[G3CheckResult], id: &str, title: &str) {
    assert!(
        results
            .iter()
            .all(|result| !(result.id() == id && result.title() == title)),
        "{results:#?}"
    );
}

pub fn assert_rule_id_absent(results: &[G3CheckResult], id: &str) {
    assert!(results.iter().all(|result| result.id() != id), "{results:#?}");
}
