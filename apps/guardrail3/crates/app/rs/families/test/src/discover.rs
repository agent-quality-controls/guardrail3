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

pub fn is_integration_test_path(rel_path: &str, root_rel_dir: &str) -> bool {
    let rel = root_relative(rel_path, root_rel_dir);
    rel.starts_with("tests/")
}

pub fn is_src_path(rel_path: &str, root_rel_dir: &str) -> bool {
    let rel = root_relative(rel_path, root_rel_dir);
    rel.starts_with("src/")
}

pub fn is_test_sidecar_path(rel_path: &str, root_rel_dir: &str) -> bool {
    let rel = root_relative(rel_path, root_rel_dir);
    rel.ends_with("_tests.rs") || rel.ends_with("_test.rs") || rel.ends_with("/tests.rs")
}

fn root_relative<'a>(rel_path: &'a str, root_rel_dir: &str) -> &'a str {
    if root_rel_dir.is_empty() {
        rel_path
    } else {
        rel_path
            .strip_prefix(root_rel_dir)
            .and_then(|rest| rest.strip_prefix('/'))
            .unwrap_or(rel_path)
    }
}
