use guardrail3_check_types::G3Severity;
use guardrail3_rs_toml_parser::RustProfile;

use crate::test_support::{Finding, findings, input};

#[test]
fn reports_committed_lockfile_as_inventory() {
    let results = crate::run::check(&input(Some(RustProfile::Service), true, false, None));

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-DEPS-FILETREE-09".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock committed".to_owned(),
                message: "Workspace root has `Cargo.lock` committed.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
            Finding {
                id: "RS-DEPS-FILETREE-10".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock tracked by git".to_owned(),
                message: "No relevant `.gitignore` masks `Cargo.lock` at the workspace root."
                    .to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
        ]
    );
}

#[test]
fn reports_missing_library_lockfile_as_info() {
    let results = crate::run::check(&input(Some(RustProfile::Library), false, false, None));

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-DEPS-FILETREE-09".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock missing".to_owned(),
                message: "Library-profile workspace is missing `Cargo.lock`.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: false,
            },
            Finding {
                id: "RS-DEPS-FILETREE-10".to_owned(),
                severity: G3Severity::Info,
                title: "Cargo.lock tracked by git".to_owned(),
                message: "No relevant `.gitignore` masks `Cargo.lock` at the workspace root."
                    .to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: true,
            },
        ]
    );
}
