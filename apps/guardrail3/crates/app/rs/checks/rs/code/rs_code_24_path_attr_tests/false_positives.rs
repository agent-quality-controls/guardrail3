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
    let rs_code_24_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-24")
        .map(|result| {
            (
                result.file.clone(),
                result.line,
                result.severity,
                result.title.clone(),
                result.message.clone(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();
    let warn_line = rest_content.lines().count() + 2;

    assert_eq!(
        files_for_rule(&results, "RS-CODE-24"),
        BTreeSet::from([rest_rel.to_owned()])
    );
    assert_eq!(
        rs_code_24_results,
        vec![(
            Some(rest_rel.to_owned()),
            Some(warn_line),
            crate::domain::report::Severity::Warn,
            "#[path] usage".to_owned(),
            "#[path = \"generated_inline.rs\"] reason: generated request DTO shim".to_owned(),
            false,
        )]
    );
}
