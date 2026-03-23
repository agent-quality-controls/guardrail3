use crate::domain::report::Severity;

use super::super::inputs::ToolchainRootInput;
use super::check;

#[test]
fn inventories_when_channel_and_components_match_policy() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = ToolchainRootInput {
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: None,
        parsed: Some(&parsed),
        parse_error: None,
        cargo_rust_version: Some("1.85"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 3);
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-02"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "toolchain channel is stable"
            && result.message == "channel = \"stable\"."
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-02"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "toolchain component `clippy` present"
            && result.message == "`clippy` is listed in `components`."
    }));
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-02"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "toolchain component `rustfmt` present"
            && result.message == "`rustfmt` is listed in `components`."
    }));
}

#[test]
fn errors_on_parse_failure() {
    let input = ToolchainRootInput {
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: None,
        parsed: None,
        parse_error: Some("expected a table"),
        cargo_rust_version: Some("1.85"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-TOOLCHAIN-02");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "rust-toolchain.toml parse error");
    assert_eq!(result.message, "Invalid TOML: expected a table");
    assert_eq!(result.file.as_deref(), Some("rust-toolchain.toml"));
}

#[test]
fn warns_when_required_component_is_missing() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\"]",
    )
    .expect("valid TOML");
    let input = ToolchainRootInput {
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: None,
        parsed: Some(&parsed),
        parse_error: None,
        cargo_rust_version: Some("1.85"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 3);
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-02"
            && result.severity == Severity::Warn
            && result.title == "toolchain component `rustfmt` missing"
            && result.message == "Add `rustfmt` to `[toolchain].components`."
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
}

#[test]
fn errors_when_channel_is_nightly() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"nightly\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = ToolchainRootInput {
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: None,
        parsed: Some(&parsed),
        parse_error: None,
        cargo_rust_version: Some("1.85"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 3);
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-02"
            && result.severity == Severity::Error
            && result.title == "toolchain channel is nightly"
            && result.message == "Use `channel = \"stable\"` or a pinned stable version."
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
}

#[test]
fn inventories_when_channel_is_pinned_stable_version() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.85.0\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = ToolchainRootInput {
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: None,
        parsed: Some(&parsed),
        parse_error: None,
        cargo_rust_version: Some("1.85"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 3);
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-02"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "toolchain channel is pinned"
            && result.message == "Pinned channel `1.85.0` is acceptable."
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
}

#[test]
fn warns_when_channel_is_missing() {
    let parsed =
        toml::from_str::<toml::Value>("[toolchain]\ncomponents = [\"clippy\", \"rustfmt\"]")
            .expect("valid TOML");
    let input = ToolchainRootInput {
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: None,
        parsed: Some(&parsed),
        parse_error: None,
        cargo_rust_version: Some("1.85"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 3);
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-02"
            && result.severity == Severity::Warn
            && result.title == "toolchain channel missing"
            && result.message == "Add `channel = \"stable\"` under `[toolchain]`."
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
}

#[test]
fn warns_when_components_array_is_missing() {
    let parsed =
        toml::from_str::<toml::Value>("[toolchain]\nchannel = \"stable\"").expect("valid TOML");
    let input = ToolchainRootInput {
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: None,
        parsed: Some(&parsed),
        parse_error: None,
        cargo_rust_version: Some("1.85"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-02"
            && result.severity == Severity::Warn
            && result.title == "toolchain components missing"
            && result.message == "Add `components = [\"clippy\", \"rustfmt\"]` under `[toolchain]`."
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
}
