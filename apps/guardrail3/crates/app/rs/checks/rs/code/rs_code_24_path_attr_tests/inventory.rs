use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};

#[test]
fn attacks_path_attr_boundary_changes_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rest_rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let handlers_rel = "apps/backend/crates/adapters/inbound/mcp/crates/app/handlers/src/lib.rs";

    let rest_content = std::fs::read_to_string(root.join(rest_rel)).expect("read rest source");
    let handlers_content =
        std::fs::read_to_string(root.join(handlers_rel)).expect("read handlers source");

    let rest_warn_line = rest_content.lines().count() + 2;
    let handlers_error_line = handlers_content.lines().count() + 2;

    write_file(
        root,
        rest_rel,
        &format!(
            "{rest_content}\n#[path = \"generated_rest.rs\"] // reason: generated transport shim\nmod generated_rest;\n"
        ),
    );
    write_file(
        root,
        handlers_rel,
        &format!("{handlers_content}\n#[path = \"../shared_handlers.rs\"]\nmod shared_handlers;\n"),
    );

    let results = run_family(root);
    let rs_code_24_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-24")
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-24"),
        BTreeSet::from([rest_rel.to_owned(), handlers_rel.to_owned()])
    );
    assert_eq!(rs_code_24_results.len(), 2);
    assert_eq!(
        rs_code_24_results
            .iter()
            .map(|result| (result.file.as_deref(), result.line, result.severity))
            .collect::<Vec<_>>(),
        vec![
            (Some(rest_rel), Some(rest_warn_line), Severity::Warn),
            (
                Some(handlers_rel),
                Some(handlers_error_line),
                Severity::Error
            ),
        ]
    );
}
