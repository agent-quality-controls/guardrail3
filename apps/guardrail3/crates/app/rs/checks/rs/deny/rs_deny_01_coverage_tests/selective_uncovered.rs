use std::collections::BTreeSet;

use crate::domain::modules::deny::build_deny_toml;
use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, run_family, write_file};

#[test]
fn errors_only_for_effective_roots_without_a_covering_deny_config() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &build_deny_toml("service", "", "", ""),
    );
    write_file(
        tmp.path(),
        "packages/shared-types/deny.toml",
        &build_deny_toml("library", "", "", ""),
    );

    let results = run_family(tmp.path());
    let coverage = results
        .iter()
        .filter(|result| result.id == "RS-DENY-01")
        .collect::<Vec<_>>();

    let actual_messages = coverage
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = BTreeSet::from([
        "validation root `.` is not covered by any allowed deny config.".to_owned(),
        "workspace root `.` is not covered by any allowed deny config.".to_owned(),
        "workspace root `apps/backend` is not covered by any allowed deny config.".to_owned(),
        "workspace root `apps/devctl` is covered by `apps/devctl/deny.toml`.".to_owned(),
        "workspace root `apps/worker` is not covered by any allowed deny config.".to_owned(),
    ]);

    assert_eq!(actual_messages, expected_messages);

    let errors = coverage
        .iter()
        .filter(|result| result.severity == Severity::Error)
        .collect::<Vec<_>>();
    assert_eq!(
        errors.len(),
        4,
        "expected exactly the uncovered roots to error: {errors:#?}"
    );
    assert!(
        errors
            .iter()
            .all(|result| !result.inventory && result.file.is_none())
    );
}
