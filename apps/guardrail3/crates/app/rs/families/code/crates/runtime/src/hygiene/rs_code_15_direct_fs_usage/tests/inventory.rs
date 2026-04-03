use super::helpers::copy_fixture;
use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::hygiene::rs_code_15_direct_fs_usage::{
    assert_attacks_direct_std_fs_imports_and_calls_in_real_owned_files_with_exact_metadata,
    assert_prefers_import_hit_when_import_and_call_share_one_line,
};
use test_support::write_file;

#[test]
fn attacks_direct_std_fs_imports_and_calls_in_real_owned_files_with_exact_metadata() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let worker_rel = "apps/worker/crates/adapters/outbound/db/src/lib.rs";

    let backend_content = test_support::read_file(root, backend_rel);
    let worker_content = test_support::read_file(root, worker_rel);

    let backend_new = format!(
        "{backend_content}\nmod nested_fs_probe {{\n    use std::fs;\n    pub fn read_probe() {{\n        let _ = std::fs::read_to_string(\"backend.txt\");\n    }}\n}}\n"
    );
    let worker_new = format!(
        "{worker_content}\nuse std::{{fs, io}}; fn fs_call_probe() {{ let _ = std::fs::read_to_string(\"jobs.txt\"); }}\n"
    );

    write_file(root, backend_rel, &backend_new);
    write_file(root, worker_rel, &worker_new);

    let backend_import_line = backend_new
        .lines()
        .position(|line| line.trim() == "use std::fs;")
        .map(|index| index + 1)
        .unwrap_or_default();
    let backend_call_line = backend_new
        .lines()
        .position(|line| line.contains("std::fs::read_to_string(\"backend.txt\")"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let worker_import_line = worker_new
        .lines()
        .position(|line| line.contains("use std::{fs, io};"))
        .map(|index| index + 1)
        .unwrap_or_default();
    assert_attacks_direct_std_fs_imports_and_calls_in_real_owned_files_with_exact_metadata(
        &run_family(root),
        backend_rel,
        worker_rel,
        backend_import_line,
        backend_call_line,
        worker_import_line,
    );
}

#[test]
fn prefers_import_hit_when_import_and_call_share_one_line() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let content = test_support::read_file(root, rel);
    let new_content = format!(
        "{content}\nuse std::fs; fn same_line_probe() {{ let _ = std::fs::read_to_string(\"same-line.txt\"); }}\n"
    );
    write_file(root, rel, &new_content);

    let line = new_content
        .lines()
        .position(|candidate| candidate.contains("use std::fs; fn same_line_probe()"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_prefers_import_hit_when_import_and_call_share_one_line(&run_family(root), rel, line);
}
