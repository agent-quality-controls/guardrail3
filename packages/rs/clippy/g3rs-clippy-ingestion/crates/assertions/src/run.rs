pub fn assert_bad_line_threshold(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_clippy_config_checks_assertions::rs_clippy_config_03_too_many_lines_threshold::assert_findings(
        results,
        &[g3rs_clippy_config_checks_assertions::rs_clippy_config_03_too_many_lines_threshold::error(
            "too-many-lines-threshold wrong value",
            "Expected 75, got 1. Set `too-many-lines-threshold = 75` in clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}

pub fn assert_same_root_conflict(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_clippy_filetree_checks_assertions::rs_clippy_filetree_01_coverage_exists::assert_findings(
        results,
        &[g3rs_clippy_filetree_checks_assertions::rs_clippy_filetree_01_coverage_exists::info(
            "workspace root covered by clippy config",
            "Workspace root is covered by `.clippy.toml`.",
            ".clippy.toml",
            true,
        )],
    );
    g3rs_clippy_filetree_checks_assertions::rs_clippy_filetree_02_same_root_conflict::assert_findings(
        results,
        &[g3rs_clippy_filetree_checks_assertions::rs_clippy_filetree_02_same_root_conflict::error(
            "same-root clippy config conflict",
            "`clippy.toml` conflicts with `.clippy.toml` at the same policy root. Keep only the highest-precedence clippy config file.",
            "clippy.toml",
            false,
        )],
    );
}

pub fn assert_library_profile_warning(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_clippy_config_checks_assertions::rs_clippy_config_15_avoid_breaking_exported_api::assert_findings(
        results,
        &[g3rs_clippy_config_checks_assertions::rs_clippy_config_15_avoid_breaking_exported_api::warn(
            "avoid-breaking-exported-api enabled",
            "`avoid-breaking-exported-api = true` suppresses useful lints. Prefer `false`.",
            "clippy.toml",
            false,
        )],
    );
}

pub fn assert_override_surface_parse_error(results: &[guardrail3_check_types::G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-CLIPPY-CONFIG-20"
                && result.title() == "cargo config override surface is not parseable"
                && result.file() == Some(".cargo/config.toml")
                && result.message().contains("Failed to parse `.cargo/config.toml`")
        }),
        "{results:#?}"
    );
}

pub fn assert_config_parse_error_contains(
    results: &[guardrail3_check_types::G3CheckResult],
    needle: &str,
) {
    g3rs_clippy_config_checks_assertions::rs_clippy_config_21_config_parseable::assert_parse_error_contains(
        results, needle,
    );
}
