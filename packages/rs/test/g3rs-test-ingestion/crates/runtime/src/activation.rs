#![expect(
    clippy::indexing_slicing,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::match_same_arms,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::shadow_unrelated,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::type_complexity,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use cargo_toml_parser::{types::CargoToml, types::Dependency};
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind};
use syn::visit::Visit;

use crate::ingest::IngestionError;
use crate::roots::{OwnedTestRoot, join_under_root};

/// `ActivationSummary` struct.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct ActivationSummary {
    /// `has_tests` item.
    pub(crate) has_tests: bool,
    /// `has_tokio_tests` item.
    pub(crate) has_tokio_tests: bool,
}

/// `summarize_root` function.
pub(crate) fn summarize_root(
    crawl: &G3RsWorkspaceCrawl,
    root: &OwnedTestRoot,
    _workspace_manifest: Option<&CargoToml>,
) -> Result<ActivationSummary, IngestionError> {
    let mut summary = ActivationSummary::default();
    let assertions_src = assertions_src_rel_path(root);
    let support_srcs = test_support_src_rel_paths(root);
    let runtime_src = join_under_root(&root.runtime_rel_dir, "src");
    let runtime_tests = join_under_root(&root.runtime_rel_dir, "tests");

    for entry in crawl.entries.iter().filter(is_rust_file) {
        let rel_path = entry.path.rel_path.as_str();
        if is_fixture_path(rel_path) {
            continue;
        }
        if !file_belongs_to_root(
            rel_path,
            &runtime_src,
            &runtime_tests,
            &assertions_src,
            &support_srcs,
        ) {
            continue;
        }

        if !entry.readable {
            return Err(IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: "file is not readable".to_owned(),
            });
        }

        let content = crate::fs::read_to_string(&entry.path.abs_path).map_err(|err| {
            IngestionError::Unreadable {
                path: entry.path.abs_path.clone(),
                reason: err.to_string(),
            }
        })?;
        let source = syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(&content))
            .map_err(|err| IngestionError::ParseFailed {
                path: entry.path.abs_path.clone(),
                reason: err.to_string(),
            })?;

        let mut visitor = ActivationVisitor::default();
        visitor.visit_file(&source);

        if rel_path.starts_with(&runtime_tests)
            || is_sidecar_path(rel_path, &runtime_src)
            || rel_path.starts_with(&assertions_src)
            || support_srcs
                .iter()
                .any(|prefix| path_is_under(rel_path, prefix))
            || visitor.has_tests
            || visitor.has_cfg_test_module
        {
            summary.has_tests = true;
        }
        if visitor.has_tokio_tests {
            summary.has_tokio_tests = true;
        }
    }

    Ok(summary)
}

/// `has_tokio_dependency` function.
pub(crate) fn has_tokio_dependency(
    cargo: &CargoToml,
    workspace_manifest: Option<&CargoToml>,
) -> bool {
    let workspace_deps = workspace_manifest
        .and_then(|manifest| manifest.workspace.as_ref())
        .map(|workspace| &workspace.dependencies);

    dependencies_have_tokio(&cargo.dependencies, workspace_deps)
        || dependencies_have_tokio(&cargo.dev_dependencies, workspace_deps)
        || dependencies_have_tokio(&cargo.build_dependencies, workspace_deps)
}

/// `dependencies_have_tokio` function.
fn dependencies_have_tokio(
    deps: &std::collections::BTreeMap<String, Dependency>,
    workspace_deps: Option<&std::collections::BTreeMap<String, Dependency>>,
) -> bool {
    deps.iter()
        .any(|(name, dependency)| dependency_is_tokio(name, dependency, workspace_deps))
}

/// `dependency_is_tokio` function.
fn dependency_is_tokio(
    dep_name: &str,
    dependency: &Dependency,
    workspace_deps: Option<&std::collections::BTreeMap<String, Dependency>>,
) -> bool {
    if dep_name == "tokio" {
        return true;
    }
    match dependency {
        Dependency::Simple(_) => false,
        Dependency::Detailed(detail) => {
            if detail.package.as_deref() == Some("tokio") {
                return true;
            }
            if detail.workspace == Some(true) {
                let Some(workspace_spec) = workspace_deps.and_then(|deps| deps.get(dep_name))
                else {
                    return dep_name == "tokio";
                };
                return dependency_is_tokio(dep_name, workspace_spec, None);
            }
            false
        }
    }
}

/// `assertions_src_rel_path` function.
fn assertions_src_rel_path(root: &OwnedTestRoot) -> String {
    if root.runtime_rel_dir == root.root_rel_dir {
        join_under_root(&root.root_rel_dir, "assertions/src")
    } else {
        format!(
            "{}/assertions/src",
            crate::roots::parent_dir(&root.runtime_rel_dir)
        )
    }
}

/// `test_support_src_rel_paths` function.
fn test_support_src_rel_paths(root: &OwnedTestRoot) -> [String; 2] {
    [
        join_under_root(&root.root_rel_dir, "test_support/src"),
        join_under_root(&root.root_rel_dir, "crates/test_support/src"),
    ]
}

/// `file_belongs_to_root` function.
fn file_belongs_to_root(
    rel_path: &str,
    runtime_src: &str,
    runtime_tests: &str,
    assertions_src: &str,
    support_srcs: &[String; 2],
) -> bool {
    path_is_under(rel_path, runtime_src)
        || path_is_under(rel_path, runtime_tests)
        || path_is_under(rel_path, assertions_src)
        || support_srcs
            .iter()
            .any(|prefix| path_is_under(rel_path, prefix))
}

/// `path_is_under` function.
fn path_is_under(rel_path: &str, prefix: &str) -> bool {
    rel_path == prefix
        || rel_path
            .strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with('/'))
}

/// `is_sidecar_path` function.
fn is_sidecar_path(rel_path: &str, runtime_src: &str) -> bool {
    let Some(after_src) = rel_path
        .strip_prefix(runtime_src)
        .and_then(|rest| rest.strip_prefix('/'))
    else {
        return false;
    };

    after_src.contains("_tests/") || after_src.ends_with("_tests/mod.rs")
}

/// `is_fixture_path` function.
fn is_fixture_path(rel_path: &str) -> bool {
    rel_path.contains("/tests/fixtures/")
        || rel_path.starts_with("tests/fixtures/")
        || rel_path.contains("_tests/fixtures/")
        || rel_path.contains("assertions/src/fixtures/")
        || rel_path.contains("test_support/src/fixtures/")
}

/// `is_rust_file` function.
fn is_rust_file(entry: &&G3RsWorkspaceEntry) -> bool {
    entry.kind == G3RsWorkspaceEntryKind::File
        && std::path::Path::new(entry.path.rel_path.as_str())
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"))
}

/// `ActivationVisitor` struct.
#[derive(Default)]
struct ActivationVisitor {
    /// `has_tests` item.
    has_tests: bool,
    /// `has_tokio_tests` item.
    has_tokio_tests: bool,
    /// `has_cfg_test_module` item.
    has_cfg_test_module: bool,
}

impl<'source> Visit<'source> for ActivationVisitor {
    fn visit_item_mod(&mut self, item: &'source syn::ItemMod) {
        if item.attrs.iter().any(is_cfg_test_attr) {
            self.has_cfg_test_module = true;
        }
        syn::visit::visit_item_mod(self, item);
    }

    fn visit_item_fn(&mut self, item: &'source syn::ItemFn) {
        self.scan_attrs(&item.attrs);
        syn::visit::visit_item_fn(self, item);
    }

    fn visit_impl_item_fn(&mut self, item: &'source syn::ImplItemFn) {
        self.scan_attrs(&item.attrs);
        syn::visit::visit_impl_item_fn(self, item);
    }
}

impl ActivationVisitor {
    /// `scan_attrs` method.
    fn scan_attrs(&mut self, attrs: &[syn::Attribute]) {
        if attrs.iter().any(is_test_attr) {
            self.has_tests = true;
        }
        if attrs.iter().any(is_tokio_test_attr) {
            self.has_tokio_tests = true;
        }
    }
}

/// `is_test_attr` function.
fn is_test_attr(attr: &syn::Attribute) -> bool {
    let predicate = cfg_predicate(attr);
    path_is_test_attr(attr.path())
        || cfg_attr_nested_metas(attr)
            .into_iter()
            .flatten()
            .any(|meta| {
                predicate
                    .as_ref()
                    .is_some_and(cfg_meta_contains_positive_test)
                    && meta_path_is_test(&meta)
            })
}

/// `is_tokio_test_attr` function.
fn is_tokio_test_attr(attr: &syn::Attribute) -> bool {
    let predicate = cfg_predicate(attr);
    path_is_tokio_test_attr(attr.path())
        || cfg_attr_nested_metas(attr)
            .into_iter()
            .flatten()
            .any(|meta| {
                predicate
                    .as_ref()
                    .is_some_and(cfg_meta_contains_positive_test)
                    && meta_path_is_tokio_test(&meta)
            })
}

/// `is_cfg_test_attr` function.
fn is_cfg_test_attr(attr: &syn::Attribute) -> bool {
    if !attr.path().is_ident("cfg") {
        return false;
    }
    cfg_predicate(attr).is_some_and(|meta| cfg_meta_requires_test(&meta))
}

/// `path_is_test_attr` function.
fn path_is_test_attr(path: &syn::Path) -> bool {
    path.is_ident("test")
        || path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "test")
}

/// `meta_path_is_test` function.
fn meta_path_is_test(meta: &syn::Meta) -> bool {
    path_is_test_attr(meta.path())
}

/// `path_is_tokio_test_attr` function.
fn path_is_tokio_test_attr(path: &syn::Path) -> bool {
    path.segments.len() == 2
        && path.segments[0].ident == "tokio"
        && path.segments[1].ident == "test"
}

/// `meta_path_is_tokio_test` function.
fn meta_path_is_tokio_test(meta: &syn::Meta) -> bool {
    path_is_tokio_test_attr(meta.path())
}

/// `cfg_attr_nested_metas` function.
fn cfg_attr_nested_metas(attr: &syn::Attribute) -> Option<Vec<syn::Meta>> {
    if !attr.path().is_ident("cfg_attr") {
        return None;
    }
    let syn::Meta::List(list) = &attr.meta else {
        return None;
    };
    let args = list
        .parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        .ok()?;
    let mut iter = args.into_iter();
    let _ = iter.next()?;
    Some(iter.collect())
}

/// `cfg_predicate` function.
fn cfg_predicate(attr: &syn::Attribute) -> Option<syn::Meta> {
    let syn::Meta::List(list) = &attr.meta else {
        return None;
    };
    if attr.path().is_ident("cfg") {
        return syn::parse2::<syn::Meta>(list.tokens.clone()).ok();
    }
    if !attr.path().is_ident("cfg_attr") {
        return None;
    }
    list.parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        .ok()?
        .into_iter()
        .next()
}

/// `cfg_meta_requires_test` function.
fn cfg_meta_requires_test(meta: &syn::Meta) -> bool {
    cfg_meta_can_be_true(meta, true) && !cfg_meta_can_be_true(meta, false)
}

/// `cfg_meta_contains_positive_test` function.
fn cfg_meta_contains_positive_test(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(path) => path.is_ident("test"),
        syn::Meta::NameValue(_) => false,
        syn::Meta::List(list) if list.path.is_ident("not") => false,
        syn::Meta::List(list) => nested_cfg_meta_items(list)
            .iter()
            .any(cfg_meta_contains_positive_test),
    }
}

/// `cfg_meta_can_be_true` function.
fn cfg_meta_can_be_true(meta: &syn::Meta, test_enabled: bool) -> bool {
    match meta {
        syn::Meta::Path(path) => !path.is_ident("test") || test_enabled,
        syn::Meta::NameValue(_) => true,
        syn::Meta::List(list) if list.path.is_ident("all") => nested_cfg_meta_items(list)
            .iter()
            .all(|meta| cfg_meta_can_be_true(meta, test_enabled)),
        syn::Meta::List(list) if list.path.is_ident("any") => nested_cfg_meta_items(list)
            .iter()
            .any(|meta| cfg_meta_can_be_true(meta, test_enabled)),
        syn::Meta::List(list) if list.path.is_ident("not") => nested_cfg_meta_items(list)
            .first()
            .is_some_and(|meta| cfg_meta_can_be_false(meta, test_enabled)),
        syn::Meta::List(_) => true,
    }
}

/// `cfg_meta_can_be_false` function.
fn cfg_meta_can_be_false(meta: &syn::Meta, test_enabled: bool) -> bool {
    match meta {
        syn::Meta::Path(path) => !path.is_ident("test") || !test_enabled,
        syn::Meta::NameValue(_) => true,
        syn::Meta::List(list) if list.path.is_ident("all") => nested_cfg_meta_items(list)
            .iter()
            .any(|meta| cfg_meta_can_be_false(meta, test_enabled)),
        syn::Meta::List(list) if list.path.is_ident("any") => nested_cfg_meta_items(list)
            .iter()
            .all(|meta| cfg_meta_can_be_false(meta, test_enabled)),
        syn::Meta::List(list) if list.path.is_ident("not") => nested_cfg_meta_items(list)
            .first()
            .is_some_and(|meta| cfg_meta_can_be_true(meta, test_enabled)),
        syn::Meta::List(_) => true,
    }
}

/// `nested_cfg_meta_items` function.
fn nested_cfg_meta_items(list: &syn::MetaList) -> Vec<syn::Meta> {
    list.parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
        .map(|items| items.into_iter().collect())
        .unwrap_or_default()
}
