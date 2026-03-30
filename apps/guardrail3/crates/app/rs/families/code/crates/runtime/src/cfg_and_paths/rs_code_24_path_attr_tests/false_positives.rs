use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_24_path_attr::{
    RuleFinding, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn skips_same_line_reasoned_non_escaping_path_attrs() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rest_rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";
    let rest_content = test_support::read_file(root, rest_rel);

    write_file(
        root,
        rest_rel,
        &format!(
            "{rest_content}\n#[path = \"generated_inline.rs\"] // reason: generated request DTO shim\nmod generated_inline;\n"
        ),
    );

    let results = run_family(root);
    let warn_line = rest_content.lines().count() + 2;

    assert_files(&results, BTreeSet::from([rest_rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "#[path] usage",
            "#[path = \"generated_inline.rs\"] reason: generated request DTO shim",
            Some(rest_rel),
            Some(warn_line),
            false,
        )],
    );
}

#[test]
fn skips_multiline_path_attr_with_reason_on_closing_line() {
    let content = "#[path =\n    \"generated_inline.rs\"\n] // reason: generated request DTO shim\nmod generated_inline;\n";
    let results = super::super::check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "#[path] usage",
            "#[path = \"generated_inline.rs\"] reason: generated request DTO shim",
            Some("src/lib.rs"),
            Some(3),
            false,
        )],
    );
}

#[test]
fn skips_multiline_cfg_attr_path_with_reason_on_closing_line() {
    let content = "#[cfg_attr(\n    test,\n    path = \"generated_inline.rs\"\n)] // reason: generated request DTO shim\nmod generated_inline;\n";
    let results = super::super::check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "#[path] usage",
            "#[path = \"generated_inline.rs\"] reason: generated request DTO shim",
            Some("src/lib.rs"),
            Some(4),
            false,
        )],
    );
}
