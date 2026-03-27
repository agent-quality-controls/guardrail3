use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_08_cfg_attr_allow_inventory::{assert_files, assert_findings, RuleFinding};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

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

    assert_files(&results, BTreeSet::from([
            backend_rel.to_owned(),
            worker_rel.to_owned(),
            nested_rel.to_owned(),
        ]));
    assert_findings(
        &results,
        &[
            RuleFinding {
                severity: Severity::Info,
                title: "conditional cfg_attr allow",
                message: "Conditional cfg_attr allow for `clippy::unwrap_used`.",
                file: Some(backend_rel),
                line: Some(backend_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "conditional cfg_attr allow",
                message: "Conditional cfg_attr allow for `clippy::unwrap_used`.",
                file: Some(nested_rel),
                line: Some(nested_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "conditional cfg_attr allow",
                message: "Conditional cfg_attr allow for `clippy::expect_used`.",
                file: Some(worker_rel),
                line: Some(worker_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "conditional cfg_attr allow",
                message: "Conditional cfg_attr allow for `clippy::panic`.",
                file: Some(worker_rel),
                line: Some(worker_line),
                inventory: true,
            },
        ],
    );
}
