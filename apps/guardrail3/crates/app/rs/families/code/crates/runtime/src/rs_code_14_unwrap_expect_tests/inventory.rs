use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_unwrap_and_expect_calls_across_real_owned_files_with_exact_metadata() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let worker_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    let backend_new = format!(
        "{backend_content}\nfn unwrap_probe() {{ let _ = Some(1).unwrap(); }}\nfn expect_probe() {{ let _ = Some(1).expect(\"backend\"); }}\n"
    );
    let worker_new = format!(
        "{worker_content}\nmod nested_method_probe {{\n    pub fn run() {{\n        let _ = Some(1).expect(\"queue\");\n        let _ = Some(1).unwrap();\n    }}\n}}\n"
    );

    write_file(root, backend_rel, &backend_new);
    write_file(root, worker_rel, &worker_new);

    let backend_unwrap_line = backend_new
        .lines()
        .position(|line| line.contains("fn unwrap_probe()"))
        .expect("backend unwrap line")
        + 1;
    let backend_expect_line = backend_new
        .lines()
        .position(|line| line.contains("fn expect_probe()"))
        .expect("backend expect line")
        + 1;
    let worker_expect_line = worker_new
        .lines()
        .position(|line| line.contains("let _ = Some(1).expect(\"queue\");"))
        .expect("worker expect line")
        + 1;
    let worker_unwrap_line = worker_new
        .lines()
        .position(|line| line.contains("let _ = Some(1).unwrap();"))
        .expect("worker unwrap line")
        + 1;

    let results = run_family(root);
    let mut rs_code_14_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-14")
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
    rs_code_14_results.sort();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-14"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
    assert_eq!(
        rs_code_14_results,
        vec![
            (
                backend_rel.to_owned(),
                Some(backend_unwrap_line),
                format!("{:?}", Severity::Warn),
                ".unwrap() usage".to_owned(),
                "`.unwrap()` found: fn unwrap_probe() { let _ = Some(1).unwrap(); }.".to_owned(),
            ),
            (
                backend_rel.to_owned(),
                Some(backend_expect_line),
                format!("{:?}", Severity::Warn),
                ".expect() usage".to_owned(),
                "`.expect()` found: fn expect_probe() { let _ = Some(1).expect(\"backend\"); }."
                    .to_owned(),
            ),
            (
                worker_rel.to_owned(),
                Some(worker_expect_line),
                format!("{:?}", Severity::Warn),
                ".expect() usage".to_owned(),
                "`.expect()` found: let _ = Some(1).expect(\"queue\");.".to_owned(),
            ),
            (
                worker_rel.to_owned(),
                Some(worker_unwrap_line),
                format!("{:?}", Severity::Warn),
                ".unwrap() usage".to_owned(),
                "`.unwrap()` found: let _ = Some(1).unwrap();.".to_owned(),
            ),
        ]
    );
}
