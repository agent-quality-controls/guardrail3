use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn inventories_large_struct_and_enum_shapes_across_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/domain/types/src/lib.rs";
    let worker_rel = "apps/worker/crates/domain/jobs/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    let backend_line = backend_content.lines().count() + 2;
    let worker_line = worker_content.lines().count() + 2;

    let mut struct_fields = String::new();
    for index in 0..16 {
        struct_fields.push_str(&format!("    field_{index}: i32,\n"));
    }

    let mut enum_variants = String::new();
    for index in 0..21 {
        enum_variants.push_str(&format!("    Variant{index},\n"));
    }

    write_file(
        root,
        backend_rel,
        &format!("{backend_content}\n\nstruct PlannerAudit {{\n{struct_fields}}}\n"),
    );
    write_file(
        root,
        worker_rel,
        &format!("{worker_content}\n\nenum QueueAudit {{\n{enum_variants}}}\n"),
    );

    let results = run_family(root);
    let rs_code_19_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-19")
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-19"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
    assert_eq!(rs_code_19_results.len(), 2);
    assert_eq!(
        rs_code_19_results
            .iter()
            .map(|result| (result.file.as_deref(), result.line))
            .collect::<Vec<_>>(),
        vec![
            (Some(backend_rel), Some(backend_line)),
            (Some(worker_rel), Some(worker_line)),
        ]
    );
}
