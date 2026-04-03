use super::helpers::copy_fixture;
use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::cfg_and_paths::rs_code_17_impl_allow_blast_radius::assert_attacks_impl_level_allows_across_multiple_owned_rust_files_with_exact_metadata;
use test_support::write_file;

#[test]
fn attacks_impl_level_allows_across_multiple_owned_rust_files_with_exact_metadata() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let worker_rel = "apps/worker/crates/app/processor/src/lib.rs";

    let backend_content = test_support::read_file(root, backend_rel);
    let worker_content = test_support::read_file(root, worker_rel);

    let backend_new = format!(
        "{backend_content}\n\nstruct ImplAudit;\n#[allow(clippy::too_many_lines)]\nimpl ImplAudit {{\n    fn first(&self) {{}}\n    fn second(&self) {{}}\n    fn third(&self) {{}}\n    fn fourth(&self) {{}}\n}}\n"
    );
    let worker_new = format!(
        "{worker_content}\n\nstruct QueueAudit;\n#[allow(clippy::too_many_arguments, clippy::too_many_lines)]\nimpl QueueAudit {{\n    fn first(&self) {{}}\n    fn second(&self) {{}}\n    fn third(&self) {{}}\n    fn fourth(&self) {{}}\n    fn fifth(&self) {{}}\n}}\n\nstruct SecondaryAudit;\n#[allow(clippy::type_complexity)]\nimpl SecondaryAudit {{\n    fn first(&self) {{}}\n    fn second(&self) {{}}\n    fn third(&self) {{}}\n    fn fourth(&self) {{}}\n}}\n"
    );

    write_file(root, backend_rel, &backend_new);
    write_file(root, worker_rel, &worker_new);

    let backend_line = backend_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::too_many_lines)]"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let worker_grouped_line = worker_new
        .lines()
        .position(|line| {
            line.contains("#[allow(clippy::too_many_arguments, clippy::too_many_lines)]")
        })
        .map(|index| index + 1)
        .unwrap_or_default();
    let worker_secondary_line = worker_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::type_complexity)]"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_attacks_impl_level_allows_across_multiple_owned_rust_files_with_exact_metadata(
        &run_family(root),
        backend_rel,
        worker_rel,
        backend_line,
        worker_grouped_line,
        worker_secondary_line,
    );
}
