pub fn assert_missing_modern_toolchain(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_01_exists::assert_findings(
        results,
        &[g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_01_exists::error(
            "rust-toolchain.toml missing",
            "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.",
            "rust-toolchain.toml",
            false,
        )],
    );
}

pub fn assert_legacy_only_without_modern(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_01_exists::assert_findings(
        results,
        &[g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_01_exists::error(
            "rust-toolchain.toml missing",
            "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.",
            "rust-toolchain.toml",
            false,
        )],
    );
    g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_04_legacy_file::assert_findings(
        results,
        &[g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_04_legacy_file::warn(
            "legacy rust-toolchain file present",
            "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly.",
            "rust-toolchain",
            false,
        )],
    );
}

pub fn assert_both_toolchain_files_present(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_01_exists::assert_findings(
        results,
        &[g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_01_exists::info(
            "rust-toolchain.toml exists",
            "Found rust-toolchain.toml at workspace root.",
            "rust-toolchain.toml",
            true,
        )],
    );
    g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_04_legacy_file::assert_findings(
        results,
        &[g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_04_legacy_file::error(
            "both rust-toolchain files present",
            "Remove the legacy `rust-toolchain` file. rustup reads it instead of `rust-toolchain.toml` when both exist, so your modern config is ignored.",
            "rust-toolchain",
            false,
        )],
    );
}

pub fn assert_modern_toolchain_exists(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_01_exists::assert_findings(
        results,
        &[g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_01_exists::info(
            "rust-toolchain.toml exists",
            "Found rust-toolchain.toml at workspace root.",
            "rust-toolchain.toml",
            true,
        )],
    );
    g3rs_toolchain_filetree_checks_assertions::rs_toolchain_filetree_04_legacy_file::assert_findings(
        results,
        &[],
    );
}

pub fn assert_nightly_toolchain_channel(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_toolchain_config_checks_assertions::rs_toolchain_config_01_channel_and_components::assert_contains(
        results,
        g3rs_toolchain_config_checks_assertions::rs_toolchain_config_01_channel_and_components::error(
            "toolchain channel is nightly",
            "Channel is set to nightly. Use `channel = \"stable\"` or a pinned stable version.",
            "rust-toolchain.toml",
            false,
        ),
    );
}
