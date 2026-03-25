use guardrail3_domain_modules::deny::build_deny_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, set_deny_ban_wrappers, write_file};

#[test]
fn local_wrapper_drift_only_reports_for_the_owned_library_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_deny_ban_wrappers(
            &build_deny_toml("service", "", "", ""),
            "regex",
            &["tree-sitter"],
        ),
    );

    let results = super::super::super::test_support::run_family(tmp.path());
    let wrapper_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-30")
        .collect::<Vec<_>>();

    assert_eq!(wrapper_results.len(), 1, "{wrapper_results:#?}");
    let result = wrapper_results[0];
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "managed ban wrappers changed");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` ban `regex` must keep wrappers `globset, ignore, tree-sitter`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
    assert!(!result.inventory);
}
