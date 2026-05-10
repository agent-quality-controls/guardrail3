#![expect(
    clippy::disallowed_methods,
    reason = "test-fixture: these cases write real guardrail3-ts.toml files into tempdirs to drive opt-out parsing through its on-disk surface; no centralized fs write helper exists in this CLI"
)]

use guardrail3_ts_app_types::SupportedFamily;
use guardrail3_ts_validate_command_assertions::family_opt_out::{
    assert_contains, assert_no_disabled, assert_not_contains,
};

use super::super::disabled_families;

#[test]
fn no_toml_returns_empty() {
    let dir = tempfile::tempdir().expect("create empty tempdir for missing-toml opt-out test");
    let disabled = disabled_families(dir.path());
    assert_no_disabled(&disabled, "missing guardrail3-ts.toml");
}

#[test]
fn malformed_toml_returns_empty() {
    let dir = tempfile::tempdir().expect("create tempdir for malformed-toml opt-out test");
    std::fs::write(
        dir.path().join("guardrail3-ts.toml"),
        "this is not [[[valid toml",
    )
    .expect("write malformed guardrail3-ts.toml fixture for opt-out test");
    let disabled = disabled_families(dir.path());
    assert_no_disabled(&disabled, "malformed guardrail3-ts.toml");
}

#[test]
fn empty_toml_returns_empty() {
    let dir = tempfile::tempdir().expect("create tempdir for empty-toml opt-out test");
    std::fs::write(dir.path().join("guardrail3-ts.toml"), "")
        .expect("write empty guardrail3-ts.toml fixture for opt-out test");
    let disabled = disabled_families(dir.path());
    assert_no_disabled(&disabled, "empty guardrail3-ts.toml");
}

#[test]
fn disabled_families_lists_eslint_and_style() {
    let dir = tempfile::tempdir().expect("create tempdir for eslint+style opt-out test");
    std::fs::write(
        dir.path().join("guardrail3-ts.toml"),
        "[checks]\neslint = false\nstyle = false\n",
    )
    .expect("write guardrail3-ts.toml fixture disabling eslint and style families");
    let disabled = disabled_families(dir.path());
    assert_contains(&disabled, SupportedFamily::Eslint);
    assert_contains(&disabled, SupportedFamily::Style);
}

#[test]
fn enabled_explicitly_true_does_not_disable() {
    let dir =
        tempfile::tempdir().expect("create tempdir for eslint=true non-disabling opt-out test");
    std::fs::write(
        dir.path().join("guardrail3-ts.toml"),
        "[checks]\neslint = true\n",
    )
    .expect("write guardrail3-ts.toml fixture with eslint = true for opt-out test");
    let disabled = disabled_families(dir.path());
    assert_not_contains(&disabled, SupportedFamily::Eslint);
}
