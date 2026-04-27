use guardrail3_check_types::G3CheckResult;

pub fn assert_missing_clippy_config_warnings(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-garde/core-method-bans"
                && result.title() == "cannot verify core garde method bans"
                && result.message()
                    == "No clippy.toml found. Create one with a `disallowed-methods` section."
                && result.file().is_none()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-garde/extractor-type-bans"
                && result.title() == "cannot verify garde extractor bans"
                && result.message()
                    == "No clippy.toml found. Create one with a `disallowed-types` section."
                && result.file().is_none()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-garde/reqwest-json-ban"
                && result.title() == "cannot verify reqwest garde ban"
                && result.message()
                    == "No clippy.toml found. Create one with a `disallowed-methods` section."
                && result.file().is_none()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-garde/additional-method-bans"
                && result.title() == "cannot verify additional garde method bans"
                && result.message()
                    == "No clippy.toml found. Create one with a `disallowed-methods` section."
                && result.file().is_none()
        }),
        "{results:#?}"
    );
}

pub fn assert_invalid_clippy_config_warnings(results: &[G3CheckResult]) {
    assert_missing_verification(
        results,
        "Failed to parse `clippy.toml` for garde clippy-ban validation: invalid clippy.toml",
        Some("clippy.toml"),
    );
}

pub fn assert_dependency_missing_without_ban_results(results: &[G3CheckResult]) {
    crate::dependency_present::assert_contains(
        results,
        crate::dependency_present::error(
            "garde dependency missing",
            "Missing `garde` dependency in `Cargo.toml`. Add `garde` to `[dependencies]` or `[workspace.dependencies]` in this Cargo.toml.",
            "Cargo.toml",
        ),
    );
    assert_no_ban_rule_results(results);
}

pub fn assert_garde_absent_without_ban_results(results: &[G3CheckResult]) {
    crate::dependency_present::assert_contains(
        results,
        crate::dependency_present::error(
            "garde dependency missing",
            "Missing `garde` dependency in `Cargo.toml`. Add `garde` to `[dependencies]` or `[workspace.dependencies]` in this Cargo.toml.",
            "Cargo.toml",
        ),
    );
    assert_no_ban_rule_results(results);
}

pub fn assert_no_results(results: &[G3CheckResult]) {
    assert!(results.is_empty(), "{results:#?}");
}

fn assert_missing_verification(results: &[G3CheckResult], message: &str, file: Option<&str>) {
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
                    && result.message() == message
                    && result.file() == file
            }),
            "{results:#?}"
        );
    }
}

fn assert_no_ban_rule_results(results: &[G3CheckResult]) {
    for id in [
        "g3rs-garde/core-method-bans",
        "g3rs-garde/extractor-type-bans",
        "g3rs-garde/reqwest-json-ban",
        "g3rs-garde/additional-method-bans",
    ] {
        assert!(
            results.iter().all(|result| result.id() != id),
            "{results:#?}"
        );
    }
}
