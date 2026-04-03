use super::helpers::copy_fixture;
use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::inventory::rs_code_19_large_type_inventory::assert_no_hits;
use test_support::write_file;

#[test]
fn skips_types_that_stay_at_or_below_threshold_in_named_tuple_and_unit_forms() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/domain/types/src/lib.rs";
    let worker_rel = "apps/worker/crates/domain/jobs/src/lib.rs";

    let backend_content = test_support::read_file(root, backend_rel);
    let worker_content = test_support::read_file(root, worker_rel);

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
    assert_no_hits(&results);
}
