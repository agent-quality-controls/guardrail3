use guardrail3_app_rs_family_code_assertions::rs_code_16_panic_macro::assert_attacks_panic_macros_across_real_owned_files_with_exact_metadata;
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn attacks_panic_macros_across_real_owned_files_with_exact_metadata() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/devctl/crates/app/core/src/lib.rs";
    let worker_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";

    let backend_content =
        test_support::read_file(root, backend_rel);
    let worker_content =
        test_support::read_file(root, worker_rel);

    let backend_new = format!(
        "{backend_content}\nmod nested_panic_probe {{\n    pub fn run() {{ panic!(\"fixups\"); }}\n    pub fn second_run() {{ core::panic!(\"still bad\"); }}\n}}\n"
    );
    let worker_new = format!(
        "{worker_content}\nimpl QueueProbe {{\n    fn queue_panic_probe(&self) {{ panic!(\"queue\"); }}\n}}\n#[cfg(test)]\nmod cfg_probe {{\n    pub fn still_counted() {{ panic!(\"prod-file cfg\"); }}\n}}\n"
    );

    write_file(root, backend_rel, &backend_new);
    write_file(root, worker_rel, &worker_new);

    let backend_first_line = backend_new
        .lines()
        .position(|line| line.contains("pub fn run()")).map(|index| index + 1).unwrap_or_default();
    let backend_second_line = backend_new
        .lines()
        .position(|line| line.contains("pub fn second_run()")).map(|index| index + 1).unwrap_or_default();
    let worker_impl_line = worker_new
        .lines()
        .position(|line| line.contains("fn queue_panic_probe(&self)")).map(|index| index + 1).unwrap_or_default();
    let worker_cfg_line = worker_new
        .lines()
        .position(|line| line.contains("pub fn still_counted()")).map(|index| index + 1).unwrap_or_default();

    assert_attacks_panic_macros_across_real_owned_files_with_exact_metadata(
        &run_family(root),
        backend_rel,
        worker_rel,
        backend_first_line,
        backend_second_line,
        worker_impl_line,
        worker_cfg_line,
    );
}
