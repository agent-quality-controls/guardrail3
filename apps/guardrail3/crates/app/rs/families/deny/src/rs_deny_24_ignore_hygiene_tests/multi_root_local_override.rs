use guardrail3_domain_modules::deny::build_deny_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    copy_fixture, run_family, set_advisory_ignores, write_file,
};

#[test]
fn local_advisory_ignore_inventory_only_hits_the_owned_local_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_advisory_ignores(
            &build_deny_toml("service", "", "", ""),
            vec![toml::Value::String("RUSTSEC-2026-0000".to_owned())],
        ),
    );

    let results = run_family(tmp.path());
    let ignore_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-24")
        .collect::<Vec<_>>();

    assert_eq!(ignore_results.len(), 1, "{ignore_results:#?}");
    let result = ignore_results[0];
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.title, "advisory ignore entry");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` ignores advisory `RUSTSEC-2026-0000`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
    assert!(result.inventory);
}
