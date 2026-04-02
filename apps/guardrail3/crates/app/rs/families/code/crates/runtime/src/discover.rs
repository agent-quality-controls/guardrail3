use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

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

pub fn is_test_root_path(rel_path: &str) -> bool {
    let segments = rel_path.split('/').collect::<Vec<_>>();
    if segments.first().is_some_and(|segment| *segment == "tests") {
        return true;
    }
    if segments.iter().any(|segment| segment.ends_with("_tests")) {
        return true;
    }
    let Some(file_name) = segments.last() else {
        return false;
    };
    file_name.ends_with("_test.rs") || file_name.ends_with("_tests.rs")
}

fn is_fixture_path(rel_path: &str) -> bool {
    rel_path.contains("/tests/fixtures/") || rel_path.starts_with("tests/fixtures/")
}
