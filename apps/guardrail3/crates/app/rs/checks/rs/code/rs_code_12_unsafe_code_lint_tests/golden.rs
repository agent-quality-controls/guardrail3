use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family};

#[test]
fn populated_golden_fixture_inventories_workspace_forbid_lints() {
    let fixture = copy_fixture();

    let results = run_family(fixture.path());
    let mut rs_code_12_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-12")
        .map(|result| {
            (
                result.file.clone().expect("file"),
                format!("{:?}", result.severity),
                result.title.clone(),
                result.message.clone(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();
    rs_code_12_results.sort();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-12"),
        BTreeSet::from([
            "apps/backend/Cargo.toml".to_owned(),
            "apps/devctl/Cargo.toml".to_owned(),
            "apps/worker/Cargo.toml".to_owned(),
        ])
    );
    assert_eq!(
        rs_code_12_results,
        vec![
            (
                "apps/backend/Cargo.toml".to_owned(),
                format!("{:?}", Severity::Info),
                "unsafe_code = forbid".to_owned(),
                "unsafe_code is set to forbid in workspace lints.".to_owned(),
                true,
            ),
            (
                "apps/devctl/Cargo.toml".to_owned(),
                format!("{:?}", Severity::Info),
                "unsafe_code = forbid".to_owned(),
                "unsafe_code is set to forbid in workspace lints.".to_owned(),
                true,
            ),
            (
                "apps/worker/Cargo.toml".to_owned(),
                format!("{:?}", Severity::Info),
                "unsafe_code = forbid".to_owned(),
                "unsafe_code is set to forbid in workspace lints.".to_owned(),
                true,
            ),
        ]
    );
}
