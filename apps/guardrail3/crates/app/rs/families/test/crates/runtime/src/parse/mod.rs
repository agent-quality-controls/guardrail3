use std::collections::BTreeSet;

use guardrail3_app_rs_ast::ast_helpers;
use syn::spanned::Spanned;
use syn::visit::Visit;

mod helpers;
mod types;

pub type CfgTestModuleInfo = self::types::CfgTestModuleInfo;
pub type FieldAccessInfo = self::types::FieldAccessInfo;
pub type FunctionInfo = self::types::FunctionInfo;
pub type ModuleInfo = self::types::ModuleInfo;
pub type ParsedTestFile = self::types::ParsedTestFile;
pub type PublicValueInfo = self::types::PublicValueInfo;
pub type PublicValueKind = self::types::PublicValueKind;
pub type ReturnKind = self::types::ReturnKind;
pub type TestFunctionInfo = self::types::TestFunctionInfo;
pub type UseBinding = self::types::UseBinding;

pub fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content))
}

pub fn analyze(ast: &syn::File, content: &str) -> ParsedTestFile {
    let check_result_aliases = collect_check_result_aliases(ast);
    let mut visitor = TestVisitor {
        out: ParsedTestFile {
            ignore_reasons: ast_helpers::find_ignore_reasons(ast, content),
            modules: Vec::new(),
            cfg_test_modules: Vec::new(),
            test_functions: Vec::new(),
            functions: Vec::new(),
            public_values: Vec::new(),
            file_value_names: BTreeSet::new(),
            file_function_names: BTreeSet::new(),
            check_result_aliases,
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
        let is_cfg_test = item.attrs.iter().any(helpers::is_cfg_test_attr);
        self.out.modules.push(ModuleInfo {
            line: helpers::span_line(item.span()),
            path_attr: item.attrs.iter().find_map(helpers::path_attr_value),
        });
        if is_cfg_test {
            self.out.cfg_test_modules.push(CfgTestModuleInfo {
                line: helpers::span_line(item.span()),
                name: item.ident.to_string(),
                has_body: item.content.is_some(),
                path_attr: item.attrs.iter().find_map(helpers::path_attr_value),
            });
        }
        syn::visit::visit_item_mod(self, item);
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        let _ = self
            .out
            .file_function_names
            .insert(item.sig.ident.to_string());
        let function = analyze_function(
            &item.attrs,
            &item.vis,
            &item.sig,
            &item.block,
            &self.out.check_result_aliases,
        );
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
        let function = analyze_function(
            &item.attrs,
            &item.vis,
            &item.sig,
            &item.block,
            &self.out.check_result_aliases,
        );
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
        helpers::collect_use_bindings(
            &item.tree,
            &mut Vec::new(),
            helpers::span_line(item.span()),
            &mut self.out.imports,
        );
        syn::visit::visit_item_use(self, item);
    }

    fn visit_item_extern_crate(&mut self, item: &'ast syn::ItemExternCrate) {
        self.out.imports.push(UseBinding {
            line: helpers::span_line(item.span()),
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
        } else if item
            .mac
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "define_result_assertions")
        {
            self.out.macro_defined_proof_functions.extend([
                "assert_findings".to_owned(),
                "assert_no_findings".to_owned(),
                "assert_contains".to_owned(),
            ]);
        }
        syn::visit::visit_item_macro(self, item);
    }

    fn visit_item_const(&mut self, item: &'ast syn::ItemConst) {
        let _ = self.out.file_value_names.insert(item.ident.to_string());
        if matches!(item.vis, syn::Visibility::Public(_)) {
            self.out.public_values.push(PublicValueInfo {
                line: helpers::span_line(item.span()),
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
                line: helpers::span_line(item.span()),
                name: item.ident.to_string(),
                kind: PublicValueKind::Static,
            });
        }
        syn::visit::visit_item_static(self, item);
    }

    fn visit_expr_call(&mut self, expr: &'ast syn::ExprCall) {
        if let Some(path) = helpers::call_path(&expr.func) {
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
    let uses_test_attr = attrs.iter().any(helpers::is_test_attr);
    if !uses_test_attr {
        return;
    }
    let uses_tokio_test_attr = attrs.iter().any(helpers::is_tokio_test_attr);
    let should_panic_attr = attrs
        .iter()
        .find(|attr| helpers::is_should_panic_attr(attr));
    let mut body_visitor = TestBodyVisitor::default();
    body_visitor.visit_block(block);
    out.push(TestFunctionInfo {
        line: helpers::span_line(sig.span()),
        name: sig.ident.to_string(),
        uses_tokio_test_attr,
        has_assertion_macro: function.has_assertion_macro,
        has_failure_enforcement: function.has_failure_enforcement,
        call_paths: function.call_paths.clone(),
        path_uses: function.path_uses.clone(),
        method_receiver_paths: body_visitor.method_receiver_paths,
        method_names: function.method_names.clone(),
        local_call_aliases: function.local_call_aliases.clone(),
        field_accesses: function.field_accesses.clone(),
        shadowed_idents: function.shadowed_idents.clone(),
        should_panic_line: should_panic_attr.map(|attr| helpers::span_line(attr.span())),
        should_panic_has_expected: should_panic_attr
            .is_some_and(helpers::should_panic_has_expected),
        tautological_assert_lines: body_visitor.tautological_assert_lines,
        weak_matches_lines: body_visitor.weak_matches_lines,
    });
}

fn analyze_function(
    attrs: &[syn::Attribute],
    vis: &syn::Visibility,
    sig: &syn::Signature,
    block: &syn::Block,
    check_result_aliases: &BTreeSet<String>,
) -> FunctionInfo {
    let mut body_visitor = TestBodyVisitor::default();
    body_visitor.visit_block(block);
    let mut arg_names = BTreeSet::new();
    let has_check_result_arg = sig.inputs.iter().any(|input| {
        if let syn::FnArg::Typed(typed) = input {
            helpers::collect_pat_idents(&typed.pat, &mut arg_names);
            type_mentions_check_result(&typed.ty, check_result_aliases)
        } else {
            false
        }
    });
    FunctionInfo {
        line: helpers::span_line(sig.span()),
        name: sig.ident.to_string(),
        is_public: matches!(vis, syn::Visibility::Public(_)),
        is_test: attrs.iter().any(helpers::is_test_attr),
        arg_count: sig.inputs.len(),
        arg_names,
        has_check_result_arg,
        return_kind: classify_return_kind(&sig.output),
        has_assertion_macro: body_visitor.has_assertion_macro,
        has_failure_enforcement: body_visitor.has_failure_enforcement,
        call_paths: body_visitor.call_paths,
        path_uses: body_visitor.path_uses,
        method_names: body_visitor.method_names,
        local_call_aliases: body_visitor.local_call_aliases,
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

fn type_mentions_check_result(ty: &syn::Type, aliases: &BTreeSet<String>) -> bool {
    match ty {
        syn::Type::Path(type_path) => type_path.path.segments.iter().any(|segment| {
            segment.ident == "CheckResult" || aliases.contains(&segment.ident.to_string())
        }),
        syn::Type::Reference(reference) => type_mentions_check_result(&reference.elem, aliases),
        syn::Type::Slice(slice) => type_mentions_check_result(&slice.elem, aliases),
        syn::Type::Array(array) => type_mentions_check_result(&array.elem, aliases),
        syn::Type::Tuple(tuple) => tuple
            .elems
            .iter()
            .any(|element| type_mentions_check_result(element, aliases)),
        syn::Type::Group(group) => type_mentions_check_result(&group.elem, aliases),
        syn::Type::Paren(paren) => type_mentions_check_result(&paren.elem, aliases),
        _ => false,
    }
}

fn collect_check_result_aliases(ast: &syn::File) -> BTreeSet<String> {
    let mut aliases = BTreeSet::new();

    loop {
        let mut changed = false;
        for item in &ast.items {
            let syn::Item::Type(item_type) = item else {
                continue;
            };
            if aliases.contains(&item_type.ident.to_string()) {
                continue;
            }
            if type_mentions_check_result(&item_type.ty, &aliases) {
                changed |= aliases.insert(item_type.ident.to_string());
            }
        }
        if !changed {
            break;
        }
    }

    aliases
}

#[derive(Default)]
struct TestBodyVisitor {
    has_assertion_macro: bool,
    has_failure_enforcement: bool,
    call_paths: Vec<Vec<String>>,
    path_uses: Vec<Vec<String>>,
    method_receiver_paths: Vec<Vec<String>>,
    method_names: Vec<String>,
    local_call_aliases: std::collections::BTreeMap<String, Vec<String>>,
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
            if helpers::is_assertion_macro_name(&name) {
                self.has_assertion_macro = true;
                self.has_failure_enforcement = true;
            }
            if name == "panic" {
                self.has_failure_enforcement = true;
            }
            if matches!(
                name.as_str(),
                "assert_eq" | "assert_ne" | "debug_assert_eq" | "debug_assert_ne"
            ) && helpers::macro_has_literal_comparison(mac)
            {
                self.tautological_assert_lines
                    .push(helpers::span_line(mac.span()));
            }
            if matches!(name.as_str(), "assert" | "debug_assert")
                && helpers::macro_has_weak_matches(mac)
            {
                self.weak_matches_lines.push(helpers::span_line(mac.span()));
            }
            if name == "assert_matches" && helpers::macro_has_weak_assert_matches(mac) {
                self.weak_matches_lines.push(helpers::span_line(mac.span()));
            }
            if helpers::is_assertion_macro_name(&name) || name == "panic" {
                helpers::visit_macro_expr_args(self, mac);
            }
        }
        syn::visit::visit_macro(self, mac);
    }

    fn visit_expr_call(&mut self, expr: &'ast syn::ExprCall) {
        if let Some(path) = helpers::call_path(&expr.func) {
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
        if let Some(path) = helpers::call_path(&expr.receiver) {
            self.method_receiver_paths.push(path);
        }
        self.method_names.push(expr.method.to_string());
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
        helpers::collect_pat_idents(&local.pat, &mut self.shadowed_idents);
        if let (Some(init), Some(name)) =
            (local.init.as_ref(), helpers::single_pat_ident(&local.pat))
        {
            if let Some(path) = helpers::call_path(&init.expr) {
                let _ = self.local_call_aliases.insert(name, path);
            }
        }
        syn::visit::visit_local(self, local);
    }
}
