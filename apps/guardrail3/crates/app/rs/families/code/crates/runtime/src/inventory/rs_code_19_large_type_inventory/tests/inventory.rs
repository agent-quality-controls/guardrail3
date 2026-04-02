use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::inventory::rs_code_19_large_type_inventory::assert_inventories_large_struct_and_enum_shapes_across_owned_files_with_exact_metadata;
use test_support::write_file;

#[test]
fn inventories_large_struct_and_enum_shapes_across_owned_files_with_exact_metadata() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/domain/types/src/lib.rs";
    let worker_rel = "apps/worker/crates/domain/jobs/src/lib.rs";

    let backend_content = test_support::read_file(root, backend_rel);
    let worker_content = test_support::read_file(root, worker_rel);

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
        .map(|index| index + 1)
        .unwrap_or_default();
    let worker_line = worker_new
        .lines()
        .position(|line| line.contains("enum QueueAudit"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_inventories_large_struct_and_enum_shapes_across_owned_files_with_exact_metadata(
        &run_family(root),
        backend_rel,
        worker_rel,
        backend_line,
        worker_line,
    );
}
