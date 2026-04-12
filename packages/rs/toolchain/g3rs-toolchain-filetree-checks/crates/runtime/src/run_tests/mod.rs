use guardrail3_check_types::G3Severity;

use crate::test_support::{Finding, findings, input};

#[test]
fn modern_only_emits_only_filetree_01_inventory() {
    let results = crate::check(&input(Some("rust-toolchain.toml"), None));

    assert_eq!(
        findings(&results),
        vec![Finding {
            id: "RS-TOOLCHAIN-FILETREE-01".to_owned(),
            severity: G3Severity::Info,
            title: "rust-toolchain.toml exists".to_owned(),
            message: "Found rust-toolchain.toml at workspace root.".to_owned(),
            file: Some("rust-toolchain.toml".to_owned()),
            inventory: true,
        }]
    );
}

#[test]
fn legacy_only_emits_missing_modern_and_legacy_warn() {
    let results = crate::check(&input(None, Some("rust-toolchain")));

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-TOOLCHAIN-FILETREE-01".to_owned(),
                severity: G3Severity::Error,
                title: "rust-toolchain.toml missing".to_owned(),
                message: "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.".to_owned(),
                file: Some("rust-toolchain.toml".to_owned()),
                inventory: false,
            },
            Finding {
                id: "RS-TOOLCHAIN-FILETREE-04".to_owned(),
                severity: G3Severity::Warn,
                title: "legacy rust-toolchain file present".to_owned(),
                message: "Migrate `rust-toolchain` to `rust-toolchain.toml` so components can be declared explicitly.".to_owned(),
                file: Some("rust-toolchain".to_owned()),
                inventory: false,
            },
        ]
    );
}

#[test]
fn both_files_emit_modern_inventory_and_legacy_conflict() {
    let results = crate::check(&input(
        Some("rust-toolchain.toml"),
        Some("rust-toolchain"),
    ));

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-TOOLCHAIN-FILETREE-01".to_owned(),
                severity: G3Severity::Info,
                title: "rust-toolchain.toml exists".to_owned(),
                message: "Found rust-toolchain.toml at workspace root.".to_owned(),
                file: Some("rust-toolchain.toml".to_owned()),
                inventory: true,
            },
            Finding {
                id: "RS-TOOLCHAIN-FILETREE-04".to_owned(),
                severity: G3Severity::Error,
                title: "both rust-toolchain files present".to_owned(),
                message: "Remove the legacy `rust-toolchain` file. rustup reads it instead of `rust-toolchain.toml` when both exist, so your modern config is ignored.".to_owned(),
                file: Some("rust-toolchain".to_owned()),
                inventory: false,
            },
        ]
    );
}

#[test]
fn neither_file_emits_only_missing_modern() {
    let results = crate::check(&input(None, None));

    assert_eq!(
        findings(&results),
        vec![Finding {
            id: "RS-TOOLCHAIN-FILETREE-01".to_owned(),
            severity: G3Severity::Error,
            title: "rust-toolchain.toml missing".to_owned(),
            message: "Expected rust-toolchain.toml at workspace root. Create a `rust-toolchain.toml` with a `[toolchain]` section containing `channel` and `components`.".to_owned(),
            file: Some("rust-toolchain.toml".to_owned()),
            inventory: false,
        }]
    );
}
