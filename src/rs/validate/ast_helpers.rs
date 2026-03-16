//! AST-based source analysis helpers using syn.
//!
//! These functions parse Rust source into an AST and inspect it
//! structurally — no grep, no false positives from strings/comments.

use proc_macro2 as _; // reason: span-locations feature needed for syn span.start()
use syn::spanned::Spanned;
use syn::visit::Visit;

/// A source location (1-based line number) paired with a descriptive string (lint name, method name, etc.).
pub(super) type Located = (usize, String);

/// Information about a `#[derive(...)]` attribute on an item.
pub struct DeriveInfo {
    /// 1-based line number of the derive attribute.
    pub line: usize,
    /// Names of the derive macros (e.g. `["Deserialize", "Validate"]`).
    pub macros: Vec<String>,
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
                if u.attrs.iter().any(|a| is_cfg_test_attr(a)) {
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

/// Check if an attribute is `#[cfg(test)]`.
fn is_cfg_test_attr(attr: &syn::Attribute) -> bool {
    if !attr.path().is_ident("cfg") {
        return false;
    }
    let Ok(nested) = attr.parse_args::<syn::Ident>() else {
        return false;
    };
    nested == "test"
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
// Internal
// ---------------------------------------------------------------------------

pub(super) fn span_line(span: proc_macro2::Span) -> usize {
    span.start().line
}

fn extract_allow_lints(attr: &syn::Attribute, out: &mut Vec<Located>) {
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

fn extract_cfg_attr_allow_lints(attr: &syn::Attribute, out: &mut Vec<Located>) {
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

fn has_garde_skip(attrs: &[syn::Attribute]) -> Option<usize> {
    for attr in attrs {
        if !attr.path().is_ident("garde") {
            continue;
        }
        if let Ok(nested) = attr.parse_args_with(
            syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
        ) {
            for path in &nested {
                if path.is_ident("skip") {
                    return Some(span_line(attr.span()));
                }
            }
        }
    }
    None
}

fn collect_outer_allows(attrs: &[syn::Attribute], out: &mut Vec<Located>) {
    for attr in attrs {
        if !matches!(attr.style, syn::AttrStyle::Inner(_)) {
            extract_allow_lints(attr, out);
        }
    }
}

fn collect_cfg_attr_allows(attrs: &[syn::Attribute], out: &mut Vec<Located>) {
    for attr in attrs {
        extract_cfg_attr_allow_lints(attr, out);
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

#[allow(clippy::wildcard_enum_match_arm)] // reason: syn TraitItem has many variants, exhaustive match is impractical
fn trait_item_attrs(item: &syn::TraitItem) -> &[syn::Attribute] {
    match item {
        syn::TraitItem::Fn(f) => &f.attrs,
        syn::TraitItem::Type(t) => &t.attrs,
        syn::TraitItem::Const(c) => &c.attrs,
        _ => &[],
    }
}

// ---------------------------------------------------------------------------
// Visitors
// ---------------------------------------------------------------------------

struct ItemAllowVisitor {
    out: Vec<Located>,
}
impl<'ast> Visit<'ast> for ItemAllowVisitor {
    fn visit_item(&mut self, i: &'ast syn::Item) {
        collect_outer_allows(item_attrs(i), &mut self.out);
        syn::visit::visit_item(self, i);
    }
    fn visit_impl_item(&mut self, i: &'ast syn::ImplItem) {
        collect_outer_allows(impl_item_attrs(i), &mut self.out);
        syn::visit::visit_impl_item(self, i);
    }
    fn visit_trait_item(&mut self, i: &'ast syn::TraitItem) {
        collect_outer_allows(trait_item_attrs(i), &mut self.out);
        syn::visit::visit_trait_item(self, i);
    }
}

struct CfgAttrAllowVisitor<'a> {
    out: &'a mut Vec<Located>,
}
impl<'ast> Visit<'ast> for CfgAttrAllowVisitor<'_> {
    fn visit_item(&mut self, i: &'ast syn::Item) {
        collect_cfg_attr_allows(item_attrs(i), self.out);
        syn::visit::visit_item(self, i);
    }
    fn visit_impl_item(&mut self, i: &'ast syn::ImplItem) {
        collect_cfg_attr_allows(impl_item_attrs(i), self.out);
        syn::visit::visit_impl_item(self, i);
    }
}

struct GardeSkipVisitor {
    out: Vec<usize>,
}
impl<'ast> Visit<'ast> for GardeSkipVisitor {
    fn visit_field(&mut self, f: &'ast syn::Field) {
        if let Some(line) = has_garde_skip(&f.attrs) {
            self.out.push(line);
        }
        syn::visit::visit_field(self, f);
    }
    fn visit_item_struct(&mut self, n: &'ast syn::ItemStruct) {
        if let Some(line) = has_garde_skip(&n.attrs) {
            self.out.push(line);
        }
        syn::visit::visit_item_struct(self, n);
    }
}

struct UnsafeVisitor {
    out: Vec<usize>,
}
impl<'ast> Visit<'ast> for UnsafeVisitor {
    fn visit_expr_unsafe(&mut self, n: &'ast syn::ExprUnsafe) {
        self.out.push(span_line(n.unsafe_token.span));
        syn::visit::visit_expr_unsafe(self, n);
    }
    fn visit_item_fn(&mut self, n: &'ast syn::ItemFn) {
        if let Some(tok) = n.sig.unsafety {
            self.out.push(span_line(tok.span));
        }
        syn::visit::visit_item_fn(self, n);
    }
    fn visit_impl_item_fn(&mut self, n: &'ast syn::ImplItemFn) {
        if let Some(tok) = n.sig.unsafety {
            self.out.push(span_line(tok.span));
        }
        syn::visit::visit_impl_item_fn(self, n);
    }
    fn visit_item_impl(&mut self, n: &'ast syn::ItemImpl) {
        if let Some(tok) = n.unsafety {
            self.out.push(span_line(tok.span));
        }
        syn::visit::visit_item_impl(self, n);
    }
    fn visit_item_trait(&mut self, n: &'ast syn::ItemTrait) {
        if let Some(tok) = n.unsafety {
            self.out.push(span_line(tok.span));
        }
        syn::visit::visit_item_trait(self, n);
    }
}

struct ForbiddenMacroVisitor {
    out: Vec<Located>,
}
impl<'ast> Visit<'ast> for ForbiddenMacroVisitor {
    fn visit_macro(&mut self, n: &'ast syn::Macro) {
        let name = path_to_string(&n.path);
        if matches!(name.as_str(), "todo" | "unimplemented" | "unreachable" | "panic") {
            self.out.push((span_line(n.path.span()), name));
        }
        syn::visit::visit_macro(self, n);
    }
}

struct UnwrapExpectVisitor {
    out: Vec<Located>,
}
impl<'ast> Visit<'ast> for UnwrapExpectVisitor {
    fn visit_expr_method_call(&mut self, n: &'ast syn::ExprMethodCall) {
        let m = n.method.to_string();
        if m == "unwrap" || m == "expect" {
            self.out.push((span_line(n.method.span()), m));
        }
        syn::visit::visit_expr_method_call(self, n);
    }
}

struct DeriveVisitor {
    out: Vec<DeriveInfo>,
}
impl DeriveVisitor {
    fn collect_derives(&mut self, attrs: &[syn::Attribute]) {
        for attr in attrs {
            if !attr.path().is_ident("derive") {
                continue;
            }
            let line = span_line(attr.span());
            if let syn::Meta::List(list) = &attr.meta {
                if let Ok(paths) = list.parse_args_with(
                    syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
                ) {
                    let macros: Vec<String> = paths.iter().map(path_to_string).collect();
                    if !macros.is_empty() {
                        self.out.push(DeriveInfo { line, macros });
                    }
                }
            }
        }
    }
}
impl<'ast> Visit<'ast> for DeriveVisitor {
    fn visit_item(&mut self, i: &'ast syn::Item) {
        self.collect_derives(item_attrs(i));
        syn::visit::visit_item(self, i);
    }
    fn visit_impl_item(&mut self, i: &'ast syn::ImplItem) {
        self.collect_derives(impl_item_attrs(i));
        syn::visit::visit_impl_item(self, i);
    }
}

struct TestAttrVisitor {
    found: bool,
}
impl<'ast> Visit<'ast> for TestAttrVisitor {
    fn visit_item_fn(&mut self, n: &'ast syn::ItemFn) {
        if !self.found && has_test_or_tokio_test(&n.attrs) {
            self.found = true;
        }
        if !self.found {
            syn::visit::visit_item_fn(self, n);
        }
    }
    fn visit_impl_item_fn(&mut self, n: &'ast syn::ImplItemFn) {
        if !self.found && has_test_or_tokio_test(&n.attrs) {
            self.found = true;
        }
        if !self.found {
            syn::visit::visit_impl_item_fn(self, n);
        }
    }
}

struct PubFnVisitor {
    count: usize,
}
impl<'ast> Visit<'ast> for PubFnVisitor {
    fn visit_item_fn(&mut self, n: &'ast syn::ItemFn) {
        if matches!(n.vis, syn::Visibility::Public(_)) {
            self.count = self.count.saturating_add(1);
        }
        syn::visit::visit_item_fn(self, n);
    }
    fn visit_impl_item_fn(&mut self, n: &'ast syn::ImplItemFn) {
        if matches!(n.vis, syn::Visibility::Public(_)) {
            self.count = self.count.saturating_add(1);
        }
        syn::visit::visit_impl_item_fn(self, n);
    }
}

struct TestCountVisitor {
    count: usize,
}
impl<'ast> Visit<'ast> for TestCountVisitor {
    fn visit_item_fn(&mut self, n: &'ast syn::ItemFn) {
        if has_test_or_tokio_test(&n.attrs) {
            self.count = self.count.saturating_add(1);
        }
        syn::visit::visit_item_fn(self, n);
    }
    fn visit_impl_item_fn(&mut self, n: &'ast syn::ImplItemFn) {
        if has_test_or_tokio_test(&n.attrs) {
            self.count = self.count.saturating_add(1);
        }
        syn::visit::visit_impl_item_fn(self, n);
    }
}

struct IgnoreVisitor<'s> {
    lines: Vec<&'s str>,
    violations: Vec<usize>,
}
impl<'ast> Visit<'ast> for IgnoreVisitor<'_> {
    fn visit_item_fn(&mut self, n: &'ast syn::ItemFn) {
        self.check_ignore_attrs(&n.attrs);
        syn::visit::visit_item_fn(self, n);
    }
    fn visit_impl_item_fn(&mut self, n: &'ast syn::ImplItemFn) {
        self.check_ignore_attrs(&n.attrs);
        syn::visit::visit_impl_item_fn(self, n);
    }
}
impl IgnoreVisitor<'_> {
    fn check_ignore_attrs(&mut self, attrs: &[syn::Attribute]) {
        for attr in attrs {
            if !attr.path().is_ident("ignore") {
                continue;
            }
            let line = span_line(attr.span());
            // 1-based to 0-based index
            let idx = line.saturating_sub(1);
            // Check same line for reason comment
            if let Some(same_line) = self.lines.get(idx) {
                if same_line.contains("// reason:") || same_line.contains("//reason:") {
                    continue;
                }
            }
            // Check previous line for reason comment
            if idx > 0 {
                if let Some(prev_line) = self.lines.get(idx.saturating_sub(1)) {
                    if prev_line.contains("// reason:") || prev_line.contains("//reason:") {
                        continue;
                    }
                }
            }
            self.violations.push(line);
        }
    }
}

/// Check if attributes contain `#[test]` or `#[tokio::test]`.
fn has_test_or_tokio_test(attrs: &[syn::Attribute]) -> bool {
    for attr in attrs {
        let path = attr.path();
        if path.is_ident("test") {
            return true;
        }
        // Check for tokio::test
        if path.segments.len() == 2 {
            let mut iter = path.segments.iter();
            if let (Some(first), Some(second)) = (iter.next(), iter.next()) {
                if first.ident == "tokio" && second.ident == "test" {
                    return true;
                }
            }
        }
    }
    false
}

struct InlineStdFsVisitor {
    out: Vec<usize>,
    in_cfg_test: bool,
}

impl InlineStdFsVisitor {
    /// Check if a path is a direct std::fs function call like `std::fs::read_to_string`.
    /// Excludes type references like `std::fs::Permissions::from_mode` (4+ segments with
    /// capitalized 3rd segment = type, not function).
    fn path_is_std_fs_call(path: &syn::Path) -> bool {
        let segs: Vec<_> = path.segments.iter().map(|s| s.ident.to_string()).collect();
        if segs.len() < 3 || segs[0] != "std" || segs[1] != "fs" {
            return false;
        }
        // If exactly 3 segments (std::fs::read_to_string), check if the function
        // name starts with lowercase (function, not type)
        if segs.len() == 3 {
            return segs[2].starts_with(|c: char| c.is_ascii_lowercase());
        }
        // 4+ segments like std::fs::Permissions::from_mode — this is a type method call, skip
        false
    }
}

impl<'ast> Visit<'ast> for InlineStdFsVisitor {
    fn visit_item_mod(&mut self, n: &'ast syn::ItemMod) {
        let was = self.in_cfg_test;
        if n.attrs.iter().any(|a| is_cfg_test_attr(a)) {
            self.in_cfg_test = true;
        }
        syn::visit::visit_item_mod(self, n);
        self.in_cfg_test = was;
    }

    fn visit_item_fn(&mut self, n: &'ast syn::ItemFn) {
        let was = self.in_cfg_test;
        if n.attrs.iter().any(|a| is_cfg_test_attr(a)) {
            self.in_cfg_test = true;
        }
        syn::visit::visit_item_fn(self, n);
        self.in_cfg_test = was;
    }

    fn visit_expr_call(&mut self, n: &'ast syn::ExprCall) {
        if !self.in_cfg_test {
            if let syn::Expr::Path(ep) = &*n.func {
                if Self::path_is_std_fs_call(&ep.path) {
                    self.out.push(span_line(ep.path.span()));
                }
            }
        }
        syn::visit::visit_expr_call(self, n);
    }

    fn visit_expr_path(&mut self, n: &'ast syn::ExprPath) {
        // Catch function pointers: `let f = std::fs::read_to_string;`
        if !self.in_cfg_test && Self::path_is_std_fs_call(&n.path) {
            let line = span_line(n.path.span());
            if !self.out.contains(&line) {
                self.out.push(line);
            }
        }
        syn::visit::visit_expr_path(self, n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn must_parse(source: &str) -> syn::File {
        #[allow(clippy::expect_used)] // reason: test helper — panic on bad input is correct
        parse_file(source).expect("test input should be valid Rust")
    }

    #[test]
    fn parse_file_valid_and_invalid() {
        assert!(parse_file("fn main() {}").is_some(), "valid Rust parses");
        assert!(parse_file("not rust {{{").is_none(), "invalid returns None");
    }

    #[test]
    fn crate_level_allow_found() {
        let allows = find_crate_level_allows(&must_parse("#![allow(dead_code)]\nfn main() {}"));
        assert_eq!(allows.len(), 1, "should find one crate-level allow");
        assert_eq!(allows.first().map(|(_, s)| s.as_str()), Some("dead_code"));
    }

    #[test]
    fn crate_level_allow_in_string_not_found() {
        let src = "fn main() { let _s = \"#![allow(dead_code)]\"; }";
        assert!(
            find_crate_level_allows(&must_parse(src)).is_empty(),
            "no match in string"
        );
    }

    #[test]
    fn crate_level_allow_multiple_lints() {
        let src = "#![allow(dead_code, unused_variables)]\nfn main() {}";
        assert_eq!(
            find_crate_level_allows(&must_parse(src)).len(),
            2,
            "two lints in one allow"
        );
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
    fn item_allow_found() {
        let attr = ["#[allow(", "clippy::unwrap_used)]"].concat(); // pre-commit safe
        let src = format!("{attr}\nfn foo() {{}}");
        let allows = find_item_allows(&must_parse(&src));
        assert_eq!(allows.len(), 1, "should find item-level allow");
        assert_eq!(allows[0].1, "clippy::unwrap_used");
    }

    #[test]
    fn item_allow_in_string_not_found() {
        let inner = ["#[allow(", "clippy::unwrap_used)]"].concat(); // reason: test data built via concat
        let src = format!("fn foo() {{ let _s = \"{inner}\"; }}");
        assert!(
            find_item_allows(&must_parse(&src)).is_empty(),
            "no match in string"
        );
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
    fn item_allow_on_impl_method() {
        let attr = ["#[allow(", "dead_code)]"].concat(); // reason: test data built via concat
        let src = format!("struct S;\nimpl S {{\n    {attr}\n    fn method(&self) {{}}\n}}");
        let allows = find_item_allows(&must_parse(&src));
        assert_eq!(allows.len(), 1, "should find allow on impl method");
        assert_eq!(allows[0].1, "dead_code");
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
    fn cfg_attr_allow_found() {
        let allows = find_cfg_attr_allows(&must_parse(
            "#[cfg_attr(test, allow(dead_code))]\nfn foo() {}",
        ));
        assert_eq!(allows.len(), 1, "should find cfg_attr allow");
        assert_eq!(allows[0].1, "dead_code");
    }

    #[test]
    fn cfg_attr_allow_in_string_not_found() {
        let inner = "#[cfg_attr(test, allow(dead_code))]";
        let src = format!("fn foo() {{ let _s = \"{inner}\"; }}");
        assert!(
            find_cfg_attr_allows(&must_parse(&src)).is_empty(),
            "no match in string"
        );
    }

    #[test]
    fn garde_skip_found() {
        let src = "use garde::Validate;\n\n\
            #[derive(Validate)]\nstruct Input {\n    #[garde(skip)]\n    name: String,\n}";
        assert_eq!(
            find_garde_skips(&must_parse(src)).len(),
            1,
            "should find garde(skip)"
        );
    }

    #[test]
    fn garde_skip_in_string_not_found() {
        let src = format!("fn foo() {{ let _s = \"{}\"; }}", "#[garde(skip)]");
        assert!(
            find_garde_skips(&must_parse(&src)).is_empty(),
            "no match in string"
        );
    }

    #[test]
    fn unsafe_block_found() {
        let src = "fn foo() { unsafe { std::ptr::null::<u8>(); } }";
        assert_eq!(
            find_unsafe_usage(&must_parse(src)).len(),
            1,
            "should find unsafe block"
        );
    }

    #[test]
    fn unsafe_fn_found() {
        assert_eq!(
            find_unsafe_usage(&must_parse("unsafe fn d() {}")).len(),
            1,
            "unsafe fn"
        );
    }

    #[test]
    fn unsafe_in_string_not_found() {
        let src = "fn foo() { let _s = \"unsafe { bad() }\"; }";
        assert!(
            find_unsafe_usage(&must_parse(src)).is_empty(),
            "no match in string"
        );
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
    fn forbidden_macros_found() {
        let m1 = find_forbidden_macros(&must_parse("fn f() { todo!(); }"));
        assert_eq!(m1.len(), 1, "todo found");
        assert_eq!(m1[0].1, "todo");
        let m2 = find_forbidden_macros(&must_parse("fn f() { unimplemented!(); }"));
        assert_eq!(m2.len(), 1, "unimplemented found");
        assert_eq!(m2[0].1, "unimplemented");
        let m3 = find_forbidden_macros(&must_parse("fn f() { panic!(\"oh\"); }"));
        assert_eq!(m3.len(), 1, "panic found");
        assert_eq!(m3[0].1, "panic");
    }

    #[test]
    fn todo_in_string_not_found() {
        let src = "fn foo() { let _s = \"todo!()\"; }";
        assert!(
            find_forbidden_macros(&must_parse(src)).is_empty(),
            "no match in string"
        );
    }

    #[test]
    #[allow(clippy::indexing_slicing)] // reason: test assertion on known-length vector
    fn unwrap_expect_found() {
        let u = find_unwrap_expect(&must_parse("fn f() { Some(1).unwrap(); }"));
        assert_eq!(u.len(), 1, "unwrap found");
        assert_eq!(u[0].1, "unwrap");
        let e = find_unwrap_expect(&must_parse("fn f() { Some(1).expect(\"m\"); }"));
        assert_eq!(e.len(), 1, "expect found");
        assert_eq!(e[0].1, "expect");
    }

    #[test]
    fn unwrap_in_string_not_found() {
        let src = "fn foo() { let _s = \".unwrap()\"; }";
        assert!(
            find_unwrap_expect(&must_parse(src)).is_empty(),
            "no match in string"
        );
    }

    #[test]
    fn std_fs_import_found() {
        assert_eq!(
            find_std_fs_imports(&must_parse("use std::fs;\nfn main() {}")).len(),
            1,
            "std::fs"
        );
        assert_eq!(
            find_std_fs_imports(&must_parse("use std::fs::read_to_string;\nfn main() {}")).len(),
            1,
            "std::fs::read_to_string"
        );
    }

    #[test]
    fn std_fs_in_string_not_found() {
        let src = "fn foo() { let _s = \"use std::fs;\"; }";
        assert!(
            find_std_fs_imports(&must_parse(src)).is_empty(),
            "no match in string"
        );
    }

    #[test]
    fn non_std_fs_not_matched() {
        assert!(
            find_std_fs_imports(&must_parse("use std::io;\nfn main() {}")).is_empty(),
            "io != fs"
        );
    }

    #[test]
    fn count_use_statements_works() {
        let two = "use std::io;\nuse std::path::Path;\nfn main() {}";
        assert_eq!(count_use_statements(&must_parse(two)), 2, "two uses");
        assert_eq!(
            count_use_statements(&must_parse("fn main() {}")),
            0,
            "no uses"
        );
    }
}
