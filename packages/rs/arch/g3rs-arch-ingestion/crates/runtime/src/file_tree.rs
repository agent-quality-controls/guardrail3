use std::collections::BTreeMap;

use g3rs_arch_types::types::{
    G3RsArchCrateNode, G3RsArchFileTreeChecksInput, G3RsArchFileTreeCrate, G3RsArchModuleDir,
    G3RsArchRustPolicyState,
};
use g3rs_workspace_crawl::G3RsWorkspaceCrawl;

use crate::error::G3RsArchIngestionError;
use crate::view::CrawlView;
use crate::workspace::{
    collect_crate_nodes, collect_dirs_recursive, collect_rs_files_recursive,
    is_test_or_example_path, is_under_crate_src,
};

pub(crate) fn ingest_for_file_tree_checks(
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

#[cfg(test)]
#[path = "file_tree_tests/mod.rs"]
mod file_tree_tests;
