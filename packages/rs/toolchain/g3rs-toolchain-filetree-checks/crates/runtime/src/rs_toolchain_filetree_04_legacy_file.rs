use g3rs_toolchain_types::G3RsToolchainFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-TOOLCHAIN-FILETREE-04";

pub(crate) fn check(input: &G3RsToolchainFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    if let (Some(legacy_rel_path), Some(_modern_rel_path)) = (
        input.legacy_toolchain_rel_path.as_deref(),
        input.toolchain_toml_rel_path.as_deref(),
    ) {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "both rust-toolchain files present".to_owned(),
            "Remove the legacy `rust-toolchain` file. rustup reads it instead of `rust-toolchain.toml` when both exist, so your modern config is ignored.".to_owned(),
            Some(legacy_rel_path.to_owned()),
            None,
        ));
    } else if let Some(legacy_rel_path) = input.legacy_toolchain_rel_path.as_deref() {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "legacy rust-toolchain file present".to_owned(),
            "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly.".to_owned(),
            Some(legacy_rel_path.to_owned()),
            None,
        ));
    }
}

#[cfg(test)]
#[path = "rs_toolchain_filetree_04_legacy_file_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rs_toolchain_filetree_04_legacy_file_tests;
