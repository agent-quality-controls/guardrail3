use guardrail3_app_rs_family_arch_assertions::rs_arch_16_workspace_local_file_placement as assertions;

use super::{check_results, entry, tree};

#[test]
fn reports_repo_root_toolchain_above_nested_workspace() {
    let tree = tree(
        &[
            ("", entry(&["apps"], &["guardrail3.toml", "rust-toolchain.toml"])),
            ("apps", entry(&["api"], &[])),
            ("apps/api", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", "[rust.checks]\narch = true\ntoolchain = true\n"),
            (
                "rust-toolchain.toml",
                "[toolchain]\nchannel = \"1.87.0\"\ncomponents = [\"rustfmt\", \"clippy\"]\n",
            ),
            ("apps/api/Cargo.toml", "[workspace]\nmembers = []\nresolver = \"2\"\n"),
        ],
    );

    let results = check_results(&tree);

    assertions::assert_error_files(&results, &["rust-toolchain.toml"]);
}
