use std::collections::BTreeMap;

use guardrail3_domain_modules::deny::build_deny_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{
    canonical_deny_toml_service, copy_fixture, run_family, write_file,
};

#[test]
fn inventories_exact_covering_deny_config_for_each_effective_rust_root() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "deny.toml", &canonical_deny_toml_service());
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

    let actual = coverage
        .iter()
        .filter(|result| result.severity == Severity::Info)
        .map(|result| {
            (
                result.message.clone(),
                (result.inventory, result.file.clone(), result.title.clone()),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let expected = BTreeMap::from([
        (
            "validation root `.` is covered by `deny.toml`.".to_owned(),
            (
                true,
                Some("deny.toml".to_owned()),
                "Rust root covered by deny config".to_owned(),
            ),
        ),
        (
            "workspace root `.` is covered by `deny.toml`.".to_owned(),
            (
                true,
                Some("deny.toml".to_owned()),
                "Rust root covered by deny config".to_owned(),
            ),
        ),
        (
            "workspace root `apps/backend` is covered by `deny.toml`.".to_owned(),
            (
                true,
                Some("deny.toml".to_owned()),
                "Rust root covered by deny config".to_owned(),
            ),
        ),
        (
            "workspace root `apps/devctl` is covered by `apps/devctl/deny.toml`.".to_owned(),
            (
                true,
                Some("apps/devctl/deny.toml".to_owned()),
                "Rust root covered by deny config".to_owned(),
            ),
        ),
        (
            "workspace root `apps/worker` is covered by `deny.toml`.".to_owned(),
            (
                true,
                Some("deny.toml".to_owned()),
                "Rust root covered by deny config".to_owned(),
            ),
        ),
    ]);

    assert_eq!(actual, expected);
}
