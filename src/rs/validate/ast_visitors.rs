//! AST visitor-based analysis helpers.
//!
//! Visitors for unsafe, forbidden macros, unwrap/expect, derive attributes,
//! test functions, pub functions, and `#[ignore]` attributes.

use syn::spanned::Spanned;
use syn::visit::Visit;

use super::ast_helpers::{item_attrs, impl_item_attrs, path_to_string, span_line, Located};

/// Information about a `#[derive(...)]` attribute on an item.
pub struct DeriveInfo {
    /// 1-based line number of the derive attribute.
    pub line: usize,
    /// Names of the derive macros (e.g. `["Deserialize", "Validate"]`).
    pub macros: Vec<String>,
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

/// Check if any function in the file has a `#[test]` or `#[tokio::test]` attribute.
pub fn has_test_functions(file: &syn::File) -> bool {
    let mut v = TestFunctionVisitor { count: 0 };
    v.visit_file(file);
    v.count > 0
}

/// Count functions with `#[test]` or `#[tokio::test]` attributes.
pub fn count_test_functions(file: &syn::File) -> usize {
    let mut v = TestFunctionVisitor { count: 0 };
    v.visit_file(file);
    v.count
}

/// Count functions with `pub` visibility (top-level and inside impl blocks).
pub fn count_pub_functions(file: &syn::File) -> usize {
    let mut v = PubFnVisitor { count: 0 };
    v.visit_file(file);
    v.count
}

/// Find `#[ignore]` attributes on functions. Returns 1-based line numbers.
pub fn find_ignore_attributes(file: &syn::File) -> Vec<usize> {
    let mut v = IgnoreAttrVisitor { out: Vec::new() };
    v.visit_file(file);
    v.out
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Check if attributes contain `#[test]` or `#[tokio::test]`.
fn has_test_attr(attrs: &[syn::Attribute]) -> bool {
    for attr in attrs {
        let path = attr.path();
        if path.is_ident("test") {
            return true;
        }
        // Check for tokio::test
        if path.segments.len() == 2 {
            let mut segs = path.segments.iter();
            if let (Some(first), Some(second)) = (segs.next(), segs.next()) {
                if first.ident == "tokio" && second.ident == "test" {
                    return true;
                }
            }
        }
    }
    false
}

/// Find `#[ignore]` attribute in attrs, returning its 1-based line number.
fn find_ignore_attr(attrs: &[syn::Attribute]) -> Option<usize> {
    for attr in attrs {
        if attr.path().is_ident("ignore") {
            return Some(span_line(attr.span()));
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Visitors
// ---------------------------------------------------------------------------

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

struct TestFunctionVisitor {
    count: usize,
}
impl<'ast> Visit<'ast> for TestFunctionVisitor {
    fn visit_item_fn(&mut self, n: &'ast syn::ItemFn) {
        if has_test_attr(&n.attrs) {
            self.count = self.count.saturating_add(1);
        }
        syn::visit::visit_item_fn(self, n);
    }
    fn visit_impl_item_fn(&mut self, n: &'ast syn::ImplItemFn) {
        if has_test_attr(&n.attrs) {
            self.count = self.count.saturating_add(1);
        }
        syn::visit::visit_impl_item_fn(self, n);
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

struct IgnoreAttrVisitor {
    out: Vec<usize>,
}
impl<'ast> Visit<'ast> for IgnoreAttrVisitor {
    fn visit_item_fn(&mut self, n: &'ast syn::ItemFn) {
        if let Some(line) = find_ignore_attr(&n.attrs) {
            self.out.push(line);
        }
        syn::visit::visit_item_fn(self, n);
    }
    fn visit_impl_item_fn(&mut self, n: &'ast syn::ImplItemFn) {
        if let Some(line) = find_ignore_attr(&n.attrs) {
            self.out.push(line);
        }
        syn::visit::visit_impl_item_fn(self, n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::ast_helpers::parse_file;

    fn must_parse(source: &str) -> syn::File {
        #[allow(clippy::expect_used)] // reason: test helper — panic on bad input is correct
        parse_file(source).expect("test input should be valid Rust")
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

    // ---- Derive attribute detection ----

    #[test]
    fn derive_attributes_found() {
        let src = "\
use serde::Deserialize;\n\
use garde::Validate;\n\
\n\
#[derive(Deserialize, Validate)]\n\
struct Foo {\n\
    name: String,\n\
}\n\
";
        let derives = find_derive_attributes(&must_parse(src));
        assert_eq!(derives.len(), 1, "should find one derive attribute");
        let d = derives.first();
        assert!(d.is_some(), "should have entry");
        if let Some(info) = d {
            assert!(
                info.macros.iter().any(|m| m == "Deserialize"),
                "should contain Deserialize"
            );
            assert!(
                info.macros.iter().any(|m| m == "Validate"),
                "should contain Validate"
            );
        }
    }

    #[test]
    fn derive_in_string_not_found() {
        let src = "fn foo() { let _s = \"#[derive(Deserialize)]\"; }";
        let derives = find_derive_attributes(&must_parse(src));
        assert!(
            derives.is_empty(),
            "derive in string should not be found"
        );
    }

    // ---- Test function detection (R-TEST-04) ----

    #[test]
    fn has_test_functions_found() {
        let src = "#[test]\nfn it_works() { assert!(true); }";
        assert!(
            has_test_functions(&must_parse(src)),
            "should find #[test]"
        );
    }

    #[test]
    fn has_test_functions_tokio() {
        let src = "#[tokio::test]\nasync fn it_works() {}";
        assert!(
            has_test_functions(&must_parse(src)),
            "should find #[tokio::test]"
        );
    }

    #[test]
    fn has_test_functions_in_string_not_found() {
        let src = "fn foo() { let _s = \"#[test]\"; }";
        assert!(
            !has_test_functions(&must_parse(src)),
            "#[test] in string should not match"
        );
    }

    #[test]
    fn has_test_functions_none() {
        let src = "fn main() {}\npub fn helper() {}";
        assert!(
            !has_test_functions(&must_parse(src)),
            "no test functions"
        );
    }

    // ---- Test and pub function counting (R-TEST-05) ----

    #[test]
    fn count_test_functions_works() {
        let src = "#[test]\nfn a() {}\n#[test]\nfn b() {}\nfn c() {}";
        assert_eq!(count_test_functions(&must_parse(src)), 2, "two tests");
    }

    #[test]
    fn count_pub_functions_works() {
        let src = "pub fn foo() {}\nfn bar() {}\npub fn baz() {}";
        assert_eq!(count_pub_functions(&must_parse(src)), 2, "two pub fns");
    }

    #[test]
    fn count_pub_functions_in_impl() {
        let src = "struct S;\nimpl S {\n    pub fn m1(&self) {}\n    fn m2(&self) {}\n    pub fn m3(&self) {}\n}";
        assert_eq!(
            count_pub_functions(&must_parse(src)),
            2,
            "two pub fns in impl"
        );
    }

    #[test]
    fn count_pub_fn_in_string_not_counted() {
        let src = "fn foo() { let _s = \"pub fn fake() {}\"; }";
        assert_eq!(
            count_pub_functions(&must_parse(src)),
            0,
            "pub fn in string should not count"
        );
    }

    // ---- #[ignore] attribute detection (R-TEST-07) ----

    #[test]
    fn find_ignore_attr_found() {
        let src = "#[test]\n#[ignore]\nfn slow_test() {}";
        assert_eq!(
            find_ignore_attributes(&must_parse(src)).len(),
            1,
            "should find #[ignore]"
        );
    }

    #[test]
    fn find_ignore_attr_not_found_when_absent() {
        let src = "#[test]\nfn fast_test() {}";
        assert!(
            find_ignore_attributes(&must_parse(src)).is_empty(),
            "no #[ignore]"
        );
    }

    #[test]
    fn find_ignore_in_string_not_found() {
        let src = "fn foo() { let _s = \"#[ignore]\"; }";
        assert!(
            find_ignore_attributes(&must_parse(src)).is_empty(),
            "#[ignore] in string should not match"
        );
    }
}
