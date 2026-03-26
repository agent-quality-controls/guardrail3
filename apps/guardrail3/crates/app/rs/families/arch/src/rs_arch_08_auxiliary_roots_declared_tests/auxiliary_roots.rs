use super::super::super::test_support::{assert_info_files, check_results, entry, tree};

#[test]
fn declared_auxiliary_roots_are_reported_as_info() {
    let results = check_results(&tree(
        &[
            ("", entry(&["fuzz"], &[])),
            ("fuzz", entry(&[], &["Cargo.toml"])),
        ],
        &[(
            "fuzz/Cargo.toml",
            "[package]\nname = \"fuzz\"\n\n[package.metadata.guardrail3]\narch_role = \"auxiliary\"\n",
        )],
    ));

    assert_info_files(&results, "RS-ARCH-08", &["fuzz/Cargo.toml"]);
}

#[test]
fn workspace_level_auxiliary_metadata_is_reported_as_info() {
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &[])),
            ("tools", entry(&["xtask"], &[])),
            ("tools/xtask", entry(&[], &["Cargo.toml"])),
        ],
        &[(
            "tools/xtask/Cargo.toml",
            "[workspace]\nmembers = []\n\n[workspace.metadata.guardrail3]\narch_role = \"auxiliary\"\n",
        )],
    ));

    assert_info_files(&results, "RS-ARCH-08", &["tools/xtask/Cargo.toml"]);
}
