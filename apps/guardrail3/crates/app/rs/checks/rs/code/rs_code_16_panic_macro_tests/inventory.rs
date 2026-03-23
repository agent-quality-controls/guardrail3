use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_panic_macros_across_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let worker_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    write_file(
        root,
        backend_rel,
        &format!("{backend_content}\nfn panic_probe() {{ panic!(\"fixups\"); }}\n"),
    );
    write_file(
        root,
        worker_rel,
        &format!("{worker_content}\nfn queue_panic_probe() {{ panic!(\"queue\"); }}\n"),
    );

    let results = run_family(root);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-16"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
}
