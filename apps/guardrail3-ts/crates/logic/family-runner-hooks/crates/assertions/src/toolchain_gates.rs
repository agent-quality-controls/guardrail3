use g3ts_hooks_contract_types::{G3TsHookCommandRequirement, PackageManager};

/// Asserts that `argvs` contains the concrete argv produced by `requirement`
/// for the given package `manager`. Used by both internal and external tests.
///
/// # Panics
///
/// Panics if the requirement does not have a concrete command for `manager`,
/// or if `argvs` does not contain the expected argv.
pub fn assert_argvs_contain_requirement(
    argvs: &[Vec<String>],
    requirement: G3TsHookCommandRequirement,
    manager: PackageManager,
) {
    let expected = requirement.concrete_command(manager);
    assert!(
        expected.is_some(),
        "G3TsHookCommandRequirement {requirement:?} should produce a concrete command for {manager:?}"
    );
    let expected = expected.unwrap_or_default();
    assert!(
        argvs.contains(&expected),
        "expected gate argv {expected:?} sourced from contract for {requirement:?}; got {argvs:?}"
    );
}

/// Asserts that `argvs` does NOT contain the concrete argv produced by
/// `requirement` for the given package `manager`.
///
/// # Panics
///
/// Panics if the requirement does not have a concrete command for `manager`,
/// or if `argvs` contains the argv that was expected to be skipped.
pub fn assert_argvs_skip_requirement(
    argvs: &[Vec<String>],
    requirement: G3TsHookCommandRequirement,
    manager: PackageManager,
    reason: &str,
) {
    let expected = requirement.concrete_command(manager);
    assert!(
        expected.is_some(),
        "G3TsHookCommandRequirement {requirement:?} should produce a concrete command for {manager:?}"
    );
    let expected = expected.unwrap_or_default();
    assert!(
        !argvs.contains(&expected),
        "{requirement:?} gate must be skipped {reason}; got {argvs:?}"
    );
}
