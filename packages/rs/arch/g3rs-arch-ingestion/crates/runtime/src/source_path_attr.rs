//! Collects every `#[path = "..."]` module attribute across the workspace's Rust sources.

use syn::spanned::Spanned;

use g3rs_arch_types::types as arch_types;

use crate::error::G3RsArchIngestionError;
use crate::source_syn_helpers::{attr_is_cfg_test, extract_path_attr_value, span_line};
use crate::view::CrawlView;
use crate::workspace::collect_rs_files_recursive;

/// Result alias for path-attribute collection.
type IngestResult<T> = Result<T, G3RsArchIngestionError>;

/// Walks every `.rs` file under each crate and collects every `#[path = "..."]` site.
pub(super) fn collect_path_attr_sites(
    view: &CrawlView<'_>,
    crate_nodes: &[arch_types::G3RsArchCrateNode],
) -> IngestResult<Vec<arch_types::G3RsArchPathAttrSite>> {
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

/// Parses a single `.rs` file and returns every `#[path = "..."]` site found inside.
fn collect_file_path_attr_sites(
    rel_path: &str,
    content: &str,
) -> Vec<arch_types::G3RsArchPathAttrSite> {
    let Ok(ast) = syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content)) else {
        return Vec::new();
    };

    ast.items
        .iter()
        .filter_map(|item| {
            let syn::Item::Mod(module) = item else {
                return None;
            };
            let path_attr = module
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("path"))?;
            Some(arch_types::G3RsArchPathAttrSite {
                rel_path: rel_path.to_owned(),
                line: span_line(path_attr.span()),
                module_name: module.ident.to_string(),
                path_value: extract_path_attr_value(path_attr),
                cfg_test_only: module.attrs.iter().any(attr_is_cfg_test),
            })
        })
        .collect()
}
