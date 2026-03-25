use std::collections::BTreeMap;

use guardrail3_domain_modules::deny::build_deny_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, run_family, write_file};

#[test]
fn coverage_uses_highest_precedence_config_at_each_policy_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        ".deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        ".cargo/deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/.deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &build_deny_toml("service", "", "", ""),
    );

    let results = run_family(tmp.path());
    let coverage = results
        .iter()
        .filter(|result| result.id == "RS-DENY-01" && result.severity == Severity::Info)
        .map(|result| (result.message.clone(), result.file.clone()))
        .collect::<BTreeMap<_, _>>();

    assert_eq!(
        coverage.get("validation root `.` is covered by `deny.toml`."),
        Some(&Some("deny.toml".to_owned()))
    );
    assert_eq!(
        coverage.get("workspace root `.` is covered by `deny.toml`."),
        Some(&Some("deny.toml".to_owned()))
    );
    assert_eq!(
        coverage.get("workspace root `apps/devctl` is covered by `apps/devctl/deny.toml`."),
        Some(&Some("apps/devctl/deny.toml".to_owned()))
    );
}
