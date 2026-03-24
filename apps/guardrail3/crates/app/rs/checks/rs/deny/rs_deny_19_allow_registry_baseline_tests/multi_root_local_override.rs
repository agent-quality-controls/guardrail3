use crate::domain::modules::deny::build_deny_toml;
use crate::domain::report::Severity;

use super::super::super::test_support::{
    copy_fixture, run_family, set_allow_registries, write_file,
};

#[test]
fn local_registry_drift_only_errors_for_the_owned_local_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_allow_registries(&build_deny_toml("service", "", "", ""), &[]),
    );

    let results = run_family(tmp.path());
    let registry_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-19")
        .collect::<Vec<_>>();

    assert_eq!(registry_results.len(), 1, "{registry_results:#?}");
    let result = registry_results[0];
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "crates.io registry not allowed");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` must include crates.io in `[sources].allow-registry`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
    assert!(!result.inventory);
}
