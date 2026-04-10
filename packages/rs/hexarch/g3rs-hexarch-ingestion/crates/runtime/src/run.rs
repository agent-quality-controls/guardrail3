use std::collections::BTreeSet;

use glob::Pattern;
use g3rs_hexarch_ingestion_types::{
    G3RsHexarchConfigChecksInput, G3RsHexarchFileTreeChecksInput, G3RsHexarchIngestionError,
    G3RsHexarchSourceChecksInput,
};
use g3rs_hexarch_types::{G3RsHexarchLayer, G3RsHexarchSourceCrateFacts};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use toml::Value;

use crate::view::CrawlView;

pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsHexarchSourceChecksInput>, G3RsHexarchIngestionError> {
    let view = CrawlView::new(crawl);
    let crates = discover_member_crates(&view)?;
    Ok(crates
        .into_iter()
        .map(|crate_facts| G3RsHexarchSourceChecksInput { crate_facts })
        .collect())
}

pub fn ingest_for_config_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsHexarchConfigChecksInput>, G3RsHexarchIngestionError> {
    Err(G3RsHexarchIngestionError::ConfigIngestionNotImplemented)
}

pub fn ingest_for_file_tree_checks(
    _crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsHexarchFileTreeChecksInput, G3RsHexarchIngestionError> {
    Err(G3RsHexarchIngestionError::FileTreeIngestionNotImplemented)
}

fn discover_member_crates(
    view: &CrawlView<'_>,
) -> Result<Vec<G3RsHexarchSourceCrateFacts>, G3RsHexarchIngestionError> {
    let mut member_dirs = BTreeSet::new();

    let Some(root_workspace) = discover_pointed_workspace(view)? else {
        return Ok(Vec::new());
    };
    member_dirs.extend(workspace_member_dirs(view, "", &root_workspace)?);

    let mut crates = Vec::new();
    for rel_dir in member_dirs {
        if rel_dir.contains("tests/fixtures/") {
            continue;
        }
        if let Some(crate_facts) = summarize_crate(view, &rel_dir) {
            crates.push(crate_facts);
        }
    }

    Ok(crates)
}

fn discover_pointed_workspace(view: &CrawlView<'_>) -> Result<Option<Value>, G3RsHexarchIngestionError> {
    let Some(entry) = view.entry("Cargo.toml") else {
        return Ok(None);
    };
    if !entry.readable {
        return Err(G3RsHexarchIngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let content = view.read_file("Cargo.toml").map_err(|err| G3RsHexarchIngestionError::Unreadable {
        path: entry.path.abs_path.clone(),
        reason: err.to_string(),
    })?;
    let parsed = toml::from_str::<Value>(&content).map_err(|err| G3RsHexarchIngestionError::ParseFailed {
        path: entry.path.abs_path.clone(),
        reason: err.to_string(),
    })?;

    Ok(parsed.get("workspace").is_some().then_some(parsed))
}

fn workspace_member_dirs(
    view: &CrawlView<'_>,
    app_root_rel_dir: &str,
    parsed: &Value,
) -> Result<Vec<String>, G3RsHexarchIngestionError> {
    let Some(workspace) = parsed.get("workspace").and_then(Value::as_table) else {
        return Ok(Vec::new());
    };

    let exclude_patterns = workspace
        .get("exclude")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(|pattern| {
            Pattern::new(&CrawlView::join_rel(app_root_rel_dir, pattern)).map_err(|err| {
                G3RsHexarchIngestionError::ParseFailed {
                    path: view
                        .entry("Cargo.toml")
                        .map(|entry| entry.path.abs_path.clone())
                        .unwrap_or_else(|| std::path::PathBuf::from("Cargo.toml")),
                    reason: format!("invalid workspace exclude pattern `{pattern}`: {err}"),
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut resolved_dirs = BTreeSet::new();

    for member in workspace
        .get("members")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        let resolved = resolve_member_pattern(view, app_root_rel_dir, member);
        if resolved.is_empty() {
            return Err(G3RsHexarchIngestionError::ParseFailed {
                path: view
                    .entry("Cargo.toml")
                    .map(|entry| entry.path.abs_path.clone())
                    .unwrap_or_else(|| std::path::PathBuf::from("Cargo.toml")),
                reason: format!("workspace member pattern `{member}` did not resolve to any Cargo.toml"),
            });
        }
        for rel_dir in resolved {
            if !exclude_patterns.iter().any(|pattern| pattern.matches(&rel_dir)) {
                let _ = resolved_dirs.insert(rel_dir);
            }
        }
    }

    Ok(resolved_dirs.into_iter().collect())
}

fn resolve_member_pattern(view: &CrawlView<'_>, workspace_root_rel_dir: &str, member: &str) -> Vec<String> {
    let pattern = CrawlView::join_rel(workspace_root_rel_dir, member);
    let has_glob = member.contains('*') || member.contains('?') || member.contains('[');

    if has_glob {
        let Ok(glob) = Pattern::new(&pattern) else {
            return Vec::new();
        };
        return view
            .all_dir_rels()
            .filter(|rel_dir| glob.matches(rel_dir))
            .filter(|rel_dir| view.file_exists(&CrawlView::join_rel(rel_dir, "Cargo.toml")))
            .map(str::to_owned)
            .collect();
    }

    view.file_exists(&CrawlView::join_rel(&pattern, "Cargo.toml"))
        .then_some(pattern)
        .into_iter()
        .collect()
}

fn summarize_crate(view: &CrawlView<'_>, rel_dir: &str) -> Option<G3RsHexarchSourceCrateFacts> {
    let cargo_rel_path = CrawlView::join_rel(rel_dir, "Cargo.toml");
    let fallback_name = rel_dir.rsplit('/').next().unwrap_or(rel_dir).to_owned();

    let Some(entry) = view.entry(&cargo_rel_path) else {
        return Some(error_facts(
            fallback_name,
            rel_dir,
            cargo_rel_path,
            "Failed to read member Cargo.toml while determining Rust source entrypoints for hexarch checks: selected Cargo.toml missing from crawl.".to_owned(),
        ));
    };
    if !entry.readable {
        return Some(error_facts(
            fallback_name,
            rel_dir,
            cargo_rel_path,
            "Failed to read member Cargo.toml while determining Rust source entrypoints for hexarch checks: file is not readable.".to_owned(),
        ));
    }

    let content = match view.read_file(&cargo_rel_path) {
        Ok(content) => content,
        Err(read_error) => {
            return Some(error_facts(
                fallback_name,
                rel_dir,
                cargo_rel_path,
                format!(
                    "Failed to read member Cargo.toml while determining Rust source entrypoints for hexarch checks: {read_error}"
                ),
            ));
        }
    };
    let parsed = match toml::from_str::<Value>(&content) {
        Ok(parsed) => parsed,
        Err(parse_error) => {
            return Some(error_facts(
                fallback_name,
                rel_dir,
                cargo_rel_path,
                format!(
                    "Failed to parse member Cargo.toml while determining Rust source entrypoints for hexarch checks: {parse_error}"
                ),
            ));
        }
    };

    let crate_name = parsed
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .unwrap_or(fallback_name);

    let mut source_error = None;
    let mut visited = BTreeSet::new();
    let entrypoints = determine_entrypoints(view, rel_dir, &cargo_rel_path, &parsed, &mut source_error);

    if entrypoints.is_empty() && source_error.is_none() {
        return None;
    }

    let mut pub_trait_count = 0;
    let mut public_free_fn_count = 0;
    let mut public_inherent_method_count = 0;

    for entrypoint in entrypoints {
        let stats = walk_module_file(view, &entrypoint, &mut source_error, &mut visited, true);
        pub_trait_count += stats.pub_trait_count;
        public_free_fn_count += stats.public_free_fn_count;
        public_inherent_method_count += stats.public_inherent_method_count;
    }

    Some(G3RsHexarchSourceCrateFacts {
        crate_name,
        rel_dir: rel_dir.to_owned(),
        layer: layer_from_path(rel_dir),
        pub_trait_count,
        public_free_fn_count,
        public_inherent_method_count,
        source_error_rel_path: source_error.as_ref().map(|(path, _)| path.clone()),
        source_error_message: source_error.map(|(_, message)| message),
    })
}

fn error_facts(
    crate_name: String,
    rel_dir: &str,
    error_rel_path: String,
    error_message: String,
) -> G3RsHexarchSourceCrateFacts {
    G3RsHexarchSourceCrateFacts {
        crate_name,
        rel_dir: rel_dir.to_owned(),
        layer: layer_from_path(rel_dir),
        pub_trait_count: 0,
        public_free_fn_count: 0,
        public_inherent_method_count: 0,
        source_error_rel_path: Some(error_rel_path),
        source_error_message: Some(error_message),
    }
}

#[derive(Default)]
struct ModuleStats {
    pub_trait_count: usize,
    public_free_fn_count: usize,
    public_inherent_method_count: usize,
}

fn determine_entrypoints(
    view: &CrawlView<'_>,
    rel_dir: &str,
    cargo_rel_path: &str,
    parsed: &Value,
    source_error: &mut Option<(String, String)>,
) -> Vec<String> {
    let mut explicit_entrypoints = BTreeSet::new();

    if let Some(lib_path) = parsed
        .get("lib")
        .and_then(Value::as_table)
        .and_then(|table| table.get("path"))
        .and_then(Value::as_str)
    {
        let _ = explicit_entrypoints.insert(CrawlView::join_rel(rel_dir, lib_path));
    }

    if let Some(bin_array) = parsed.get("bin").and_then(Value::as_array) {
        for bin in bin_array.iter().filter_map(Value::as_table) {
            if let Some(path) = bin.get("path").and_then(Value::as_str) {
                let _ = explicit_entrypoints.insert(CrawlView::join_rel(rel_dir, path));
            }
        }
    }

    if explicit_entrypoints.is_empty() {
        let src_rel = CrawlView::join_rel(rel_dir, "src");
        if !view.dir_exists(&src_rel) {
            return Vec::new();
        }

        for fallback in [
            CrawlView::join_rel(rel_dir, "src/lib.rs"),
            CrawlView::join_rel(rel_dir, "src/main.rs"),
        ] {
            if view.file_exists(&fallback) {
                let _ = explicit_entrypoints.insert(fallback);
            }
        }
    }

    let existing = explicit_entrypoints
        .into_iter()
        .filter(|entrypoint| view.file_exists(entrypoint))
        .collect::<Vec<_>>();

    if existing.is_empty() {
        let has_explicit = parsed
            .get("lib")
            .and_then(Value::as_table)
            .and_then(|table| table.get("path"))
            .and_then(Value::as_str)
            .is_some()
            || parsed
                .get("bin")
                .and_then(Value::as_array)
                .is_some_and(|bins| {
                    bins.iter().filter_map(Value::as_table).any(|bin| {
                        bin.get("path").and_then(Value::as_str).is_some()
                    })
                });

        *source_error = Some(if has_explicit {
            (
                cargo_rel_path.to_owned(),
                format!(
                    "Failed to determine Rust source entrypoint for hexarch checks: configured target path(s) not found: {}.",
                    collect_configured_entrypoints(rel_dir, parsed).join(", ")
                ),
            )
        } else {
            (
                CrawlView::join_rel(rel_dir, "src"),
                "Failed to determine Rust source entrypoint for hexarch checks: expected src/lib.rs or src/main.rs.".to_owned(),
            )
        });
    }

    existing
}

fn collect_configured_entrypoints(rel_dir: &str, parsed: &Value) -> Vec<String> {
    let mut entrypoints = BTreeSet::new();

    if let Some(lib_path) = parsed
        .get("lib")
        .and_then(Value::as_table)
        .and_then(|table| table.get("path"))
        .and_then(Value::as_str)
    {
        let _ = entrypoints.insert(CrawlView::join_rel(rel_dir, lib_path));
    }

    if let Some(bin_array) = parsed.get("bin").and_then(Value::as_array) {
        for bin in bin_array.iter().filter_map(Value::as_table) {
            if let Some(path) = bin.get("path").and_then(Value::as_str) {
                let _ = entrypoints.insert(CrawlView::join_rel(rel_dir, path));
            }
        }
    }

    entrypoints.into_iter().collect()
}

fn walk_module_file(
    view: &CrawlView<'_>,
    rel_path: &str,
    source_error: &mut Option<(String, String)>,
    visited: &mut BTreeSet<String>,
    public_module: bool,
) -> ModuleStats {
    if source_error.is_some() || !visited.insert(rel_path.to_owned()) {
        return ModuleStats::default();
    }

    let Some(entry) = view.entry(rel_path) else {
        *source_error = Some((rel_path.to_owned(), "Path not in scope for hexarch checks".to_owned()));
        return ModuleStats::default();
    };
    if !entry.readable {
        *source_error = Some((
            rel_path.to_owned(),
            "Failed to read Rust source file for hexarch checks: file is not readable".to_owned(),
        ));
        return ModuleStats::default();
    }

    let content = match view.read_file(rel_path) {
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

    count_items(view, rel_path, &parsed.items, source_error, visited, public_module)
}

fn count_items(
    view: &CrawlView<'_>,
    rel_path: &str,
    items: &[syn::Item],
    source_error: &mut Option<(String, String)>,
    visited: &mut BTreeSet<String>,
    public_module: bool,
) -> ModuleStats {
    let mut stats = ModuleStats::default();
    for item in items {
        if source_error.is_some() || is_cfg_test_only(item.attrs()) {
            continue;
        }
        match item {
            syn::Item::Trait(item_trait) => {
                if public_module && matches!(item_trait.vis, syn::Visibility::Public(_)) {
                    stats.pub_trait_count += 1;
                }
            }
            syn::Item::Fn(item_fn) => {
                if public_module && matches!(item_fn.vis, syn::Visibility::Public(_)) {
                    stats.public_free_fn_count += 1;
                }
            }
            syn::Item::Impl(item_impl) => {
                if public_module && item_impl.trait_.is_none() {
                    stats.public_inherent_method_count += item_impl
                        .items
                        .iter()
                        .filter_map(|item| match item {
                            syn::ImplItem::Fn(item_fn)
                                if matches!(item_fn.vis, syn::Visibility::Public(_)) =>
                            {
                                Some(())
                            }
                            _ => None,
                        })
                        .count();
                }
            }
            syn::Item::Mod(item_mod) => {
                let child_public =
                    public_module && matches!(item_mod.vis, syn::Visibility::Public(_));
                if let Some((_, nested_items)) = &item_mod.content {
                    let nested =
                        count_items(view, rel_path, nested_items, source_error, visited, child_public);
                    stats.pub_trait_count += nested.pub_trait_count;
                    stats.public_free_fn_count += nested.public_free_fn_count;
                    stats.public_inherent_method_count += nested.public_inherent_method_count;
                } else if let Some(module_path) = resolve_module_path(view, rel_path, item_mod) {
                    let nested =
                        walk_module_file(view, &module_path, source_error, visited, child_public);
                    stats.pub_trait_count += nested.pub_trait_count;
                    stats.public_free_fn_count += nested.public_free_fn_count;
                    stats.public_inherent_method_count += nested.public_inherent_method_count;
                }
            }
            _ => {}
        }
    }
    stats
}

fn resolve_module_path(view: &CrawlView<'_>, rel_path: &str, item_mod: &syn::ItemMod) -> Option<String> {
    let file_dir = rel_path.rsplit_once('/').map_or("", |(dir, _)| dir);
    if let Some(path_attr) = module_path_attr(item_mod) {
        let candidate = CrawlView::join_rel(file_dir, &path_attr);
        return view.file_exists(&candidate).then_some(candidate);
    }

    let module_name = item_mod.ident.to_string();
    let file_name = rel_path.rsplit('/').next().unwrap_or(rel_path);
    let stem = file_name.strip_suffix(".rs").unwrap_or(file_name);
    let module_dir = if matches!(stem, "lib" | "main" | "mod") {
        file_dir.to_owned()
    } else {
        CrawlView::join_rel(file_dir, stem)
    };

    let direct = CrawlView::join_rel(&module_dir, &format!("{module_name}.rs"));
    if view.file_exists(&direct) {
        return Some(direct);
    }

    let nested = CrawlView::join_rel(&module_dir, &format!("{module_name}/mod.rs"));
    view.file_exists(&nested).then_some(nested)
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

fn layer_from_path(path: &str) -> Option<G3RsHexarchLayer> {
    if contains_segment(path, "domain") {
        Some(G3RsHexarchLayer::Domain)
    } else if contains_segment(path, "ports") {
        Some(G3RsHexarchLayer::Ports)
    } else if contains_segment(path, "app") {
        Some(G3RsHexarchLayer::App)
    } else if contains_segment(path, "adapters") {
        Some(G3RsHexarchLayer::Adapters)
    } else {
        None
    }
}

fn contains_segment(path: &str, segment: &str) -> bool {
    path.split('/').any(|part| part == segment)
}
