//! AST visitor structs for source analysis.
//!
//! Each visitor walks the syn AST to collect specific patterns
//! (allows, unsafe blocks, forbidden macros, etc.).

use syn::spanned::Spanned;
use syn::visit::Visit;

use super::ast_helpers::{
    DeriveInfo, Located, attrs_have_allow_lint, expr_attrs, extract_allow_lints,
    extract_cfg_attr_allow_lints, impl_item_attrs, is_cfg_test_attr, item_attrs, item_ident,
    path_to_string, span_line,
};

// ---------------------------------------------------------------------------
// Visitor-only helpers
// ---------------------------------------------------------------------------

#[allow(clippy::wildcard_enum_match_arm)] // reason: syn TraitItem has many variants, exhaustive match is impractical
fn trait_item_attrs(item: &syn::TraitItem) -> &[syn::Attribute] {
    match item {
        syn::TraitItem::Fn(f) => &f.attrs,
        syn::TraitItem::Type(t) => &t.attrs,
        syn::TraitItem::Const(c) => &c.attrs,
        _ => &[],
    }
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

// ---------------------------------------------------------------------------
// Visitors
// ---------------------------------------------------------------------------

pub struct ItemAllowVisitor {
    pub out: Vec<Located>,
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
    fn visit_local(&mut self, n: &'ast syn::Local) {
        collect_outer_allows(&n.attrs, &mut self.out);
        syn::visit::visit_local(self, n);
    }
    fn visit_arm(&mut self, n: &'ast syn::Arm) {
        collect_outer_allows(&n.attrs, &mut self.out);
        syn::visit::visit_arm(self, n);
    }
    fn visit_stmt(&mut self, n: &'ast syn::Stmt) {
        // Catch #[allow(...)] on expression statements (not covered by visit_item or visit_local)
        if let syn::Stmt::Expr(expr, _) = n {
            collect_outer_allows(expr_attrs(expr), &mut self.out);
        }
        syn::visit::visit_stmt(self, n);
    }
}

pub struct CfgAttrAllowVisitor<'a> {
    pub out: &'a mut Vec<Located>,
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

pub struct GardeSkipVisitor {
    pub out: Vec<usize>,
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

/// Known primitive types that are always valid to skip with `#[garde(skip)]`.
/// Types that are always valid to skip in garde — either primitives or types
/// that garde fundamentally cannot validate (maps, trait objects).
const SKIP_OK_TYPES: &[&str] = &[
    // Primitives
    "bool", "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128",
    "isize", "f32", "f64",
    // Collections where garde can't dive into values
    "BTreeMap", "HashMap", "BTreeSet", "HashSet",
];

/// Information about a `#[garde(skip)]` field, including its type.
pub struct GardeSkipInfo {
    pub line: usize,
    pub field_name: String,
    pub field_type: String,
    pub is_primitive: bool,
    pub has_subcommand_attr: bool,
}

/// Check if a type is one that garde cannot meaningfully validate.
#[allow(clippy::wildcard_enum_match_arm)] // reason: syn Type has many variants
fn type_is_unvalidatable(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(tp) => {
            let last_seg = tp.path.segments.last();
            let Some(seg) = last_seg else {
                return false;
            };
            let ident = seg.ident.to_string();
            if SKIP_OK_TYPES.iter().any(|&p| p == ident) {
                return true;
            }
            // Type aliases ending in Map/Set are likely BTreeMap/HashMap aliases
            if ident.ends_with("Map") || ident.ends_with("Set") {
                return true;
            }
            // Option<unvalidatable> is also unvalidatable
            if ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                    if args.args.len() == 1 {
                        if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                            return type_is_unvalidatable(inner);
                        }
                    }
                }
            }
            false
        }
        // Trait objects, references, etc. — can't validate
        syn::Type::TraitObject(_) | syn::Type::Reference(_) => true,
        _ => false,
    }
}

fn type_to_string(ty: &syn::Type) -> String {
    use quote::ToTokens;
    ty.to_token_stream().to_string().replace(' ', "")
}

pub struct GardeSkipTypedVisitor {
    pub out: Vec<GardeSkipInfo>,
}
impl<'ast> Visit<'ast> for GardeSkipTypedVisitor {
    fn visit_field(&mut self, f: &'ast syn::Field) {
        if let Some(line) = has_garde_skip(&f.attrs) {
            let field_name = f
                .ident
                .as_ref()
                .map_or_else(|| "<unnamed>".to_owned(), std::string::ToString::to_string);
            let field_type = type_to_string(&f.ty);
            let is_primitive = type_is_unvalidatable(&f.ty);
            // Check if field has #[command(subcommand)] — clap subcommand enums can't be validated
            let has_subcommand_attr = f.attrs.iter().any(|a| {
                a.path().is_ident("command")
                    && a.meta
                        .require_list()
                        .ok()
                        .and_then(|list| list.parse_args::<syn::Ident>().ok())
                        .is_some_and(|ident| ident == "subcommand")
            });
            self.out.push(GardeSkipInfo {
                line,
                field_name,
                field_type,
                is_primitive: is_primitive || has_subcommand_attr,
                has_subcommand_attr,
            });
        }
        syn::visit::visit_field(self, f);
    }
}

/// Check if a struct (given its fields) has any non-primitive fields.
pub fn struct_has_non_primitive_fields(item: &syn::ItemStruct) -> bool {
    match &item.fields {
        syn::Fields::Named(fields) => fields.named.iter().any(|f| !type_is_unvalidatable(&f.ty)),
        syn::Fields::Unnamed(fields) => {
            fields.unnamed.iter().any(|f| !type_is_unvalidatable(&f.ty))
        }
        syn::Fields::Unit => false,
    }
}

pub struct UnsafeVisitor {
    pub out: Vec<usize>,
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

pub struct ForbiddenMacroVisitor {
    pub out: Vec<Located>,
}
impl<'ast> Visit<'ast> for ForbiddenMacroVisitor {
    fn visit_macro(&mut self, n: &'ast syn::Macro) {
        let name = path_to_string(&n.path);
        let base = name.rsplit("::").next().unwrap_or(&name);
        if matches!(base, "todo" | "unimplemented" | "unreachable" | "panic") {
            self.out.push((span_line(n.path.span()), name));
        }
        syn::visit::visit_macro(self, n);
    }
}

#[derive(Default)]
pub struct UnwrapExpectVisitor {
    pub out: Vec<Located>,
    /// Scope has #[allow(clippy::unwrap_used)]
    unwrap_allowed: bool,
    /// Scope has #[allow(clippy::expect_used)]
    expect_allowed: bool,
}

impl UnwrapExpectVisitor {
    fn save_and_apply(&mut self, attrs: &[syn::Attribute]) -> (bool, bool) {
        let was = (self.unwrap_allowed, self.expect_allowed);
        self.unwrap_allowed |= attrs_have_allow_lint(attrs, "unwrap_used");
        self.expect_allowed |= attrs_have_allow_lint(attrs, "expect_used");
        was
    }
    fn restore(&mut self, was: (bool, bool)) {
        self.unwrap_allowed = was.0;
        self.expect_allowed = was.1;
    }
}

impl<'ast> Visit<'ast> for UnwrapExpectVisitor {
    fn visit_item_fn(&mut self, n: &'ast syn::ItemFn) {
        let was = self.save_and_apply(&n.attrs);
        syn::visit::visit_item_fn(self, n);
        self.restore(was);
    }
    fn visit_impl_item_fn(&mut self, n: &'ast syn::ImplItemFn) {
        let was = self.save_and_apply(&n.attrs);
        syn::visit::visit_impl_item_fn(self, n);
        self.restore(was);
    }
    fn visit_item_mod(&mut self, n: &'ast syn::ItemMod) {
        let was = self.save_and_apply(&n.attrs);
        syn::visit::visit_item_mod(self, n);
        self.restore(was);
    }
    fn visit_local(&mut self, n: &'ast syn::Local) {
        let was = self.save_and_apply(&n.attrs);
        syn::visit::visit_local(self, n);
        self.restore(was);
    }
    fn visit_expr_method_call(&mut self, n: &'ast syn::ExprMethodCall) {
        let m = n.method.to_string();
        let skip = match m.as_str() {
            "unwrap" => self.unwrap_allowed,
            "expect" => self.expect_allowed,
            _ => true, // not unwrap/expect, always skip
        };
        if !skip {
            self.out.push((span_line(n.method.span()), m));
        }
        syn::visit::visit_expr_method_call(self, n);
    }
}

pub struct DeriveVisitor {
    pub out: Vec<DeriveInfo>,
}
impl DeriveVisitor {
    /// Collect all derive macros from all `#[derive(...)]` attributes on a single item
    /// into ONE `DeriveInfo`. This correctly handles split derives like
    /// `#[derive(Deserialize)] #[derive(Validate)]` — they produce a single entry
    /// with macros `["Deserialize", "Validate"]`.
    fn collect_derives(
        &mut self,
        attrs: &[syn::Attribute],
        has_non_primitive: bool,
        name: Option<String>,
    ) {
        let mut macros = Vec::new();
        let mut first_line: Option<usize> = None;

        for attr in attrs {
            if !attr.path().is_ident("derive") {
                continue;
            }
            if first_line.is_none() {
                first_line = Some(span_line(attr.span()));
            }
            if let syn::Meta::List(list) = &attr.meta {
                if let Ok(paths) = list.parse_args_with(
                    syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
                ) {
                    macros.extend(paths.iter().map(path_to_string));
                }
            }
        }

        if let Some(line) = first_line {
            if !macros.is_empty() {
                self.out.push(DeriveInfo {
                    line,
                    macros,
                    has_non_primitive_fields: has_non_primitive,
                    name,
                });
            }
        }
    }
}
impl<'ast> Visit<'ast> for DeriveVisitor {
    fn visit_item(&mut self, i: &'ast syn::Item) {
        let has_non_primitive = if let syn::Item::Struct(s) = i {
            struct_has_non_primitive_fields(s)
        } else {
            // Enums and other items: conservatively assume they have non-primitive fields
            true
        };
        let item_name = item_ident(i).map(std::string::ToString::to_string);
        self.collect_derives(item_attrs(i), has_non_primitive, item_name);
        syn::visit::visit_item(self, i);
    }
    fn visit_impl_item(&mut self, i: &'ast syn::ImplItem) {
        self.collect_derives(impl_item_attrs(i), true, None);
        syn::visit::visit_impl_item(self, i);
    }
}

pub struct TestAttrVisitor {
    pub found: bool,
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

pub struct PubFnVisitor {
    pub count: usize,
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

pub struct TestCountVisitor {
    pub count: usize,
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

pub struct IgnoreVisitor<'s> {
    pub lines: Vec<&'s str>,
    pub violations: Vec<usize>,
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
            // #[ignore = "reason"] (NameValue) and #[ignore(...)] (List) provide
            // the reason inline — only bare #[ignore] (Path) needs a comment.
            if !matches!(attr.meta, syn::Meta::Path(_)) {
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

pub struct InlineStdFsVisitor {
    pub out: Vec<usize>,
    pub in_cfg_test: bool,
    pub in_allowed_scope: bool,
}

impl InlineStdFsVisitor {
    /// Check if a path is a direct `std::fs` usage like `std::fs::read_to_string`
    /// or `std::fs::File::open`. Accepts ALL paths with 3+ segments starting with
    /// `std::fs`. Type-path concerns are moot because `visit_expr_path` and
    /// `visit_expr_call` only fire on expression paths, not type paths.
    #[allow(clippy::indexing_slicing)] // reason: length checked >= 3 before indexing [0], [1]
    fn path_is_std_fs_call(path: &syn::Path) -> bool {
        let segs: Vec<_> = path.segments.iter().map(|s| s.ident.to_string()).collect();
        segs.len() >= 3 && segs[0] == "std" && segs[1] == "fs"
    }
}

impl<'ast> Visit<'ast> for InlineStdFsVisitor {
    fn visit_item_mod(&mut self, n: &'ast syn::ItemMod) {
        let (was_test, was_allow) = (self.in_cfg_test, self.in_allowed_scope);
        self.in_cfg_test |= n.attrs.iter().any(is_cfg_test_attr);
        self.in_allowed_scope |= attrs_have_allow_lint(&n.attrs, "disallowed_methods");
        syn::visit::visit_item_mod(self, n);
        (self.in_cfg_test, self.in_allowed_scope) = (was_test, was_allow);
    }
    fn visit_item_fn(&mut self, n: &'ast syn::ItemFn) {
        let (was_test, was_allow) = (self.in_cfg_test, self.in_allowed_scope);
        self.in_cfg_test |= n.attrs.iter().any(is_cfg_test_attr);
        self.in_allowed_scope |= attrs_have_allow_lint(&n.attrs, "disallowed_methods");
        syn::visit::visit_item_fn(self, n);
        (self.in_cfg_test, self.in_allowed_scope) = (was_test, was_allow);
    }
    fn visit_impl_item_fn(&mut self, n: &'ast syn::ImplItemFn) {
        let was = self.in_allowed_scope;
        self.in_allowed_scope |= attrs_have_allow_lint(&n.attrs, "disallowed_methods");
        syn::visit::visit_impl_item_fn(self, n);
        self.in_allowed_scope = was;
    }
    fn visit_expr_call(&mut self, n: &'ast syn::ExprCall) {
        if !self.in_cfg_test && !self.in_allowed_scope {
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
        if !self.in_cfg_test && !self.in_allowed_scope && Self::path_is_std_fs_call(&n.path) {
            let line = span_line(n.path.span());
            if !self.out.contains(&line) {
                self.out.push(line);
            }
        }
        syn::visit::visit_expr_path(self, n);
    }
}

