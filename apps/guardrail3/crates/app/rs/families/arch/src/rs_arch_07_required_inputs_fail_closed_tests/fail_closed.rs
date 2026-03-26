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
