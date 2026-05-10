use g3ts_hooks_types::{G3TsHookScriptKind, G3TsHooksSourceChecksInput};

/// Fails the calling test when `inputs` does not contain exactly one pre-commit input.
///
/// # Panics
/// Panics on count, kind, or rel-path mismatch, which the assertion treats as a test failure.
pub fn assert_only_pre_commit_script(inputs: &[G3TsHooksSourceChecksInput]) {
    assert_eq!(
        inputs.len(),
        1,
        "expected only pre-commit input: {inputs:#?}"
    );
    let Some(input) = inputs.first() else {
        return;
    };
    assert_eq!(
        input.kind(),
        G3TsHookScriptKind::PreCommit,
        "expected pre-commit input"
    );
    assert_eq!(
        input.rel_path(),
        ".githooks/pre-commit",
        "expected pre-commit rel_path"
    );
}

/// Fails the calling test when `inputs` does not contain a pre-commit plus verifier input.
///
/// # Panics
/// Panics on count or shape mismatch, which the assertion treats as a test failure.
pub fn assert_verifier_script_present(inputs: &[G3TsHooksSourceChecksInput]) {
    assert_eq!(
        inputs.len(),
        2,
        "expected pre-commit plus verifier input: {inputs:#?}"
    );
    assert!(
        inputs
            .iter()
            .any(|input| input.kind() == G3TsHookScriptKind::Verifier
                && input.rel_path() == "scripts/g3ts/verify"),
        "expected verifier input in {inputs:#?}"
    );
}

/// Fails the calling test when the first pre-commit input does not list `expected` in its app package roots.
///
/// # Panics
/// Panics on mismatch, which the assertion treats as a test failure.
pub fn assert_pre_commit_app_root(inputs: &[G3TsHooksSourceChecksInput], expected: &str) {
    let Some(input) = inputs.first() else {
        assert!(
            !inputs.is_empty(),
            "expected at least one pre-commit input, got: {inputs:#?}"
        );
        return;
    };
    assert!(
        input
            .app_package_roots()
            .iter()
            .any(|root| root == expected),
        "expected app package root `{expected}` in {inputs:#?}"
    );
}
