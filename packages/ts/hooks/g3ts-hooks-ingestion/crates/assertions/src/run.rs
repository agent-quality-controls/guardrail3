use g3ts_hooks_types::{G3TsHookScriptKind, G3TsHooksSourceChecksInput};

pub fn assert_only_pre_commit_script(inputs: &[G3TsHooksSourceChecksInput]) {
    assert_eq!(
        inputs.len(),
        1,
        "expected only pre-commit input: {inputs:#?}"
    );
    let input = inputs
        .first()
        .expect("expected source ingestion to return pre-commit input");
    assert_eq!(input.kind(), G3TsHookScriptKind::PreCommit);
    assert_eq!(input.rel_path(), ".githooks/pre-commit");
}

pub fn assert_modular_script_present(inputs: &[G3TsHooksSourceChecksInput], rel_path: &str) {
    assert_eq!(
        inputs.len(),
        2,
        "expected pre-commit plus modular input: {inputs:#?}"
    );
    assert!(
        inputs.iter().any(
            |input| input.kind() == G3TsHookScriptKind::Modular && input.rel_path() == rel_path
        ),
        "expected modular input {rel_path} in {inputs:#?}"
    );
}

pub fn assert_pre_commit_app_root(inputs: &[G3TsHooksSourceChecksInput], expected: &str) {
    let input = inputs
        .first()
        .expect("expected source ingestion to return pre-commit input");
    assert!(
        input
            .app_package_roots()
            .iter()
            .any(|root| root == expected),
        "expected app package root `{expected}` in {inputs:#?}"
    );
}
