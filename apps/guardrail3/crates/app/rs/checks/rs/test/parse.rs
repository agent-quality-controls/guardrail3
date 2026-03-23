use syn::parse::{Parse, Parser};
use syn::spanned::Spanned;
use syn::visit::Visit;

#[derive(Debug, Clone, Default)]
pub struct ParsedTestFile {
    pub pub_fn_count: usize,
    pub ignore_without_reason_lines: Vec<usize>,
    pub cfg_test_modules: Vec<CfgTestModuleInfo>,
    pub test_functions: Vec<TestFunctionInfo>,
}

#[derive(Debug, Clone)]
pub struct CfgTestModuleInfo {
    pub line: usize,
    pub name: String,
    pub has_body: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TestFunctionInfo {
    pub line: usize,
    pub name: String,
    pub inside_cfg_test_module: bool,
    pub has_result_return: bool,
    pub has_assertion_macro: bool,
    pub has_assert_like_call: bool,
    pub should_panic_line: Option<usize>,
    pub should_panic_has_expected: bool,
    pub tautological_assert_lines: Vec<usize>,
    pub weak_matches_lines: Vec<usize>,
}

pub fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content))
}

pub fn effective_non_comment_line_count(content: &str) -> usize {
    content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with("//")
        })
        .count()
}

pub fn analyze(ast: &syn::File, content: &str) -> ParsedTestFile {
    let mut visitor = TestVisitor {
        out: ParsedTestFile {
            pub_fn_count: crate::app::rs::validate::ast_helpers::count_pub_fn_decls(ast),
            ignore_without_reason_lines:
                crate::app::rs::validate::ast_helpers::find_ignore_without_reason(ast, content),
            cfg_test_modules: Vec::new(),
            test_functions: Vec::new(),
        },
        cfg_test_depth: 0,
    };
    visitor.visit_file(ast);
    visitor.out
}

struct TestVisitor {
    out: ParsedTestFile,
    cfg_test_depth: usize,
}

impl<'ast> Visit<'ast> for TestVisitor {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        let is_cfg_test = item.attrs.iter().any(is_cfg_test_attr);
        if is_cfg_test {
            self.out.cfg_test_modules.push(CfgTestModuleInfo {
                line: span_line(item.span()),
                name: item.ident.to_string(),
                has_body: item.content.is_some(),
            });
        }
        if is_cfg_test {
            self.cfg_test_depth = self.cfg_test_depth.saturating_add(1);
        }
        syn::visit::visit_item_mod(self, item);
        if is_cfg_test {
            self.cfg_test_depth = self.cfg_test_depth.saturating_sub(1);
        }
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        maybe_push_test_function(
            &item.attrs,
            &item.sig,
            &item.block,
            self.cfg_test_depth > 0,
            &mut self.out.test_functions,
        );
        syn::visit::visit_item_fn(self, item);
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        maybe_push_test_function(
            &item.attrs,
            &item.sig,
            &item.block,
            self.cfg_test_depth > 0,
            &mut self.out.test_functions,
        );
        syn::visit::visit_impl_item_fn(self, item);
    }
}

fn maybe_push_test_function(
    attrs: &[syn::Attribute],
    sig: &syn::Signature,
    block: &syn::Block,
    inside_cfg_test_module: bool,
    out: &mut Vec<TestFunctionInfo>,
) {
    if !attrs.iter().any(is_test_attr) {
        return;
    }
    let should_panic_attr = attrs
        .iter()
        .find(|attr| attr.path().is_ident("should_panic"));
    let mut body_visitor = TestBodyVisitor::default();
    body_visitor.visit_block(block);
    out.push(TestFunctionInfo {
        line: span_line(sig.span()),
        name: sig.ident.to_string(),
        inside_cfg_test_module,
        has_result_return: signature_returns_result(sig),
        has_assertion_macro: body_visitor.has_assertion_macro,
        has_assert_like_call: body_visitor.has_assert_like_call,
        should_panic_line: should_panic_attr.map(|attr| span_line(attr.span())),
        should_panic_has_expected: should_panic_attr.is_some_and(should_panic_has_expected),
        tautological_assert_lines: body_visitor.tautological_assert_lines,
        weak_matches_lines: body_visitor.weak_matches_lines,
    });
}

#[derive(Default)]
struct TestBodyVisitor {
    has_assertion_macro: bool,
    has_assert_like_call: bool,
    tautological_assert_lines: Vec<usize>,
    weak_matches_lines: Vec<usize>,
}

impl<'ast> Visit<'ast> for TestBodyVisitor {
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if let Some(name) = mac
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
        {
            if is_assertion_macro_name(&name) {
                self.has_assertion_macro = true;
            }
            if (name == "assert_eq" || name == "assert_ne") && macro_has_literal_comparison(mac) {
                self.tautological_assert_lines.push(span_line(mac.span()));
            }
            if name == "assert" && macro_has_weak_matches(mac) {
                self.weak_matches_lines.push(span_line(mac.span()));
            }
        }
        syn::visit::visit_macro(self, mac);
    }

    fn visit_expr_call(&mut self, expr: &'ast syn::ExprCall) {
        if call_ident(&expr.func)
            .as_deref()
            .is_some_and(is_assert_like_name)
        {
            self.has_assert_like_call = true;
        }
        syn::visit::visit_expr_call(self, expr);
    }

    fn visit_expr_method_call(&mut self, expr: &'ast syn::ExprMethodCall) {
        if is_assert_like_name(&expr.method.to_string()) {
            self.has_assert_like_call = true;
        }
        syn::visit::visit_expr_method_call(self, expr);
    }
}

fn is_test_attr(attr: &syn::Attribute) -> bool {
    let path = attr.path();
    path.is_ident("test")
        || path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "test")
}

fn is_cfg_test_attr(attr: &syn::Attribute) -> bool {
    if !attr.path().is_ident("cfg") {
        return false;
    }
    match &attr.meta {
        syn::Meta::List(list) => list.tokens.to_string().replace(' ', "") == "test",
        _ => false,
    }
}

fn signature_returns_result(sig: &syn::Signature) -> bool {
    match &sig.output {
        syn::ReturnType::Type(_, ty) => type_is_result(ty),
        syn::ReturnType::Default => false,
    }
}

fn type_is_result(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "Result"),
        _ => false,
    }
}

fn is_assertion_macro_name(name: &str) -> bool {
    matches!(
        name,
        "assert"
            | "assert_eq"
            | "assert_ne"
            | "assert_matches"
            | "debug_assert"
            | "debug_assert_eq"
            | "debug_assert_ne"
    )
}

fn is_assert_like_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.contains("assert") || lower.contains("verify") || lower.contains("expect")
}

fn call_ident(expr: &syn::Expr) -> Option<String> {
    match expr {
        syn::Expr::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        _ => None,
    }
}

fn macro_has_literal_comparison(mac: &syn::Macro) -> bool {
    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
    let Ok(args) = parser.parse2(mac.tokens.clone()) else {
        return false;
    };
    if args.len() < 2 {
        return false;
    }
    args.iter()
        .take(2)
        .all(|expr| matches!(expr, syn::Expr::Lit(_)))
}

fn macro_has_weak_matches(mac: &syn::Macro) -> bool {
    let Ok(expr) = syn::parse2::<syn::Expr>(mac.tokens.clone()) else {
        return false;
    };
    let syn::Expr::Macro(expr_macro) = expr else {
        return false;
    };
    let Some(name) = expr_macro
        .mac
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
    else {
        return false;
    };
    if name != "matches" {
        return false;
    }
    let Ok(args) = syn::parse2::<MatchesArgs>(expr_macro.mac.tokens.clone()) else {
        return false;
    };
    pattern_contains_wild(&args.pattern)
}

fn pattern_contains_wild(pattern: &syn::Pat) -> bool {
    match pattern {
        syn::Pat::Wild(_) => true,
        syn::Pat::Tuple(tuple) => tuple.elems.iter().any(pattern_contains_wild),
        syn::Pat::TupleStruct(tuple) => tuple.elems.iter().any(pattern_contains_wild),
        syn::Pat::Struct(strct) => strct
            .fields
            .iter()
            .any(|field| pattern_contains_wild(&field.pat)),
        syn::Pat::Slice(slice) => slice.elems.iter().any(pattern_contains_wild),
        syn::Pat::Reference(reference) => pattern_contains_wild(&reference.pat),
        syn::Pat::Or(or) => or.cases.iter().any(pattern_contains_wild),
        syn::Pat::Paren(paren) => pattern_contains_wild(&paren.pat),
        syn::Pat::Ident(_) | syn::Pat::Path(_) | syn::Pat::Lit(_) | syn::Pat::Range(_) => false,
        _ => false,
    }
}

fn should_panic_has_expected(attr: &syn::Attribute) -> bool {
    match &attr.meta {
        syn::Meta::List(list) => list.tokens.to_string().contains("expected"),
        _ => false,
    }
}

fn span_line(span: proc_macro2::Span) -> usize {
    span.start().line
}

struct MatchesArgs {
    #[allow(dead_code)] // reason: only the pattern matters for this rule
    expr: syn::Expr,
    #[allow(dead_code)] // reason: optional guard is accepted but not inspected
    guard: Option<syn::Expr>,
    pattern: syn::Pat,
}

impl Parse for MatchesArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let expr = input.parse::<syn::Expr>()?;
        let _ = input.parse::<syn::Token![,]>()?;
        let pattern = syn::Pat::parse_multi_with_leading_vert(input)?;
        let guard = if input.peek(syn::Token![if]) {
            let _ = input.parse::<syn::Token![if]>()?;
            Some(input.parse::<syn::Expr>()?)
        } else {
            None
        };
        Ok(Self {
            expr,
            guard,
            pattern,
        })
    }
}
