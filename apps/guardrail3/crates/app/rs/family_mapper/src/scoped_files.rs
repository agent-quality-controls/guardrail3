use std::collections::BTreeSet;

use guardrail3_domain_project_tree::ProjectTreeView;

#[must_use]
pub fn filter_for_roots(
    tree: &dyn ProjectTreeView,
    scoped_files: Option<&BTreeSet<String>>,
    root_rels: &[String],
    validation_scope: Option<&str>,
) -> Option<BTreeSet<String>> {
    let explicit = scoped_files.map(|files| {
        files
            .iter()
            .filter(|path| scoped_path_is_live(tree, path))
            .filter(|path| root_rels.iter().any(|root| path_is_under_root(path, root)))
            .cloned()
            .collect::<BTreeSet<_>>()
    });
    let derived = validation_scope.map(|scope| collect_scope_files(tree, scope, root_rels));

    match (explicit, derived) {
        (Some(explicit), Some(derived)) => Some(explicit.intersection(&derived).cloned().collect()),
        (Some(explicit), None) => Some(explicit),
        (None, Some(derived)) => Some(derived),
        (None, None) => None,
    }
}

fn scoped_path_is_live(tree: &dyn ProjectTreeView, rel_path: &str) -> bool {
    tree.file_exists(rel_path) || tree.dir_exists(rel_path)
}

fn path_is_under_root(rel_path: &str, root_rel: &str) -> bool {
    root_rel.is_empty()
        || rel_path == root_rel
        || rel_path
            .strip_prefix(root_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}

fn collect_scope_files(
    tree: &dyn ProjectTreeView,
    scope_rel: &str,
    root_rels: &[String],
) -> BTreeSet<String> {
    let mut files = BTreeSet::new();

    if tree.file_exists(scope_rel) {
        if root_rels
            .iter()
            .any(|root| path_is_under_root(scope_rel, root))
        {
            let _ = files.insert(scope_rel.to_owned());
        }
        return files;
    }

    for (dir_rel, entry) in tree.structure() {
        if !(dir_rel == scope_rel || path_is_under_root(dir_rel, scope_rel)) {
            continue;
        }
        for file in entry.files() {
            let rel_path = join_rel(dir_rel, file);
            if root_rels
                .iter()
                .any(|root| path_is_under_root(&rel_path, root))
            {
                let _ = files.insert(rel_path);
            }
        }
    }

    files
}

fn join_rel(parent: &str, child: &str) -> String {
    if parent.is_empty() {
        child.to_owned()
    } else {
        format!("{parent}/{child}")
    }
}
