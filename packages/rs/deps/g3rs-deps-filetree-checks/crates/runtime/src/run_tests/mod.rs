use guardrail3_check_types::G3Severity;
use guardrail3_rs_toml_parser::RustProfile;

use crate::test_support::{Finding, findings, input};

#[test]
fn run_emits_lockfile_and_gitignore_findings_together() {
    let results = crate::run::check(&input(
        Some(RustProfile::Service),
        false,
        true,
        Some(".gitignore"),
    ));

    assert_eq!(
        findings(&results),
        vec![
            Finding {
                id: "RS-DEPS-FILETREE-09".to_owned(),
                severity: G3Severity::Error,
                title: "Cargo.lock missing".to_owned(),
                message: "`Cargo.lock` is missing. Run `cargo generate-lockfile` and commit the result.".to_owned(),
                file: Some("Cargo.lock".to_owned()),
                inventory: false,
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
