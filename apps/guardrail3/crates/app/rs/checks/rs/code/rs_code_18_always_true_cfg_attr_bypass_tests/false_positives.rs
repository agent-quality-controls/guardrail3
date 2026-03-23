use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_genuinely_conditional_cfg_attr_forms() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/commands/src/lib.rs";
    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");

    write_file(
        root,
        backend_rel,
        &format!(
            "{backend_content}\n#[cfg_attr(test, allow(clippy::unwrap_used))]\nfn test_only_probe() {{}}\n#[cfg_attr(feature = \"serde\", allow(clippy::expect_used))]\nfn feature_probe() {{}}\n#[cfg_attr(any(test, feature = \"serde\"), allow(clippy::panic))]\nfn mixed_probe() {{}}\n"
        ),
    );

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-18"), BTreeSet::new());
}
