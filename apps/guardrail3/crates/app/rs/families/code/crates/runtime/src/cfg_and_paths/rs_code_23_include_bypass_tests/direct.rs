use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_23_include_bypass::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_plain_include_bypass() {
    let results = check_source("src/lib.rs", "include!(\"../generated.rs\");", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "include! bypass",
            "`include!()` pulls in Rust code outside the scanned file boundary.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn inventories_build_script_include_pattern() {
    let content = "include!(concat!(env!(\"OUT_DIR\"), \"/generated.rs\"));";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Info,
            "build-script include! inventory",
            "`include!(concat!(env!(\"OUT_DIR\"), ...))` detected. Review generated-code boundary.",
            Some("src/lib.rs"),
            Some(1),
            true,
        )],
    );
}

#[test]
fn errors_on_non_concat_include_with_out_dir_reference() {
    let content = "include!(my_macro!(env!(\"OUT_DIR\"), \"../escape.rs\"));";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "include! bypass",
            "`include!()` pulls in Rust code outside the scanned file boundary.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_build_script_include_with_parent_traversal() {
    let content = "include!(concat!(env!(\"OUT_DIR\"), \"/../escape.rs\"));";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "include path traversal",
            "`include!()` build-script pattern appends a path containing `..`.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn keeps_build_script_include_inventory_for_non_traversing_double_dot_filename() {
    let content = "include!(concat!(env!(\"OUT_DIR\"), \"/generated..rs\"));";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Info,
            "build-script include! inventory",
            "`include!(concat!(env!(\"OUT_DIR\"), ...))` detected. Review generated-code boundary.",
            Some("src/lib.rs"),
            Some(1),
            true,
        )],
    );
}

#[test]
fn warns_on_include_path_traversal() {
    let content = "const BYTES: &[u8] = include_bytes!(\"../fixtures/payload.bin\");";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "include path traversal",
            "`include_bytes!()` uses a path containing `..`.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}
