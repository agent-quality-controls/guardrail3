use super::check;
use crate::test_support::{findings, input_with_raw, parsed_rust_policy};
use guardrail3_rs_toml_parser::RustProfile;

#[test]
fn warns_when_enabled_for_non_library_policy() {
    let input = input_with_raw(
        "clippy.toml",
        "avoid-breaking-exported-api = true\n",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Service), true),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert_eq!(
        findings(&results),
        vec![crate::test_support::Finding {
            id: "RS-CLIPPY-CONFIG-15".to_owned(),
            severity: guardrail3_check_types::G3Severity::Warn,
            title: "avoid-breaking-exported-api enabled".to_owned(),
            message: "`avoid-breaking-exported-api = true` suppresses useful lints. Prefer `false`.".to_owned(),
            file: Some("clippy.toml".to_owned()),
            inventory: false,
        }]
    );
}

#[test]
fn inventories_enabled_setting_for_published_library_policy() {
    let input = input_with_raw(
        "clippy.toml",
        "avoid-breaking-exported-api = true\n",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Library), true),
        true,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "library keeps avoid-breaking-exported-api enabled"
            && finding.inventory
    }));
}

#[test]
fn inventories_explicit_false_setting() {
    let input = input_with_raw(
        "clippy.toml",
        "avoid-breaking-exported-api = false\n",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Service), true),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "avoid-breaking-exported-api explicitly false"
            && finding.inventory
    }));
}

#[test]
fn warns_when_setting_is_missing() {
    let input = input_with_raw(
        "clippy.toml",
        "",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Service), true),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "avoid-breaking-exported-api not set"
            && finding.severity == guardrail3_check_types::G3Severity::Warn
    }));
}

#[test]
fn warns_when_setting_has_wrong_type() {
    let input = input_with_raw(
        "clippy.toml",
        "avoid-breaking-exported-api = 7\n",
        parsed_rust_policy("guardrail3-rs.toml", Some(RustProfile::Service), true),
        false,
        Vec::new(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);

    assert!(findings(&results).iter().any(|finding| {
        finding.title == "avoid-breaking-exported-api wrong type"
            && finding.message.contains("integer")
    }));
}
