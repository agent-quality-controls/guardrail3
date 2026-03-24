use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_panic_macros_across_real_owned_files_with_exact_metadata() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let worker_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    let backend_new = format!(
        "{backend_content}\nmod nested_panic_probe {{\n    pub fn run() {{ panic!(\"fixups\"); }}\n    pub fn second_run() {{ core::panic!(\"still bad\"); }}\n}}\n"
    );
    let worker_new = format!(
        "{worker_content}\nimpl QueueProbe {{\n    fn queue_panic_probe(&self) {{ panic!(\"queue\"); }}\n}}\n#[cfg(test)]\nmod cfg_probe {{\n    pub fn still_counted() {{ panic!(\"prod-file cfg\"); }}\n}}\n"
    );

    write_file(root, backend_rel, &backend_new);
    write_file(root, worker_rel, &worker_new);

    let backend_first_line = backend_new
        .lines()
        .position(|line| line.contains("pub fn run()"))
        .expect("backend first line")
        + 1;
    let backend_second_line = backend_new
        .lines()
        .position(|line| line.contains("pub fn second_run()"))
        .expect("backend second line")
        + 1;
    let worker_impl_line = worker_new
        .lines()
        .position(|line| line.contains("fn queue_panic_probe(&self)"))
        .expect("worker impl line")
        + 1;
    let worker_cfg_line = worker_new
        .lines()
        .position(|line| line.contains("pub fn still_counted()"))
        .expect("worker cfg line")
        + 1;

    let results = run_family(root);
    let mut rs_code_16_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-16")
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
    rs_code_16_results.sort();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-16"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
    assert_eq!(
        rs_code_16_results,
        vec![
            (
                backend_rel.to_owned(),
                Some(backend_first_line),
                format!("{:?}", Severity::Warn),
                "panic! macro".to_owned(),
                "`panic!()` macro found: pub fn run() { panic!(\"fixups\"); }.".to_owned(),
            ),
            (
                backend_rel.to_owned(),
                Some(backend_second_line),
                format!("{:?}", Severity::Warn),
                "panic! macro".to_owned(),
                "`panic!()` macro found: pub fn second_run() { core::panic!(\"still bad\"); }."
                    .to_owned(),
            ),
            (
                worker_rel.to_owned(),
                Some(worker_cfg_line),
                format!("{:?}", Severity::Warn),
                "panic! macro".to_owned(),
                "`panic!()` macro found: pub fn still_counted() { panic!(\"prod-file cfg\"); }."
                    .to_owned(),
            ),
            (
                worker_rel.to_owned(),
                Some(worker_impl_line),
                format!("{:?}", Severity::Warn),
                "panic! macro".to_owned(),
                "`panic!()` macro found: fn queue_panic_probe(&self) { panic!(\"queue\"); }."
                    .to_owned(),
            ),
        ]
    );
}
