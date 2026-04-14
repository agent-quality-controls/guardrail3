use super::check;
use crate::test_support::{findings, input_with_raw, override_facts};
use g3rs_clippy_config_checks_types::G3RsClippyRustPolicyState;

#[test]
fn inventories_clean_state_when_no_overrides_exist() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        G3RsClippyRustPolicyState::Missing,
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "no clippy config dir overrides found" && finding.inventory
    }));
}

#[test]
fn errors_on_override_surface() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        G3RsClippyRustPolicyState::Missing,
        false,
        vec![override_facts(".cargo/config.toml", None)],
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        findings(&results),
        vec![crate::test_support::Finding {
            id: "RS-CLIPPY-CONFIG-20".to_owned(),
            severity: guardrail3_check_types::G3Severity::Error,
            title: "clippy config dir override is forbidden".to_owned(),
            message: "`.cargo/config.toml` sets `CLIPPY_CONF_DIR`, which bypasses the routed clippy policy-root model. Remove the `CLIPPY_CONF_DIR` setting from `.cargo/config.toml`.".to_owned(),
            file: Some(".cargo/config.toml".to_owned()),
            inventory: false,
        }]
    );
}

#[test]
fn errors_on_malformed_override_surface() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        G3RsClippyRustPolicyState::Missing,
        false,
        vec![override_facts(".cargo/config", Some("bad env"))],
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        findings(&results),
        vec![crate::test_support::Finding {
            id: "RS-CLIPPY-CONFIG-20".to_owned(),
            severity: guardrail3_check_types::G3Severity::Error,
            title: "cargo config override surface is not parseable".to_owned(),
            message: "Failed to parse `.cargo/config` while checking for forbidden `CLIPPY_CONF_DIR` overrides: bad env".to_owned(),
            file: Some(".cargo/config".to_owned()),
            inventory: false,
        }]
    );
}
