use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::inventory::rs_code_29_large_trait_inventory::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_trait_with_thirteen_methods() {
    let mut methods = String::new();
    for index in 0..13 {
        methods.push_str(&format!("    fn m{index}(&self);\n"));
    }
    let content = format!("pub trait Service {{\n{methods}}}");
    let results = check_source("src/lib.rs", &content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "large trait surface",
            "Trait `Service` has 13 methods (warn above 8, error above 12).",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn warns_on_trait_with_nine_methods() {
    let mut methods = String::new();
    for index in 0..9 {
        methods.push_str(&format!("    fn m{index}(&self);\n"));
    }
    let content = format!("pub trait Service {{\n{methods}}}");
    let results = check_source("src/lib.rs", &content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "large trait surface",
            "Trait `Service` has 9 methods (warn above 8, error above 12).",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}
