use super::helpers::{dependency, run_check, target_dependency};

#[test]
fn normalized_dependency_identity_uses_package_name_for_allowlist() {
    let results = run_check(true, &["serde"], &[dependency("serde")]);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-DEPS-CONFIG-01");
    assert!(results[0].inventory());
    assert!(
        results[0].message().contains("Dependency `serde`"),
        "expected normalized package identity in allowlist result: {results:#?}"
    );
}

#[test]
fn normalized_target_dependency_uses_same_allowlist_contract() {
    let results = run_check(true, &["serde"], &[target_dependency("serde", "cfg(unix)")]);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-DEPS-CONFIG-01");
    assert!(results[0].inventory());
}
