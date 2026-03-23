use crate::domain::project_tree::ProjectTree;

use super::dependency_facts::MemberDependencyFacts;

#[derive(Debug, Clone)]
pub struct SourceCrateFacts {
    pub crate_name: String,
    pub rel_dir: String,
    pub layer: Option<super::dependency_facts::Layer>,
    pub pub_trait_count: usize,
    pub impl_count: usize,
    pub source_error_rel_path: Option<String>,
    pub source_error_message: Option<String>,
}

pub fn collect(tree: &ProjectTree, members: &[MemberDependencyFacts]) -> Vec<SourceCrateFacts> {
    let mut crates = Vec::new();
    for member in members {
        let src_rel = format!("{}/src", member.rel_dir);
        if !tree.dir_exists(&src_rel) {
            continue;
        }
        let mut stats = SourceStats::default();
        let mut source_error = None;
        walk_source_dir(tree, &src_rel, &mut stats, &mut source_error);
        crates.push(SourceCrateFacts {
            crate_name: member.name.clone(),
            rel_dir: member.rel_dir.clone(),
            layer: member.layer,
            pub_trait_count: stats.pub_trait_count,
            impl_count: stats.impl_count,
            source_error_rel_path: source_error.as_ref().map(|(rel_path, _)| rel_path.clone()),
            source_error_message: source_error.map(|(_, message)| message),
        });
    }
    crates
}

#[derive(Default)]
struct SourceStats {
    pub_trait_count: usize,
    impl_count: usize,
}

fn walk_source_dir(
    tree: &ProjectTree,
    rel_dir: &str,
    stats: &mut SourceStats,
    source_error: &mut Option<(String, String)>,
) {
    if source_error.is_some() {
        return;
    }
    let Some(entry) = tree.dir_contents(rel_dir) else {
        return;
    };
    for file in &entry.files {
        if source_error.is_some() {
            return;
        }
        if !file.ends_with(".rs") {
            continue;
        }
        let rel_path = format!("{rel_dir}/{file}");
        let abs_path = tree.abs_path(&rel_path);
        let content = match crate::fs::read_file_err(&abs_path) {
            Ok(content) => content,
            Err(read_error) => {
                *source_error = Some((
                    rel_path.clone(),
                    format!("Failed to read Rust source file for hexarch checks: {read_error}"),
                ));
                return;
            }
        };
        let parsed = match syn::parse_file(&content) {
            Ok(parsed) => parsed,
            Err(parse_error) => {
                *source_error = Some((
                    rel_path.clone(),
                    format!("Failed to parse Rust source file for hexarch checks: {parse_error}"),
                ));
                return;
            }
        };
        count_items(&parsed.items, stats);
    }
    for child in &entry.dirs {
        walk_source_dir(tree, &format!("{rel_dir}/{child}"), stats, source_error);
    }
}

fn count_items(items: &[syn::Item], stats: &mut SourceStats) {
    for item in items {
        match item {
            syn::Item::Trait(item_trait) => {
                if matches!(item_trait.vis, syn::Visibility::Public(_)) {
                    stats.pub_trait_count += 1;
                }
            }
            syn::Item::Impl(_) => stats.impl_count += 1,
            syn::Item::Mod(item_mod) => {
                if let Some((_, nested_items)) = &item_mod.content {
                    count_items(nested_items, stats);
                }
            }
            _ => {}
        }
    }
}
