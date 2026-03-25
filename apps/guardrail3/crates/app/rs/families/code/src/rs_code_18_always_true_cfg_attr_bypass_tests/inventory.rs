use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_always_true_cfg_attr_bypasses_across_multiple_owned_files_with_exact_metadata() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let worker_rel = "apps/worker/crates/app/processor/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    let backend_new = format!(
        "{backend_content}\n#[cfg_attr(all(), allow(clippy::unwrap_used))]\nfn cfg_attr_backend_probe() {{}}\nmod nested_cfg_attr_probe {{\n    #[cfg_attr(any(unix, windows), allow(clippy::panic))]\n    pub fn helper() {{}}\n}}\n"
    );
    let worker_new = format!(
        "{worker_content}\nstruct WorkerProbe;\nimpl WorkerProbe {{\n    #[cfg_attr(not(never_target), allow(clippy::expect_used, clippy::panic))]\n    fn cfg_attr_worker_probe(&self) {{}}\n}}\n"
    );

    write_file(root, backend_rel, &backend_new);
    write_file(root, worker_rel, &worker_new);

    let backend_top_line = backend_new
        .lines()
        .position(|line| line.contains("#[cfg_attr(all(), allow(clippy::unwrap_used))]"))
        .expect("backend top line")
        + 1;
    let backend_nested_line = backend_new
        .lines()
        .position(|line| line.contains("#[cfg_attr(any(unix, windows), allow(clippy::panic))]"))
        .expect("backend nested line")
        + 1;
    let worker_line = worker_new
        .lines()
        .position(|line| {
            line.contains(
                "#[cfg_attr(not(never_target), allow(clippy::expect_used, clippy::panic))]",
            )
        })
        .expect("worker line")
        + 1;

    let results = run_family(root);
    let mut rs_code_18_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-18")
        .map(|result| {
            (
                result.file.clone().expect("file"),
                result.line,
                format!("{:?}", result.severity),
                result.title.clone(),
                result.message.clone(),
            )
        })
        .collect::<Vec<_>>();
    rs_code_18_results.sort();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-18"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
    assert_eq!(files_for_rule(&results, "RS-CODE-08"), BTreeSet::new());
    assert_eq!(
        rs_code_18_results,
        vec![
            (
                backend_rel.to_owned(),
                Some(backend_top_line),
                format!("{:?}", Severity::Error),
                "always-true cfg_attr bypass".to_owned(),
                "`#[cfg_attr(..., allow(clippy::unwrap_used))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead."
                    .to_owned(),
            ),
            (
                backend_rel.to_owned(),
                Some(backend_nested_line),
                format!("{:?}", Severity::Error),
                "always-true cfg_attr bypass".to_owned(),
                "`#[cfg_attr(..., allow(clippy::panic))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead."
                    .to_owned(),
            ),
            (
                worker_rel.to_owned(),
                Some(worker_line),
                format!("{:?}", Severity::Error),
                "always-true cfg_attr bypass".to_owned(),
                "`#[cfg_attr(..., allow(clippy::expect_used))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead."
                    .to_owned(),
            ),
            (
                worker_rel.to_owned(),
                Some(worker_line),
                format!("{:?}", Severity::Error),
                "always-true cfg_attr bypass".to_owned(),
                "`#[cfg_attr(..., allow(clippy::panic))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead."
                    .to_owned(),
            ),
        ]
    );
}
