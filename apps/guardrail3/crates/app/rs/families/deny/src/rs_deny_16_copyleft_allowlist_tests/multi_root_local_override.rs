use guardrail3_domain_modules::deny::build_deny_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    add_allowed_license, copy_fixture, run_family, write_file,
};

#[test]
fn local_copyleft_allowance_only_warns_for_the_owned_local_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &add_allowed_license(&build_deny_toml("service", "", "", ""), "GPL-3.0-only"),
    );

    let results = run_family(tmp.path());
    let license_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-16")
        .collect::<Vec<_>>();

    assert_eq!(license_results.len(), 1, "{license_results:#?}");
    let result = license_results[0];
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "copyleft license allowed");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` allows copyleft license `GPL-3.0-only`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
}
