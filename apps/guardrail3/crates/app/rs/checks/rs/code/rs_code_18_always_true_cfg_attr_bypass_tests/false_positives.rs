use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_genuinely_conditional_cfg_attr_forms() {
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
            "{backend_content}\n#[cfg_attr(test, allow(clippy::unwrap_used))]\nfn test_only_probe() {{}}\n#[cfg_attr(feature = \"serde\", allow(clippy::expect_used))]\nfn feature_probe() {{}}\n#[cfg_attr(any(test, feature = \"serde\"), allow(clippy::panic))]\nfn mixed_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        worker_rel,
        &format!(
            "{worker_content}\nstruct ImplProbe;\nimpl ImplProbe {{\n    #[cfg_attr(feature = \"debug-tools\", allow(clippy::too_many_lines))]\n    fn method_probe(&self) {{}}\n}}\n"
        ),
    );

    let results = run_family(root);
    let rs_code_18_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-18")
        .collect::<Vec<_>>();

    assert_eq!(files_for_rule(&results, "RS-CODE-18"), BTreeSet::new());
    assert!(rs_code_18_results.is_empty());
}
