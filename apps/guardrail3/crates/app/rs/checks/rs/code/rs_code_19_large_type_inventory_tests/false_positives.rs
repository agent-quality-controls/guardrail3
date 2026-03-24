use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_types_that_stay_at_or_below_threshold_in_named_tuple_and_unit_forms() {
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
    let mut tuple_fields = String::new();
    for index in 0..15 {
        if index > 0 {
            tuple_fields.push_str(", ");
        }
        tuple_fields.push_str("i32");
    }

    write_file(
        root,
        backend_rel,
        &format!(
            "{backend_content}\n\nstruct PlannerThreshold {{\n{struct_fields}}}\nstruct TupleThreshold({tuple_fields});\nstruct Marker;\n"
        ),
    );
    write_file(
        root,
        worker_rel,
        &format!(
            "{worker_content}\n\nenum QueueThreshold {{\n{enum_variants}}}\nstruct SmallTuple(i32, i32);\n"
        ),
    );

    let results = run_family(root);
    let rs_code_19_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-19")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-19"), BTreeSet::new());
    assert!(rs_code_19_results.is_empty());
}
