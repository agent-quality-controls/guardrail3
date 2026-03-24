use crate::domain::modules::deny::build_deny_toml;
use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, run_family, set_section_string, write_file};

#[test]
fn local_stricter_advisory_value_only_inventories_for_the_owned_local_root() {
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
            "advisories",
            "unmaintained",
            "deny",
        ),
    );

    let results = run_family(tmp.path());
    let advisory_results = results
        .iter()
        .filter(|result| result.id == "RS-DENY-06")
        .collect::<Vec<_>>();

    assert_eq!(advisory_results.len(), 1, "{advisory_results:#?}");
    let result = advisory_results[0];
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(
        result.title,
        "advisories `unmaintained` stricter than baseline"
    );
    assert_eq!(
        result.message,
        "`apps/devctl/deny.toml` sets `[advisories].unmaintained = \"deny\"`."
    );
    assert_eq!(result.file.as_deref(), Some("apps/devctl/deny.toml"));
    assert!(result.inventory);
}
