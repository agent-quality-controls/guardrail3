use std::collections::BTreeMap;
use std::path::PathBuf;

use g3rs_arch_ingestion_types::{
    G3RsArchConfigChecksInput, G3RsArchFileTreeChecksInput, G3RsArchIngestionError,
    G3RsArchSourceChecksInput,
};
use g3rs_arch_types::types::{
    G3RsArchBoundaryRef, G3RsArchConfigCrate, G3RsArchCrateNode, G3RsArchCrateStructure,
    G3RsArchDependencyCounts, G3RsArchDependencyEdge, G3RsArchFacadeItem,
    G3RsArchFacadeSurface, G3RsArchFeatureContract, G3RsArchFeatureExport,
    G3RsArchFileTreeCrate, G3RsArchModuleDir, G3RsArchRustPolicyState, G3RsArchSourceCrate,
    G3RsArchSourceFile,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;
use glob::Pattern;
use proc_macro2::Span;
use syn::spanned::Spanned;
use toml::Value;

use crate::view::CrawlView;

pub fn ingest_for_source_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsArchSourceChecksInput>, G3RsArchIngestionError> {
    let view = CrawlView::new(crawl);
    let crate_nodes = collect_crate_nodes(&view)?;
    let facade_surfaces = collect_facade_surfaces(&view, &crate_nodes);
    let source_files = collect_rs_files(&view, &crate_nodes)?;

    Ok(vec![G3RsArchSourceChecksInput {
        crates: collect_source_crates(&crate_nodes),
        facade_surfaces,
        source_files,
    }])
}

pub fn ingest_for_config_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<Vec<G3RsArchConfigChecksInput>, G3RsArchIngestionError> {
    let view = CrawlView::new(crawl);
    let crate_nodes = collect_crate_nodes(&view)?;
    let facade_surfaces = collect_facade_surfaces(&view, &crate_nodes);
    let dependency_edges = collect_dependency_edges(&view, &crate_nodes)?;

    Ok(vec![G3RsArchConfigChecksInput {
        crates: collect_config_crates(&crate_nodes, &facade_surfaces),
        dependency_edges,
    }])
}

pub fn ingest_for_file_tree_checks(
    crawl: &G3RsWorkspaceCrawl,
) -> Result<G3RsArchFileTreeChecksInput, G3RsArchIngestionError> {
    let view = CrawlView::new(crawl);
    let crate_nodes = collect_crate_nodes(&view)?;
    let module_dirs = collect_module_dirs(&view, &crate_nodes)?;

    Ok(G3RsArchFileTreeChecksInput {
        crates: collect_file_tree_crates(&crate_nodes),
        module_dirs,
        rust_policy: ingest_rust_policy(&view),
    })
}

fn ingest_rust_policy(view: &CrawlView<'_>) -> G3RsArchRustPolicyState {
    let Some(entry) = view.entry("guardrail3-rs.toml") else {
        return G3RsArchRustPolicyState::Missing;
    };
    if !entry.readable {
        return G3RsArchRustPolicyState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "file is not readable".to_owned(),
        };
    }
    let content = match view.read_file("guardrail3-rs.toml") {
        Ok(content) => content,
        Err(err) => {
            return G3RsArchRustPolicyState::Unreadable {
                rel_path: entry.path.rel_path.clone(),
                reason: err.to_string(),
            };
        }
    };
    let parsed = match guardrail3_rs_toml_parser::parse(&content) {
        Ok(parsed) => parsed,
        Err(err) => {
            return G3RsArchRustPolicyState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: err.to_string(),
            };
        }
    };
    G3RsArchRustPolicyState::Parsed {
        rel_path: entry.path.rel_path.clone(),
        waivers: parsed.waivers,
    }
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

fn collect_config_crates(
    crate_nodes: &[G3RsArchCrateNode],
    facade_surfaces: &[G3RsArchFacadeSurface],
) -> Vec<G3RsArchConfigCrate> {
    let requires_feature_contract = facade_surfaces
        .iter()
        .filter(|surface| surface.is_lib_rs && surface.pub_export_count > 0)
        .map(|surface| surface.rel_path.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    crate_nodes
        .iter()
        .map(|node| G3RsArchConfigCrate {
            rel_dir: node.rel_dir.clone(),
            cargo_rel_path: node.cargo_rel_path.clone(),
            shared: node.shared,
            production_dependency_count: node.dependency_counts.production,
            dev_dependency_count: node.dependency_counts.dev,
            requires_feature_contract: node
                .lib_rs_rel
                .as_deref()
                .is_some_and(|rel| requires_feature_contract.contains(rel)),
            has_default_feature: node.feature_contract.has_default_feature,
            has_all_feature: node.feature_contract.has_all_feature,
            all_feature_deps: node.feature_contract.all_feature_deps.clone(),
            default_feature_deps: node.feature_contract.default_feature_deps.clone(),
        })
        .collect()
}

fn collect_file_tree_crates(crate_nodes: &[G3RsArchCrateNode]) -> Vec<G3RsArchFileTreeCrate> {
    crate_nodes
        .iter()
        .map(|node| G3RsArchFileTreeCrate {
            rel_dir: node.rel_dir.clone(),
            cargo_rel_path: node.cargo_rel_path.clone(),
            has_package: node.has_package,
            has_lib_rs: node.has_lib_rs,
            has_main_rs: node.has_main_rs,
            sibling_rs_file_count: node.structure.sibling_rs_file_count,
            sibling_dir_count: node.structure.sibling_dir_count,
            max_module_depth: node.structure.max_module_depth,
            cargo_parse_error: node.cargo_parse_error.clone(),
        })
        .collect()
}

fn collect_crate_nodes(
    view: &CrawlView<'_>,
) -> Result<Vec<G3RsArchCrateNode>, G3RsArchIngestionError> {
    let mut cargo_dirs = discover_crate_dirs(view)?;
    cargo_dirs.sort();
    cargo_dirs.dedup();
    let crate_dirs = cargo_dirs.iter().map(String::as_str).collect::<Vec<_>>();

    let mut nodes = cargo_dirs
        .iter()
        .map(|dir| build_crate_node(view, dir, &crate_dirs))
        .collect::<Result<Vec<_>, _>>()?;

    let rel_dirs = nodes
        .iter()
        .map(|node| node.rel_dir.clone())
        .collect::<Vec<_>>();
    for rel_dir in rel_dirs {
        let parent = find_parent_dir(&rel_dir, &nodes);
        if let Some(node) = nodes.iter_mut().find(|node| node.rel_dir == rel_dir) {
            node.parent_rel_dir = parent;
        }
    }

    Ok(nodes)
}

fn discover_crate_dirs(view: &CrawlView<'_>) -> Result<Vec<String>, G3RsArchIngestionError> {
    let Some(root_entry) = view.entry("Cargo.toml") else {
        return Ok(Vec::new());
    };
    if !root_entry.readable {
        return Err(G3RsArchIngestionError::Unreadable {
            path: root_entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let content =
        view.read_file("Cargo.toml")
            .map_err(|err| G3RsArchIngestionError::Unreadable {
                path: root_entry.path.abs_path.clone(),
                reason: err.to_string(),
            })?;
    let parsed =
        toml::from_str::<Value>(&content).map_err(|err| G3RsArchIngestionError::ParseFailed {
            path: root_entry.path.abs_path.clone(),
            reason: err.to_string(),
        })?;

    let mut dirs = Vec::new();
    if parsed.get("package").is_some() {
        dirs.push(String::new());
    }
    dirs.extend(select_workspace_member_dirs(view, &parsed)?);

    Ok(dirs)
}

fn select_workspace_member_dirs(
    view: &CrawlView<'_>,
    root_manifest: &Value,
) -> Result<Vec<String>, G3RsArchIngestionError> {
    let Some(workspace) = root_manifest.get("workspace").and_then(Value::as_table) else {
        return Ok(Vec::new());
    };

    let member_patterns = workspace
        .get("members")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(|pattern| {
            Pattern::new(pattern).map_err(|err| G3RsArchIngestionError::ParseFailed {
                path: view.root_abs_path().join("Cargo.toml"),
                reason: format!("invalid workspace member pattern `{pattern}`: {err}"),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let exclude_patterns = workspace
        .get("exclude")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(|pattern| {
            Pattern::new(pattern).map_err(|err| G3RsArchIngestionError::ParseFailed {
                path: view.root_abs_path().join("Cargo.toml"),
                reason: format!("invalid workspace exclude pattern `{pattern}`: {err}"),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let all_matching_member_dirs = view
        .all_dir_rels()
        .filter(|rel_dir| !rel_dir.is_empty())
        .filter(|rel_dir| view.file_exists(&CrawlView::join_rel(rel_dir, "Cargo.toml")))
        .filter(|rel_dir| {
            member_patterns
                .iter()
                .any(|pattern| pattern.matches(rel_dir))
        })
        .map(str::to_owned)
        .collect::<Vec<_>>();

    for pattern in workspace
        .get("members")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        let parsed_pattern =
            Pattern::new(pattern).map_err(|err| G3RsArchIngestionError::ParseFailed {
                path: view.root_abs_path().join("Cargo.toml"),
                reason: format!("invalid workspace member pattern `{pattern}`: {err}"),
            })?;
        if !all_matching_member_dirs
            .iter()
            .any(|member_dir| parsed_pattern.matches(member_dir))
        {
            return Err(G3RsArchIngestionError::ParseFailed {
                path: view.root_abs_path().join("Cargo.toml"),
                reason: format!(
                    "workspace member pattern `{pattern}` did not resolve to any Cargo.toml"
                ),
            });
        }
    }

    Ok(all_matching_member_dirs
        .into_iter()
        .filter(|rel_dir| {
            !exclude_patterns
                .iter()
                .any(|pattern| pattern.matches(rel_dir))
        })
        .collect())
}

fn build_crate_node(
    view: &CrawlView<'_>,
    dir: &str,
    crate_dirs: &[&str],
) -> Result<G3RsArchCrateNode, G3RsArchIngestionError> {
    let cargo_rel_path = CrawlView::join_rel(dir, "Cargo.toml");
    let entry = view
        .entry(&cargo_rel_path)
        .ok_or_else(|| G3RsArchIngestionError::Unreadable {
            path: view.root_abs_path().join(&cargo_rel_path),
            reason: "selected Cargo.toml missing from crawl".to_owned(),
        })?;
    if !entry.readable {
        return Err(G3RsArchIngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: "file is not readable".to_owned(),
        });
    }

    let content =
        view.read_file(&cargo_rel_path)
            .map_err(|err| G3RsArchIngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: err.to_string(),
            })?;

    let parsed = toml::from_str::<Value>(&content).ok();
    let parse_error = toml::from_str::<Value>(&content)
        .err()
        .map(|err| err.to_string());

    let has_package = parsed
        .as_ref()
        .and_then(|value| value.get("package"))
        .is_some();
    let has_workspace = parsed
        .as_ref()
        .and_then(|value| value.get("workspace"))
        .is_some();
    let package_name = parsed
        .as_ref()
        .and_then(|value| value.get("package"))
        .and_then(|package| package.get("name"))
        .and_then(Value::as_str)
        .map(str::to_owned);
    let shared = parsed
        .as_ref()
        .and_then(|value| value.get("package"))
        .and_then(|package| package.get("metadata"))
        .and_then(|metadata| metadata.get("guardrail3"))
        .and_then(|guardrail| guardrail.get("shared"))
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let custom_lib_path = parsed
        .as_ref()
        .and_then(|value| value.get("lib"))
        .and_then(|lib| lib.get("path"))
        .and_then(Value::as_str);
    let default_lib = CrawlView::join_rel(dir, "src/lib.rs");
    let lib_rs_rel = if let Some(custom) = custom_lib_path {
        let full = CrawlView::join_rel(dir, custom);
        if view.file_exists(&full) {
            Some(full)
        } else {
            None
        }
    } else if view.file_exists(&default_lib) {
        Some(default_lib)
    } else {
        None
    };
    let has_lib_rs = lib_rs_rel.is_some();
    let has_main_rs = view.file_exists(&CrawlView::join_rel(dir, "src/main.rs"));

    let features = parsed
        .as_ref()
        .and_then(|value| value.get("features"))
        .and_then(Value::as_table);
    let has_default_feature = features.is_some_and(|table| table.contains_key("default"));
    let has_all_feature = features.is_some_and(|table| table.contains_key("all"));
    let all_feature_deps = feature_list(features.and_then(|table| table.get("all")));
    let default_feature_deps = feature_list(features.and_then(|table| table.get("default")));
    let (production_dependency_count, dev_dependency_count) =
        parsed.as_ref().map_or((0, 0), count_dependencies);
    let src_dir = CrawlView::join_rel(dir, "src");
    let (sibling_rs_file_count, sibling_dir_count) = if view.dir_contents(&src_dir).is_some() {
        count_siblings(view, &src_dir, dir, crate_dirs)
    } else {
        count_siblings(view, dir, dir, crate_dirs)
    };
    let max_module_depth = measure_module_depth(view, dir, crate_dirs);

    Ok(G3RsArchCrateNode {
        rel_dir: dir.to_owned(),
        cargo_rel_path,
        package_name,
        has_package,
        has_workspace,
        has_lib_rs,
        has_main_rs,
        lib_rs_rel,
        parent_rel_dir: None,
        shared,
        feature_contract: G3RsArchFeatureContract {
            has_default_feature,
            has_all_feature,
            all_feature_deps,
            default_feature_deps,
        },
        dependency_counts: G3RsArchDependencyCounts {
            production: production_dependency_count,
            dev: dev_dependency_count,
        },
        structure: G3RsArchCrateStructure {
            sibling_rs_file_count,
            sibling_dir_count,
            max_module_depth,
        },
        cargo_parse_error: parse_error,
    })
}

fn collect_facade_surfaces(
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

fn collect_rs_files(
    view: &CrawlView<'_>,
    crate_nodes: &[G3RsArchCrateNode],
) -> Result<Vec<G3RsArchSourceFile>, G3RsArchIngestionError> {
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
            Ok(G3RsArchSourceFile { rel_path, content })
        })
        .collect()
}

fn collect_module_dirs(
    view: &CrawlView<'_>,
    crate_nodes: &[G3RsArchCrateNode],
) -> Result<Vec<G3RsArchModuleDir>, G3RsArchIngestionError> {
    let crate_dirs = crate_nodes
        .iter()
        .map(|node| node.rel_dir.as_str())
        .collect::<Vec<_>>();
    let mut module_dirs = BTreeMap::<String, G3RsArchModuleDir>::new();

    collect_module_dirs_from_mod_declarations(view, crate_nodes, &crate_dirs, &mut module_dirs)?;
    collect_module_dirs_from_directory_scan(view, crate_nodes, &crate_dirs, &mut module_dirs);

    Ok(module_dirs.into_values().collect())
}

fn collect_module_dirs_from_mod_declarations(
    view: &CrawlView<'_>,
    crate_nodes: &[G3RsArchCrateNode],
    crate_dirs: &[&str],
    module_dirs: &mut BTreeMap<String, G3RsArchModuleDir>,
) -> Result<(), G3RsArchIngestionError> {
    let mut rs_file_rels = Vec::<String>::new();
    for node in crate_nodes {
        collect_rs_files_recursive(
            view,
            &node.rel_dir,
            &node.rel_dir,
            crate_dirs,
            &mut rs_file_rels,
        );
    }
    rs_file_rels.sort();
    rs_file_rels.dedup();

    for rel_path in rs_file_rels {
        if is_test_or_example_path(&rel_path) {
            continue;
        }

        let entry = view
            .entry(&rel_path)
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
        let ast = syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(&content)).map_err(
            |err| G3RsArchIngestionError::ParseFailed {
                path: entry.path.abs_path.clone(),
                reason: err.to_string(),
            },
        )?;

        let dir = rel_path
            .rsplit_once('/')
            .map_or(String::new(), |(prefix, _)| prefix.to_owned());

        for item in &ast.items {
            let syn::Item::Mod(module) = item else {
                continue;
            };
            if module.content.is_some() {
                continue;
            }

            let mod_name = module.ident.to_string();
            let mod_dir = CrawlView::join_rel(&dir, &mod_name);
            let Some(mod_entry) = view.dir_contents(&mod_dir) else {
                continue;
            };
            let rs_file_count = mod_entry
                .files()
                .iter()
                .filter(|file| file.ends_with(".rs"))
                .count();
            if rs_file_count == 0 {
                continue;
            }

            let has_mod_rs = mod_entry.files().iter().any(|file| file == "mod.rs");
            let sibling_file = format!("{mod_name}.rs");
            let has_sibling_file = view
                .dir_contents(&dir)
                .is_some_and(|entry| entry.files().iter().any(|file| file == &sibling_file));

            let _ = module_dirs.insert(
                mod_dir.clone(),
                G3RsArchModuleDir {
                    dir_rel: mod_dir,
                    mod_decl_file: rel_path.clone(),
                    mod_decl_line: module.ident.span().start().line,
                    is_pub: matches!(module.vis, syn::Visibility::Public(_)),
                    has_mod_rs,
                    has_sibling_file,
                    rs_file_count,
                },
            );
        }
    }

    Ok(())
}

fn collect_module_dirs_from_directory_scan(
    view: &CrawlView<'_>,
    crate_nodes: &[G3RsArchCrateNode],
    crate_dirs: &[&str],
    module_dirs: &mut BTreeMap<String, G3RsArchModuleDir>,
) {
    let mut src_dirs = Vec::<String>::new();
    for node in crate_nodes {
        let src_dir = CrawlView::join_rel(&node.rel_dir, "src");
        if view.dir_contents(&src_dir).is_some() {
            collect_dirs_recursive(view, &node.rel_dir, &src_dir, crate_dirs, &mut src_dirs);
        }
    }
    src_dirs.sort();
    src_dirs.dedup();

    for dir in src_dirs {
        if module_dirs.contains_key(&dir) || is_test_or_example_path(&dir) {
            continue;
        }

        let Some(entry) = view.dir_contents(&dir) else {
            continue;
        };
        let rs_files = entry
            .files()
            .iter()
            .filter(|file| file.ends_with(".rs"))
            .collect::<Vec<_>>();
        if rs_files.is_empty() || !is_under_crate_src(&dir, crate_nodes) {
            continue;
        }
        if rs_files
            .iter()
            .any(|file| **file == "lib.rs" || **file == "main.rs")
        {
            continue;
        }

        let has_mod_rs = rs_files.iter().any(|file| **file == "mod.rs");
        let _ = module_dirs.insert(
            dir.clone(),
            G3RsArchModuleDir {
                dir_rel: dir,
                mod_decl_file: String::new(),
                mod_decl_line: 0,
                is_pub: false,
                has_mod_rs,
                has_sibling_file: false,
                rs_file_count: rs_files.len(),
            },
        );
    }
}

fn collect_rs_files_recursive(
    view: &CrawlView<'_>,
    root_dir: &str,
    dir: &str,
    crate_dirs: &[&str],
    rel_paths: &mut Vec<String>,
) {
    let Some(entry) = view.dir_contents(dir) else {
        return;
    };
    for file in entry.files() {
        if file.ends_with(".rs") {
            rel_paths.push(CrawlView::join_rel(dir, file));
        }
    }
    for subdir in entry.dirs() {
        let child_dir = CrawlView::join_rel(dir, subdir);
        if should_stop_at_nested_crate(view, root_dir, &child_dir, crate_dirs) {
            continue;
        }
        collect_rs_files_recursive(view, root_dir, &child_dir, crate_dirs, rel_paths);
    }
}

fn collect_dirs_recursive(
    view: &CrawlView<'_>,
    root_dir: &str,
    dir: &str,
    crate_dirs: &[&str],
    result: &mut Vec<String>,
) {
    result.push(dir.to_owned());
    let Some(entry) = view.dir_contents(dir) else {
        return;
    };
    for subdir in entry.dirs() {
        let child_dir = CrawlView::join_rel(dir, subdir);
        if should_stop_at_nested_crate(view, root_dir, &child_dir, crate_dirs) {
            continue;
        }
        collect_dirs_recursive(view, root_dir, &child_dir, crate_dirs, result);
    }
}

fn should_stop_at_nested_crate(
    view: &CrawlView<'_>,
    root_dir: &str,
    child_dir: &str,
    crate_dirs: &[&str],
) -> bool {
    if child_dir == root_dir {
        return false;
    }
    crate_dirs.iter().any(|crate_dir| *crate_dir == child_dir)
        || view.file_exists(&CrawlView::join_rel(child_dir, "Cargo.toml"))
}

fn collect_dependency_edges(
    view: &CrawlView<'_>,
    crate_nodes: &[G3RsArchCrateNode],
) -> Result<Vec<G3RsArchDependencyEdge>, G3RsArchIngestionError> {
    let mut edges = Vec::new();
    let node_map = crate_nodes
        .iter()
        .map(|node| (node.rel_dir.as_str(), node))
        .collect::<BTreeMap<_, _>>();

    for node in crate_nodes {
        if node.cargo_parse_error.is_some() {
            continue;
        }
        let content = view.read_file(&node.cargo_rel_path).map_err(|err| {
            G3RsArchIngestionError::Unreadable {
                path: view
                    .abs_path(&node.cargo_rel_path)
                    .unwrap_or_else(|| PathBuf::from(&node.cargo_rel_path)),
                reason: err.to_string(),
            }
        })?;
        let Ok(parsed) = toml::from_str::<Value>(&content) else {
            continue;
        };

        for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
            if let Some(deps) = parsed.get(section).and_then(Value::as_table) {
                collect_section_edges(
                    &node.rel_dir,
                    &node.cargo_rel_path,
                    section,
                    deps,
                    crate_nodes,
                    &node_map,
                    &mut edges,
                );
            }
        }

        if let Some(targets) = parsed.get("target").and_then(Value::as_table) {
            for target_cfg in targets.values() {
                let Some(target_table) = target_cfg.as_table() else {
                    continue;
                };
                for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
                    if let Some(deps) = target_table.get(section).and_then(Value::as_table) {
                        collect_section_edges(
                            &node.rel_dir,
                            &node.cargo_rel_path,
                            section,
                            deps,
                            crate_nodes,
                            &node_map,
                            &mut edges,
                        );
                    }
                }
            }
        }
    }

    Ok(edges)
}

fn collect_section_edges(
    source_rel_dir: &str,
    source_cargo_rel: &str,
    section: &str,
    deps: &toml::map::Map<String, Value>,
    crate_nodes: &[G3RsArchCrateNode],
    node_map: &BTreeMap<&str, &G3RsArchCrateNode>,
    edges: &mut Vec<G3RsArchDependencyEdge>,
) {
    for (alias, value) in deps {
        let Some(raw_path) = extract_path(value) else {
            continue;
        };
        let resolved_target_rel = Some(normalize_path(source_rel_dir, &raw_path));
        let target_is_crate = resolved_target_rel
            .as_ref()
            .is_some_and(|rel| node_map.contains_key(rel.as_str()));
        if !target_is_crate {
            continue;
        }
        let crossed_boundary = resolved_target_rel
            .as_ref()
            .and_then(|target_rel| boundary_violation(crate_nodes, source_rel_dir, target_rel));
        let is_direct_child = resolved_target_rel
            .as_ref()
            .is_some_and(|target_rel| is_direct_child(crate_nodes, source_rel_dir, target_rel));
        let target_shared = resolved_target_rel
            .as_ref()
            .and_then(|target_rel| node_map.get(target_rel.as_str()))
            .is_some_and(|node| node.shared);

        edges.push(G3RsArchDependencyEdge {
            source_rel_dir: source_rel_dir.to_owned(),
            source_cargo_rel: source_cargo_rel.to_owned(),
            dep_alias: alias.clone(),
            raw_path,
            resolved_target_rel,
            target_is_crate,
            section: section.to_owned(),
            crossed_boundary,
            is_direct_child,
            target_shared,
        });
    }
}

fn extract_path(value: &Value) -> Option<String> {
    match value {
        Value::Table(table) => table.get("path").and_then(Value::as_str).map(str::to_owned),
        _ => None,
    }
}

fn boundary_violation(
    crate_nodes: &[G3RsArchCrateNode],
    source_rel_dir: &str,
    target_rel_dir: &str,
) -> Option<G3RsArchBoundaryRef> {
    let mut current = target_rel_dir;
    loop {
        let Some((parent, _)) = current.rsplit_once('/') else {
            let has_root = crate_nodes.iter().any(|node| node.rel_dir.is_empty());
            if has_root && !target_rel_dir.is_empty() && !source_rel_dir.is_empty() {
                return Some(G3RsArchBoundaryRef::RootWorkspace);
            }
            return None;
        };
        if crate_nodes.iter().any(|node| node.rel_dir == parent)
            && parent != target_rel_dir
            && parent != source_rel_dir
            && !is_inside(source_rel_dir, parent)
        {
            return Some(G3RsArchBoundaryRef::Crate(parent.to_owned()));
        }
        current = parent;
    }
}

fn is_direct_child(crate_nodes: &[G3RsArchCrateNode], parent_rel: &str, child_rel: &str) -> bool {
    if !is_inside(child_rel, parent_rel) {
        return false;
    }
    parent_of(crate_nodes, child_rel).is_some_and(|rel| rel == parent_rel)
}

fn parent_of<'a>(crate_nodes: &'a [G3RsArchCrateNode], rel_dir: &str) -> Option<&'a str> {
    let mut current = rel_dir;
    loop {
        let Some((parent, _)) = current.rsplit_once('/') else {
            return crate_nodes
                .iter()
                .find(|node| node.rel_dir.is_empty() && !rel_dir.is_empty())
                .map(|node| node.rel_dir.as_str());
        };
        if let Some(node) = crate_nodes.iter().find(|node| node.rel_dir == parent) {
            return Some(node.rel_dir.as_str());
        }
        current = parent;
    }
}

fn find_parent_dir(rel_dir: &str, crate_nodes: &[G3RsArchCrateNode]) -> Option<String> {
    parent_of(crate_nodes, rel_dir).map(str::to_owned)
}

fn is_inside(inner: &str, outer: &str) -> bool {
    if outer.is_empty() {
        return !inner.is_empty();
    }
    inner.starts_with(outer) && inner.as_bytes().get(outer.len()) == Some(&b'/')
}

fn normalize_path(base: &str, rel: &str) -> String {
    let mut parts = if base.is_empty() {
        Vec::new()
    } else {
        base.split('/').collect::<Vec<_>>()
    };
    for segment in rel.split('/') {
        match segment {
            ".." => {
                let _ = parts.pop();
            }
            "." | "" => {}
            value => parts.push(value),
        }
    }
    parts.join("/")
}

fn count_dependencies(parsed: &Value) -> (usize, usize) {
    let mut production_count = 0;
    let mut dev_count = 0;
    if let Some(deps) = parsed.get("dependencies").and_then(Value::as_table) {
        production_count += deps.len();
    }
    if let Some(deps) = parsed.get("build-dependencies").and_then(Value::as_table) {
        production_count += deps.len();
    }
    if let Some(deps) = parsed.get("dev-dependencies").and_then(Value::as_table) {
        dev_count += deps.len();
    }
    (production_count, dev_count)
}

fn count_siblings(
    view: &CrawlView<'_>,
    dir: &str,
    root_dir: &str,
    crate_dirs: &[&str],
) -> (usize, usize) {
    let Some(entry) = view.dir_contents(dir) else {
        return (0, 0);
    };
    let sibling_rs_file_count = entry
        .files()
        .iter()
        .filter(|file| file.ends_with(".rs"))
        .count();
    let sibling_dir_count = entry
        .dirs()
        .iter()
        .filter(|subdir| {
            let child_dir = CrawlView::join_rel(dir, subdir);
            !should_stop_at_nested_crate(view, root_dir, &child_dir, crate_dirs)
        })
        .count();
    (sibling_rs_file_count, sibling_dir_count)
}

fn measure_module_depth(view: &CrawlView<'_>, crate_dir: &str, crate_dirs: &[&str]) -> usize {
    let src_dir = CrawlView::join_rel(crate_dir, "src");
    let base_dir = if view.dir_contents(&src_dir).is_some() {
        src_dir
    } else {
        crate_dir.to_owned()
    };
    measure_depth_recursive(view, crate_dir, &base_dir, crate_dirs, 0)
}

fn measure_depth_recursive(
    view: &CrawlView<'_>,
    root_dir: &str,
    dir: &str,
    crate_dirs: &[&str],
    depth: usize,
) -> usize {
    let Some(entry) = view.dir_contents(dir) else {
        return depth;
    };
    let has_rs = entry.files().iter().any(|file| file.ends_with(".rs"));
    let current = if has_rs { depth } else { 0 };
    let max_child = entry
        .dirs()
        .iter()
        .map(|subdir| {
            let child_dir = CrawlView::join_rel(dir, subdir);
            if should_stop_at_nested_crate(view, root_dir, &child_dir, crate_dirs) {
                0
            } else {
                measure_depth_recursive(view, root_dir, &child_dir, crate_dirs, depth + 1)
            }
        })
        .max()
        .unwrap_or(0);
    current.max(max_child)
}

fn is_test_or_example_path(rel_path: &str) -> bool {
    rel_path
        .split('/')
        .any(|segment| matches!(segment, "tests" | "examples" | "benches" | "target"))
}

fn is_under_crate_src(dir: &str, crate_nodes: &[G3RsArchCrateNode]) -> bool {
    crate_nodes.iter().any(|node| {
        let src_prefix = if node.rel_dir.is_empty() {
            "src".to_owned()
        } else {
            format!("{}/src", node.rel_dir)
        };
        dir.starts_with(&src_prefix)
            && (dir.len() == src_prefix.len()
                || dir.as_bytes().get(src_prefix.len()) == Some(&b'/'))
    })
}

fn feature_list(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
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
