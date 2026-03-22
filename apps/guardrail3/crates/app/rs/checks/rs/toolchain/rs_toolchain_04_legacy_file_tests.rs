use crate::domain::report::Severity;

use super::super::inputs::ToolchainRootInput;
use super::check;

#[test]
fn warns_when_only_legacy_toolchain_file_exists() {
    let input = ToolchainRootInput {
        toolchain_toml_rel: None,
        legacy_toolchain_rel: Some("rust-toolchain"),
        parsed: None,
        parse_error: None,
        cargo_rust_version: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-04"
            && result.severity == Severity::Warn
            && result.title == "legacy rust-toolchain file present"
            && result.message
                == "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly."
            && result.file.as_deref() == Some("rust-toolchain")
    }));
}

#[test]
fn warns_when_both_legacy_and_modern_toolchain_files_exist() {
    let input = ToolchainRootInput {
        toolchain_toml_rel: Some("rust-toolchain.toml"),
        legacy_toolchain_rel: Some("rust-toolchain"),
        parsed: None,
        parse_error: None,
        cargo_rust_version: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.iter().any(|result| {
        result.id == "RS-TOOLCHAIN-04"
            && result.severity == Severity::Warn
            && result.title == "both rust-toolchain files present"
            && result.message == "Remove the legacy `rust-toolchain` file to avoid ambiguity."
            && result.file.as_deref() == Some("rust-toolchain")
    }));
}
