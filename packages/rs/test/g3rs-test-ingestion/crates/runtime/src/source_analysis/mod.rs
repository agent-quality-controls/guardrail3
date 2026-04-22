mod pipeline;
mod proof_helpers;

pub(crate) use pipeline::{
    analyze_file_tree_files, analyze_source_files, file_activates_test_rules,
};
