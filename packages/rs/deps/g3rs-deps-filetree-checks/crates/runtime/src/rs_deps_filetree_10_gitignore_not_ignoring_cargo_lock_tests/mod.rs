use guardrail3_check_types::G3Severity;

use crate::test_support::{Finding, findings, input};

#[test]
fn reports_ignored_lockfile() {
    let results = crate::run::check(&input(None, true, true, Some(".gitignore")));

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
                severity: G3Severity::Error,
                title: "Cargo.lock ignored in gitignore".to_owned(),
                message: "`.gitignore` ignores `Cargo.lock`. Remove the line ignoring `Cargo.lock` from this `.gitignore`.".to_owned(),
                file: Some(".gitignore".to_owned()),
                inventory: false,
            },
        ]
    );
}
