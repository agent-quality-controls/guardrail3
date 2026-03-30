use guardrail3_domain_project_tree::ProjectTree;

pub fn rust_file_rels(tree: &ProjectTree) -> Vec<String> {
    let mut rels: Vec<String> = tree
        .structure()
        .iter()
        .flat_map(|(dir_rel, entry)| {
            entry.files().iter().filter_map(|file_name| {
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

pub fn is_test_path(rel_path: &str) -> bool {
    rel_path == "tests.rs"
        || rel_path.starts_with("tests/")
        || rel_path.contains("/tests/")
        || rel_path.contains("_tests/")
        || rel_path.contains("__tests__")
        || rel_path.ends_with("_test.rs")
        || rel_path.ends_with("_tests.rs")
}

fn is_fixture_path(rel_path: &str) -> bool {
    rel_path.contains("/tests/fixtures/") || rel_path.starts_with("tests/fixtures/")
}
