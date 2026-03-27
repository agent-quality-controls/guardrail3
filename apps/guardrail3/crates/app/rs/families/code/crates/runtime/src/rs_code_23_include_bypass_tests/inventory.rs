use std::collections::BTreeSet;

use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_23_include_bypass::{
    assert_files,
    assert_findings,
    RuleFinding,
};
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn attacks_include_bypass_variants_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rest_rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let handlers_rel = "apps/backend/crates/adapters/inbound/mcp/crates/app/handlers/src/lib.rs";

    let rest_content = test_support::read_file(root, rest_rel);
    let handlers_content =
        test_support::read_file(root, handlers_rel);

    let rest_line = rest_content.lines().count() + 2;
    let handlers_info_line = handlers_content.lines().count() + 2;
    let handlers_warn_line = handlers_content.lines().count() + 3;

    write_file(
        root,
        rest_rel,
        &format!("{rest_content}\ninclude!(\"../generated_rest.rs\");\n"),
    );
    write_file(
        root,
        handlers_rel,
        &format!(
            "{handlers_content}\ninclude!(concat!(env!(\"OUT_DIR\"), \"/generated_handlers.rs\"));\nconst MCP_SCHEMA: &str = include_str!(\"../schema.json\");\n"
        ),
    );

    let results = run_family(root);
    assert_files(&results, BTreeSet::from([rest_rel.to_owned(), handlers_rel.to_owned()]));
    assert_findings(
        &results,
        &[
            RuleFinding {
                severity: Severity::Info,
                title: "build-script include! inventory",
                message: "`include!(concat!(env!(\"OUT_DIR\"), ...))` detected. Review generated-code boundary.",
                file: Some(handlers_rel),
                line: Some(handlers_info_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Warn,
                title: "include path traversal",
                message: "`include_str!()` uses a path containing `..`.",
                file: Some(handlers_rel),
                line: Some(handlers_warn_line),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "include! bypass",
                message: "`include!()` pulls in Rust code outside the scanned file boundary.",
                file: Some(rest_rel),
                line: Some(rest_line),
                inventory: false,
            },
        ],
    );
}
