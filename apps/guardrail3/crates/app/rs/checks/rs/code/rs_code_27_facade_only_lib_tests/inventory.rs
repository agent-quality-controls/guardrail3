use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn errors_on_non_facade_items_in_real_library_lib_rs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content =
        std::fs::read_to_string(root.join(package_rel)).expect("read package source");

    let mutated =
        format!("{package_content}\n\nuse crate::TenantSlug;\npub fn mutate_surface() {{}}\n");
    write_file(root, package_rel, &mutated);

    let results = run_family(root);
    let mut rs_code_27_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-27")
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
    rs_code_27_results.sort_by_key(|(file, line, severity, _, _, _)| {
        (
            file.clone().unwrap_or_default(),
            *line,
            format!("{severity:?}"),
        )
    });
    let use_line = mutated
        .lines()
        .position(|line| line.contains("use crate::TenantSlug;"))
        .expect("private use line")
        + 1;
    let fn_line = mutated
        .lines()
        .position(|line| line.contains("pub fn mutate_surface()"))
        .expect("function line")
        + 1;

    assert_eq!(
        files_for_rule(&results, "RS-CODE-27"),
        BTreeSet::from([package_rel.to_owned()])
    );
    assert_eq!(
        rs_code_27_results,
        vec![
            (
                Some(package_rel.to_owned()),
                Some(use_line),
                Severity::Error,
                "lib.rs should stay facade-only".to_owned(),
                "lib.rs contains private use `crate::TenantSlug`. Keep lib.rs limited to facade declarations and type/const definitions.".to_owned(),
                false,
            ),
            (
                Some(package_rel.to_owned()),
                Some(fn_line),
                Severity::Error,
                "lib.rs should stay facade-only".to_owned(),
                "lib.rs contains function `mutate_surface`. Keep lib.rs limited to facade declarations and type/const definitions.".to_owned(),
                false,
            ),
        ]
    );
}
