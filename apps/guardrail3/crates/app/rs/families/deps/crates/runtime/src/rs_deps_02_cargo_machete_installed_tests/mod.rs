#![allow(unused_imports)]
pub(crate) use crate::test_harness::{collected_facts, tool_facts, tool_input};
pub(crate) use guardrail3_app_rs_family_deps_assertions::rs_deps_02_cargo_machete_installed::{
    ExpectedRuleResult, assert_rule_results,
};
pub(crate) use test_support::project_tree;

mod exactness;
mod golden;
