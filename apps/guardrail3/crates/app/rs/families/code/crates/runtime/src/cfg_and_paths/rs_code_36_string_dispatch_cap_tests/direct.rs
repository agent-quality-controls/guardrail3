use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_36_string_dispatch_cap::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_match_with_too_many_string_arms() {
    let arms = (0..11)
        .map(|index| format!("\"v{index}\" => {index},"))
        .collect::<Vec<_>>()
        .join("\n");
    let content =
        format!("pub fn dispatch(value: &str) -> usize {{ match value {{ {arms} _ => 0 }} }}");
    let results = check_source("src/lib.rs", &content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "string dispatch is too large",
            "match site has 11 string-literal branches (cap 10). Replace string dispatch with typed models.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_if_else_chain_with_too_many_string_branches() {
    let mut chain = String::new();
    for index in 0..11 {
        let prefix = if index == 0 { "if" } else { "else if" };
        chain.push_str(&format!("{prefix} value == \"v{index}\" {{ {index} }} "));
    }
    chain.push_str("else { 0 }");
    let content = format!("pub fn dispatch(value: &str) -> usize {{ {chain} }}");
    let results = check_source("src/lib.rs", &content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "string dispatch is too large",
            "if/else if chain site has 11 string-literal branches (cap 10). Replace string dispatch with typed models.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}
