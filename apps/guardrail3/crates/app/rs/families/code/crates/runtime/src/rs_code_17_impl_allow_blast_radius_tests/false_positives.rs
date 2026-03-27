use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_threshold_method_level_and_non_impl_allow_near_misses() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let worker_rel = "apps/worker/crates/app/processor/src/lib.rs";
    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let worker_content =
        std::fs::read_to_string(root.join(worker_rel)).expect("read worker source");

    write_file(
        root,
        backend_rel,
        &format!(
            "{backend_content}\n\n#[allow(clippy::too_many_lines)]\nstruct NotAnImpl;\n\nstruct ThresholdImpl;\n#[allow(clippy::too_many_lines)]\nimpl ThresholdImpl {{\n    const LIMIT: usize = 3;\n    fn first(&self) {{}}\n    fn second(&self) {{}}\n    fn third(&self) {{}}\n    type Output = ();\n}}\n"
        ),
    );
    write_file(
        root,
        worker_rel,
        &format!(
            "{worker_content}\n\nstruct MethodScoped;\nimpl MethodScoped {{\n    #[allow(clippy::too_many_lines)]\n    fn first(&self) {{}}\n    fn second(&self) {{}}\n    fn third(&self) {{}}\n    fn fourth(&self) {{}}\n}}\n"
        ),
    );

    let results = run_family(root);
    let rs_code_17_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-17")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-17"), BTreeSet::new());
    assert!(rs_code_17_results.is_empty());
}
