use std::collections::{BTreeMap, BTreeSet};

use syn::spanned::Spanned;
use syn::visit::Visit;

use super::helpers;
use super::types::{
    AssertionBodyInfo, FieldAccessInfo, FunctionBodyFacts, FunctionInfo, FunctionSignatureInfo,
    ReturnKind, TestFunctionInfo, TestHarnessFacts,
};

pub(super) fn maybe_push_test_function(
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
        assertions: function.assertions.clone(),
        body: function.body.clone(),
        harness: TestHarnessFacts {
            uses_tokio_test_attr,
            method_receiver_paths: body_visitor.method_receiver_paths,
            should_panic_line: should_panic_attr.map(|attr| helpers::span_line(attr.span())),
            should_panic_has_expected: should_panic_attr
                .is_some_and(helpers::should_panic_has_expected),
            tautological_assert_lines: body_visitor.tautological_assert_lines,
            weak_matches_lines: body_visitor.weak_matches_lines,
        },
    });
}

pub(super) fn analyze_function(
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
        signature: FunctionSignatureInfo {
            arg_count: sig.inputs.len(),
            arg_names,
            has_check_result_arg,
            return_kind: classify_return_kind(&sig.output),
        },
        assertions: AssertionBodyInfo {
            has_assertion_macro: body_visitor.has_assertion_macro,
            has_failure_enforcement: body_visitor.has_failure_enforcement,
        },
        body: FunctionBodyFacts {
            call_paths: body_visitor.call_paths,
            path_uses: body_visitor.path_uses,
            method_names: body_visitor.method_names,
            local_call_aliases: body_visitor.local_call_aliases,
            field_accesses: body_visitor.field_accesses,
            shadowed_idents: body_visitor.shadowed_idents,
        },
        string_literals: body_visitor.string_literals,
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

pub(super) fn collect_check_result_aliases(source: &syn::File) -> BTreeSet<String> {
    let mut aliases = BTreeSet::new();

    loop {
        let mut changed = false;
        for item in &source.items {
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
pub(super) struct TestBodyVisitor {
    has_assertion_macro: bool,
    has_failure_enforcement: bool,
    call_paths: Vec<Vec<String>>,
    path_uses: Vec<Vec<String>>,
    method_receiver_paths: Vec<Vec<String>>,
    method_names: Vec<String>,
    local_call_aliases: BTreeMap<String, Vec<String>>,
    field_accesses: Vec<FieldAccessInfo>,
    string_literals: Vec<String>,
    shadowed_idents: BTreeSet<String>,
    tautological_assert_lines: Vec<usize>,
    weak_matches_lines: Vec<usize>,
}

impl<'source> Visit<'source> for TestBodyVisitor {
    fn visit_macro(&mut self, mac: &'source syn::Macro) {
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

    fn visit_expr_call(&mut self, expr: &'source syn::ExprCall) {
        if let Some(path) = helpers::call_path(&expr.func) {
            self.call_paths.push(path);
        }
        syn::visit::visit_expr_call(self, expr);
    }

    fn visit_expr_path(&mut self, expr: &'source syn::ExprPath) {
        self.path_uses.push(
            expr.path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect(),
        );
        syn::visit::visit_expr_path(self, expr);
    }

    fn visit_expr_field(&mut self, expr: &'source syn::ExprField) {
        if let syn::Member::Named(name) = &expr.member {
            self.field_accesses.push(FieldAccessInfo {
                name: name.to_string(),
            });
        }
        syn::visit::visit_expr_field(self, expr);
    }

    fn visit_expr_method_call(&mut self, expr: &'source syn::ExprMethodCall) {
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

    fn visit_lit_str(&mut self, lit: &'source syn::LitStr) {
        self.string_literals.push(lit.value());
        syn::visit::visit_lit_str(self, lit);
    }

    fn visit_local(&mut self, local: &'source syn::Local) {
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
