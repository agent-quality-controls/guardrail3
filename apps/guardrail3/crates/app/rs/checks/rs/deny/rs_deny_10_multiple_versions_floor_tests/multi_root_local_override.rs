use crate::domain::modules::deny::build_deny_toml;
use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, run_family, set_section_string, write_file};

#[test]
fn local_multiple_versions_weakening_only_warns_for_the_owned_local_root() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_section_string(
            &build_deny_toml("service", "", "", ""),
            "bans",
            "multiple-versions",
            "warn",
        ),
    );

    let results = run_family(tmp.path());
    let ban_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-10")
        .collect::<Vec<_>>();

    assert_eq!(ban_results.len(), 1, "{ban_results:#?}");
    let result = ban_results[0];
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "multiple-versions weaker than baseline");
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` sets `[bans].multiple-versions = \"warn\"`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
}
