use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

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
    let mut rs_code_24_results = results
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
    rs_code_24_results.sort_by_key(|(file, line, severity, _, _, _)| {
        (
            file.clone().unwrap_or_default(),
            *line,
            format!("{severity:?}"),
        )
    });

    assert_eq!(
        files_for_rule(&results, "RS-CODE-24"),
        BTreeSet::from([rest_rel.to_owned(), handlers_rel.to_owned()])
    );
    assert_eq!(
        rs_code_24_results,
        vec![
            (
                Some(handlers_rel.to_owned()),
                Some(handlers_error_line),
                Severity::Error,
                "#[path] escapes parent directory".to_owned(),
                "`#[path = \"../shared_handlers.rs\"]` escapes the standard module boundary."
                    .to_owned(),
                false,
            ),
            (
                Some(rest_rel.to_owned()),
                Some(rest_warn_line),
                Severity::Warn,
                "#[path] usage".to_owned(),
                "#[path = \"generated_rest.rs\"] reason: generated transport shim".to_owned(),
                false,
            ),
        ]
    );
}
