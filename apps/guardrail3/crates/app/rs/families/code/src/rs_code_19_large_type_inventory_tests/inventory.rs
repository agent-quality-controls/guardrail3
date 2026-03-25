use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn inventories_large_struct_and_enum_shapes_across_owned_files_with_exact_metadata() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/domain/types/src/lib.rs";
    let worker_rel = "apps/worker/crates/domain/jobs/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    let mut struct_fields = String::new();
    for index in 0..16 {
        struct_fields.push_str(&format!("    field_{index}: i32,\n"));
    }

    let mut enum_variants = String::new();
    for index in 0..21 {
        enum_variants.push_str(&format!("    Variant{index},\n"));
    }

    let backend_new = format!("{backend_content}\n\nstruct PlannerAudit {{\n{struct_fields}}}\n");
    let worker_new = format!("{worker_content}\n\nenum QueueAudit {{\n{enum_variants}}}\n");

    write_file(root, backend_rel, &backend_new);
    write_file(root, worker_rel, &worker_new);

    let backend_line = backend_new
        .lines()
        .position(|line| line.contains("struct PlannerAudit"))
        .expect("backend line")
        + 1;
    let worker_line = worker_new
        .lines()
        .position(|line| line.contains("enum QueueAudit"))
        .expect("worker line")
        + 1;

    let results = run_family(root);
    let mut rs_code_19_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-19")
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
    rs_code_19_results.sort();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-19"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
    assert_eq!(
        rs_code_19_results,
        vec![
            (
                backend_rel.to_owned(),
                Some(backend_line),
                format!("{:?}", Severity::Info),
                "large type inventory".to_owned(),
                "struct `PlannerAudit` has 16 fields (inventory threshold 15).".to_owned(),
                true,
            ),
            (
                worker_rel.to_owned(),
                Some(worker_line),
                format!("{:?}", Severity::Info),
                "large type inventory".to_owned(),
                "enum `QueueAudit` has 21 items (inventory threshold 20).".to_owned(),
                true,
            ),
        ]
    );
}
