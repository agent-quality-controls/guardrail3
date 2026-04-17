use crate::rs_clippy_config_20_forbid_clippy_conf_dir_override::check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_20_forbid_clippy_conf_dir_override as assertions;
use g3rs_clippy_types::G3RsClippyRustPolicyState;
use test_support::{cargo_config, input_with_raw, missing_cargo_root, parse_error_cargo_config};

#[test]
fn inventories_clean_state_when_no_overrides_exist() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        G3RsClippyRustPolicyState::Missing,
        missing_cargo_root(),
        Vec::new(),
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assertions::assert_no_overrides_inventory(&results);
}

#[test]
fn errors_on_override_surface() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        G3RsClippyRustPolicyState::Missing,
        missing_cargo_root(),
        Vec::new(),
        vec![cargo_config(
            ".cargo/config.toml",
            "[env]\nCLIPPY_CONF_DIR = \"config/clippy\"\n",
        )],
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        assertions::findings(&results),
        vec![assertions::error(
            "clippy config dir override is forbidden",
            "`.cargo/config.toml` sets `CLIPPY_CONF_DIR`, which bypasses the routed clippy policy-root model. Remove the `CLIPPY_CONF_DIR` setting from `.cargo/config.toml`.",
            ".cargo/config.toml",
            false,
        )]
    );
}

#[test]
fn errors_on_malformed_override_surface() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        G3RsClippyRustPolicyState::Missing,
        missing_cargo_root(),
        Vec::new(),
        vec![parse_error_cargo_config(".cargo/config", "bad env")],
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        assertions::findings(&results),
        vec![assertions::error(
            "cargo config override surface is not parseable",
            "Failed to parse `.cargo/config` while checking for forbidden `CLIPPY_CONF_DIR` overrides: bad env",
            ".cargo/config",
            false,
        )]
    );
}
