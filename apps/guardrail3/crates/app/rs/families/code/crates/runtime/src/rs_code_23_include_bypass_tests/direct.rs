use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_23_include_bypass::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_plain_include_bypass() {
    let results = check_source("src/lib.rs", "include!(\"../generated.rs\");", false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "include! bypass",
            message: "`include!()` pulls in Rust code outside the scanned file boundary.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}

#[test]
fn inventories_build_script_include_pattern() {
    let content = "include!(concat!(env!(\"OUT_DIR\"), \"/generated.rs\"));";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Info,
            title: "build-script include! inventory",
            message: "`include!(concat!(env!(\"OUT_DIR\"), ...))` detected. Review generated-code boundary.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: true,
        }],
    );
}

#[test]
fn errors_on_non_concat_include_with_out_dir_reference() {
    let content = "include!(my_macro!(env!(\"OUT_DIR\"), \"../escape.rs\"));";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Error,
            title: "include! bypass",
            message: "`include!()` pulls in Rust code outside the scanned file boundary.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}

#[test]
fn warns_on_include_path_traversal() {
    let content = "const BYTES: &[u8] = include_bytes!(\"../fixtures/payload.bin\");";
    let results = check_source("src/lib.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Warn,
            title: "include path traversal",
            message: "`include_bytes!()` uses a path containing `..`.",
            file: Some("src/lib.rs"),
            line: Some(1),
            inventory: false,
        }],
    );
}
