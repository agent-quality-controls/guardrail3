use super::{check_results, entry, tree};
use guardrail3_app_rs_family_topology_assertions::rs_topology_08_auxiliary_roots_declared as assertions;

#[test]
fn no_auxiliary_info_results_when_no_auxiliary_roots_exist() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
        ],
        &[(
            "apps/backend/Cargo.toml",
            "[workspace]\nmembers = []\nresolver = \"2\"\n",
        )],
    ));

    assertions::assert_no_info_files(&results, "RS-TOPOLOGY-08");
}

#[test]
fn governed_roots_with_auxiliary_metadata_do_not_emit_auxiliary_info() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
        ],
        &[(
            "apps/backend/Cargo.toml",
            "[workspace]\nmembers = []\nresolver = \"2\"\n\n[workspace.metadata.guardrail3]\ntopology_role = \"auxiliary\"\n",
        )],
    ));

    assertions::assert_no_info_files(&results, "RS-TOPOLOGY-08");
}
