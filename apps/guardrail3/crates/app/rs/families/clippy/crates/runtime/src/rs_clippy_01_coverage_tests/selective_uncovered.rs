use std::collections::BTreeSet;

use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, run_family, write_file};

#[test]
fn errors_only_for_roots_without_an_allowed_covering_config() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/devctl/clippy.toml",
        &build_clippy_toml("service", false, true, "", ""),
    );
    write_file(
        tmp.path(),
        "packages/shared-types/clippy.toml",
        &build_clippy_toml("library", false, true, "", ""),
    );

    let results = run_family(tmp.path());
    let coverage = results
        .iter()
        .filter(|result| result.id == "RS-CLIPPY-01")
        .collect::<Vec<_>>();

    let actual_messages = coverage
        .iter()
        .map(|result| result.message.clone())
        .collect::<BTreeSet<_>>();
    let expected_messages = BTreeSet::from([
        "workspace root `apps/backend` is not covered by any allowed clippy.toml at the validation root, a workspace root, or a standalone package root."
            .to_owned(),
        "workspace root `apps/devctl` is covered by `apps/devctl/clippy.toml`.".to_owned(),
        "workspace root `apps/worker` is not covered by any allowed clippy.toml at the validation root, a workspace root, or a standalone package root."
            .to_owned(),
        "workspace root is not covered by any allowed clippy.toml at the validation root, a workspace root, or a standalone package root."
            .to_owned(),
    ]);

    assert_eq!(actual_messages, expected_messages);

    let errors = coverage
        .iter()
        .filter(|result| result.severity == Severity::Error)
        .collect::<Vec<_>>();
    assert_eq!(
        errors.len(),
        3,
        "expected exactly the uncovered roots to error: {errors:#?}"
    );
    assert!(errors.iter().all(|result| !result.inventory));
    assert_eq!(
        errors
            .iter()
            .filter_map(|result| result.file.as_deref())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["", "apps/backend", "apps/worker"]),
    );
}
