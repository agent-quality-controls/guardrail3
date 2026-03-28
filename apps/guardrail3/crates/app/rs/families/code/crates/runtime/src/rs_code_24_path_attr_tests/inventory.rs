use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_24_path_attr::{
    RuleFinding, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn attacks_path_attr_boundary_changes_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rest_rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let handlers_rel = "apps/backend/crates/adapters/inbound/mcp/crates/app/handlers/src/lib.rs";

    let rest_content = test_support::read_file(root, rest_rel);
    let handlers_content = test_support::read_file(root, handlers_rel);

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
    assert_files(
        &results,
        BTreeSet::from([rest_rel.to_owned(), handlers_rel.to_owned()]),
    );
    assert_findings(
        &results,
        &[
            RuleFinding {
                severity: Severity::Error,
                title: "#[path] escapes parent directory",
                message: "`#[path = \"../shared_handlers.rs\"]` escapes the standard module boundary.",
                file: Some(handlers_rel),
                line: Some(handlers_error_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Warn,
                title: "#[path] usage",
                message: "#[path = \"generated_rest.rs\"] reason: generated transport shim",
                file: Some(rest_rel),
                line: Some(rest_warn_line),
                inventory: false,
            },
        ],
    );
}
