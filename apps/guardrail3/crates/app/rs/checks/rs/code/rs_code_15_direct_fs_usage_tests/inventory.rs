use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_direct_std_fs_imports_and_calls_in_real_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let worker_rel = "apps/worker/crates/adapters/outbound/db/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    write_file(
        root,
        backend_rel,
        &format!("use std::fs;\n{backend_content}"),
    );
    write_file(
        root,
        worker_rel,
        &format!(
            "{worker_content}\nfn fs_call_probe() {{ let _ = std::fs::read_to_string(\"jobs.txt\"); }}\n"
        ),
    );

    let results = run_family(root);

    assert_eq!(
        files_for_rule(&results, "RS-CODE-15"),
        BTreeSet::from([backend_rel.to_owned(), worker_rel.to_owned()])
    );
}
