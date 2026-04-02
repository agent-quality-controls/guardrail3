use std::path::Path;

use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_domain_report::CheckResult;
use guardrail3_outbound_traits::{FileSystem, ToolChecker};

pub fn check(
    fs: &dyn FileSystem,
    root: &Path,
    tree: &ProjectTree,
    tc: &dyn ToolChecker,
) -> Vec<CheckResult> {
    let surface = FamilyView::build(
        tree.root().clone(),
        tree.structure(),
        tree.content(),
        &[],
        &hook_file_rels(tree),
        &hook_dir_rels(tree),
        None,
    );
    guardrail3_app_rs_family_hooks_shared::check(fs, root, &surface, tc)
}

fn hook_file_rels(tree: &ProjectTree) -> Vec<String> {
    let mut rels = [
        ".githooks/pre-commit",
        "hooks/pre-commit",
        ".husky/pre-commit",
        "lefthook.yml",
        "lefthook.yaml",
        ".lefthook.yml",
        ".lefthook.yaml",
    ]
    .into_iter()
    .filter(|rel_path| tree.file_exists(rel_path))
    .map(str::to_owned)
    .collect::<Vec<_>>();

    rels.extend(dir_file_rels(tree, ".githooks/pre-commit.d"));
    rels.extend(dir_file_rels(tree, ".guardrail3/overrides/pre-commit.d"));
    rels
}

fn hook_dir_rels(tree: &ProjectTree) -> Vec<String> {
    [
        ".githooks/pre-commit.d",
        ".guardrail3/overrides/pre-commit.d",
    ]
    .into_iter()
    .filter(|rel_path| tree.dir_exists(rel_path))
    .map(str::to_owned)
    .collect()
}

fn dir_file_rels(tree: &ProjectTree, dir_rel: &str) -> Vec<String> {
    tree.dir_contents(dir_rel)
        .map(|entry| {
            entry.files()
                .iter()
                .map(|file_name| ProjectTree::join_rel(dir_rel, file_name))
                .collect()
        })
        .unwrap_or_default()
}

mod deploy_checks;
mod hook_checks;
mod hook_script_checks;
mod tool_checks;
#[cfg(feature = "api")]
pub mod validate;
