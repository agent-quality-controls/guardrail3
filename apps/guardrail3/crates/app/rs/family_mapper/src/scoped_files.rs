use std::collections::BTreeSet;

use guardrail3_domain_project_tree::ProjectTree;

#[must_use]
pub fn filter_for_roots(
    tree: &ProjectTree,
    scoped_files: Option<&BTreeSet<String>>,
    root_rels: &[String],
) -> Option<BTreeSet<String>> {
    scoped_files.map(|files| {
        files
            .iter()
            .filter(|path| scoped_path_is_live(tree, path))
            .filter(|path| root_rels.iter().any(|root| path_is_under_root(path, root)))
            .cloned()
            .collect()
    })
}

fn scoped_path_is_live(tree: &ProjectTree, rel_path: &str) -> bool {
    tree.file_exists(rel_path) || tree.dir_exists(rel_path)
}

fn path_is_under_root(rel_path: &str, root_rel: &str) -> bool {
    root_rel.is_empty()
        || rel_path == root_rel
        || rel_path
            .strip_prefix(root_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}
