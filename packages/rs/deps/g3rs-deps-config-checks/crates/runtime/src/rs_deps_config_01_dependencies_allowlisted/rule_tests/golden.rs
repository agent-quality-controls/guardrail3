use super::helpers::{dependency, run_check};

#[test]
fn inventories_allowlisted_runtime_dependency() {
    let results = run_check(true, &["serde"], &[dependency("serde")]);

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-CONFIG-01"
            && result.inventory()
            && result.message().contains("Dependency `serde`")
    }));
}

#[test]
fn allowlist_check_uses_normalized_package_name() {
    let results = run_check(true, &["serde"], &[dependency("serde")]);

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-CONFIG-01"
            && result.inventory()
            && result.message().contains("Dependency `serde`")
    }));
}
