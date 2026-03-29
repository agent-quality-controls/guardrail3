use std::collections::BTreeSet;

use guardrail3_app_rs_ast::ast_helpers;
use syn::parse::{Parse, Parser};
use syn::spanned::Spanned;
use syn::visit::Visit;

#[derive(Debug, Clone, Default)]
pub struct ParsedTestFile {
    pub ignore_without_reason_lines: Vec<usize>,
    pub modules: Vec<ModuleInfo>,
    pub cfg_test_modules: Vec<CfgTestModuleInfo>,
    pub test_functions: Vec<TestFunctionInfo>,
    pub functions: Vec<FunctionInfo>,
    pub public_values: Vec<PublicValueInfo>,
    pub file_value_names: BTreeSet<String>,
    pub file_function_names: BTreeSet<String>,
    pub file_call_paths: Vec<Vec<String>>,
    pub imports: Vec<UseBinding>,
    pub macro_defined_proof_functions: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub line: usize,
    pub path_attr: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CfgTestModuleInfo {
    pub line: usize,
    pub name: String,
    pub has_body: bool,
    pub path_attr: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TestFunctionInfo {
    pub line: usize,
    pub name: String,
    pub uses_tokio_test_attr: bool,
    pub has_assertion_macro: bool,
    pub has_failure_enforcement: bool,
    pub call_paths: Vec<Vec<String>>,
    pub path_uses: Vec<Vec<String>>,
    pub method_receiver_paths: Vec<Vec<String>>,
    pub field_accesses: Vec<FieldAccessInfo>,
    pub string_literals: Vec<String>,
    pub shadowed_idents: BTreeSet<String>,
    pub should_panic_line: Option<usize>,
    pub should_panic_has_expected: bool,
    pub tautological_assert_lines: Vec<usize>,
    pub weak_matches_lines: Vec<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct FunctionInfo {
    pub line: usize,
    pub name: String,
    pub is_public: bool,
    pub is_test: bool,
    pub arg_count: usize,
    pub arg_names: BTreeSet<String>,
    pub has_check_result_arg: bool,
    pub return_kind: ReturnKind,
    pub has_assertion_macro: bool,
    pub has_failure_enforcement: bool,
    pub call_paths: Vec<Vec<String>>,
    pub path_uses: Vec<Vec<String>>,
    pub field_accesses: Vec<FieldAccessInfo>,
    pub string_literals: Vec<String>,
    pub shadowed_idents: BTreeSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReturnKind {
    None,
    Other,
    StringLike,
    PathLike,
}

impl Default for ReturnKind {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone)]
pub struct PublicValueInfo {
    pub line: usize,
    pub name: String,
    pub kind: PublicValueKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicValueKind {
    Const,
    Static,
}

#[derive(Debug, Clone)]
pub struct FieldAccessInfo {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct UseBinding {
    pub line: usize,
    pub path_segments: Vec<String>,
    pub local_name: Option<String>,
}

pub fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content))
}

pub fn analyze(ast: &syn::File, content: &str) -> ParsedTestFile {
    let mut visitor = TestVisitor {
        out: ParsedTestFile {
            ignore_without_reason_lines: ast_helpers::find_ignore_without_reason(ast, content),
            modules: Vec::new(),
            cfg_test_modules: Vec::new(),
            test_functions: Vec::new(),
            functions: Vec::new(),
            public_values: Vec::new(),
            file_value_names: BTreeSet::new(),
            file_function_names: BTreeSet::new(),
            file_call_paths: Vec::new(),
            imports: Vec::new(),
            macro_defined_proof_functions: BTreeSet::new(),
        },
    };
    visitor.visit_file(ast);
    visitor.out
}

struct TestVisitor {
    out: ParsedTestFile,
}

impl<'ast> Visit<'ast> for TestVisitor {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        let is_cfg_test = item.attrs.iter().any(is_cfg_test_attr);
        self.out.modules.push(ModuleInfo {
            line: span_line(item.span()),
            path_attr: item.attrs.iter().find_map(path_attr_value),
        });
        if is_cfg_test {
            self.out.cfg_test_modules.push(CfgTestModuleInfo {
                line: span_line(item.span()),
                name: item.ident.to_string(),
                has_body: item.content.is_some(),
                path_attr: item.attrs.iter().find_map(path_attr_value),
            });
        }
        syn::visit::visit_item_mod(self, item);
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        let _ = self
            .out
            .file_function_names
            .insert(item.sig.ident.to_string());
        let function = analyze_function(&item.attrs, &item.vis, &item.sig, &item.block);
        maybe_push_test_function(
            &item.attrs,
            &item.sig,
            &item.block,
            &function,
            &mut self.out.test_functions,
        );
        self.out.functions.push(function);
        syn::visit::visit_item_fn(self, item);
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        let _ = self
            .out
            .file_function_names
            .insert(item.sig.ident.to_string());
        let function = analyze_function(&item.attrs, &item.vis, &item.sig, &item.block);
        maybe_push_test_function(
            &item.attrs,
            &item.sig,
            &item.block,
            &function,
            &mut self.out.test_functions,
        );
        self.out.functions.push(function);
        syn::visit::visit_impl_item_fn(self, item);
    }

    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        collect_use_bindings(
            &item.tree,
            &mut Vec::new(),
            span_line(item.span()),
            &mut self.out.imports,
        );
        syn::visit::visit_item_use(self, item);
    }

    fn visit_item_extern_crate(&mut self, item: &'ast syn::ItemExternCrate) {
        self.out.imports.push(UseBinding {
            line: span_line(item.span()),
            path_segments: vec![item.ident.to_string()],
            local_name: item.rename.as_ref().map(|(_, ident)| ident.to_string()),
        });
        syn::visit::visit_item_extern_crate(self, item);
    }

    fn visit_item_macro(&mut self, item: &'ast syn::ItemMacro) {
        if item
            .mac
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "define_rule_assertions")
        {
            self.out.macro_defined_proof_functions.extend([
                "assert_rule_quiet".to_owned(),
                "assert_rule_count".to_owned(),
                "assert_rule_files".to_owned(),
                "assert_rule_results".to_owned(),
            ]);
        }
        syn::visit::visit_item_macro(self, item);
    }

    fn visit_item_const(&mut self, item: &'ast syn::ItemConst) {
        let _ = self.out.file_value_names.insert(item.ident.to_string());
        if matches!(item.vis, syn::Visibility::Public(_)) {
            self.out.public_values.push(PublicValueInfo {
                line: span_line(item.span()),
                name: item.ident.to_string(),
                kind: PublicValueKind::Const,
            });
        }
        syn::visit::visit_item_const(self, item);
    }

    fn visit_item_static(&mut self, item: &'ast syn::ItemStatic) {
        let _ = self.out.file_value_names.insert(item.ident.to_string());
        if matches!(item.vis, syn::Visibility::Public(_)) {
            self.out.public_values.push(PublicValueInfo {
                line: span_line(item.span()),
                name: item.ident.to_string(),
                kind: PublicValueKind::Static,
            });
        }
        syn::visit::visit_item_static(self, item);
    }

    fn visit_expr_call(&mut self, expr: &'ast syn::ExprCall) {
        if let Some(path) = call_path(&expr.func) {
            self.out.file_call_paths.push(path);
        }
        syn::visit::visit_expr_call(self, expr);
    }
}

fn maybe_push_test_function(
    attrs: &[syn::Attribute],
    sig: &syn::Signature,
    block: &syn::Block,
    function: &FunctionInfo,
    out: &mut Vec<TestFunctionInfo>,
) {
    let uses_test_attr = attrs.iter().any(is_test_attr);
    if !uses_test_attr {
        return;
    }
    let uses_tokio_test_attr = attrs.iter().any(is_tokio_test_attr);
    let should_panic_attr = attrs
        .iter()
        .find(|attr| attr.path().is_ident("should_panic"));
    let mut body_visitor = TestBodyVisitor::default();
    body_visitor.visit_block(block);
    out.push(TestFunctionInfo {
        line: span_line(sig.span()),
        name: sig.ident.to_string(),
        uses_tokio_test_attr,
        has_assertion_macro: function.has_assertion_macro,
        has_failure_enforcement: function.has_failure_enforcement,
        call_paths: function.call_paths.clone(),
        path_uses: function.path_uses.clone(),
        method_receiver_paths: body_visitor.method_receiver_paths,
        field_accesses: function.field_accesses.clone(),
        string_literals: function.string_literals.clone(),
        shadowed_idents: function.shadowed_idents.clone(),
        should_panic_line: should_panic_attr.map(|attr| span_line(attr.span())),
        should_panic_has_expected: should_panic_attr.is_some_and(should_panic_has_expected),
        tautological_assert_lines: body_visitor.tautological_assert_lines,
        weak_matches_lines: body_visitor.weak_matches_lines,
    });
}

fn analyze_function(
    attrs: &[syn::Attribute],
    vis: &syn::Visibility,
    sig: &syn::Signature,
    block: &syn::Block,
) -> FunctionInfo {
    let mut body_visitor = TestBodyVisitor::default();
    body_visitor.visit_block(block);
    let mut arg_names = BTreeSet::new();
    let has_check_result_arg = sig.inputs.iter().any(|input| {
        if let syn::FnArg::Typed(typed) = input {
            collect_pat_idents(&typed.pat, &mut arg_names);
            type_mentions_check_result(&typed.ty)
        } else {
            false
        }
    });
    FunctionInfo {
        line: span_line(sig.span()),
        name: sig.ident.to_string(),
        is_public: matches!(vis, syn::Visibility::Public(_)),
        is_test: attrs.iter().any(is_test_attr),
        arg_count: sig.inputs.len(),
        arg_names,
        has_check_result_arg,
        return_kind: classify_return_kind(&sig.output),
        has_assertion_macro: body_visitor.has_assertion_macro,
        has_failure_enforcement: body_visitor.has_failure_enforcement,
        call_paths: body_visitor.call_paths,
        path_uses: body_visitor.path_uses,
        field_accesses: body_visitor.field_accesses,
        string_literals: body_visitor.string_literals,
        shadowed_idents: body_visitor.shadowed_idents,
    }
}

fn classify_return_kind(output: &syn::ReturnType) -> ReturnKind {
    let syn::ReturnType::Type(_, ty) = output else {
        return ReturnKind::None;
    };
    classify_type_kind(ty)
}

fn classify_type_kind(ty: &syn::Type) -> ReturnKind {
    match ty {
        syn::Type::Path(type_path) => {
            let last = type_path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string());
            match last.as_deref() {
                Some("String") | Some("str") => ReturnKind::StringLike,
                Some("Path") | Some("PathBuf") => ReturnKind::PathLike,
                _ => ReturnKind::Other,
            }
        }
        syn::Type::Reference(reference) => classify_type_kind(&reference.elem),
        syn::Type::Slice(slice) => match classify_type_kind(&slice.elem) {
            ReturnKind::StringLike => ReturnKind::StringLike,
            ReturnKind::PathLike => ReturnKind::PathLike,
            _ => ReturnKind::Other,
        },
        _ => ReturnKind::Other,
    }
}

fn type_mentions_check_result(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .iter()
            .any(|segment| segment.ident == "CheckResult"),
        syn::Type::Reference(reference) => type_mentions_check_result(&reference.elem),
        syn::Type::Slice(slice) => type_mentions_check_result(&slice.elem),
        syn::Type::Array(array) => type_mentions_check_result(&array.elem),
        syn::Type::Tuple(tuple) => tuple.elems.iter().any(type_mentions_check_result),
        syn::Type::Group(group) => type_mentions_check_result(&group.elem),
        syn::Type::Paren(paren) => type_mentions_check_result(&paren.elem),
        _ => false,
    }
}

#[derive(Default)]
struct TestBodyVisitor {
    has_assertion_macro: bool,
    has_failure_enforcement: bool,
    call_paths: Vec<Vec<String>>,
    path_uses: Vec<Vec<String>>,
    method_receiver_paths: Vec<Vec<String>>,
    field_accesses: Vec<FieldAccessInfo>,
    string_literals: Vec<String>,
    shadowed_idents: BTreeSet<String>,
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
                self.has_failure_enforcement = true;
            }
            if name == "panic" {
                self.has_failure_enforcement = true;
            }
            if matches!(
                name.as_str(),
                "assert_eq" | "assert_ne" | "debug_assert_eq" | "debug_assert_ne"
            ) && macro_has_literal_comparison(mac)
            {
                self.tautological_assert_lines.push(span_line(mac.span()));
            }
            if matches!(name.as_str(), "assert" | "debug_assert") && macro_has_weak_matches(mac) {
                self.weak_matches_lines.push(span_line(mac.span()));
            }
            if is_assertion_macro_name(&name) || name == "panic" {
                visit_macro_expr_args(self, mac);
            }
        }
        syn::visit::visit_macro(self, mac);
    }

    fn visit_expr_call(&mut self, expr: &'ast syn::ExprCall) {
        if let Some(path) = call_path(&expr.func) {
            self.call_paths.push(path);
        }
        syn::visit::visit_expr_call(self, expr);
    }

    fn visit_expr_path(&mut self, expr: &'ast syn::ExprPath) {
        self.path_uses.push(
            expr.path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect(),
        );
        syn::visit::visit_expr_path(self, expr);
    }

    fn visit_expr_field(&mut self, expr: &'ast syn::ExprField) {
        if let syn::Member::Named(name) = &expr.member {
            self.field_accesses.push(FieldAccessInfo {
                name: name.to_string(),
            });
        }
        syn::visit::visit_expr_field(self, expr);
    }

    fn visit_expr_method_call(&mut self, expr: &'ast syn::ExprMethodCall) {
        if let Some(path) = call_path(&expr.receiver) {
            self.method_receiver_paths.push(path);
        }
        if matches!(
            expr.method.to_string().as_str(),
            "unwrap" | "expect" | "unwrap_err" | "expect_err"
        ) {
            self.has_failure_enforcement = true;
        }
        syn::visit::visit_expr_method_call(self, expr);
    }

    fn visit_lit_str(&mut self, lit: &'ast syn::LitStr) {
        self.string_literals.push(lit.value());
        syn::visit::visit_lit_str(self, lit);
    }

    fn visit_local(&mut self, local: &'ast syn::Local) {
        collect_pat_idents(&local.pat, &mut self.shadowed_idents);
        syn::visit::visit_local(self, local);
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

fn is_tokio_test_attr(attr: &syn::Attribute) -> bool {
    let path = attr.path();
    path.segments.len() == 2
        && path.segments[0].ident == "tokio"
        && path.segments[1].ident == "test"
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

fn call_path(expr: &syn::Expr) -> Option<Vec<String>> {
    match expr {
        syn::Expr::Path(path) => path
            .path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .pipe(Some),
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
    args.iter().take(2).all(expr_is_literal_like)
}

fn macro_has_weak_matches(mac: &syn::Macro) -> bool {
    let Ok(expr) = syn::parse2::<syn::Expr>(mac.tokens.clone()) else {
        return false;
    };
    let expr = peel_parens(expr);
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

fn visit_macro_expr_args(visitor: &mut TestBodyVisitor, mac: &syn::Macro) {
    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
    let Ok(args) = parser.parse2(mac.tokens.clone()) else {
        return;
    };
    for expr in args {
        visitor.visit_expr(&expr);
    }
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
    let mut has_expected = false;
    let _ = attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("expected") {
            let value: syn::LitStr = meta.value()?.parse()?;
            has_expected = !value.value().trim().is_empty();
        }
        Ok(())
    });
    has_expected
}

fn span_line(span: proc_macro2::Span) -> usize {
    span.start().line
}

fn path_attr_value(attr: &syn::Attribute) -> Option<String> {
    if !attr.path().is_ident("path") {
        return None;
    }
    match &attr.meta {
        syn::Meta::NameValue(name_value) => match &name_value.value {
            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                syn::Lit::Str(value) => Some(value.value()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

fn collect_use_bindings(
    tree: &syn::UseTree,
    prefix: &mut Vec<String>,
    line: usize,
    out: &mut Vec<UseBinding>,
) {
    match tree {
        syn::UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_use_bindings(&path.tree, prefix, line, out);
            let _ = prefix.pop();
        }
        syn::UseTree::Name(name) => {
            let mut path_segments = prefix.clone();
            path_segments.push(name.ident.to_string());
            out.push(UseBinding {
                line,
                path_segments,
                local_name: Some(name.ident.to_string()),
            });
        }
        syn::UseTree::Rename(rename) => {
            let mut path_segments = prefix.clone();
            path_segments.push(rename.ident.to_string());
            out.push(UseBinding {
                line,
                path_segments,
                local_name: Some(rename.rename.to_string()),
            });
        }
        syn::UseTree::Glob(_) => {
            out.push(UseBinding {
                line,
                path_segments: prefix.clone(),
                local_name: None,
            });
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_use_bindings(item, prefix, line, out);
            }
        }
    }
}

fn expr_is_literal_like(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Lit(_) => true,
        syn::Expr::Paren(paren) => expr_is_literal_like(&paren.expr),
        syn::Expr::Group(group) => expr_is_literal_like(&group.expr),
        _ => false,
    }
}

fn peel_parens(expr: syn::Expr) -> syn::Expr {
    match expr {
        syn::Expr::Paren(paren) => peel_parens(*paren.expr),
        syn::Expr::Group(group) => peel_parens(*group.expr),
        other => other,
    }
}

fn collect_pat_idents(pat: &syn::Pat, out: &mut BTreeSet<String>) {
    match pat {
        syn::Pat::Ident(ident) => {
            let _ = out.insert(ident.ident.to_string());
        }
        syn::Pat::Tuple(tuple) => {
            for element in &tuple.elems {
                collect_pat_idents(element, out);
            }
        }
        syn::Pat::TupleStruct(tuple) => {
            for element in &tuple.elems {
                collect_pat_idents(element, out);
            }
        }
        syn::Pat::Struct(strct) => {
            for field in &strct.fields {
                collect_pat_idents(&field.pat, out);
            }
        }
        syn::Pat::Slice(slice) => {
            for element in &slice.elems {
                collect_pat_idents(element, out);
            }
        }
        syn::Pat::Reference(reference) => collect_pat_idents(&reference.pat, out),
        syn::Pat::Type(typed) => collect_pat_idents(&typed.pat, out),
        syn::Pat::Or(or) => {
            for case in &or.cases {
                collect_pat_idents(case, out);
            }
        }
        syn::Pat::Paren(paren) => collect_pat_idents(&paren.pat, out),
        _ => {}
    }
}

trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}

impl<T> Pipe for T {}

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
