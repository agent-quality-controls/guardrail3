use std::collections::BTreeMap;

use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use super::crate_tree::CrateTree;

/// An item found in a facade file (lib.rs or mod.rs).
#[derive(Debug, Clone)]
pub(crate) struct FacadeItem {
    /// 1-based line number.
    pub line: usize,
    /// Kind of item: "function", "inline module", "impl", "private use", etc.
    pub kind: &'static str,
    /// Name of the item.
    pub name: String,
    /// Whether the item is a broad re-export: `pub use foo;` or `pub use foo::*;`.
    pub is_broad_reexport: bool,
    /// The feature gate on this item, if any (e.g., "types" from #[cfg(feature = "types")]).
    pub feature_gate: Option<String>,
    /// Whether this is gated on the "all" feature directly.
    pub gated_on_all: bool,
}

/// Facade surface for a single file.
#[derive(Debug, Clone)]
#[allow(dead_code)] // Fields collected for rule expansion and diagnostics.
pub(crate) struct FacadeSurface {
    /// Repo-relative path to the file.
    pub rel_path: String,
    /// Whether this is a lib.rs file.
    pub is_lib_rs: bool,
    /// Whether this is a mod.rs file.
    pub is_mod_rs: bool,
    /// Items found in this facade that are violations (implementation logic).
    pub body_items: Vec<FacadeItem>,
    /// Public module declarations (pub mod foo;).
    pub pub_mods: Vec<FacadeItem>,
    /// Public use re-exports (pub use foo::Bar;).
    pub pub_uses: Vec<FacadeItem>,
    /// Total count of pub exports (mods + uses).
    pub pub_export_count: usize,
    /// Count of pub exports without a feature gate.
    pub ungated_export_count: usize,
    /// Count of pub exports gated on "all" directly.
    pub gated_on_all_count: usize,
}

/// Map of rel_path → FacadeSurface for all lib.rs and mod.rs files.
pub(crate) type FacadeSurfaceMap = BTreeMap<String, FacadeSurface>;

pub(super) fn collect(tree: &ProjectTree, crate_tree: &CrateTree) -> FacadeSurfaceMap {
    let mut map = BTreeMap::new();

    for node in crate_tree.nodes.values() {
        // Collect lib.rs facade.
        if let Some(lib_rel) = &node.lib_rs_rel {
            if let Some(surface) = analyze_facade(tree, lib_rel, true, false) {
                let _ = map.insert(lib_rel.clone(), surface);
            }
        }
    }

    // Collect mod.rs files within crate source trees.
    for node in crate_tree.nodes.values() {
        collect_mod_rs_recursive(tree, &node.rel_dir, &mut map);
    }

    map
}

fn collect_mod_rs_recursive(tree: &ProjectTree, dir: &str, map: &mut FacadeSurfaceMap) {
    let Some(entry) = tree.dir_contents(dir) else {
        return;
    };
    if entry.files().iter().any(|f| f == "mod.rs") {
        let mod_rs = ProjectTree::join_rel(dir, "mod.rs");
        if let Some(surface) = analyze_facade(tree, &mod_rs, false, true) {
            let _ = map.insert(mod_rs, surface);
        }
    }
    for subdir in entry.dirs() {
        let child = ProjectTree::join_rel(dir, subdir);
        collect_mod_rs_recursive(tree, &child, map);
    }
}

fn analyze_facade(
    tree: &ProjectTree,
    rel_path: &str,
    is_lib_rs: bool,
    is_mod_rs: bool,
) -> Option<FacadeSurface> {
    // Try cached content first, then read from disk.
    let owned_content;
    let content = if let Some(cached) = tree.file_content(rel_path) {
        cached
    } else {
        let abs = tree.abs_path(rel_path)?;
        owned_content = guardrail3_shared_fs::read_file_err(&abs).ok()?;
        &owned_content
    };
    let ast = syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content)).ok()?;

    let mut body_items = Vec::new();
    let mut pub_mods = Vec::new();
    let mut pub_uses = Vec::new();
    let mut ungated_export_count = 0;
    let mut gated_on_all_count = 0;

    for item in &ast.items {
        let feature_gate = extract_feature_gate(&item);
        let gated_on_all = feature_gate.as_deref() == Some("all");

        match item {
            syn::Item::Mod(m) => {
                if is_pub(&m.vis) {
                    if m.content.is_some() {
                        // Inline module with body — violation.
                        body_items.push(FacadeItem {
                            line: span_line(m),
                            kind: "inline module",
                            name: m.ident.to_string(),
                            is_broad_reexport: false,
                            feature_gate: feature_gate.clone(),
                            gated_on_all,
                        });
                    } else {
                        let info = FacadeItem {
                            line: span_line(m),
                            kind: "pub mod",
                            name: m.ident.to_string(),
                            is_broad_reexport: false,
                            feature_gate: feature_gate.clone(),
                            gated_on_all,
                        };
                        if feature_gate.is_none() {
                            ungated_export_count += 1;
                        }
                        if gated_on_all {
                            gated_on_all_count += 1;
                        }
                        pub_mods.push(info);
                    }
                } else if m.content.is_some() {
                    body_items.push(FacadeItem {
                        line: span_line(m),
                        kind: "inline module",
                        name: m.ident.to_string(),
                        is_broad_reexport: false,
                        feature_gate,
                        gated_on_all,
                    });
                }
                // Non-pub mod without body (mod foo;) — fine in facades.
            }
            syn::Item::Use(u) => {
                if is_pub(&u.vis) {
                    let is_broad = is_broad_reexport(&u.tree);
                    let name = use_tree_name(&u.tree);
                    let info = FacadeItem {
                        line: span_line(u),
                        kind: "pub use",
                        name,
                        is_broad_reexport: is_broad,
                        feature_gate: feature_gate.clone(),
                        gated_on_all,
                    };
                    if feature_gate.is_none() {
                        ungated_export_count += 1;
                    }
                    if gated_on_all {
                        gated_on_all_count += 1;
                    }
                    pub_uses.push(info);
                } else {
                    body_items.push(FacadeItem {
                        line: span_line(u),
                        kind: "private use",
                        name: use_tree_name(&u.tree),
                        is_broad_reexport: false,
                        feature_gate,
                        gated_on_all,
                    });
                }
            }
            syn::Item::Fn(f) => {
                body_items.push(FacadeItem {
                    line: span_line(f),
                    kind: "function",
                    name: f.sig.ident.to_string(),
                    is_broad_reexport: false,
                    feature_gate,
                    gated_on_all,
                });
            }
            syn::Item::Impl(i) => {
                body_items.push(FacadeItem {
                    line: span_line(i),
                    kind: "impl",
                    name: "impl".to_owned(),
                    is_broad_reexport: false,
                    feature_gate,
                    gated_on_all,
                });
            }
            syn::Item::ExternCrate(e) => {
                body_items.push(FacadeItem {
                    line: span_line(e),
                    kind: "extern crate",
                    name: e.ident.to_string(),
                    is_broad_reexport: false,
                    feature_gate,
                    gated_on_all,
                });
            }
            syn::Item::Static(s) => {
                body_items.push(FacadeItem {
                    line: span_line(s),
                    kind: "static",
                    name: s.ident.to_string(),
                    is_broad_reexport: false,
                    feature_gate,
                    gated_on_all,
                });
            }
            syn::Item::ForeignMod(_) => {
                body_items.push(FacadeItem {
                    line: span_line(item),
                    kind: "extern block",
                    name: "extern".to_owned(),
                    is_broad_reexport: false,
                    feature_gate,
                    gated_on_all,
                });
            }
            syn::Item::Macro(m) => {
                let name = m
                    .ident
                    .as_ref()
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| "macro".to_owned());
                body_items.push(FacadeItem {
                    line: span_line(m),
                    kind: "macro item",
                    name,
                    is_broad_reexport: false,
                    feature_gate,
                    gated_on_all,
                });
            }
            // Allowed: Const, Enum, Struct, Trait, TraitAlias, Type, Union, Verbatim.
            _ => {}
        }
    }

    let pub_export_count = pub_mods.len() + pub_uses.len();

    Some(FacadeSurface {
        rel_path: rel_path.to_owned(),
        is_lib_rs,
        is_mod_rs,
        body_items,
        pub_mods,
        pub_uses,
        pub_export_count,
        ungated_export_count,
        gated_on_all_count,
    })
}

fn is_pub(vis: &syn::Visibility) -> bool {
    matches!(vis, syn::Visibility::Public(_))
}

fn span_line<T: syn::spanned::Spanned>(item: &T) -> usize {
    item.span().start().line
}

fn is_broad_reexport(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Glob(_) => true,
        syn::UseTree::Path(p) => is_broad_reexport_inner(&p.tree),
        // Top-level `pub use foo;` re-exports entire crate/module.
        syn::UseTree::Name(_) => true,
        // Top-level `pub use foo as bar;` also re-exports entire crate/module under alias.
        syn::UseTree::Rename(_) => true,
        syn::UseTree::Group(g) => g.items.iter().any(is_broad_reexport),
    }
}

/// Inside a path (e.g., `pub use foo::X`), Name means specific item, not broad.
fn is_broad_reexport_inner(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Glob(_) => true,
        syn::UseTree::Path(p) => is_broad_reexport_inner(&p.tree),
        // `pub use foo::Bar;` — specific item, not broad.
        syn::UseTree::Name(_) => false,
        // `pub use foo::Bar as Baz;` — specific item with alias, not broad.
        syn::UseTree::Rename(_) => false,
        syn::UseTree::Group(g) => g.items.iter().any(is_broad_reexport_inner),
    }
}

fn use_tree_name(tree: &syn::UseTree) -> String {
    match tree {
        syn::UseTree::Path(p) => {
            let child = use_tree_name(&p.tree);
            if child.is_empty() {
                p.ident.to_string()
            } else {
                format!("{}::{}", p.ident, child)
            }
        }
        syn::UseTree::Name(n) => n.ident.to_string(),
        syn::UseTree::Rename(r) => format!("{} as {}", r.ident, r.rename),
        syn::UseTree::Glob(_) => "*".to_owned(),
        syn::UseTree::Group(_) => "{...}".to_owned(),
    }
}

fn extract_feature_gate(item: &syn::Item) -> Option<String> {
    let attrs = match item {
        syn::Item::Mod(m) => &m.attrs,
        syn::Item::Use(u) => &u.attrs,
        syn::Item::Fn(f) => &f.attrs,
        syn::Item::Struct(s) => &s.attrs,
        syn::Item::Enum(e) => &e.attrs,
        syn::Item::Type(t) => &t.attrs,
        syn::Item::Const(c) => &c.attrs,
        syn::Item::Trait(t) => &t.attrs,
        syn::Item::Impl(i) => &i.attrs,
        syn::Item::Static(s) => &s.attrs,
        syn::Item::Macro(m) => &m.attrs,
        syn::Item::ExternCrate(e) => &e.attrs,
        _ => return None,
    };

    for attr in attrs {
        if !attr.path().is_ident("cfg") {
            continue;
        }
        let Ok(meta) = attr.parse_args::<syn::Meta>() else {
            continue;
        };
        if let Some(feature) = extract_feature_from_meta(&meta) {
            return Some(feature);
        }
    }
    None
}

fn extract_feature_from_meta(meta: &syn::Meta) -> Option<String> {
    match meta {
        syn::Meta::NameValue(nv) if nv.path.is_ident("feature") => {
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) = &nv.value
            {
                return Some(s.value());
            }
            None
        }
        syn::Meta::List(list) if list.path.is_ident("all") || list.path.is_ident("any") => {
            // Search inside all(...) or any(...) for a feature = "..." entry.
            let Ok(nested) =
                list.parse_args_with(syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
            else {
                return None;
            };
            for inner in &nested {
                if let Some(feature) = extract_feature_from_meta(inner) {
                    return Some(feature);
                }
            }
            None
        }
        _ => None,
    }
}
