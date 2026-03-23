use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_types_that_stay_exactly_on_the_threshold() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/domain/types/src/lib.rs";
    let worker_rel = "apps/worker/crates/domain/jobs/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    let mut struct_fields = String::new();
    for index in 0..15 {
        struct_fields.push_str(&format!("    field_{index}: i32,\n"));
    }

    let mut enum_variants = String::new();
    for index in 0..20 {
        enum_variants.push_str(&format!("    Variant{index},\n"));
    }

    write_file(
        root,
        backend_rel,
        &format!("{backend_content}\n\nstruct PlannerThreshold {{\n{struct_fields}}}\n"),
    );
    write_file(
        root,
        worker_rel,
        &format!("{worker_content}\n\nenum QueueThreshold {{\n{enum_variants}}}\n"),
    );

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-19"), BTreeSet::new());
}
