use guardrail3_domain_modules::deny::build_deny_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, run_family, set_section_bool, write_file};

#[test]
fn local_graph_all_features_drift_only_errors_for_the_owned_local_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_section_bool(
            &build_deny_toml("service", "", "", ""),
            "graph",
            "all-features",
            false,
        ),
    );

    let results = run_family(tmp.path());
    let graph_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-07")
        .collect::<Vec<_>>();

    assert_eq!(graph_results.len(), 1, "{graph_results:#?}");
    let result = graph_results[0];
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "graph all-features must be true");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` must set `[graph].all-features = true`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
}
