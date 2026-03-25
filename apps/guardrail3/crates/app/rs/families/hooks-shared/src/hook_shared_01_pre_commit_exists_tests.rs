use guardrail3_domain_report::Severity;

use super::super::facts::{HookScriptFacts, HookScriptKind};
use super::check;

#[test]
fn errors_when_pre_commit_is_missing() {
    let mut results = Vec::new();
    check(None, &mut results);
    assert_eq!(results[0].severity, Severity::Error);
    assert!(!results[0].inventory);
}

#[test]
fn inventories_existing_pre_commit_script() {
    let script = HookScriptFacts {
        rel_path: ".githooks/pre-commit".to_owned(),
        kind: HookScriptKind::PreCommit,
        content: "#!/usr/bin/env bash\n".to_owned(),
    };
    let mut results = Vec::new();
    check(Some(&script), &mut results);
    assert!(results[0].inventory);
}
