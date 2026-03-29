use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_18_always_true_cfg_attr_bypass::assert_attacks_always_true_cfg_attr_bypasses_across_multiple_owned_files_with_exact_metadata;
use test_support::write_file;

#[test]
fn attacks_always_true_cfg_attr_bypasses_across_multiple_owned_files_with_exact_metadata() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let worker_rel = "apps/worker/crates/app/processor/src/lib.rs";

    let backend_content = test_support::read_file(root, backend_rel);
    let worker_content = test_support::read_file(root, worker_rel);

    let backend_new = format!(
        "{backend_content}\n#[cfg_attr(all(), allow(clippy::unwrap_used))]\nfn cfg_attr_backend_probe() {{}}\nmod nested_cfg_attr_probe {{\n    #[cfg_attr(not(any()), allow(clippy::panic))]\n    pub fn helper() {{}}\n}}\n"
    );
    let worker_new = format!(
        "{worker_content}\nstruct WorkerProbe;\nimpl WorkerProbe {{\n    #[cfg_attr(all(), allow(clippy::expect_used, clippy::panic))]\n    fn cfg_attr_worker_probe(&self) {{}}\n}}\n"
    );

    write_file(root, backend_rel, &backend_new);
    write_file(root, worker_rel, &worker_new);

    let backend_top_line = backend_new
        .lines()
        .position(|line| line.contains("#[cfg_attr(all(), allow(clippy::unwrap_used))]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let backend_nested_line = backend_new
        .lines()
        .position(|line| line.contains("#[cfg_attr(not(any()), allow(clippy::panic))]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let worker_line = worker_new
        .lines()
        .position(|line| line.contains("#[cfg_attr(all(), allow(clippy::expect_used, clippy::panic))]"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_attacks_always_true_cfg_attr_bypasses_across_multiple_owned_files_with_exact_metadata(
        &run_family(root),
        backend_rel,
        worker_rel,
        backend_top_line,
        backend_nested_line,
        worker_line,
    );
}
