//! AST-based source analysis helpers using syn.
//!
//! These functions parse Rust source into an AST and inspect it
//! structurally — no grep, no false positives from strings/comments.

use proc_macro2 as _; // reason: span-locations feature needed for syn span.start()
use syn::spanned::Spanned;
use syn::visit::Visit;

use super::ast_visitors::{
    CfgAttrAllowVisitor, DeriveVisitor, ForbiddenMacroVisitor, GardeSkipTypedVisitor,
    GardeSkipVisitor, IgnoreVisitor, InlineStdFsVisitor, ItemAllowVisitor, PubFnVisitor,
    TestAttrVisitor, TestCountVisitor, UnsafeVisitor, UnwrapExpectVisitor,
};

pub use super::ast_visitors::{GardeSkipInfo, struct_has_non_primitive_fields};

/// A source location (1-based line number) paired with a descriptive string (lint name, method name, etc.).
pub(super) type Located = (usize, String);

/// Information about a `#[derive(...)]` attribute on an item.
pub struct DeriveInfo {
    /// 1-based line number of the derive attribute.
    pub line: usize,
    /// Names of the derive macros (e.g. `["Deserialize", "Validate"]`).
    pub macros: Vec<String>,
    /// Whether the struct has at least one non-primitive field (String, Vec, custom types, etc.).
    /// `false` for enums, unit structs, or structs with only primitive fields.
    pub has_non_primitive_fields: bool,
    /// Name of the struct/enum (if identifiable from the item).
    pub name: Option<String>,
}

/// Parse a Rust source file. Returns `None` if parsing fails.
/// Strips UTF-8 BOM if present.
pub fn parse_file(source: &str) -> Option<syn::File> {
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);
    syn::parse_file(source).ok()
}

/// Find `#![allow(...)]` crate-level attributes. Returns `(line, lint_name)`.
pub fn find_crate_level_allows(file: &syn::File) -> Vec<Located> {
    let mut out = Vec::new();
    for attr in &file.attrs {
        if matches!(attr.style, syn::AttrStyle::Inner(_)) {
            extract_allow_lints(attr, &mut out);
        }
    }
    out
}

/// Find `#[allow(...)]` item-level attributes. Returns `(line, lint_name)`.
pub fn find_item_allows(file: &syn::File) -> Vec<Located> {
    let mut v = ItemAllowVisitor { out: Vec::new() };
    v.visit_file(file);
    v.out
}

/// Find `#[cfg_attr(..., allow(...))]` attributes. Returns `(line, lint_name)`.
pub fn find_cfg_attr_allows(file: &syn::File) -> Vec<Located> {
    let mut out = Vec::new();
    for attr in &file.attrs {
        extract_cfg_attr_allow_lints(attr, &mut out);
    }
    let mut v = CfgAttrAllowVisitor { out: &mut out };
    v.visit_file(file);
    out
}

/// Find lines with `#[garde(skip)]`.
pub fn find_garde_skips(file: &syn::File) -> Vec<usize> {
    let mut v = GardeSkipVisitor { out: Vec::new() };
    v.visit_file(file);
    v.out
}

/// Find `#[garde(skip)]` fields with type information.
pub fn find_garde_skips_with_types(file: &syn::File) -> Vec<GardeSkipInfo> {
    let mut v = GardeSkipTypedVisitor { out: Vec::new() };
    v.visit_file(file);
    v.out
}

/// Find lines with `unsafe` blocks or `unsafe fn` declarations.
pub fn find_unsafe_usage(file: &syn::File) -> Vec<usize> {
    let mut v = UnsafeVisitor { out: Vec::new() };
    v.visit_file(file);
    v.out
}

/// Find `todo!()`, `unimplemented!()`, `panic!()`. Returns `(line, macro_name)`.
pub fn find_forbidden_macros(file: &syn::File) -> Vec<Located> {
    let mut v = ForbiddenMacroVisitor { out: Vec::new() };
    v.visit_file(file);
    v.out
}

/// Find `.unwrap()` and `.expect()` calls. Returns `(line, method_name)`.
pub fn find_unwrap_expect(file: &syn::File) -> Vec<Located> {
    let mut v = UnwrapExpectVisitor { out: Vec::new() };
    v.visit_file(file);
    v.out
}

/// Find all `#[derive(...)]` attributes in a parsed file.
/// Returns one `DeriveInfo` per derive attribute found, with its line and macro names.
pub fn find_derive_attributes(file: &syn::File) -> Vec<DeriveInfo> {
    let mut v = DeriveVisitor { out: Vec::new() };
    v.visit_file(file);
    v.out
}

/// Find `use std::fs` import lines. Skips imports gated by `#[cfg(test)]`.
pub fn find_std_fs_imports(file: &syn::File) -> Vec<usize> {
    file.items
        .iter()
        .filter_map(|item| {
            if let syn::Item::Use(u) = item {
                // Skip cfg(test)-gated imports
                if u.attrs.iter().any(is_cfg_test_attr) {
                    return None;
                }
                if use_tree_matches_std_fs(&u.tree) {
                    return Some(u.span().start().line);
                }
            }
            None
        })
        .collect()
}

/// Find inline `std::fs::*` calls (e.g., `std::fs::read_to_string(...)`).
/// Skips calls inside `#[cfg(test)]` functions and modules.
pub fn find_inline_std_fs_calls(file: &syn::File) -> Vec<usize> {
    let mut v = InlineStdFsVisitor {
        out: Vec::new(),
        in_cfg_test: false,
    };
    v.visit_file(file);
    v.out
}

/// Check if the file contains at least one `#[test]` or `#[tokio::test]` attribute.
pub fn has_test_attribute(file: &syn::File) -> bool {
    let mut v = TestAttrVisitor { found: false };
    v.visit_file(file);
    v.found
}

/// Count `pub fn` declarations (including in impl blocks and traits).
pub fn count_pub_fn_decls(file: &syn::File) -> usize {
    let mut v = PubFnVisitor { count: 0 };
    v.visit_file(file);
    v.count
}

/// Count `#[test]` and `#[tokio::test]` attributes.
pub fn count_test_attrs(file: &syn::File) -> usize {
    let mut v = TestCountVisitor { count: 0 };
    v.visit_file(file);
    v.count
}

/// Find `#[ignore]` attributes without a `// reason:` comment on same or previous line.
/// Returns 1-based line numbers of violations.
/// Requires the original source to check for reason comments.
pub fn find_ignore_without_reason(file: &syn::File, source: &str) -> Vec<usize> {
    let mut v = IgnoreVisitor {
        lines: source.lines().collect(),
        violations: Vec::new(),
    };
    v.visit_file(file);
    v.violations
}

/// Count top-level `use` statements.
pub fn count_use_statements(file: &syn::File) -> usize {
    file.items
        .iter()
        .filter(|item| matches!(item, syn::Item::Use(_)))
        .count()
}

// ---------------------------------------------------------------------------
// Internal helpers (shared with ast_visitors via pub(super))
// ---------------------------------------------------------------------------

pub(super) fn span_line(span: proc_macro2::Span) -> usize {
    span.start().line
}

pub(super) fn extract_allow_lints(attr: &syn::Attribute, out: &mut Vec<Located>) {
    if !attr.path().is_ident("allow") {
        return;
    }
    let line = span_line(attr.span());
    if let Ok(nested) = attr
        .parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated)
    {
        for path in &nested {
            out.push((line, path_to_string(path)));
        }
    }
}

pub(super) fn extract_cfg_attr_allow_lints(attr: &syn::Attribute, out: &mut Vec<Located>) {
    if !attr.path().is_ident("cfg_attr") {
        return;
    }
    let line = span_line(attr.span());
    let Ok(meta_list) = attr.meta.require_list() else {
        return;
    };
    let mut iter = meta_list.tokens.clone().into_iter().peekable();
    while let Some(token) = iter.next() {
        if let proc_macro2::TokenTree::Ident(ref ident) = token {
            if ident == "allow" {
                if let Some(proc_macro2::TokenTree::Group(group)) = iter.peek() {
                    if group.delimiter() == proc_macro2::Delimiter::Parenthesis {
                        if let Ok(paths) = syn::parse2::<LintList>(group.stream()) {
                            for path in &paths.0 {
                                out.push((line, path_to_string(path)));
                            }
                        }
                    }
                }
            }
        }
    }
}

struct LintList(syn::punctuated::Punctuated<syn::Path, syn::Token![,]>);
impl syn::parse::Parse for LintList {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self(syn::punctuated::Punctuated::parse_terminated(input)?))
    }
}

fn use_tree_matches_std_fs(tree: &syn::UseTree) -> bool {
    if let syn::UseTree::Path(p) = tree {
        if p.ident == "std" {
            return use_subtree_is_fs(&p.tree);
        }
    }
    false
}

fn use_subtree_is_fs(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Name(n) => n.ident == "fs",
        syn::UseTree::Path(p) => p.ident == "fs",
        syn::UseTree::Rename(r) => r.ident == "fs",
        syn::UseTree::Group(g) => g.items.iter().any(use_subtree_is_fs),
        syn::UseTree::Glob(_) => false,
    }
}

pub(super) fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

/// Check if an attribute is `#[cfg(test)]`.
pub(super) fn is_cfg_test_attr(attr: &syn::Attribute) -> bool {
    if !attr.path().is_ident("cfg") {
        return false;
    }
    let Ok(nested) = attr.parse_args::<syn::Ident>() else {
        return false;
    };
    nested == "test"
}

/// Extract the identifier (name) from a syn Item, if it has one.
#[allow(clippy::wildcard_enum_match_arm)] // reason: syn Item has many variants, exhaustive match is impractical
pub(super) fn item_ident(item: &syn::Item) -> Option<&syn::Ident> {
    match item {
        syn::Item::Fn(f) => Some(&f.sig.ident),
        syn::Item::Struct(s) => Some(&s.ident),
        syn::Item::Enum(e) => Some(&e.ident),
        syn::Item::Mod(m) => Some(&m.ident),
        syn::Item::Trait(t) => Some(&t.ident),
        syn::Item::Type(t) => Some(&t.ident),
        syn::Item::Const(c) => Some(&c.ident),
        syn::Item::Static(s) => Some(&s.ident),
        _ => None,
    }
}

#[allow(clippy::wildcard_enum_match_arm)] // reason: syn Item has many variants, exhaustive match is impractical
pub(super) fn item_attrs(item: &syn::Item) -> &[syn::Attribute] {
    match item {
        syn::Item::Fn(f) => &f.attrs,
        syn::Item::Struct(s) => &s.attrs,
        syn::Item::Enum(e) => &e.attrs,
        syn::Item::Impl(i) => &i.attrs,
        syn::Item::Mod(m) => &m.attrs,
        syn::Item::Trait(t) => &t.attrs,
        syn::Item::Type(t) => &t.attrs,
        syn::Item::Const(c) => &c.attrs,
        syn::Item::Static(s) => &s.attrs,
        syn::Item::Use(u) => &u.attrs,
        _ => &[],
    }
}

#[allow(clippy::wildcard_enum_match_arm)] // reason: syn ImplItem has many variants, exhaustive match is impractical
pub(super) fn impl_item_attrs(item: &syn::ImplItem) -> &[syn::Attribute] {
    match item {
        syn::ImplItem::Fn(f) => &f.attrs,
        syn::ImplItem::Type(t) => &t.attrs,
        syn::ImplItem::Const(c) => &c.attrs,
        _ => &[],
    }
}

