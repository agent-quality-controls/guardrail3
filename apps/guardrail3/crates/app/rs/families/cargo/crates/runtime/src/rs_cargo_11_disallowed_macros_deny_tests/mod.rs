#![allow(unused_imports)]
pub(crate) use guardrail3_app_rs_family_cargo_assertions::rs_cargo_11_disallowed_macros_deny::{
    ExpectedRuleResult, assert_rule_results, check_results, rule_results,
};
pub(crate) use test_support::{entry, tree};
#[path = "cases.rs"] // reason: shared cargo test fixture fragments
mod cases;
