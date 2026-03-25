use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_non_library_profiles_and_non_glob_reexports() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let worker_rel = "apps/worker/crates/domain/jobs/src/lib.rs";
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    write_file(
        root,
        worker_rel,
        &format!(
            "{worker_content}\n\npub mod internal {{ pub struct Visible; }}\npub use internal::Visible;\n"
        ),
    );

    let results = run_family(root);
    let rs_code_26_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-26")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-26"), BTreeSet::new());
    assert!(rs_code_26_results.is_empty());
}
