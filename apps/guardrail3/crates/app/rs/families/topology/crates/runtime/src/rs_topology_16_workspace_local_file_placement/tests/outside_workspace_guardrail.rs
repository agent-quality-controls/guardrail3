use guardrail3_app_rs_family_topology_assertions::rs_topology_16_workspace_local_file_placement as assertions;

use super::{check_results, entry, tree};

#[test]
fn reports_guardrail_policy_file_outside_every_legal_workspace() {
    let tree = tree(
        &[
            ("", entry(&["apps", "tools"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
            ("tools", entry(&["helper"], &[])),
            ("tools/helper", entry(&[], &["guardrail3.toml"])),
        ],
        &[
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = []\nresolver = \"2\"\n",
            ),
            (
                "tools/helper/guardrail3.toml",
                "[rust.checks]\ncargo = true\ndeps = true\ngarde = true\n",
            ),
        ],
    );

    let results = check_results(&tree);

    assertions::assert_error_files(&results, &["tools/helper/guardrail3.toml"]);
}
