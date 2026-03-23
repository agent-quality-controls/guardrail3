use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn skips_same_line_reasoned_non_escaping_path_attrs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rest_rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let rest_content = std::fs::read_to_string(root.join(rest_rel)).expect("read rest source");

    write_file(
        root,
        rest_rel,
        &format!(
            "{rest_content}\n#[path = \"generated_inline.rs\"] // reason: generated request DTO shim\nmod generated_inline;\n"
        ),
    );

    let results = run_family(root);
    let rule_files = files_for_rule(&results, "RS-CODE-24");
    assert_eq!(rule_files, BTreeSet::from([rest_rel.to_owned()]));
    assert_eq!(
        results
            .iter()
            .filter(|result| result.id == "RS-CODE-24")
            .count(),
        1
    );
}
