use guardrail3_app_rs_family_toolchain_assertions::rs_toolchain_02_channel_and_components::{
    ExpectedRuleResult, assert_rule_results,
};
use guardrail3_domain_report::Severity;

use super::{check, test_input};

#[test]
fn inventories_when_channel_and_components_match_policy() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain channel is stable",
                message: "channel = \"stable\".",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `clippy` present",
                message: "`clippy` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `rustfmt` present",
                message: "`rustfmt` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
        ],
    );
}

#[test]
fn errors_on_parse_failure() {
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        None,
        Some("expected a table"),
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Severity::Error,
            inventory: false,
            title: "rust-toolchain.toml parse error",
            message: "Invalid TOML: expected a table",
            file: Some("rust-toolchain.toml"),
        }],
    );
}

#[test]
fn warns_when_required_component_is_missing() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain channel is stable",
                message: "channel = \"stable\".",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `clippy` present",
                message: "`clippy` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Warn,
                inventory: false,
                title: "toolchain component `rustfmt` missing",
                message: "Add `rustfmt` to `[toolchain].components`.",
                file: Some("rust-toolchain.toml"),
            },
        ],
    );
}

#[test]
fn errors_when_channel_is_nightly() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"nightly\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Severity::Error,
                inventory: false,
                title: "toolchain channel is nightly",
                message: "Use `channel = \"stable\"` or a pinned stable version.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `clippy` present",
                message: "`clippy` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `rustfmt` present",
                message: "`rustfmt` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
        ],
    );
}

#[test]
fn errors_when_channel_is_pinned_nightly() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"nightly-2026-03-01\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Severity::Error,
                inventory: false,
                title: "toolchain channel is nightly",
                message: "Use `channel = \"stable\"` or a pinned stable version.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `clippy` present",
                message: "`clippy` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `rustfmt` present",
                message: "`rustfmt` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
        ],
    );
}

#[test]
fn errors_when_version_like_channel_contains_nightly_suffix() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.85.0-nightly\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Severity::Error,
                inventory: false,
                title: "toolchain channel is nightly",
                message: "Use `channel = \"stable\"` or a pinned stable version.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `clippy` present",
                message: "`clippy` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `rustfmt` present",
                message: "`rustfmt` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
        ],
    );
}

#[test]
fn errors_when_channel_is_beta() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"beta\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Severity::Error,
                inventory: false,
                title: "toolchain channel is beta",
                message: "Use `channel = \"stable\"` or a pinned stable version.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `clippy` present",
                message: "`clippy` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `rustfmt` present",
                message: "`rustfmt` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
        ],
    );
}

#[test]
fn errors_when_version_like_channel_contains_beta_suffix() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.85.0-beta\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Severity::Error,
                inventory: false,
                title: "toolchain channel is beta",
                message: "Use `channel = \"stable\"` or a pinned stable version.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `clippy` present",
                message: "`clippy` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `rustfmt` present",
                message: "`rustfmt` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
        ],
    );
}

#[test]
fn errors_when_channel_is_not_a_string() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = 185\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Severity::Error,
                inventory: false,
                title: "toolchain channel is invalid",
                message: "`[toolchain].channel` must be a string.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `clippy` present",
                message: "`clippy` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `rustfmt` present",
                message: "`rustfmt` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
        ],
    );
}

#[test]
fn inventories_when_channel_is_pinned_stable_version() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.85.0\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain channel is pinned",
                message: "Pinned channel `1.85.0` is acceptable.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `clippy` present",
                message: "`clippy` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `rustfmt` present",
                message: "`rustfmt` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
        ],
    );
}

#[test]
fn errors_when_components_are_not_a_string_array() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", 1]",
    )
    .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain channel is stable",
                message: "channel = \"stable\".",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Error,
                inventory: false,
                title: "toolchain components are invalid",
                message: "`[toolchain].components` must be an array of strings.",
                file: Some("rust-toolchain.toml"),
            },
        ],
    );
}

#[test]
fn warns_when_channel_is_missing() {
    let parsed =
        toml::from_str::<toml::Value>("[toolchain]\ncomponents = [\"clippy\", \"rustfmt\"]")
            .expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Severity::Warn,
                inventory: false,
                title: "toolchain channel missing",
                message: "Add `channel = \"stable\"` under `[toolchain]`.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `clippy` present",
                message: "`clippy` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain component `rustfmt` present",
                message: "`rustfmt` is listed in `components`.",
                file: Some("rust-toolchain.toml"),
            },
        ],
    );
}

#[test]
fn warns_when_components_array_is_missing() {
    let parsed =
        toml::from_str::<toml::Value>("[toolchain]\nchannel = \"stable\"").expect("valid TOML");
    let input = test_input(
        Some("rust-toolchain.toml"),
        None,
        Some(&parsed),
        None,
        Some("1.85"),
        None,
    );
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_rule_results(
        &results,
        &[
            ExpectedRuleResult {
                severity: Severity::Info,
                inventory: true,
                title: "toolchain channel is stable",
                message: "channel = \"stable\".",
                file: Some("rust-toolchain.toml"),
            },
            ExpectedRuleResult {
                severity: Severity::Warn,
                inventory: false,
                title: "toolchain components missing",
                message: "Add `components = [\"clippy\", \"rustfmt\"]` under `[toolchain]`.",
                file: Some("rust-toolchain.toml"),
            },
        ],
    );
}
