use guardrail3_domain_project_tree::ProjectTree;

pub fn rust_file_rels(tree: &ProjectTree) -> Vec<String> {
    let mut rels: Vec<String> = tree
        .structure
        .iter()
        .flat_map(|(dir_rel, entry)| {
            entry.files.iter().filter_map(|file_name| {
                if !file_name.ends_with(".rs") {
                    return None;
                }
                let rel = ProjectTree::join_rel(dir_rel, file_name);
                if is_fixture_path(&rel) {
                    None
                } else {
                    Some(rel)
                }
            })
        })
        .collect();
    rels.sort();
    rels
}

pub fn is_fixture_path(rel_path: &str) -> bool {
    rel_path.contains("/tests/fixtures/") || rel_path.starts_with("tests/fixtures/")
}

pub fn root_relative<'a>(rel_path: &'a str, root_rel_dir: &str) -> &'a str {
    if root_rel_dir.is_empty() {
        rel_path
    } else {
        rel_path
            .strip_prefix(root_rel_dir)
            .and_then(|rest| rest.strip_prefix('/'))
            .unwrap_or(rel_path)
    }
}

pub fn path_is_under(rel_path: &str, prefix: &str) -> bool {
    rel_path == prefix
        || rel_path
            .strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with('/'))
}

pub fn file_stem(rel_path: &str) -> Option<&str> {
    rel_path
        .rsplit('/')
        .next()
        .and_then(|name| name.strip_suffix(".rs"))
}

pub fn parent_dir(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
}

pub fn join_under_root(root_rel_dir: &str, child_rel: &str) -> String {
    if root_rel_dir.is_empty() {
        child_rel.to_owned()
    } else {
        ProjectTree::join_rel(root_rel_dir, child_rel)
    }
}
