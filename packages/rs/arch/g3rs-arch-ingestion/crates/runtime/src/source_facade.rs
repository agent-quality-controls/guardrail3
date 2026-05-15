//! Materialises facade surfaces for `lib.rs` and every `mod.rs` in the workspace.

use proc_macro2::Span;
use syn::spanned::Spanned;

use g3rs_arch_types::types as arch_types;

use crate::source_syn_helpers::{
    extract_feature_gate, is_broad_reexport, is_pub, span_line, use_tree_name,
};
use crate::view::CrawlView;
use crate::workspace::should_stop_at_nested_crate;

/// Mutable counters tracking export gating while walking a facade file.
struct ExportCounters {
    /// Number of `pub` exports that have no `#[cfg(...)]` gate.
    ungated: usize,
    /// Number of `pub` exports gated on a feature literally named `all`.
    gated_on_all: usize,
}

impl ExportCounters {
    /// Creates a fresh counter pair starting at zero.
    const fn new() -> Self {
        Self {
            ungated: 0,
            gated_on_all: 0,
        }
    }

    /// Updates the counters for one observed `pub` export.
    const fn record(&mut self, feature_gate: Option<&str>, gated_on_all: bool) {
        if feature_gate.is_none() {
            self.ungated = self.ungated.saturating_add(1);
        }
        if gated_on_all {
            self.gated_on_all = self.gated_on_all.saturating_add(1);
        }
    }
}

/// Mutable accumulators built up while walking a facade file.
struct FacadeAccumulators {
    /// Inline non-export items (functions, impls, statics, ...).
    body_items: Vec<arch_types::G3RsArchFacadeItem>,
    /// Broad re-exports (`pub use foo::*` and similar).
    broad_reexports: Vec<arch_types::G3RsArchFacadeItem>,
    /// Every `pub` export with its feature gate.
    pub_exports: Vec<arch_types::G3RsArchFeatureExport>,
    /// Counters summarizing gating of `pub_exports`.
    counters: ExportCounters,
}

impl FacadeAccumulators {
    /// Creates empty accumulators.
    const fn new() -> Self {
        Self {
            body_items: Vec::new(),
            broad_reexports: Vec::new(),
            pub_exports: Vec::new(),
            counters: ExportCounters::new(),
        }
    }
}

/// Walks every crate to materialise its facade surfaces (lib.rs and every mod.rs).
pub(crate) fn collect_facade_surfaces(
    view: &CrawlView<'_>,
    crate_nodes: &[arch_types::G3RsArchCrateNode],
) -> Vec<arch_types::G3RsArchFacadeSurface> {
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

/// Recursively descends into `dir` and emits a facade surface for every `mod.rs` reached.
fn collect_mod_rs_recursive(
    view: &CrawlView<'_>,
    root_dir: &str,
    dir: &str,
    crate_dirs: &[&str],
    surfaces: &mut Vec<arch_types::G3RsArchFacadeSurface>,
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

/// Parses a single facade file and returns the surface description, or `None`
/// when the file is unreadable or not parseable.
fn analyze_facade(
    view: &CrawlView<'_>,
    rel_path: &str,
    is_lib_rs: bool,
    is_mod_rs: bool,
) -> Option<arch_types::G3RsArchFacadeSurface> {
    let content = view.read_file(rel_path).ok()?;
    let ast = syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(&content)).ok()?;

    let mut acc = FacadeAccumulators::new();
    for item in &ast.items {
        analyze_facade_item(item, &mut acc);
    }

    Some(arch_types::G3RsArchFacadeSurface {
        rel_path: rel_path.to_owned(),
        is_lib_rs,
        is_mod_rs,
        body_items: acc.body_items,
        broad_reexports: acc.broad_reexports,
        pub_export_count: acc.pub_exports.len(),
        pub_exports: acc.pub_exports,
        ungated_export_count: acc.counters.ungated,
        gated_on_all_count: acc.counters.gated_on_all,
    })
}

/// Routes one parsed item into the matching accumulator branch.
fn analyze_facade_item(item: &syn::Item, acc: &mut FacadeAccumulators) {
    let feature_gate = extract_feature_gate(item);
    let gated_on_all = feature_gate.as_deref() == Some("all");

    match item {
        syn::Item::Mod(module) => analyze_mod_item(module, feature_gate, gated_on_all, acc),
        syn::Item::Use(item_use) => analyze_use_item(item_use, feature_gate, gated_on_all, acc),
        syn::Item::Fn(item_fn) => acc.body_items.push(simple_item(
            item_fn.span(),
            "function",
            item_fn.sig.ident.to_string(),
            feature_gate,
            gated_on_all,
        )),
        syn::Item::Impl(item_impl) => acc.body_items.push(simple_item(
            item_impl.span(),
            "impl",
            "impl".to_owned(),
            feature_gate,
            gated_on_all,
        )),
        syn::Item::ExternCrate(extern_crate) => acc.body_items.push(simple_item(
            extern_crate.span(),
            "extern crate",
            extern_crate.ident.to_string(),
            feature_gate,
            gated_on_all,
        )),
        syn::Item::Static(item_static) => acc.body_items.push(simple_item(
            item_static.span(),
            "static",
            item_static.ident.to_string(),
            feature_gate,
            gated_on_all,
        )),
        syn::Item::ForeignMod(foreign_mod) => acc.body_items.push(simple_item(
            foreign_mod.span(),
            "extern block",
            "extern".to_owned(),
            feature_gate,
            gated_on_all,
        )),
        syn::Item::Macro(item_macro) => acc.body_items.push(simple_item(
            item_macro.span(),
            "macro item",
            item_macro
                .ident
                .as_ref()
                .map_or_else(|| "macro".to_owned(), std::string::ToString::to_string),
            feature_gate,
            gated_on_all,
        )),
        syn::Item::Const(_)
        | syn::Item::Enum(_)
        | syn::Item::Struct(_)
        | syn::Item::Trait(_)
        | syn::Item::TraitAlias(_)
        | syn::Item::Type(_)
        | syn::Item::Union(_)
        | syn::Item::Verbatim(_)
        | _ => {}
    }
}

/// Handles a `mod` item, classifying it as an inline-module body item or a `pub` export.
fn analyze_mod_item(
    module: &syn::ItemMod,
    feature_gate: Option<String>,
    gated_on_all: bool,
    acc: &mut FacadeAccumulators,
) {
    if is_pub(&module.vis) {
        if module.content.is_some() {
            acc.body_items.push(arch_types::G3RsArchFacadeItem {
                line: span_line(module.span()),
                kind: "inline module",
                name: module.ident.to_string(),
                is_broad_reexport: false,
                feature_gate,
                gated_on_all,
            });
            return;
        }
        acc.pub_exports.push(arch_types::G3RsArchFeatureExport {
            line: span_line(module.span()),
            name: module.ident.to_string(),
            feature_gate: feature_gate.clone(),
            gated_on_all,
        });
        acc.counters.record(feature_gate.as_deref(), gated_on_all);
        return;
    }
    if module.content.is_some() {
        acc.body_items.push(arch_types::G3RsArchFacadeItem {
            line: span_line(module.span()),
            kind: "inline module",
            name: module.ident.to_string(),
            is_broad_reexport: false,
            feature_gate,
            gated_on_all,
        });
    }
}

/// Handles a `use` item, recording it as a `pub` re-export when applicable.
fn analyze_use_item(
    item_use: &syn::ItemUse,
    feature_gate: Option<String>,
    gated_on_all: bool,
    acc: &mut FacadeAccumulators,
) {
    if !is_pub(&item_use.vis) {
        return;
    }
    let is_broad = is_broad_reexport(&item_use.tree);
    let line = span_line(item_use.span());
    let name = use_tree_name(&item_use.tree);
    if is_broad {
        acc.broad_reexports.push(arch_types::G3RsArchFacadeItem {
            line,
            kind: "pub use",
            name: name.clone(),
            is_broad_reexport: true,
            feature_gate: feature_gate.clone(),
            gated_on_all,
        });
    }
    acc.counters.record(feature_gate.as_deref(), gated_on_all);
    acc.pub_exports.push(arch_types::G3RsArchFeatureExport {
        line,
        name,
        feature_gate,
        gated_on_all,
    });
}

/// Builds a `G3RsArchFacadeItem` for an item whose payload is a single span and identifier.
fn simple_item(
    span: Span,
    kind: &'static str,
    name: String,
    feature_gate: Option<String>,
    gated_on_all: bool,
) -> arch_types::G3RsArchFacadeItem {
    arch_types::G3RsArchFacadeItem {
        line: span_line(span),
        kind,
        name,
        is_broad_reexport: false,
        feature_gate,
        gated_on_all,
    }
}
