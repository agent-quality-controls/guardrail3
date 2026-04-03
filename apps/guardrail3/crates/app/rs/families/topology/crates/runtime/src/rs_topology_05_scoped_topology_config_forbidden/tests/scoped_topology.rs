use super::{check_results, entry, tree};
use guardrail3_app_rs_family_topology_assertions::rs_topology_05_scoped_topology_config_forbidden as assertions;

#[test]
fn app_scoped_topology_config_is_forbidden() {
    let config = "[rust.checks]\ntopology = true\nhexarch = true\n\n[rust.apps.backend.checks]\ntopology = false\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &["guardrail3.toml"])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = []\nresolver = \"2\"\n",
            ),
        ],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-05", &["guardrail3.toml"]);
}

#[test]
fn package_scoped_topology_config_is_forbidden() {
    let config = "[rust.checks]\ntopology = true\nhexarch = true\n\n[rust.packages.checks]\ntopology = false\n";
    let results = check_results(&tree(
        &[
            ("", entry(&["packages"], &["guardrail3.toml"])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[
            ("guardrail3.toml", config),
            (
                "packages/shared/Cargo.toml",
                "[package]\nname = \"shared\"\n",
            ),
        ],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-05", &["guardrail3.toml"]);
}
