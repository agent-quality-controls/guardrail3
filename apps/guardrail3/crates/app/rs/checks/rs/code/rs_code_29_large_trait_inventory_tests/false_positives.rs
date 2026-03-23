use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_non_library_traits_and_threshold_boundary() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let worker_rel = "apps/worker/crates/domain/jobs/src/lib.rs";
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

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

    assert_eq!(files_for_rule(&results, "RS-CODE-29"), BTreeSet::new());
}
