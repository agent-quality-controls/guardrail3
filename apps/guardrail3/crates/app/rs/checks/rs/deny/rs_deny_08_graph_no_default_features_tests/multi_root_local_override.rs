use crate::domain::modules::deny::build_deny_toml;
use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, run_family, set_section_bool, write_file};

#[test]
fn local_graph_no_default_features_drift_only_errors_for_the_owned_local_root() {
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
            "no-default-features",
            true,
        ),
    );

    let results = run_family(tmp.path());
    let graph_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-08")
        .collect::<Vec<_>>();

    assert_eq!(graph_results.len(), 1, "{graph_results:#?}");
    let result = graph_results[0];
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.title, "graph no-default-features must be false");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` must set `[graph].no-default-features = false`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
}
