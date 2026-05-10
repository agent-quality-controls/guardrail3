/// `collect` module.
mod collect;
/// `crate_base` module.
mod crate_base;
/// `deps` module.
mod deps;
/// `paths` module.
mod paths;
/// `repo` module.
mod repo;
/// `root` module.
mod root;
/// `workflow_predicates` module.
mod workflow_predicates;

pub(crate) use collect::{config_result, filetree_result, repo_root_result, source_result};
