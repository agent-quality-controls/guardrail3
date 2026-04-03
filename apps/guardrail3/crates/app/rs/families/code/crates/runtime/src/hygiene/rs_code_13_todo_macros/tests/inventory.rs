use super::helpers::copy_fixture;
use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::hygiene::rs_code_13_todo_macros::assert_inventories_todo_unimplemented_and_unreachable_macros_in_real_files;
use test_support::write_file;

#[test]
fn inventories_todo_unimplemented_and_unreachable_macros_in_real_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let worker_rel = "apps/worker/crates/adapters/outbound/db/src/lib.rs";

    let backend_content = test_support::read_file(root, backend_rel);
    let worker_content = test_support::read_file(root, worker_rel);

    write_file(
        root,
        backend_rel,
        &format!(
            "{backend_content}\nmod nested_probe {{\n    pub fn todo_probe() {{ todo!() }}\n}}\n"
        ),
    );
    write_file(
        root,
        worker_rel,
        &format!("{worker_content}\nfn worker_probe() {{ unimplemented!(); unreachable!(); }}\n"),
    );

    let backend_new = format!(
        "{backend_content}\nmod nested_probe {{\n    pub fn todo_probe() {{ todo!() }}\n}}\n"
    );
    let worker_new =
        format!("{worker_content}\nfn worker_probe() {{ unimplemented!(); unreachable!(); }}\n");
    let backend_line = backend_new
        .lines()
        .position(|line| line.contains("pub fn todo_probe()"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let worker_line = worker_new
        .lines()
        .position(|line| line.contains("fn worker_probe()"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_inventories_todo_unimplemented_and_unreachable_macros_in_real_files(
        &run_family(root),
        backend_rel,
        worker_rel,
        backend_line,
        worker_line,
    );
}
