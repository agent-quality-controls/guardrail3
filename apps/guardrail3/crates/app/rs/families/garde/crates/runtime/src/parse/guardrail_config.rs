use std::collections::BTreeMap;
use std::collections::BTreeSet;

use super::fields;
use super::{GuardrailConfigParseKind, GuardrailConfigValidationSite};

#[derive(Debug, Clone)]
struct GuardrailConfigParseCandidate {
    line: usize,
    binding_names: BTreeSet<String>,
    parse_kind: GuardrailConfigParseKind,
    inline_validated: bool,
}

pub(super) fn collect_unvalidated_guardrail_sites(
    block: &syn::Block,
    output: &syn::ReturnType,
) -> Vec<GuardrailConfigValidationSite> {
    let validated_names = collect_validate_call_receivers(block);
    let non_validate_use_lines = collect_non_validate_use_lines(block);
    let function_returns_guardrail = return_type_contains_guardrail_config(output);
    let mut candidates = Vec::new();
    collect_guardrail_parse_candidates_from_block(
        block,
        function_returns_guardrail,
        &mut candidates,
    );
    candidates
        .into_iter()
        .filter(|candidate| {
            !candidate.inline_validated
                && !candidate.binding_names.iter().any(|name| {
                    has_post_parse_validate_before_use(
                        name,
                        candidate.line,
                        &validated_names,
                        &non_validate_use_lines,
                    )
                })
        })
        .map(|candidate| GuardrailConfigValidationSite {
            line: candidate.line,
            parse_kind: candidate.parse_kind,
        })
        .collect()
}

fn collect_validate_call_receivers(block: &syn::Block) -> BTreeMap<String, Vec<usize>> {
    #[derive(Default)]
    struct ValidateCallVisitor {
        receivers: BTreeMap<String, Vec<usize>>,
    }

    impl<'ast> syn::visit::Visit<'ast> for ValidateCallVisitor {
        fn visit_expr_method_call(&mut self, expr: &'ast syn::ExprMethodCall) {
            if expr.method == "validate" {
                if let Some(name) = receiver_ident(&expr.receiver) {
                    self.receivers
                        .entry(name)
                        .or_default()
                        .push(fields::span_line(syn::spanned::Spanned::span(expr)));
                }
            }
            syn::visit::visit_expr_method_call(self, expr);
        }

        fn visit_item_fn(&mut self, _item: &'ast syn::ItemFn) {}

        fn visit_item_mod(&mut self, _item: &'ast syn::ItemMod) {}
    }

    let mut visitor = ValidateCallVisitor::default();
    syn::visit::Visit::visit_block(&mut visitor, block);
    visitor.receivers
}

fn collect_non_validate_use_lines(block: &syn::Block) -> BTreeMap<String, Vec<usize>> {
    #[derive(Default)]
    struct NonValidateUseVisitor {
        uses: BTreeMap<String, Vec<usize>>,
    }

    impl<'ast> syn::visit::Visit<'ast> for NonValidateUseVisitor {
        fn visit_expr_path(&mut self, expr: &'ast syn::ExprPath) {
            if let Some(name) = expr.path.get_ident() {
                self.uses
                    .entry(name.to_string())
                    .or_default()
                    .push(fields::span_line(syn::spanned::Spanned::span(expr)));
            }
            syn::visit::visit_expr_path(self, expr);
        }

        fn visit_expr_method_call(&mut self, expr: &'ast syn::ExprMethodCall) {
            if expr.method == "validate" {
                for argument in &expr.args {
                    self.visit_expr(argument);
                }
                return;
            }
            syn::visit::visit_expr_method_call(self, expr);
        }

        fn visit_expr_assign(&mut self, expr: &'ast syn::ExprAssign) {
            self.visit_expr(&expr.right);
        }

        fn visit_item_fn(&mut self, _item: &'ast syn::ItemFn) {}

        fn visit_item_mod(&mut self, _item: &'ast syn::ItemMod) {}
    }

    let mut visitor = NonValidateUseVisitor::default();
    syn::visit::Visit::visit_block(&mut visitor, block);
    visitor.uses
}

fn has_post_parse_validate_before_use(
    binding_name: &str,
    parse_line: usize,
    validated_names: &BTreeMap<String, Vec<usize>>,
    non_validate_use_lines: &BTreeMap<String, Vec<usize>>,
) -> bool {
    let Some(validate_lines) = validated_names.get(binding_name) else {
        return false;
    };
    let use_lines = non_validate_use_lines.get(binding_name);

    validate_lines
        .iter()
        .copied()
        .filter(|line| *line > parse_line)
        .any(|validate_line| {
            use_lines.is_none_or(|lines| {
                !lines
                    .iter()
                    .any(|line| *line > parse_line && *line < validate_line)
            })
        })
}

fn collect_guardrail_parse_candidates_from_block(
    block: &syn::Block,
    function_returns_guardrail: bool,
    out: &mut Vec<GuardrailConfigParseCandidate>,
) {
    let empty = BTreeSet::new();
    for statement in &block.stmts {
        match statement {
            syn::Stmt::Local(local) => {
                if let Some(init) = &local.init {
                    let binding_names = binding_names_from_pat(&local.pat);
                    collect_guardrail_parse_candidates_from_expr(
                        &init.expr,
                        function_returns_guardrail,
                        &binding_names,
                        false,
                        out,
                    );
                    if let Some((_, diverge)) = &init.diverge {
                        collect_guardrail_parse_candidates_from_expr(
                            diverge,
                            function_returns_guardrail,
                            &empty,
                            false,
                            out,
                        );
                    }
                }
            }
            syn::Stmt::Expr(expr, _) => collect_guardrail_parse_candidates_from_expr(
                expr,
                function_returns_guardrail,
                &empty,
                false,
                out,
            ),
            syn::Stmt::Item(_) => {}
            syn::Stmt::Macro(_) => {}
        }
    }
}

fn collect_guardrail_parse_candidates_from_expr(
    expr: &syn::Expr,
    function_returns_guardrail: bool,
    binding_names: &BTreeSet<String>,
    in_validate_receiver: bool,
    out: &mut Vec<GuardrailConfigParseCandidate>,
) {
    match expr {
        syn::Expr::Call(call) => {
            if let Some(parse_kind) =
                guardrail_parse_kind_for_call(call, function_returns_guardrail)
            {
                out.push(GuardrailConfigParseCandidate {
                    line: fields::span_line(syn::spanned::Spanned::span(call)),
                    binding_names: binding_names.clone(),
                    parse_kind,
                    inline_validated: in_validate_receiver,
                });
            }
            for argument in &call.args {
                collect_guardrail_parse_candidates_from_expr(
                    argument,
                    function_returns_guardrail,
                    binding_names,
                    false,
                    out,
                );
            }
        }
        syn::Expr::MethodCall(call) => {
            if guardrail_parse_kind_for_try_into(call).is_some() {
                out.push(GuardrailConfigParseCandidate {
                    line: fields::span_line(syn::spanned::Spanned::span(call)),
                    binding_names: binding_names.clone(),
                    parse_kind: GuardrailConfigParseKind::TryInto,
                    inline_validated: in_validate_receiver,
                });
            }

            let receiver_in_validate = in_validate_receiver || call.method == "validate";
            collect_guardrail_parse_candidates_from_expr(
                &call.receiver,
                function_returns_guardrail,
                binding_names,
                receiver_in_validate,
                out,
            );
            for argument in &call.args {
                collect_guardrail_parse_candidates_from_expr(
                    argument,
                    function_returns_guardrail,
                    binding_names,
                    false,
                    out,
                );
            }
        }
        syn::Expr::Match(expr_match) => {
            let mut arm_binding_names = binding_names.clone();
            for arm in &expr_match.arms {
                arm_binding_names.extend(binding_names_from_pat(&arm.pat));
            }
            collect_guardrail_parse_candidates_from_expr(
                &expr_match.expr,
                function_returns_guardrail,
                &arm_binding_names,
                in_validate_receiver,
                out,
            );
            for arm in &expr_match.arms {
                if let Some((_, guard)) = &arm.guard {
                    collect_guardrail_parse_candidates_from_expr(
                        guard,
                        function_returns_guardrail,
                        binding_names,
                        false,
                        out,
                    );
                }
                collect_guardrail_parse_candidates_from_expr(
                    &arm.body,
                    function_returns_guardrail,
                    binding_names,
                    false,
                    out,
                );
            }
        }
        syn::Expr::Block(expr_block) => {
            collect_guardrail_parse_candidates_from_block(
                &expr_block.block,
                function_returns_guardrail,
                out,
            );
        }
        syn::Expr::If(expr_if) => {
            collect_guardrail_parse_candidates_from_expr(
                &expr_if.cond,
                function_returns_guardrail,
                binding_names,
                false,
                out,
            );
            collect_guardrail_parse_candidates_from_block(
                &expr_if.then_branch,
                function_returns_guardrail,
                out,
            );
            if let Some((_, else_branch)) = &expr_if.else_branch {
                collect_guardrail_parse_candidates_from_expr(
                    else_branch,
                    function_returns_guardrail,
                    binding_names,
                    false,
                    out,
                );
            }
        }
        syn::Expr::Return(expr_return) => {
            if let Some(value) = &expr_return.expr {
                collect_guardrail_parse_candidates_from_expr(
                    value,
                    function_returns_guardrail,
                    binding_names,
                    false,
                    out,
                );
            }
        }
        syn::Expr::Try(expr_try) => collect_guardrail_parse_candidates_from_expr(
            &expr_try.expr,
            function_returns_guardrail,
            binding_names,
            in_validate_receiver,
            out,
        ),
        syn::Expr::Paren(expr_paren) => collect_guardrail_parse_candidates_from_expr(
            &expr_paren.expr,
            function_returns_guardrail,
            binding_names,
            in_validate_receiver,
            out,
        ),
        syn::Expr::Group(expr_group) => collect_guardrail_parse_candidates_from_expr(
            &expr_group.expr,
            function_returns_guardrail,
            binding_names,
            in_validate_receiver,
            out,
        ),
        syn::Expr::Reference(expr_reference) => collect_guardrail_parse_candidates_from_expr(
            &expr_reference.expr,
            function_returns_guardrail,
            binding_names,
            in_validate_receiver,
            out,
        ),
        syn::Expr::Cast(expr_cast) => collect_guardrail_parse_candidates_from_expr(
            &expr_cast.expr,
            function_returns_guardrail,
            binding_names,
            in_validate_receiver,
            out,
        ),
        syn::Expr::Assign(expr_assign) => collect_guardrail_parse_candidates_from_expr(
            &expr_assign.right,
            function_returns_guardrail,
            binding_names,
            false,
            out,
        ),
        syn::Expr::Await(expr_await) => collect_guardrail_parse_candidates_from_expr(
            &expr_await.base,
            function_returns_guardrail,
            binding_names,
            in_validate_receiver,
            out,
        ),
        _ => {}
    }
}

fn guardrail_parse_kind_for_call(
    call: &syn::ExprCall,
    function_returns_guardrail: bool,
) -> Option<GuardrailConfigParseKind> {
    let syn::Expr::Path(path) = &*call.func else {
        return None;
    };
    if !path_is_toml_from_str(&path.path) {
        return None;
    }
    let Some(last_segment) = path.path.segments.last() else {
        return None;
    };
    match &last_segment.arguments {
        syn::PathArguments::AngleBracketed(arguments) => arguments
            .args
            .iter()
            .filter_map(|argument| match argument {
                syn::GenericArgument::Type(ty) => Some(ty),
                _ => None,
            })
            .any(type_contains_guardrail_config)
            .then_some(GuardrailConfigParseKind::TomlFromStr),
        syn::PathArguments::None => {
            function_returns_guardrail.then_some(GuardrailConfigParseKind::TomlFromStr)
        }
        _ => None,
    }
}

fn guardrail_parse_kind_for_try_into(
    call: &syn::ExprMethodCall,
) -> Option<GuardrailConfigParseKind> {
    (call.method == "try_into"
        && call.turbofish.as_ref().is_some_and(|turbofish| {
            turbofish.args.iter().any(|argument| {
                matches!(argument, syn::GenericArgument::Type(ty) if type_contains_guardrail_config(ty))
            })
        }))
    .then_some(GuardrailConfigParseKind::TryInto)
}

fn path_is_toml_from_str(path: &syn::Path) -> bool {
    path.segments.len() == 2
        && path
            .segments
            .first()
            .is_some_and(|segment| segment.ident == "toml")
        && path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "from_str")
}

fn receiver_ident(expr: &syn::Expr) -> Option<String> {
    match expr {
        syn::Expr::Path(path) => path.path.get_ident().map(|ident| ident.to_string()),
        syn::Expr::Paren(expr_paren) => receiver_ident(&expr_paren.expr),
        syn::Expr::Group(expr_group) => receiver_ident(&expr_group.expr),
        syn::Expr::Reference(expr_reference) => receiver_ident(&expr_reference.expr),
        _ => None,
    }
}

fn binding_names_from_pat(pat: &syn::Pat) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    collect_binding_names_from_pat(pat, &mut names);
    names
}

fn collect_binding_names_from_pat(pat: &syn::Pat, names: &mut BTreeSet<String>) {
    match pat {
        syn::Pat::Ident(ident) => {
            let _ = names.insert(ident.ident.to_string());
        }
        syn::Pat::Paren(pat) => collect_binding_names_from_pat(&pat.pat, names),
        syn::Pat::Reference(pat) => collect_binding_names_from_pat(&pat.pat, names),
        syn::Pat::Tuple(tuple) => {
            for element in &tuple.elems {
                collect_binding_names_from_pat(element, names);
            }
        }
        syn::Pat::TupleStruct(tuple_struct) => {
            for element in &tuple_struct.elems {
                collect_binding_names_from_pat(element, names);
            }
        }
        syn::Pat::Struct(struct_pat) => {
            for field in &struct_pat.fields {
                collect_binding_names_from_pat(&field.pat, names);
            }
        }
        syn::Pat::Slice(slice) => {
            for element in &slice.elems {
                collect_binding_names_from_pat(element, names);
            }
        }
        syn::Pat::Type(pat_type) => collect_binding_names_from_pat(&pat_type.pat, names),
        _ => {}
    }
}

fn return_type_contains_guardrail_config(output: &syn::ReturnType) -> bool {
    match output {
        syn::ReturnType::Default => false,
        syn::ReturnType::Type(_, ty) => type_contains_guardrail_config(ty),
    }
}

fn type_contains_guardrail_config(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => {
            if type_path
                .path
                .segments
                .iter()
                .any(|segment| segment.ident == "GuardrailConfig")
            {
                return true;
            }
            type_path.path.segments.iter().any(|segment| match &segment.arguments {
                syn::PathArguments::AngleBracketed(arguments) => arguments.args.iter().any(|argument| {
                    matches!(argument, syn::GenericArgument::Type(ty) if type_contains_guardrail_config(ty))
                }),
                _ => false,
            })
        }
        syn::Type::Paren(paren) => type_contains_guardrail_config(&paren.elem),
        syn::Type::Reference(reference) => type_contains_guardrail_config(&reference.elem),
        syn::Type::Group(group) => type_contains_guardrail_config(&group.elem),
        syn::Type::Tuple(tuple) => tuple.elems.iter().any(type_contains_guardrail_config),
        _ => false,
    }
}
