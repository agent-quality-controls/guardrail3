use crate::domain::project_tree::ProjectTree;

use super::dependency_facts::MemberDependencyFacts;

#[derive(Debug, Clone)]
pub struct SourceCrateFacts {
    pub crate_name: String,
    pub rel_dir: String,
    pub layer: Option<super::dependency_facts::Layer>,
    pub pub_trait_count: usize,
    pub impl_count: usize,
}

pub fn collect(tree: &ProjectTree, members: &[MemberDependencyFacts]) -> Vec<SourceCrateFacts> {
    let mut crates = Vec::new();
    for member in members {
        let src_rel = format!("{}/src", member.rel_dir);
        if !tree.dir_exists(&src_rel) {
            continue;
        }
        let mut stats = SourceStats::default();
        walk_source_dir(tree, &src_rel, &mut stats);
        crates.push(SourceCrateFacts {
            crate_name: member.name.clone(),
            rel_dir: member.rel_dir.clone(),
            layer: member.layer,
            pub_trait_count: stats.pub_trait_count,
            impl_count: stats.impl_count,
        });
    }
    crates
}

#[derive(Default)]
struct SourceStats {
    pub_trait_count: usize,
    impl_count: usize,
}

fn walk_source_dir(tree: &ProjectTree, rel_dir: &str, stats: &mut SourceStats) {
    let Some(entry) = tree.dir_contents(rel_dir) else {
        return;
    };
    for file in &entry.files {
        if !file.ends_with(".rs") {
            continue;
        }
        let rel_path = format!("{rel_dir}/{file}");
        let abs_path = tree.abs_path(&rel_path);
        let Ok(content) = std::fs::read_to_string(abs_path) else {
            continue;
        };
        let Ok(parsed) = syn::parse_file(&content) else {
            continue;
        };
        for item in parsed.items {
            match item {
                syn::Item::Trait(item_trait) => {
                    if matches!(item_trait.vis, syn::Visibility::Public(_)) {
                        stats.pub_trait_count += 1;
                    }
                }
                syn::Item::Impl(_) => stats.impl_count += 1,
                _ => {}
            }
        }
    }
    for child in &entry.dirs {
        walk_source_dir(tree, &format!("{rel_dir}/{child}"), stats);
    }
}
