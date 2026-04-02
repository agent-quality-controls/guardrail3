pub(crate) use super::{
    collected_facts, collected_facts_with_validation_scope, direct_dependency_cap_facts,
    run_with_facts,
};
pub(crate) use test_support::{dir_entry, project_tree};

mod exactness;
mod normalization;
