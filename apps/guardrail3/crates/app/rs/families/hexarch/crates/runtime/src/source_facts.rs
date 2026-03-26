use std::collections::BTreeSet;

use guardrail3_domain_project_tree::ProjectTree;

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
        let mut visited = BTreeSet::new();
        walk_reachable_source(tree, &src_rel, &mut stats, &mut source_error, &mut visited);
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

fn walk_reachable_source(
    tree: &ProjectTree,
    src_rel: &str,
    stats: &mut SourceStats,
    source_error: &mut Option<(String, String)>,
    visited: &mut BTreeSet<String>,
) {
    if source_error.is_some() {
        return;
    }

    let entrypoints = [format!("{src_rel}/lib.rs"), format!("{src_rel}/main.rs")]
        .into_iter()
        .filter(|entrypoint| tree.file_exists(entrypoint))
        .collect::<Vec<_>>();
    if entrypoints.is_empty() {
        *source_error = Some((
            src_rel.to_owned(),
            "Failed to determine Rust source entrypoint for hexarch checks: expected src/lib.rs or src/main.rs.".to_owned(),
        ));
        return;
    }

    for entrypoint in entrypoints {
        walk_module_file(tree, &entrypoint, stats, source_error, visited);
    }
}

fn walk_module_file(
    tree: &ProjectTree,
    rel_path: &str,
    stats: &mut SourceStats,
    source_error: &mut Option<(String, String)>,
    visited: &mut BTreeSet<String>,
) {
    if source_error.is_some() || !visited.insert(rel_path.to_owned()) {
        return;
    }

    let abs_path = tree.abs_path(rel_path);
    let content = match guardrail3_shared_fs::read_file_err(&abs_path) {
        Ok(content) => content,
        Err(read_error) => {
            *source_error = Some((
                rel_path.to_owned(),
                format!("Failed to read Rust source file for hexarch checks: {read_error}"),
            ));
            return;
        }
    };
    let parsed = match syn::parse_file(&content) {
        Ok(parsed) => parsed,
        Err(parse_error) => {
            *source_error = Some((
                rel_path.to_owned(),
                format!("Failed to parse Rust source file for hexarch checks: {parse_error}"),
            ));
            return;
        }
    };

    count_items(tree, rel_path, &parsed.items, stats, source_error, visited);
}

fn count_items(
    tree: &ProjectTree,
    rel_path: &str,
    items: &[syn::Item],
    stats: &mut SourceStats,
    source_error: &mut Option<(String, String)>,
    visited: &mut BTreeSet<String>,
) {
    for item in items {
        if source_error.is_some() || is_cfg_test_only(item.attrs()) {
            continue;
        }
        match item {
            syn::Item::Trait(item_trait) => {
                if matches!(item_trait.vis, syn::Visibility::Public(_)) {
                    stats.pub_trait_count += 1;
                }
            }
            syn::Item::Impl(_) => stats.impl_count += 1,
            syn::Item::Mod(item_mod) => {
                if let Some((_, nested_items)) = &item_mod.content {
                    count_items(tree, rel_path, nested_items, stats, source_error, visited);
                } else if let Some(module_path) = resolve_module_path(tree, rel_path, item_mod) {
                    walk_module_file(tree, &module_path, stats, source_error, visited);
                }
            }
            _ => {}
        }
    }
}

fn resolve_module_path(
    tree: &ProjectTree,
    rel_path: &str,
    item_mod: &syn::ItemMod,
) -> Option<String> {
    let file_dir = rel_path.rsplit_once('/').map_or("", |(dir, _)| dir);
    if let Some(path_attr) = module_path_attr(item_mod) {
        let candidate = ProjectTree::join_rel(file_dir, &path_attr);
        return tree.file_exists(&candidate).then_some(candidate);
    }

    let module_name = item_mod.ident.to_string();
    let file_name = rel_path.rsplit('/').next().unwrap_or(rel_path);
    let stem = file_name.strip_suffix(".rs").unwrap_or(file_name);
    let module_dir = if matches!(stem, "lib" | "main" | "mod") {
        file_dir.to_owned()
    } else {
        ProjectTree::join_rel(file_dir, stem)
    };

    let direct = ProjectTree::join_rel(&module_dir, &format!("{module_name}.rs"));
    if tree.file_exists(&direct) {
        return Some(direct);
    }

    let nested = ProjectTree::join_rel(&module_dir, &format!("{module_name}/mod.rs"));
    tree.file_exists(&nested).then_some(nested)
}

fn module_path_attr(item_mod: &syn::ItemMod) -> Option<String> {
    item_mod.attrs.iter().find_map(|attr| {
        if !attr.path().is_ident("path") {
            return None;
        }

        match &attr.meta {
            syn::Meta::NameValue(name_value) => match &name_value.value {
                syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                    syn::Lit::Str(value) => Some(value.value()),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    })
}

fn is_cfg_test_only(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("cfg") {
            return false;
        }

        let Ok(meta) = attr.parse_args::<syn::Meta>() else {
            return false;
        };
        cfg_meta_is_test_only(&meta)
    })
}

fn cfg_meta_is_test_only(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(path) => path.is_ident("test"),
        syn::Meta::List(list) => {
            let Some(ident) = list.path.get_ident().map(|ident| ident.to_string()) else {
                return false;
            };
            let nested = list
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )
                .ok();
            match (ident.as_str(), nested) {
                ("all", Some(items)) | ("any", Some(items)) => {
                    !items.is_empty() && items.iter().all(cfg_meta_is_test_only)
                }
                _ => false,
            }
        }
        syn::Meta::NameValue(_) => false,
    }
}

trait ItemAttrs {
    fn attrs(&self) -> &[syn::Attribute];
}

impl ItemAttrs for syn::Item {
    fn attrs(&self) -> &[syn::Attribute] {
        match self {
            syn::Item::Const(item) => &item.attrs,
            syn::Item::Enum(item) => &item.attrs,
            syn::Item::ExternCrate(item) => &item.attrs,
            syn::Item::Fn(item) => &item.attrs,
            syn::Item::ForeignMod(item) => &item.attrs,
            syn::Item::Impl(item) => &item.attrs,
            syn::Item::Macro(item) => &item.attrs,
            syn::Item::Mod(item) => &item.attrs,
            syn::Item::Static(item) => &item.attrs,
            syn::Item::Struct(item) => &item.attrs,
            syn::Item::Trait(item) => &item.attrs,
            syn::Item::TraitAlias(item) => &item.attrs,
            syn::Item::Type(item) => &item.attrs,
            syn::Item::Union(item) => &item.attrs,
            syn::Item::Use(item) => &item.attrs,
            _ => &[],
        }
    }
}
