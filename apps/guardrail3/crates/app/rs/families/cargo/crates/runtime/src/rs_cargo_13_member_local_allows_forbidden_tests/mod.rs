#![allow(unused_imports)]
pub(crate) use guardrail3_app_rs_family_cargo_assertions::rs_cargo_13_member_local_allows_forbidden::{
    check_results, rule_results, assert_rule_results, ExpectedRuleResult,
};
pub(crate) use test_support::{entry, tree};
#[path = "cases.rs"] // reason: shared cargo test fixture fragments
mod cases;
