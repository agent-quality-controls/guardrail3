use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn warns_on_glob_reexport_in_real_library_lib_rs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let package_rel = "packages/shared-types/src/lib.rs";
    let package_content =
        std::fs::read_to_string(root.join(package_rel)).expect("read package source");

    let mutated = format!(
        "{package_content}\n\nmod internal {{ pub struct Hidden; }}\npub use internal::*;\n"
    );
    write_file(root, package_rel, &mutated);

    let results = run_family(root);
    let glob_line = mutated
        .lines()
        .position(|line| line.contains("pub use internal::*;"))
        .expect("glob re-export line")
        + 1;
    let rs_code_26_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-26")
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
        files_for_rule(&results, "RS-CODE-26"),
        BTreeSet::from([package_rel.to_owned()])
    );
    assert_eq!(
        rs_code_26_results,
        vec![(
            Some(package_rel.to_owned()),
            Some(glob_line),
            Severity::Warn,
            "glob re-export in lib.rs".to_owned(),
            "`pub use internal::*` creates an unstable API surface.".to_owned(),
            false,
        )]
    );
}
