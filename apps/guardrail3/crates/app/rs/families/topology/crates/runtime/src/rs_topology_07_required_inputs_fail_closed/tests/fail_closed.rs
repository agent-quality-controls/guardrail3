use super::{check_results, entry, tree};
use guardrail3_app_rs_family_topology_assertions::rs_topology_07_required_inputs_fail_closed as assertions;

#[test]
fn malformed_guardrail_config_emits_required_input_failure() {
    let results = check_results(&tree(
        &[("", entry(&["apps"], &["guardrail3.toml"]))],
        &[("guardrail3.toml", "[rust.checks]\nlibarch = \"nope\"\n")],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-07", &["guardrail3.toml"]);
}

#[test]
fn unreadable_present_guardrail_config_emits_required_input_failure() {
    let results = check_results(&tree(&[("", entry(&["apps"], &["guardrail3.toml"]))], &[]));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-07", &["guardrail3.toml"]);
}

#[test]
fn missing_cargo_content_emits_required_input_failure() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
        ],
        &[],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-07", &["apps/backend/Cargo.toml"]);
}

#[test]
fn malformed_auxiliary_metadata_emits_required_input_failure() {
    let results = check_results(&tree(
        &[
            ("", entry(&["fuzz"], &[])),
            ("fuzz", entry(&[], &["Cargo.toml"])),
        ],
        &[(
            "fuzz/Cargo.toml",
            "[package]\nname = \"fuzz\"\n\n[package.metadata.guardrail3]\ntopology_role = \"sidecar\"\n",
        )],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-07", &["fuzz/Cargo.toml"]);
}

#[test]
fn malformed_auxiliary_candidate_cargo_toml_emits_required_input_failure() {
    let results = check_results(&tree(
        &[
            ("", entry(&["tools"], &[])),
            ("tools", entry(&["worker"], &[])),
            ("tools/worker", entry(&[], &["Cargo.toml"])),
        ],
        &[("tools/worker/Cargo.toml", "[package\nname = \"worker\"\n")],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-07", &["tools/worker/Cargo.toml"]);
}

#[test]
fn malformed_app_owned_cargo_toml_emits_required_input_failure() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
        ],
        &[("apps/backend/Cargo.toml", "[workspace\nmembers = []\n")],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-07", &["apps/backend/Cargo.toml"]);
}

#[test]
fn malformed_package_owned_cargo_toml_emits_required_input_failure() {
    let results = check_results(&tree(
        &[
            ("", entry(&["packages"], &[])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[(
            "packages/shared/Cargo.toml",
            "[package\nname = \"shared\"\n",
        )],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-07", &["packages/shared/Cargo.toml"]);
}

#[test]
fn governed_root_declaring_auxiliary_metadata_emits_required_input_failure() {
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

    assertions::assert_error_files(&results, "RS-TOPOLOGY-07", &["apps/backend/Cargo.toml"]);
}

#[test]
fn governed_package_root_declaring_auxiliary_metadata_emits_required_input_failure() {
    let results = check_results(&tree(
        &[
            ("", entry(&["packages"], &[])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[(
            "packages/shared/Cargo.toml",
            "[package]\nname = \"shared\"\nedition = \"2024\"\n\n[package.metadata.guardrail3]\ntopology_role = \"auxiliary\"\n",
        )],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-07", &["packages/shared/Cargo.toml"]);
}

#[test]
fn governed_package_workspace_metadata_declaring_auxiliary_emits_required_input_failure() {
    let results = check_results(&tree(
        &[
            ("", entry(&["packages"], &[])),
            ("packages", entry(&["shared"], &[])),
            ("packages/shared", entry(&[], &["Cargo.toml"])),
        ],
        &[(
            "packages/shared/Cargo.toml",
            "[workspace]\nmembers = []\nresolver = \"2\"\n\n[workspace.metadata.guardrail3]\ntopology_role = \"auxiliary\"\n",
        )],
    ));

    assertions::assert_error_files(&results, "RS-TOPOLOGY-07", &["packages/shared/Cargo.toml"]);
}
