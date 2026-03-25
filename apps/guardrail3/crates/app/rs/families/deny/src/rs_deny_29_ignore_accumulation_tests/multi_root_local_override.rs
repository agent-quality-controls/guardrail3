use guardrail3_domain_modules::deny::build_deny_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    copy_fixture, run_family, set_advisory_ignores, write_file,
};

#[test]
fn local_large_ignore_list_only_warns_for_the_owned_local_root() {
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
            ["A", "B", "C", "D", "E", "F"]
                .into_iter()
                .map(|id| toml::Value::String(id.to_owned()))
                .collect(),
        ),
    );

    let results = run_family(tmp.path());
    let ignore_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-29")
        .collect::<Vec<_>>();

    assert_eq!(ignore_results.len(), 1, "{ignore_results:#?}");
    let result = ignore_results[0];
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "advisory ignore list is large");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` has 6 `[advisories].ignore` entries (threshold: 5)."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
    assert!(!result.inventory);
}
