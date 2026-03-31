use guardrail3_app_rs_family_arch_assertions::rs_arch_16_workspace_local_file_placement as assertions;

use super::{check_results, entry, tree};

#[test]
fn reports_nested_rustfmt_config_outside_validation_root() {
    let tree = tree(
        &[
            ("", entry(&["apps"], &["guardrail3.toml"])),
            ("apps", entry(&["api"], &[])),
            ("apps/api", entry(&[], &["Cargo.toml", "rustfmt.toml"])),
        ],
        &[
            (
                "guardrail3.toml",
                "[rust.checks]\narch = true\nfmt = true\n",
            ),
            (
                "apps/api/Cargo.toml",
                "[workspace]\nmembers = []\nresolver = \"2\"\n",
            ),
            (
                "apps/api/rustfmt.toml",
                "edition = \"2024\"\nstyle_edition = \"2024\"\n",
            ),
        ],
    );

    let results = check_results(&tree);

    assertions::assert_error_files(&results, &["apps/api/rustfmt.toml"]);
}
