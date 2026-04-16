use std::collections::{BTreeMap, BTreeSet};

use g3rs_apparch_types::{
    G3RsApparchPublicItem, G3RsApparchPublicItemKind, G3RsApparchSourceChecksInput,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use super::error::G3RsApparchIngestionError;
use super::model::CrateRecord;
use super::workspace::{collect_workspace_crates, load_workspace_root};
use crate::view::CrawlView;

#[cfg(test)]
#[path = "source_tests/mod.rs"]
mod source_tests;

pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsApparchSourceChecksInput, G3RsApparchIngestionError> {
    let view = CrawlView::new(crawl);
    let workspace = load_workspace_root(&view)?;
    let records = collect_workspace_crates(&view, &workspace)?;

    let mut public_items = Vec::new();
    for record in &records {
        public_items.extend(collect_public_items_for_crate(&view, record)?);
    }

    Ok(G3RsApparchSourceChecksInput {
        crates: records.iter().map(|record| record.krate.clone()).collect(),
        public_items,
    })
}

fn collect_public_items_for_crate(
    view: &CrawlView<'_>,
    record: &CrateRecord,
) -> Result<Vec<G3RsApparchPublicItem>, G3RsApparchIngestionError> {
    let entrypoints = determine_entrypoints(view, record);
    let mut public_items = Vec::new();
    let mut visited = BTreeMap::new();
    for entrypoint in entrypoints {
        walk_module_file(
            view,
            &entrypoint,
            &record.krate.cargo_rel_path,
            &mut visited,
            true,
            &mut public_items,
        )?;
    }
    Ok(public_items)
}

fn determine_entrypoints(view: &CrawlView<'_>, record: &CrateRecord) -> Vec<String> {
    let mut entrypoints = BTreeSet::new();
    let rel_dir = &record.krate.rel_dir;

    if let Some(lib) = &record.cargo.lib
        && let Some(path) = &lib.path
    {
        let rel_path = CrawlView::join_rel(rel_dir, path);
        if view.file_exists(&rel_path) {
            let _ = entrypoints.insert(rel_path);
        }
    }

    for bin in &record.cargo.bin {
        if let Some(path) = &bin.path {
            let rel_path = CrawlView::join_rel(rel_dir, path);
            if view.file_exists(&rel_path) {
                let _ = entrypoints.insert(rel_path);
            }
        }
    }

    for rel_path in [
        CrawlView::join_rel(rel_dir, "src/lib.rs"),
        CrawlView::join_rel(rel_dir, "src/main.rs"),
    ] {
        if view.file_exists(&rel_path) {
            let _ = entrypoints.insert(rel_path);
        }
    }

    entrypoints.into_iter().collect()
}

fn walk_module_file(
    view: &CrawlView<'_>,
    rel_path: &str,
    cargo_rel_path: &str,
    visited: &mut BTreeMap<String, bool>,
    public_module: bool,
    public_items: &mut Vec<G3RsApparchPublicItem>,
) -> Result<(), G3RsApparchIngestionError> {
    match visited.get(rel_path).copied() {
        Some(true) => return Ok(()),
        Some(false) if !public_module => return Ok(()),
        _ => {
            let _ = visited.insert(rel_path.to_owned(), public_module);
        }
    }
    let Some(entry) = view.entry(rel_path) else {
        return Err(G3RsApparchIngestionError::NormalizationFailed {
            path: std::path::PathBuf::from(rel_path),
            reason: "source path not present in crawl".to_owned(),
        });
    };
    if !entry.readable {
        return Err(G3RsApparchIngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }
    let content = view
        .read_file(rel_path)
        .map_err(|error| G3RsApparchIngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: error.to_string(),
        })?;
    let parsed = syn::parse_file(&content).map_err(|error| G3RsApparchIngestionError::ParseFailed {
        path: entry.path.abs_path.clone(),
        reason: error.to_string(),
    })?;
    if is_cfg_test_only(&parsed.attrs) {
        return Ok(());
    }
    walk_items(
        view,
        rel_path,
        cargo_rel_path,
        &parsed.items,
        visited,
        public_module,
        public_items,
    )
}

fn walk_items(
    view: &CrawlView<'_>,
    rel_path: &str,
    cargo_rel_path: &str,
    items: &[syn::Item],
    visited: &mut BTreeMap<String, bool>,
    public_module: bool,
    public_items: &mut Vec<G3RsApparchPublicItem>,
) -> Result<(), G3RsApparchIngestionError> {
    for item in items {
        if is_cfg_test_only(item.attrs()) {
            continue;
        }
        match item {
            syn::Item::Trait(item_trait) => {
                if public_module && matches!(item_trait.vis, syn::Visibility::Public(_)) {
                    public_items.push(G3RsApparchPublicItem {
                        cargo_rel_path: cargo_rel_path.to_owned(),
                        rel_path: rel_path.to_owned(),
                        item_name: item_trait.ident.to_string(),
                        owner_name: None,
                        kind: G3RsApparchPublicItemKind::Trait,
                    });
                }
            }
            syn::Item::Fn(item_fn) => {
                if public_module && matches!(item_fn.vis, syn::Visibility::Public(_)) {
                    public_items.push(G3RsApparchPublicItem {
                        cargo_rel_path: cargo_rel_path.to_owned(),
                        rel_path: rel_path.to_owned(),
                        item_name: item_fn.sig.ident.to_string(),
                        owner_name: None,
                        kind: G3RsApparchPublicItemKind::FreeFunction,
                    });
                }
            }
            syn::Item::Impl(item_impl) => {
                if !public_module || item_impl.trait_.is_some() {
                    continue;
                }
                let owner_name = self_type_name(item_impl.self_ty.as_ref());
                for impl_item in &item_impl.items {
                    let syn::ImplItem::Fn(method) = impl_item else {
                        continue;
                    };
                    if !matches!(method.vis, syn::Visibility::Public(_)) {
                        continue;
                    }
                    public_items.push(G3RsApparchPublicItem {
                        cargo_rel_path: cargo_rel_path.to_owned(),
                        rel_path: rel_path.to_owned(),
                        item_name: method.sig.ident.to_string(),
                        owner_name: owner_name.clone(),
                        kind: G3RsApparchPublicItemKind::InherentMethod,
                    });
                }
            }
            syn::Item::Mod(item_mod) => {
                if let Some((_, nested_items)) = &item_mod.content {
                    walk_items(
                        view,
                        rel_path,
                        cargo_rel_path,
                        nested_items,
                        visited,
                        public_module,
                        public_items,
                    )?;
                } else {
                    let module_path = resolve_module_path(view, rel_path, item_mod)?;
                    walk_module_file(
                        view,
                        &module_path,
                        cargo_rel_path,
                        visited,
                        public_module,
                        public_items,
                    )?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn self_type_name(self_ty: &syn::Type) -> Option<String> {
    match self_ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        _ => None,
    }
}

fn resolve_module_path(
    view: &CrawlView<'_>,
    rel_path: &str,
    item_mod: &syn::ItemMod,
) -> Result<String, G3RsApparchIngestionError> {
    let file_dir = rel_path.rsplit_once('/').map_or("", |(dir, _)| dir);
    if let Some(path_attr) = module_path_attr(item_mod) {
        let candidate = CrawlView::join_rel(file_dir, &path_attr);
        return view.file_exists(&candidate).then_some(candidate).ok_or_else(|| {
            G3RsApparchIngestionError::NormalizationFailed {
                path: std::path::PathBuf::from(rel_path),
                reason: format!(
                    "declared module `{}` points to missing file `{}`",
                    item_mod.ident, path_attr
                ),
            }
        });
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
        return Ok(direct);
    }

    let nested = CrawlView::join_rel(&module_dir, &format!("{module_name}/mod.rs"));
    if view.file_exists(&nested) {
        return Ok(nested);
    }

    Err(G3RsApparchIngestionError::NormalizationFailed {
        path: std::path::PathBuf::from(rel_path),
        reason: format!(
            "declared module `{}` has no backing file under `{}`",
            item_mod.ident, file_dir
        ),
    })
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
