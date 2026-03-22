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

    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-02"
            && result.severity == Severity::Error
            && result.title == "rust-toolchain.toml parse error"
            && result.message == "Invalid TOML: expected a table"
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
}

#[test]
fn warns_when_required_component_is_missing() {
    let parsed =
        toml::from_str::<toml::Value>("[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\"]")
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

    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-02"
            && result.severity == Severity::Warn
            && result.title == "toolchain component `rustfmt` missing"
            && result.message == "Add `rustfmt` to `[toolchain].components`."
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
}
