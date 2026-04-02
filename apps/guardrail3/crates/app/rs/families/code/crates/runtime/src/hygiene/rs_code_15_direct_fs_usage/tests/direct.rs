use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::hygiene::rs_code_15_direct_fs_usage::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_std_fs_import() {
    let content = "use std::fs;\nfn main() {}";
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "direct std::fs import",
            "Direct `use std::fs` import found: `use std::fs;`.",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_inline_std_fs_call() {
    let content = "fn main() { let _ = std::fs::read_to_string(\"foo\"); }";
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "direct std::fs call",
            "Direct `std::fs::*` call found: `fn main() { let _ = std::fs::read_to_string(\"foo\"); }`.",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn still_errors_inside_allow_scoped_std_fs_usage() {
    let content = "#[allow(clippy::disallowed_methods)]\nfn main() { let _ = std::fs::read_to_string(\"foo\"); }";
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "direct std::fs call",
            "Direct `std::fs::*` call found: `fn main() { let _ = std::fs::read_to_string(\"foo\"); }`.",
            Some("src/foo.rs"),
            Some(2),
            false,
        )],
    );
}

#[test]
fn errors_on_std_alias_fs_call() {
    let content = "use std as s;\nfn main() { let _ = s::fs::read_to_string(\"foo\"); }";
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "direct std::fs call",
            "Direct `std::fs::*` call found: `fn main() { let _ = s::fs::read_to_string(\"foo\"); }`.",
            Some("src/foo.rs"),
            Some(2),
            false,
        )],
    );
}

#[test]
fn errors_on_extern_crate_std_alias_fs_call() {
    let content = "extern crate std as s;\nfn main() { let _ = s::fs::read_to_string(\"foo\"); }";
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "direct std::fs call",
            "Direct `std::fs::*` call found: `fn main() { let _ = s::fs::read_to_string(\"foo\"); }`.",
            Some("src/foo.rs"),
            Some(2),
            false,
        )],
    );
}
