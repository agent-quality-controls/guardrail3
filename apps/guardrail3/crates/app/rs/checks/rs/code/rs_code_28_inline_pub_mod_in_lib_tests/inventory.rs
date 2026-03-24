use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn warns_on_inline_public_module_in_real_library_lib_rs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content =
        std::fs::read_to_string(root.join(package_rel)).expect("read package source");

    let mutated = format!("{package_content}\n\npub mod api {{ pub fn ping() {{}} }}\n");
    write_file(root, package_rel, &mutated);

    let results = run_family(root);
    let inline_line = mutated
        .lines()
        .position(|line| line.contains("pub mod api"))
        .expect("inline pub mod line")
        + 1;
    let rs_code_28_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-28")
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

    assert_eq!(
        files_for_rule(&results, "RS-CODE-28"),
        BTreeSet::from([package_rel.to_owned()])
    );
    assert_eq!(
        rs_code_28_results,
        vec![(
            Some(package_rel.to_owned()),
            Some(inline_line),
            Severity::Warn,
            "inline public module in lib.rs".to_owned(),
            "`pub mod api { ... }` should live in its own file.".to_owned(),
            false,
        )]
    );
}
