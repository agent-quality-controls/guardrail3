pub fn assert_modern_only(results: &[guardrail3_check_types::G3CheckResult]) {
    crate::exists::assert_findings(
        results,
        &[crate::exists::info(
            "rust-toolchain.toml exists",
            "Found rust-toolchain.toml at workspace root.",
            "rust-toolchain.toml",
            true,
        )],
    );
    crate::legacy_file::assert_findings(results, &[]);
}

pub fn assert_legacy_only_without_modern(results: &[guardrail3_check_types::G3CheckResult]) {
    crate::exists::assert_findings(
        results,
        &[crate::exists::error(
            "rust-toolchain.toml missing",
            "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.",
            "rust-toolchain.toml",
            false,
        )],
    );
    crate::legacy_file::assert_findings(
        results,
        &[crate::legacy_file::warn(
            "legacy rust-toolchain file present",
            "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly.",
            "rust-toolchain",
            false,
        )],
    );
}

pub fn assert_both_files_present(results: &[guardrail3_check_types::G3CheckResult]) {
    crate::exists::assert_findings(
        results,
        &[crate::exists::info(
            "rust-toolchain.toml exists",
            "Found rust-toolchain.toml at workspace root.",
            "rust-toolchain.toml",
            true,
        )],
    );
    crate::legacy_file::assert_findings(
        results,
        &[crate::legacy_file::error(
            "both rust-toolchain files present",
            "Remove the legacy `rust-toolchain` file. rustup reads it instead of `rust-toolchain.toml` when both exist, so your modern config is ignored.",
            "rust-toolchain",
            false,
        )],
    );
}

pub fn assert_missing_modern_only(results: &[guardrail3_check_types::G3CheckResult]) {
    crate::exists::assert_findings(
        results,
        &[crate::exists::error(
            "rust-toolchain.toml missing",
            "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.",
            "rust-toolchain.toml",
            false,
        )],
    );
    crate::legacy_file::assert_findings(results, &[]);
}
