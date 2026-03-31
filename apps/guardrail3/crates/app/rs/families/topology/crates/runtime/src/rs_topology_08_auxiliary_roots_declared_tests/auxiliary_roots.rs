use super::{check_results, entry, tree};
use guardrail3_app_rs_family_topology_assertions::rs_topology_08_auxiliary_roots_declared as assertions;

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

    assertions::assert_info_files(&results, "RS-TOPOLOGY-08", &["fuzz/Cargo.toml"]);
    assertions::assert_all_info_inventory(&results, "RS-TOPOLOGY-08");
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

    assertions::assert_info_files(&results, "RS-TOPOLOGY-08", &["tools/xtask/Cargo.toml"]);
    assertions::assert_all_info_inventory(&results, "RS-TOPOLOGY-08");
}
