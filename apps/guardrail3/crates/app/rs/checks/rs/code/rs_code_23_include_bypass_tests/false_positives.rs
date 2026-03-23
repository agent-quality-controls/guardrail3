use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn ignores_non_traversing_include_str_without_rust_include_bypass() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rest_rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let rest_content = std::fs::read_to_string(root.join(rest_rel)).expect("read rest source");

    write_file(
        root,
        rest_rel,
        &format!(
            "{rest_content}\nconst LOCAL_TEMPLATE: &str = include_str!(\"embedded_schema.json\");\n"
        ),
    );

    let results = run_family(root);

    assert_eq!(files_for_rule(&results, "RS-CODE-23"), BTreeSet::new());
}
