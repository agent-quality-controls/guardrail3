use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_direct_std_fs_imports_and_calls_in_real_owned_files_with_exact_metadata() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let worker_rel = "apps/worker/crates/adapters/outbound/db/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    let backend_new = format!(
        "{backend_content}\nmod nested_fs_probe {{\n    use std::fs;\n    pub fn read_probe() {{\n        let _ = std::fs::read_to_string(\"backend.txt\");\n    }}\n}}\n"
    );
    let worker_new = format!(
        "{worker_content}\nuse std::{{fs, io}}; fn fs_call_probe() {{ let _ = std::fs::read_to_string(\"jobs.txt\"); }}\n"
    );

    write_file(root, backend_rel, &backend_new);
    write_file(root, worker_rel, &worker_new);

    let backend_import_line = backend_new
        .lines()
        .position(|line| line.trim() == "use std::fs;")
        .expect("backend import line")
        + 1;
    let backend_call_line = backend_new
        .lines()
        .position(|line| line.contains("std::fs::read_to_string(\"backend.txt\")"))
        .expect("backend call line")
        + 1;
    let worker_import_line = worker_new
        .lines()
        .position(|line| line.contains("use std::{fs, io};"))
        .expect("worker import line")
        + 1;
    let results = run_family(root);
    let mut rs_code_15_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-15")
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
    rs_code_15_results.sort();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-15"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
    assert_eq!(
        rs_code_15_results,
        vec![
            (
                backend_rel.to_owned(),
                Some(backend_import_line),
                format!("{:?}", Severity::Error),
                "direct std::fs import".to_owned(),
                "Direct `use std::fs` import found: `use std::fs;`.".to_owned(),
            ),
            (
                backend_rel.to_owned(),
                Some(backend_call_line),
                format!("{:?}", Severity::Error),
                "direct std::fs call".to_owned(),
                "Direct `std::fs::*` call found: `let _ = std::fs::read_to_string(\"backend.txt\");`."
                    .to_owned(),
            ),
            (
                worker_rel.to_owned(),
                Some(worker_import_line),
                format!("{:?}", Severity::Error),
                "direct std::fs import".to_owned(),
                "Direct `use std::fs` import found: `use std::{fs, io}; fn fs_call_probe() { let _ = std::fs::read_to_string(\"jobs.txt\"); }`."
                    .to_owned(),
            ),
        ]
    );
}

#[test]
fn prefers_import_hit_when_import_and_call_share_one_line() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let content = std::fs::read_to_string(root.join(rel)).expect("read source");
    let new_content = format!(
        "{content}\nuse std::fs; fn same_line_probe() {{ let _ = std::fs::read_to_string(\"same-line.txt\"); }}\n"
    );
    write_file(root, rel, &new_content);

    let line = new_content
        .lines()
        .position(|candidate| candidate.contains("use std::fs; fn same_line_probe()"))
        .expect("same line")
        + 1;

    let results = run_family(root);
    let rs_code_15_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-15" && result.file.as_deref() == Some(rel))
        .map(|result| (result.line, result.title.clone(), result.message.clone()))
        .collect::<Vec<_>>();

    assert_eq!(
        rs_code_15_results,
        vec![(
            Some(line),
            "direct std::fs import".to_owned(),
            "Direct `use std::fs` import found: `use std::fs; fn same_line_probe() { let _ = std::fs::read_to_string(\"same-line.txt\"); }`."
                .to_owned(),
        )]
    );
}
