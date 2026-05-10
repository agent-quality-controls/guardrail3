pub fn assert_bad_line_threshold(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_clippy_config_checks_assertions::too_many_lines_threshold::assert_findings(
        results,
        &[
            g3rs_clippy_config_checks_assertions::too_many_lines_threshold::error(
                "too-many-lines-threshold wrong value",
                "Expected 75, got 1. Set `too-many-lines-threshold = 75` in clippy.toml.",
                "clippy.toml",
                false,
            ),
        ],
    );
}

pub fn assert_same_root_conflict(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_clippy_filetree_checks_assertions::run::rule::assert_same_root_conflict(results);
}

pub fn assert_library_profile_warning(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_clippy_config_checks_assertions::avoid_breaking_exported_api::assert_findings(
        results,
        &[
            g3rs_clippy_config_checks_assertions::avoid_breaking_exported_api::warn(
                "avoid-breaking-exported-api enabled",
                "`avoid-breaking-exported-api = true` suppresses useful lints. Prefer `false`.",
                "clippy.toml",
                false,
            ),
        ],
    );
}

/// Assert that the result set contains the cargo-config-override parse-error finding.
///
/// # Panics
///
/// Panics when no matching finding is present.
pub fn assert_override_surface_parse_error(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-clippy/forbid-clippy-conf-dir-override"
                && result.title() == "cargo config override surface is not parseable"
                && result.file() == Some(".cargo/config.toml")
                && result
                    .message()
                    .contains("Failed to parse `.cargo/config.toml`")
        }),
        "{results:#?}"
    );
}

pub fn assert_config_parse_error_contains(
    results: &[guardrail3_check_types::G3CheckResult],
    needle: &str,
) {
    g3rs_clippy_config_checks_assertions::config_parseable::assert_parse_error_contains(
        results, needle,
    );
}

/// Assert the input snapshot has exactly one waiver matching the expected fields.
///
/// # Panics
///
/// Panics when the waivers vec is not length 1 or any field mismatches.
pub fn assert_single_waiver(
    input: &g3rs_clippy_types::G3RsClippyConfigChecksInput,
    rule: &str,
    file: &str,
    selector: &str,
    reason: &str,
) {
    assert_eq!(input.waivers.len(), 1, "{input:#?}");
    let [waiver] = input.waivers.as_slice() else {
        return;
    };
    assert_eq!(waiver.rule, rule, "{input:#?}");
    assert_eq!(waiver.file, file, "{input:#?}");
    assert_eq!(waiver.selector, selector, "{input:#?}");
    assert_eq!(waiver.reason, reason, "{input:#?}");
}
