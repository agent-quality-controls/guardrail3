use crate::domain::report::Severity;

use super::super::inputs::ToolchainRootInput;
use super::check;

#[test]
fn warns_when_pinned_toolchain_is_older_than_msrv() {
    let parsed =
        toml::from_str::<toml::Value>("[toolchain]\nchannel = \"1.84.0\"\ncomponents = [\"clippy\", \"rustfmt\"]")
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

    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-03"
            && result.severity == Severity::Warn
            && result.title == "pinned toolchain is older than MSRV"
            && result.message
                == "Pinned toolchain `1.84.0` is older than Cargo rust-version `1.85.0`."
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
}

#[test]
fn inventories_when_pinned_toolchain_satisfies_msrv() {
    let parsed =
        toml::from_str::<toml::Value>("[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]")
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

    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-03"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "pinned toolchain satisfies MSRV"
            && result.message
                == "Pinned toolchain `1.85.1` is compatible with Cargo rust-version `1.85.0`."
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
}

#[test]
fn inventories_when_msrv_is_missing() {
    let parsed =
        toml::from_str::<toml::Value>("[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]")
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

    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-03"
            && result.inventory
            && result.severity == Severity::Info
            && result.title == "Cargo rust-version not declared"
            && result.message
                == "No `rust-version` found in Cargo.toml, so MSRV consistency cannot be checked."
            && result.file.as_deref() == Some("rust-toolchain.toml")
    }));
}
