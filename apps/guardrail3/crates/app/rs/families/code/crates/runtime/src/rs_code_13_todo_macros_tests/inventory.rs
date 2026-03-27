use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn inventories_todo_unimplemented_and_unreachable_macros_in_real_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let worker_rel = "apps/worker/crates/adapters/outbound/db/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    write_file(
        root,
        backend_rel,
        &format!(
            "{backend_content}\nmod nested_probe {{\n    pub fn todo_probe() {{ todo!() }}\n}}\n"
        ),
    );
    write_file(
        root,
        worker_rel,
        &format!("{worker_content}\nfn worker_probe() {{ unimplemented!(); unreachable!(); }}\n"),
    );

    let results = run_family(root);
    let mut rs_code_13_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-13")
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
    rs_code_13_results.sort();

    let backend_new = format!(
        "{backend_content}\nmod nested_probe {{\n    pub fn todo_probe() {{ todo!() }}\n}}\n"
    );
    let worker_new =
        format!("{worker_content}\nfn worker_probe() {{ unimplemented!(); unreachable!(); }}\n");
    let backend_line = backend_new
        .lines()
        .position(|line| line.contains("pub fn todo_probe()"))
        .expect("backend macro line")
        + 1;
    let worker_line = worker_new
        .lines()
        .position(|line| line.contains("fn worker_probe()"))
        .expect("worker macro line")
        + 1;

    assert_eq!(
        files_for_rule(&results, "RS-CODE-13"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
    assert_eq!(
        rs_code_13_results,
        vec![
            (
                backend_rel.to_owned(),
                Some(backend_line),
                format!("{:?}", Severity::Warn),
                "todo! macro".to_owned(),
                "`todo!()` macro found: pub fn todo_probe() { todo!() }.".to_owned(),
            ),
            (
                worker_rel.to_owned(),
                Some(worker_line),
                format!("{:?}", Severity::Info),
                "unreachable! macro".to_owned(),
                "`unreachable!()` macro found: fn worker_probe() { unimplemented!(); unreachable!(); }."
                    .to_owned(),
            ),
            (
                worker_rel.to_owned(),
                Some(worker_line),
                format!("{:?}", Severity::Warn),
                "unimplemented! macro".to_owned(),
                "`unimplemented!()` macro found: fn worker_probe() { unimplemented!(); unreachable!(); }."
                    .to_owned(),
            ),
        ]
    );
}
