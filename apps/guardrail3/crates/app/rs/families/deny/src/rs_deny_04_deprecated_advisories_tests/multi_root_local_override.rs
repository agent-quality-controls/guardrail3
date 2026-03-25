use guardrail3_domain_modules::deny::build_deny_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, run_family, write_file};

#[test]
fn local_deprecated_advisory_fields_only_warn_for_the_owned_local_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    let local = build_deny_toml("service", "", "", "")
        .replace("[advisories]\n", "[advisories]\nvulnerability = \"deny\"\n");
    write_file(tmp.path(), "apps/devctl/deny.toml", &local);

    let results = run_family(tmp.path());
    let advisory_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-04")
        .collect::<Vec<_>>();

    assert_eq!(advisory_results.len(), 1, "{advisory_results:#?}");
    let result = advisory_results[0];
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "deprecated advisory field `vulnerability`");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` uses deprecated `[advisories].vulnerability`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
}
