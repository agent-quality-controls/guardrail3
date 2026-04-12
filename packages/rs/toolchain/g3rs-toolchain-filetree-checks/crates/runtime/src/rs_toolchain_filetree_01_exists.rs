use g3rs_toolchain_filetree_checks_types::G3RsToolchainFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-TOOLCHAIN-FILETREE-01";

pub(crate) fn check(input: &G3RsToolchainFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    match input.toolchain_toml_rel_path.as_deref() {
        Some(rel_path) => results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "rust-toolchain.toml exists".to_owned(),
                "Found rust-toolchain.toml at workspace root.".to_owned(),
                Some(rel_path.to_owned()),
                None,
            )
            .into_inventory(),
        ),
        None => results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "rust-toolchain.toml missing".to_owned(),
            "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.".to_owned(),
            Some("rust-toolchain.toml".to_owned()),
            None,
        )),
    }
}

#[cfg(test)]
#[path = "rs_toolchain_filetree_01_exists_tests/mod.rs"]
mod tests;
