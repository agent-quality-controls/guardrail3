use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::inventory::rs_code_29_large_trait_inventory::assert_no_hits;
use test_support::write_file;

#[test]
fn skips_non_library_traits_and_threshold_boundary() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let worker_rel = "apps/worker/crates/domain/jobs/src/lib.rs";
    let worker_content = test_support::read_file(root, worker_rel);

    let mut methods = String::new();
    for index in 0..8 {
        methods.push_str(&format!("    fn m{index}(&self);\n"));
    }

    write_file(
        root,
        worker_rel,
        &format!("{worker_content}\n\npub trait WorkerSurface {{\n{methods}}}\n"),
    );

    let results = run_family(root);
    assert_no_hits(&results);
}
