use proc_macro2::Span;
use syn::spanned::Spanned;

use g3rs_arch_types::types::{
    G3RsArchCrateNode, G3RsArchFacadeItem, G3RsArchFacadeSurface, G3RsArchFeatureExport,
    G3RsArchPathAttrSite, G3RsArchSourceChecksInput, G3RsArchSourceCrate,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use crate::error::G3RsArchIngestionError;
use crate::view::CrawlView;
use crate::workspace::{
    collect_crate_nodes, collect_rs_files_recursive, should_stop_at_nested_crate,
};

pub(crate) fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsArchSourceChecksInput>, G3RsArchIngestionError> {
    let view = CrawlView::new(crawl);
    let crate_nodes = collect_crate_nodes(&view)?;
    let facade_surfaces = collect_facade_surfaces(&view, &crate_nodes);
    let path_attr_sites = collect_path_attr_sites(&view, &crate_nodes)?;

    Ok(vec![G3RsArchSourceChecksInput {
        crates: collect_source_crates(&crate_nodes),
        facade_surfaces,
        path_attr_sites,
    }])
}

fn collect_source_crates(crate_nodes: &[G3RsArchCrateNode]) -> Vec<G3RsArchSourceCrate> {
    crate_nodes
        .iter()
        .map(|node| G3RsArchSourceCrate {
            rel_dir: node.rel_dir.clone(),
            lib_rs_rel: node.lib_rs_rel.clone(),
        })
        .collect()
}

pub(crate) fn collect_facade_surfaces(
    view: &CrawlView<'_>,
    crate_nodes: &[G3RsArchCrateNode],
) -> Vec<G3RsArchFacadeSurface> {
    let mut surfaces = Vec::new();
    let crate_dirs = crate_nodes
        .iter()
        .map(|node| node.rel_dir.as_str())
        .collect::<Vec<_>>();

    for node in crate_nodes {
        if let Some(lib_rel) = &node.lib_rs_rel {
            if let Some(surface) = analyze_facade(view, lib_rel, true, false) {
                surfaces.push(surface);
            }
        }
    }

    for node in crate_nodes {
        collect_mod_rs_recursive(
            view,
            &node.rel_dir,
            &node.rel_dir,
            &crate_dirs,
            &mut surfaces,
        );
    }

    surfaces
}

fn collect_mod_rs_recursive(
    view: &CrawlView<'_>,
    root_dir: &str,
    dir: &str,
    crate_dirs: &[&str],
    surfaces: &mut Vec<G3RsArchFacadeSurface>,
) {
    let Some(entry) = view.dir_contents(dir) else {
        return;
    };
    if entry.files().iter().any(|file| file == "mod.rs") {
        let rel_path = CrawlView::join_rel(dir, "mod.rs");
        if let Some(surface) = analyze_facade(view, &rel_path, false, true) {
            surfaces.push(surface);
        }
    }
    for subdir in entry.dirs() {
        let child_dir = CrawlView::join_rel(dir, subdir);
        if should_stop_at_nested_crate(view, root_dir, &child_dir, crate_dirs) {
            continue;
        }
        collect_mod_rs_recursive(view, root_dir, &child_dir, crate_dirs, surfaces);
    }
}

fn analyze_facade(
    view: &CrawlView<'_>,
    rel_path: &str,
    is_lib_rs: bool,
    is_mod_rs: bool,
) -> Option<G3RsArchFacadeSurface> {
    let content = view.read_file(rel_path).ok()?;
    let ast = syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(&content)).ok()?;

    let mut body_items = Vec::new();
    let mut broad_reexports = Vec::new();
    let mut pub_exports = Vec::new();
    let mut ungated_export_count = 0;
    let mut gated_on_all_count = 0;

    for item in &ast.items {
        let feature_gate = extract_feature_gate(item);
        let gated_on_all = feature_gate.as_deref() == Some("all");

        match item {
            syn::Item::Mod(module) => {
                if is_pub(&module.vis) {
                    if module.content.is_some() {
                        body_items.push(G3RsArchFacadeItem {
                            line: span_line(module.span()),
                            kind: "inline module",
                            name: module.ident.to_string(),
                            is_broad_reexport: false,
                            feature_gate: feature_gate.clone(),
                            gated_on_all,
                        });
                    } else {
                        pub_exports.push(G3RsArchFeatureExport {
                            line: span_line(module.span()),
                            name: module.ident.to_string(),
                            feature_gate: feature_gate.clone(),
                            gated_on_all,
                        });
                        if feature_gate.is_none() {
                            ungated_export_count += 1;
                        }
                        if gated_on_all {
                            gated_on_all_count += 1;
                        }
                    }
                } else if module.content.is_some() {
                    body_items.push(G3RsArchFacadeItem {
                        line: span_line(module.span()),
                        kind: "inline module",
                        name: module.ident.to_string(),
                        is_broad_reexport: false,
                        feature_gate,
                        gated_on_all,
                    });
                }
            }
            syn::Item::Use(item_use) => {
                if is_pub(&item_use.vis) {
                    let is_broad = is_broad_reexport(&item_use.tree);
                    let item = G3RsArchFacadeItem {
                        line: span_line(item_use.span()),
                        kind: "pub use",
                        name: use_tree_name(&item_use.tree),
                        is_broad_reexport: is_broad,
                        feature_gate: feature_gate.clone(),
                        gated_on_all,
                    };
                    if is_broad {
                        broad_reexports.push(item.clone());
                    }
                    pub_exports.push(G3RsArchFeatureExport {
                        line: item.line,
                        name: item.name.clone(),
                        feature_gate: feature_gate.clone(),
                        gated_on_all,
                    });
                    if feature_gate.is_none() {
                        ungated_export_count += 1;
                    }
                    if gated_on_all {
                        gated_on_all_count += 1;
                    }
                }
            }
            syn::Item::Fn(item_fn) => body_items.push(simple_item(
                item_fn.span(),
                "function",
                item_fn.sig.ident.to_string(),
                feature_gate,
                gated_on_all,
            )),
            syn::Item::Impl(item_impl) => body_items.push(simple_item(
                item_impl.span(),
                "impl",
                "impl".to_owned(),
                feature_gate,
                gated_on_all,
            )),
            syn::Item::ExternCrate(item) => body_items.push(simple_item(
                item.span(),
                "extern crate",
                item.ident.to_string(),
                feature_gate,
                gated_on_all,
            )),
            syn::Item::Static(item) => body_items.push(simple_item(
                item.span(),
                "static",
                item.ident.to_string(),
                feature_gate,
                gated_on_all,
            )),
            syn::Item::ForeignMod(item) => body_items.push(simple_item(
                item.span(),
                "extern block",
                "extern".to_owned(),
                feature_gate,
                gated_on_all,
            )),
            syn::Item::Macro(item) => body_items.push(simple_item(
                item.span(),
                "macro item",
                item.ident
                    .as_ref()
                    .map_or_else(|| "macro".to_owned(), std::string::ToString::to_string),
                feature_gate,
                gated_on_all,
            )),
            _ => {}
        }
    }

    Some(G3RsArchFacadeSurface {
        rel_path: rel_path.to_owned(),
        is_lib_rs,
        is_mod_rs,
        body_items,
        broad_reexports,
        pub_exports: pub_exports.clone(),
        pub_export_count: pub_exports.len(),
        ungated_export_count,
        gated_on_all_count,
    })
}

fn simple_item(
    span: Span,
    kind: &'static str,
    name: String,
    feature_gate: Option<String>,
    gated_on_all: bool,
) -> G3RsArchFacadeItem {
    G3RsArchFacadeItem {
        line: span_line(span),
        kind,
        name,
        is_broad_reexport: false,
        feature_gate,
        gated_on_all,
    }
}

fn collect_path_attr_sites(
    view: &CrawlView<'_>,
    crate_nodes: &[G3RsArchCrateNode],
) -> Result<Vec<G3RsArchPathAttrSite>, G3RsArchIngestionError> {
    let mut rel_paths = Vec::new();
    let crate_dirs = crate_nodes
        .iter()
        .map(|node| node.rel_dir.as_str())
        .collect::<Vec<_>>();
    for node in crate_nodes {
        collect_rs_files_recursive(
            view,
            &node.rel_dir,
            &node.rel_dir,
            &crate_dirs,
            &mut rel_paths,
        );
    }
    rel_paths.sort();
    rel_paths.dedup();

    rel_paths
        .into_iter()
        .map(|rel_path| {
            let entry =
                view.entry(&rel_path)
                    .ok_or_else(|| G3RsArchIngestionError::Unreadable {
                        path: view.root_abs_path().join(&rel_path),
                        reason: "selected Rust source missing from crawl".to_owned(),
                    })?;
            if !entry.readable {
                return Err(G3RsArchIngestionError::Unreadable {
                    path: entry.path.abs_path.clone(),
                    reason: "file is not readable".to_owned(),
                });
            }
            let content =
                view.read_file(&rel_path)
                    .map_err(|err| G3RsArchIngestionError::Unreadable {
                        path: entry.path.abs_path.clone(),
                        reason: err.to_string(),
                    })?;
            Ok(collect_file_path_attr_sites(&rel_path, &content))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|sites| sites.into_iter().flatten().collect())
}

fn collect_file_path_attr_sites(rel_path: &str, content: &str) -> Vec<G3RsArchPathAttrSite> {
    let Ok(ast) = syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content)) else {
        return Vec::new();
    };

    ast.items
        .iter()
        .filter_map(|item| {
            let syn::Item::Mod(module) = item else {
                return None;
            };
            let path_attr = module.attrs.iter().find(|attr| attr.path().is_ident("path"))?;
            Some(G3RsArchPathAttrSite {
                rel_path: rel_path.to_owned(),
                line: span_line(path_attr.span()),
                module_name: module.ident.to_string(),
                path_value: extract_path_attr_value(path_attr),
                cfg_test_only: module.attrs.iter().any(attr_is_cfg_test),
            })
        })
        .collect()
}

fn is_pub(visibility: &syn::Visibility) -> bool {
    matches!(visibility, syn::Visibility::Public(_))
}

fn span_line(span: Span) -> usize {
    span.start().line
}

fn use_tree_name(tree: &syn::UseTree) -> String {
    match tree {
        syn::UseTree::Path(path) => {
            format!("{}::{}", path.ident, use_tree_name(&path.tree))
        }
        syn::UseTree::Name(name) => name.ident.to_string(),
        syn::UseTree::Rename(rename) => rename.ident.to_string(),
        syn::UseTree::Glob(_) => "*".to_owned(),
        syn::UseTree::Group(_) => "{...}".to_owned(),
    }
}

fn is_broad_reexport(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Glob(_) => true,
        syn::UseTree::Path(path) => is_broad_reexport(&path.tree),
        _ => false,
    }
}

fn extract_feature_gate(item: &syn::Item) -> Option<String> {
    let attrs = match item {
        syn::Item::Mod(module) => &module.attrs,
        syn::Item::Use(item_use) => &item_use.attrs,
        syn::Item::Fn(item_fn) => &item_fn.attrs,
        syn::Item::Impl(item_impl) => &item_impl.attrs,
        syn::Item::ExternCrate(item) => &item.attrs,
        syn::Item::Static(item) => &item.attrs,
        syn::Item::ForeignMod(item) => &item.attrs,
        syn::Item::Macro(item) => &item.attrs,
        _ => return None,
    };

    for attr in attrs {
        if !attr.path().is_ident("cfg") {
            continue;
        }
        let Ok(expr) = attr.parse_args::<syn::Expr>() else {
            continue;
        };
        if let Some(feature) = extract_feature_expr(&expr) {
            return Some(feature);
        }
    }
    None
}

fn extract_feature_expr(expr: &syn::Expr) -> Option<String> {
    match expr {
        syn::Expr::Assign(assign) => {
            let syn::Expr::Path(path) = &*assign.left else {
                return None;
            };
            if path.path.is_ident("feature") {
                let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(value),
                    ..
                }) = &*assign.right
                else {
                    return None;
                };
                return Some(value.value());
            }
            None
        }
        syn::Expr::Call(call) => call.args.iter().find_map(extract_feature_expr),
        _ => None,
    }
}

fn extract_path_attr_value(attr: &syn::Attribute) -> Option<String> {
    let syn::Meta::NameValue(name_value) = &attr.meta else {
        return None;
    };
    let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(value),
        ..
    }) = &name_value.value
    else {
        return None;
    };
    Some(value.value())
}

fn attr_is_cfg_test(attr: &syn::Attribute) -> bool {
    if !attr.path().is_ident("cfg") {
        return false;
    }
    let syn::Meta::List(list) = &attr.meta else {
        return false;
    };
    let Ok(meta) = list.parse_args::<syn::Meta>() else {
        return false;
    };
    cfg_meta_is_test(&meta, true)
}

fn cfg_meta_is_test(meta: &syn::Meta, positive: bool) -> bool {
    match meta {
        syn::Meta::Path(path) => positive && path.is_ident("test"),
        syn::Meta::List(list) if list.path.is_ident("all") || list.path.is_ident("any") => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|items| items.iter().any(|item| cfg_meta_is_test(item, positive))),
        syn::Meta::List(list) if list.path.is_ident("not") => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|items| items.iter().any(|item| cfg_meta_is_test(item, !positive))),
        _ => false,
    }
}

#[cfg(test)]
#[path = "source_tests/mod.rs"]
mod source_tests;
