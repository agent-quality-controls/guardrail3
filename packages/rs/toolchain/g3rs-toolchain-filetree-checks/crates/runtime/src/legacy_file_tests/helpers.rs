use g3rs_toolchain_types::G3RsToolchainFileTreeChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::legacy_file as rule;

pub(super) fn input(
    toolchain_toml_rel_path: Option<&str>,
    legacy_toolchain_rel_path: Option<&str>,
) -> G3RsToolchainFileTreeChecksInput {
    G3RsToolchainFileTreeChecksInput {
        toolchain_toml_rel_path: toolchain_toml_rel_path.map(str::to_owned),
        legacy_toolchain_rel_path: legacy_toolchain_rel_path.map(str::to_owned),
    }
}

pub(super) fn run_check(
    toolchain_toml_rel_path: Option<&str>,
    legacy_toolchain_rel_path: Option<&str>,
) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    rule::check(
        &input(toolchain_toml_rel_path, legacy_toolchain_rel_path),
        &mut results,
    );
    results
}
