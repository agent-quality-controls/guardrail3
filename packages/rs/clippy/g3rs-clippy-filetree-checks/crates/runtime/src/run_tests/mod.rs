use guardrail3_check_types::G3Severity;

use crate::test_support::{Finding, findings, input};

#[test]
fn clean_root_emits_only_coverage_inventory() {
    let results = crate::check(&input(Some(".clippy.toml"), &[]));

    assert_eq!(
        findings(&results),
        vec![Finding {
            id: "RS-CLIPPY-FILETREE-01".to_owned(),
            severity: G3Severity::Info,
            title: "workspace root covered by clippy config".to_owned(),
            message: "Workspace root is covered by `.clippy.toml`.".to_owned(),
            file: Some(".clippy.toml".to_owned()),
            inventory: true,
        }]
    );
}

#[test]
fn plain_clippy_toml_also_counts_as_root_coverage() {
    let results = crate::check(&input(Some("clippy.toml"), &[]));

    assert_eq!(
        findings(&results),
        vec![Finding {
            id: "RS-CLIPPY-FILETREE-01".to_owned(),
            severity: G3Severity::Info,
            title: "workspace root covered by clippy config".to_owned(),
            message: "Workspace root is covered by `clippy.toml`.".to_owned(),
            file: Some("clippy.toml".to_owned()),
            inventory: true,
        }]
    );
}

#[test]
fn missing_root_emits_only_uncovered_error() {
    let results = crate::check(&input(None, &[]));

    assert_eq!(
        findings(&results),
        vec![Finding {
            id: "RS-CLIPPY-FILETREE-01".to_owned(),
            severity: G3Severity::Error,
            title: "workspace root uncovered by clippy config".to_owned(),
            message: "Add `clippy.toml` or `.clippy.toml` at the workspace root so clippy policy is not left to defaults.".to_owned(),
            file: Some("clippy.toml".to_owned()),
            inventory: false,
        }]
    );
}

#[test]
fn same_root_dual_config_emits_coverage_inventory_and_conflict_error() {
    let results = crate::check(&input(
        Some(".clippy.toml"),
        &[("clippy.toml", ".clippy.toml")],
    ));

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-CLIPPY-FILETREE-01".to_owned(),
                severity: G3Severity::Info,
                title: "workspace root covered by clippy config".to_owned(),
                message: "Workspace root is covered by `.clippy.toml`.".to_owned(),
                file: Some(".clippy.toml".to_owned()),
                inventory: true,
            },
            Finding {
                id: "RS-CLIPPY-FILETREE-02".to_owned(),
                severity: G3Severity::Error,
                title: "same-root clippy config conflict".to_owned(),
                message: "`clippy.toml` conflicts with `.clippy.toml` at the same policy root. Keep only the highest-precedence clippy config file.".to_owned(),
                file: Some("clippy.toml".to_owned()),
                inventory: false,
            },
        ]
    );
}
