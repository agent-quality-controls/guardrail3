use super::super::super::test_support::{assert_error_files, check_results, entry, tree};

#[test]
fn malformed_guardrail_config_emits_required_input_failure() {
    let results = check_results(&tree(
        &[("", entry(&["apps"], &["guardrail3.toml"]))],
        &[("guardrail3.toml", "[rust.checks]\nlibarch = \"nope\"\n")],
    ));

    assert_error_files(&results, "RS-ARCH-07", &["guardrail3.toml"]);
}

#[test]
fn unreadable_present_guardrail_config_emits_required_input_failure() {
    let results = check_results(&tree(&[("", entry(&["apps"], &["guardrail3.toml"]))], &[]));

    assert_error_files(&results, "RS-ARCH-07", &["guardrail3.toml"]);
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

    assert_error_files(&results, "RS-ARCH-07", &["apps/backend/Cargo.toml"]);
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

    assert_error_files(&results, "RS-ARCH-07", &["fuzz/Cargo.toml"]);
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

    assert_error_files(&results, "RS-ARCH-07", &["tools/worker/Cargo.toml"]);
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

    assert!(
        super::super::super::test_support::error_results(&results, "RS-ARCH-07").is_empty(),
        "app-owned roots should classify by path without forcing Cargo.toml parse for auxiliary metadata: {results:#?}"
    );
}
