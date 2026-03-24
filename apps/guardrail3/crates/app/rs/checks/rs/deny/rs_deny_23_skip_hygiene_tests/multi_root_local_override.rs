use crate::domain::modules::deny::build_deny_toml;
use crate::domain::report::Severity;

use super::super::super::test_support::{add_skip_entry, copy_fixture, run_family, write_file};

#[test]
fn local_skip_inventory_only_hits_the_owned_local_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &add_skip_entry(
            &build_deny_toml("service", "", "", ""),
            toml::Value::String("plain-crate".to_owned()),
        ),
    );

    let results = run_family(tmp.path());
    let skip_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-23")
        .collect::<Vec<_>>();

    assert_eq!(skip_results.len(), 1, "{skip_results:#?}");
    let result = skip_results[0];
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "skip entry");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` has skip entry `plain-crate`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
    assert!(result.inventory);
}
