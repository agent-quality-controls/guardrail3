use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn ignores_allow_attrs_that_do_not_cover_extern_blocks() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let api_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let api_content = std::fs::read_to_string(root.join(api_rel)).expect("read api source");

    write_file(
        root,
        api_rel,
        &format!("{api_content}\n#[allow(dead_code)]\nfn local_probe() {{}}\n"),
    );

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-20"), BTreeSet::new());
}
