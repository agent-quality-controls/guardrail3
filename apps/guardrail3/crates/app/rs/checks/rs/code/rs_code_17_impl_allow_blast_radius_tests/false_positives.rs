use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn allows_impl_level_suppression_when_blast_radius_stays_at_threshold() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");

    write_file(
        root,
        backend_rel,
        &format!(
            "{backend_content}\n\nstruct ThresholdImpl;\n#[allow(clippy::too_many_lines)]\nimpl ThresholdImpl {{\n    fn first(&self) {{}}\n    fn second(&self) {{}}\n    fn third(&self) {{}}\n}}\n"
        ),
    );

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-17"), BTreeSet::new());
}
