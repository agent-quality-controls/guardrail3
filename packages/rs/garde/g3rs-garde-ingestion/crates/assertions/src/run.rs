use guardrail3_check_types::G3CheckResult;

pub fn assert_missing_garde_dependency(results: &[G3CheckResult]) {
    g3rs_garde_config_checks_assertions::dependency_present::rule::assert_contains(
        results,
        g3rs_garde_config_checks_assertions::dependency_present::rule::error(
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
        (
            "g3rs-garde/core-method-bans",
            "cannot verify core garde method bans",
        ),
        (
            "g3rs-garde/extractor-type-bans",
            "cannot verify garde extractor bans",
        ),
        (
            "g3rs-garde/reqwest-json-ban",
            "cannot verify reqwest garde ban",
        ),
        (
            "g3rs-garde/additional-method-bans",
            "cannot verify additional garde method bans",
        ),
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
    assert!(
        results.iter().all(|result| result.id() != id),
        "{results:#?}"
    );
}
