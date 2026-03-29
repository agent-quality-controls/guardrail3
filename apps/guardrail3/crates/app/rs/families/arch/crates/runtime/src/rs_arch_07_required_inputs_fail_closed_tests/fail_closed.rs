use super::{check_results, entry, tree};
use guardrail3_app_rs_family_arch_assertions::rs_arch_07_required_inputs_fail_closed as assertions;

#[test]
fn malformed_guardrail_config_emits_required_input_failure() {
    let results = check_results(&tree(
        &[("", entry(&["apps"], &["guardrail3.toml"]))],
        &[("guardrail3.toml", "[rust.checks]\nlibarch = \"nope\"\n")],
    ));

    assertions::assert_error_files(&results, "RS-ARCH-07", &["guardrail3.toml"]);
}

#[test]
fn unreadable_present_guardrail_config_emits_required_input_failure() {
    let results = check_results(&tree(&[("", entry(&["apps"], &["guardrail3.toml"]))], &[]));

    assertions::assert_error_files(&results, "RS-ARCH-07", &["guardrail3.toml"]);
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

    assertions::assert_error_files(&results, "RS-ARCH-07", &["apps/backend/Cargo.toml"]);
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
            "[package]\nname = \"fuzz\"\n\n[package.metadata.guardrail3]\narch_role = \"sidecar\"\n",
        )],
    ));

    assertions::assert_error_files(&results, "RS-ARCH-07", &["fuzz/Cargo.toml"]);
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

    assertions::assert_error_files(&results, "RS-ARCH-07", &["tools/worker/Cargo.toml"]);
}

#[test]
fn malformed_app_owned_cargo_toml_does_not_emit_required_input_failure() {
    let results = check_results(&tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&[], &["Cargo.toml"])),
        ],
        &[("apps/backend/Cargo.toml", "[workspace\nmembers = []\n")],
    ));

    assertions::assert_no_error_files(&results, "RS-ARCH-07");
}
