use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn inventories_conditional_cfg_attr_allows_across_real_owned_files_with_exact_metadata() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let worker_rel = "apps/worker/crates/app/processor/src/lib.rs";
    let nested_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");
    let nested_content =
        std::fs::read_to_string(root.join(nested_rel)).expect("read nested source");

    let backend_new = format!(
        "{backend_content}\n#[cfg_attr(test, allow(clippy::unwrap_used))]\nfn cfg_attr_backend_probe() {{}}\n"
    );
    let worker_new = format!(
        "{worker_content}\n#[cfg_attr(feature = \"serde\", allow(clippy::expect_used, clippy::panic))]\nfn cfg_attr_worker_probe() {{}}\n"
    );
    let nested_new = format!(
        "{nested_content}\nmod cfg_attr_probe {{\n    #[cfg_attr(any(test, feature = \"debug-tools\"), allow(clippy::unwrap_used))]\n    pub fn helper() {{}}\n}}\n"
    );

    write_file(root, backend_rel, &backend_new);
    write_file(root, worker_rel, &worker_new);
    write_file(root, nested_rel, &nested_new);

    let backend_line = backend_new
        .lines()
        .position(|line| line.contains("#[cfg_attr(test, allow(clippy::unwrap_used))]"))
        .expect("backend line")
        + 1;
    let worker_line = worker_new
        .lines()
        .position(|line| {
            line.contains(
                "#[cfg_attr(feature = \"serde\", allow(clippy::expect_used, clippy::panic))]",
            )
        })
        .expect("worker line")
        + 1;
    let nested_line = nested_new
        .lines()
        .position(|line| {
            line.contains(
                "#[cfg_attr(any(test, feature = \"debug-tools\"), allow(clippy::unwrap_used))]",
            )
        })
        .expect("nested line")
        + 1;

    let results = run_family(root);
    let mut rs_code_08_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-08")
        .map(|result| {
            (
                result.file.clone().expect("file"),
                result.line,
                format!("{:?}", result.severity),
                result.title.clone(),
                result.message.clone(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();
    rs_code_08_results.sort();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-08"),
        BTreeSet::from([
            backend_rel.to_owned(),
            worker_rel.to_owned(),
            nested_rel.to_owned(),
        ])
    );
    assert_eq!(
        rs_code_08_results,
        vec![
            (
                backend_rel.to_owned(),
                Some(backend_line),
                format!("{:?}", Severity::Info),
                "conditional cfg_attr allow".to_owned(),
                "Conditional cfg_attr allow for `clippy::unwrap_used`.".to_owned(),
                true,
            ),
            (
                nested_rel.to_owned(),
                Some(nested_line),
                format!("{:?}", Severity::Info),
                "conditional cfg_attr allow".to_owned(),
                "Conditional cfg_attr allow for `clippy::unwrap_used`.".to_owned(),
                true,
            ),
            (
                worker_rel.to_owned(),
                Some(worker_line),
                format!("{:?}", Severity::Info),
                "conditional cfg_attr allow".to_owned(),
                "Conditional cfg_attr allow for `clippy::expect_used`.".to_owned(),
                true,
            ),
            (
                worker_rel.to_owned(),
                Some(worker_line),
                format!("{:?}", Severity::Info),
                "conditional cfg_attr allow".to_owned(),
                "Conditional cfg_attr allow for `clippy::panic`.".to_owned(),
                true,
            ),
        ]
    );
}
