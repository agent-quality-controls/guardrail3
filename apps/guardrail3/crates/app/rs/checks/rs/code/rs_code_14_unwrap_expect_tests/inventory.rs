use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_unwrap_and_expect_calls_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let worker_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    write_file(
        root,
        backend_rel,
        &format!("{backend_content}\nfn unwrap_probe() {{ let _ = Some(1).unwrap(); }}\n"),
    );
    write_file(
        root,
        worker_rel,
        &format!("{worker_content}\nfn expect_probe() {{ let _ = Some(1).expect(\"queue\"); }}\n"),
    );

    let results = run_family(root);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-14"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
}
