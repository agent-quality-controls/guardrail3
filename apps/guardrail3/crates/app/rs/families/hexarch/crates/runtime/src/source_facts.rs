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
        let mut stats = SourceStats::default();
        let mut source_error = None;
        let mut visited = BTreeSet::new();
        let entrypoints = determine_entrypoints(tree, member, &mut source_error);
        if entrypoints.is_empty() && source_error.is_none() {
            continue;
        }
        for entrypoint in entrypoints {
            let module_stats =
                walk_module_file(tree, &entrypoint, &mut source_error, &mut visited);
            stats.pub_trait_count += module_stats.pub_trait_count;
            stats.impl_count += module_stats.impl_count;
        }
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

#[derive(Default)]
struct ModuleStats {
    pub_trait_count: usize,
    impl_count: usize,
    has_any_trait: bool,
}

fn determine_entrypoints(
    tree: &ProjectTree,
    member: &MemberDependencyFacts,
    source_error: &mut Option<(String, String)>,
) -> Vec<String> {
    let src_rel = format!("{}/src", member.rel_dir);
    let explicit_entrypoints = tree
        .file_content(&member.cargo_rel_path)
        .and_then(|content| parse_manifest_entrypoints(&member.rel_dir, content, source_error))
        .unwrap_or_default();

    if !explicit_entrypoints.is_empty() {
        let existing = explicit_entrypoints
            .iter()
            .filter(|entrypoint| tree.file_exists(entrypoint))
            .cloned()
            .collect::<Vec<_>>();
        if existing.is_empty() {
            let configured = explicit_entrypoints.join(", ");
            *source_error = Some((
                member.cargo_rel_path.clone(),
                format!(
                    "Failed to determine Rust source entrypoint for hexarch checks: configured target path(s) not found: {configured}."
                ),
            ));
        }
        return existing;
    }

    if !tree.dir_exists(&src_rel) {
        return Vec::new();
    }

    let inferred = [format!("{src_rel}/lib.rs"), format!("{src_rel}/main.rs")]
        .into_iter()
        .filter(|entrypoint| tree.file_exists(entrypoint))
        .collect::<Vec<_>>();
    if inferred.is_empty() {
        *source_error = Some((
            src_rel.to_owned(),
            "Failed to determine Rust source entrypoint for hexarch checks: expected src/lib.rs or src/main.rs.".to_owned(),
        ));
        return Vec::new();
    }
    inferred
}

fn walk_module_file(
    tree: &ProjectTree,
    rel_path: &str,
    source_error: &mut Option<(String, String)>,
    visited: &mut BTreeSet<String>,
) -> ModuleStats {
    if source_error.is_some() || !visited.insert(rel_path.to_owned()) {
        return ModuleStats::default();
    }

    let abs_path = tree.abs_path(rel_path);
    let content = match guardrail3_shared_fs::read_file_err(&abs_path) {
        Ok(content) => content,
        Err(read_error) => {
            *source_error = Some((
                rel_path.to_owned(),
                format!("Failed to read Rust source file for hexarch checks: {read_error}"),
            ));
            return ModuleStats::default();
        }
    };
    let parsed = match syn::parse_file(&content) {
        Ok(parsed) => parsed,
        Err(parse_error) => {
            *source_error = Some((
                rel_path.to_owned(),
                format!("Failed to parse Rust source file for hexarch checks: {parse_error}"),
            ));
            return ModuleStats::default();
        }
    };

    count_items(tree, rel_path, &parsed.items, source_error, visited)
}

fn parse_manifest_entrypoints(
    member_rel_dir: &str,
    content: &str,
    source_error: &mut Option<(String, String)>,
) -> Option<Vec<String>> {
    let parsed = match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => parsed,
        Err(parse_error) => {
            if source_error.is_none() {
                *source_error = Some((
                    format!("{member_rel_dir}/Cargo.toml"),
                    format!(
                        "Failed to parse member Cargo.toml while determining Rust source entrypoints for hexarch checks: {parse_error}"
                    ),
                ));
            }
            return None;
        }
    };

    let mut entrypoints = BTreeSet::new();
    if let Some(lib_path) = parsed
        .get("lib")
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("path"))
        .and_then(toml::Value::as_str)
    {
        let _ = entrypoints.insert(ProjectTree::join_rel(member_rel_dir, lib_path));
    }

    if let Some(bin_array) = parsed.get("bin").and_then(toml::Value::as_array) {
        for bin in bin_array.iter().filter_map(toml::Value::as_table) {
            if let Some(path) = bin.get("path").and_then(toml::Value::as_str) {
                let _ = entrypoints.insert(ProjectTree::join_rel(member_rel_dir, path));
            }
        }
    }

    Some(entrypoints.into_iter().collect())
}

fn count_items(
    tree: &ProjectTree,
    rel_path: &str,
    items: &[syn::Item],
    source_error: &mut Option<(String, String)>,
    visited: &mut BTreeSet<String>,
) -> ModuleStats {
    let mut stats = ModuleStats::default();
    for item in items {
        if source_error.is_some() || is_cfg_test_only(item.attrs()) {
            continue;
        }
        match item {
            syn::Item::Trait(item_trait) => {
                stats.has_any_trait = true;
                if matches!(item_trait.vis, syn::Visibility::Public(_)) {
                    stats.pub_trait_count += 1;
                }
            }
            syn::Item::Impl(_) => stats.impl_count += 1,
            syn::Item::Mod(item_mod) => {
                if let Some((_, nested_items)) = &item_mod.content {
                    let nested =
                        count_items(tree, rel_path, nested_items, source_error, visited);
                    stats.pub_trait_count += nested.pub_trait_count;
                    if nested.has_any_trait {
                        stats.impl_count += nested.impl_count;
                    }
                    stats.has_any_trait |= nested.has_any_trait;
                } else if let Some(module_path) = resolve_module_path(tree, rel_path, item_mod) {
                    let nested =
                        walk_module_file(tree, &module_path, source_error, visited);
                    stats.pub_trait_count += nested.pub_trait_count;
                    if nested.has_any_trait {
                        stats.impl_count += nested.impl_count;
                    }
                    stats.has_any_trait |= nested.has_any_trait;
                }
            }
            _ => {}
        }
    }
    if !stats.has_any_trait {
        stats.impl_count = 0;
    }
    stats
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
