use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_23_include_bypass::{
    RuleFinding, Severity, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn attacks_include_bypass_variants_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rest_rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let handlers_rel = "apps/backend/crates/adapters/inbound/mcp/crates/app/handlers/src/lib.rs";

    let rest_content = test_support::read_file(root, rest_rel);
    let handlers_content = test_support::read_file(root, handlers_rel);

    let rest_line = rest_content.lines().count() + 2;
    let handlers_info_line = handlers_content.lines().count() + 2;
    let handlers_warn_line = handlers_content.lines().count() + 3;
    let handlers_build_warn_line = handlers_content.lines().count() + 4;

    write_file(
        root,
        rest_rel,
        &format!("{rest_content}\ninclude!(\"../generated_rest.rs\");\n"),
    );
    write_file(
        root,
        handlers_rel,
        &format!(
            "{handlers_content}\ninclude!(concat!(env!(\"OUT_DIR\"), \"/generated_handlers.rs\"));\nconst MCP_SCHEMA: &str = include_str!(\"../schema.json\");\ninclude!(concat!(env!(\"OUT_DIR\"), \"/../escape.rs\"));\n"
        ),
    );

    let results = run_family(root);
    assert_files(
        &results,
        BTreeSet::from([rest_rel.to_owned(), handlers_rel.to_owned()]),
    );
    assert_findings(
        &results,
        &[
            RuleFinding::new(
                Severity::Info,
                "build-script include! inventory",
                "`include!(concat!(env!(\"OUT_DIR\"), ...))` detected. Review generated-code boundary.",
                Some(handlers_rel),
                Some(handlers_info_line),
                true,
            ),
            RuleFinding::new(
                Severity::Warn,
                "include path traversal",
                "`include_str!()` uses a path containing `..`.",
                Some(handlers_rel),
                Some(handlers_warn_line),
                false,
            ),
            RuleFinding::new(
                Severity::Warn,
                "include path traversal",
                "`include!()` build-script pattern appends a path containing `..`.",
                Some(handlers_rel),
                Some(handlers_build_warn_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "include! bypass",
                "`include!()` pulls in Rust code outside the scanned file boundary.",
                Some(rest_rel),
                Some(rest_line),
                false,
            ),
        ],
    );
}
