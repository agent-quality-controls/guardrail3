use guardrail3_domain_report::Severity;

use super::super::inputs::ToolchainRootInput;
use super::check;

#[test]
fn warns_when_pinned_toolchain_is_older_than_msrv() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.84.0\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = ToolchainRootInput {
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: None,
        parsed: Some(&parsed),
        parse_error: None,
        cargo_rust_version: Some("1.85.0"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-TOOLCHAIN-03");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "pinned toolchain is older than MSRV");
    assert_eq!(
        result.message,
        "Pinned toolchain `1.84.0` is older than Cargo rust-version `1.85.0`."
    );
    assert_eq!(result.file.as_deref(), Some("rust-toolchain.toml"));
}

#[test]
fn inventories_when_pinned_toolchain_satisfies_msrv() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = ToolchainRootInput {
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: None,
        parsed: Some(&parsed),
        parse_error: None,
        cargo_rust_version: Some("1.85.0"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-TOOLCHAIN-03");
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "pinned toolchain satisfies MSRV");
    assert_eq!(
        result.message,
        "Pinned toolchain `1.85.1` is compatible with Cargo rust-version `1.85.0`."
    );
    assert_eq!(result.file.as_deref(), Some("rust-toolchain.toml"));
}

#[test]
fn inventories_when_msrv_is_missing() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = ToolchainRootInput {
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: None,
        parsed: Some(&parsed),
        parse_error: None,
        cargo_rust_version: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-TOOLCHAIN-03");
    assert!(result.inventory);
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "Cargo rust-version not declared");
    assert_eq!(
        result.message,
        "No `rust-version` found in Cargo.toml, so MSRV consistency cannot be checked."
    );
    assert_eq!(result.file.as_deref(), Some("rust-toolchain.toml"));
}

#[test]
fn emits_no_result_for_stable_channel() {
    let parsed = toml::from_str::<toml::Value>(
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]",
    )
    .expect("valid TOML");
    let input = ToolchainRootInput {
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: None,
        parsed: Some(&parsed),
        parse_error: None,
        cargo_rust_version: Some("1.85.0"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}
