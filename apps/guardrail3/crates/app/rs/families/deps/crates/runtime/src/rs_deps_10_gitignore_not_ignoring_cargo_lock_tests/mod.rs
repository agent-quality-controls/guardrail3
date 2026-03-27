#![allow(unused_imports)]
pub(crate) use crate::test_harness::{collected_facts, lockfile_facts, lockfile_input};
pub(crate) use guardrail3_app_rs_family_deps_assertions::rs_deps_10_gitignore_not_ignoring_cargo_lock::{
    assert_rule_results, ExpectedRuleResult,
};
pub(crate) use test_support::{dir_entry, project_tree};

mod golden;
mod precedence;
