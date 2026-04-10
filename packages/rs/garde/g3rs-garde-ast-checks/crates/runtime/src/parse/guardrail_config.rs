use std::collections::{BTreeMap, BTreeSet};

use super::fields;
use super::{GuardrailConfigParseKind, GuardrailConfigValidationSite};

#[derive(Debug, Clone)]
struct GuardrailConfigParseCandidate {
    line: usize,
    binding_names: BTreeSet<String>,
    parse_kind: GuardrailConfigParseKind,
    inline_validated: bool,
}

pub(crate) fn collect_unvalidated_guardrail_sites(
    block: &syn::Block,
    output: &syn::ReturnType,
    module_path_aliases: &BTreeMap<String, String>,
) -> Vec<GuardrailConfigValidationSite> {
    let validated_names = collect_validate_call_receivers(block);
    let non_validate_use_lines = collect_non_validate_use_lines(block);
    let binding_aliases = collect_binding_aliases(block);
    let function_returns_guardrail = return_type_contains_guardrail_config(output);
    let mut candidates = Vec::new();
    collect_guardrail_parse_candidates_from_block(
        block,
        function_returns_guardrail,
        module_path_aliases,
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
                        &binding_aliases,
                    )
                })
        })
        .map(|candidate| GuardrailConfigValidationSite {
            line: candidate.line,
            parse_kind: candidate.parse_kind,
        })
        .collect()
}

#[derive(Debug, Clone)]
struct BindingAlias {
    from_name: String,
    alias_name: String,
    line: usize,
    ignored_use_lines: BTreeSet<usize>,
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
    binding_aliases: &[BindingAlias],
) -> bool {
    let tracked_names = tracked_binding_names(binding_name, parse_line, binding_aliases);
    let alias_rebind_lines = binding_aliases
        .iter()
        .filter(|alias| {
            alias.line > parse_line
                && tracked_names.contains(&alias.from_name)
                && tracked_names.contains(&alias.alias_name)
        })
        .flat_map(|alias| alias.ignored_use_lines.iter().copied())
        .collect::<BTreeSet<_>>();
    tracked_names.iter().any(|tracked_name| {
        validated_names
            .get(tracked_name)
            .into_iter()
            .flat_map(|lines| lines.iter().copied())
            .filter(|line| *line > parse_line)
            .any(|validate_line| {
                !tracked_names.iter().any(|used_name| {
                    non_validate_use_lines
                        .get(used_name)
                        .is_some_and(|lines| {
                            lines.iter()
                                .any(|line| {
                                    *line > parse_line
                                        && *line < validate_line
                                        && !alias_rebind_lines.contains(line)
                                })
                        })
                })
            })
    })
}

fn collect_guardrail_parse_candidates_from_block(
    block: &syn::Block,
    function_returns_guardrail: bool,
    module_path_aliases: &BTreeMap<String, String>,
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
                        module_path_aliases,
                        &binding_names,
                        false,
                        out,
                    );
                    if let Some((_, diverge)) = &init.diverge {
                        collect_guardrail_parse_candidates_from_expr(
                            diverge,
                            function_returns_guardrail,
                            module_path_aliases,
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
                module_path_aliases,
                &empty,
                false,
                out,
            ),
            syn::Stmt::Item(_) | syn::Stmt::Macro(_) => {}
        }
    }
}

fn collect_guardrail_parse_candidates_from_expr(
    expr: &syn::Expr,
    function_returns_guardrail: bool,
    module_path_aliases: &BTreeMap<String, String>,
    binding_names: &BTreeSet<String>,
    in_validate_receiver: bool,
    out: &mut Vec<GuardrailConfigParseCandidate>,
) {
    match expr {
        syn::Expr::Call(call) => {
            if let Some(parse_kind) =
                guardrail_parse_kind_for_call(call, function_returns_guardrail, module_path_aliases)
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
                    module_path_aliases,
                    binding_names,
                    false,
                    out,
                );
            }
        }
        syn::Expr::MethodCall(call) => {
            if guardrail_parse_kind_for_try_into(call, function_returns_guardrail).is_some() {
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
                module_path_aliases,
                binding_names,
                receiver_in_validate,
                out,
            );
            for argument in &call.args {
                collect_guardrail_parse_candidates_from_expr(
                    argument,
                    function_returns_guardrail,
                    module_path_aliases,
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
                module_path_aliases,
                &arm_binding_names,
                in_validate_receiver,
                out,
            );
            for arm in &expr_match.arms {
                if let Some((_, guard)) = &arm.guard {
                    collect_guardrail_parse_candidates_from_expr(
                        guard,
                        function_returns_guardrail,
                        module_path_aliases,
                        binding_names,
                        false,
                        out,
                    );
                }
                collect_guardrail_parse_candidates_from_expr(
                    &arm.body,
                    function_returns_guardrail,
                    module_path_aliases,
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
                module_path_aliases,
                out,
            );
        }
        syn::Expr::If(expr_if) => {
            collect_guardrail_parse_candidates_from_expr(
                &expr_if.cond,
                function_returns_guardrail,
                module_path_aliases,
                binding_names,
                false,
                out,
            );
            collect_guardrail_parse_candidates_from_block(
                &expr_if.then_branch,
                function_returns_guardrail,
                module_path_aliases,
                out,
            );
            if let Some((_, else_branch)) = &expr_if.else_branch {
                collect_guardrail_parse_candidates_from_expr(
                    else_branch,
                    function_returns_guardrail,
                    module_path_aliases,
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
                    module_path_aliases,
                    binding_names,
                    false,
                    out,
                );
            }
        }
        syn::Expr::Try(expr_try) => collect_guardrail_parse_candidates_from_expr(
            &expr_try.expr,
            function_returns_guardrail,
            module_path_aliases,
            binding_names,
            in_validate_receiver,
            out,
        ),
        syn::Expr::Paren(expr_paren) => collect_guardrail_parse_candidates_from_expr(
            &expr_paren.expr,
            function_returns_guardrail,
            module_path_aliases,
            binding_names,
            in_validate_receiver,
            out,
        ),
        syn::Expr::Group(expr_group) => collect_guardrail_parse_candidates_from_expr(
            &expr_group.expr,
            function_returns_guardrail,
            module_path_aliases,
            binding_names,
            in_validate_receiver,
            out,
        ),
        syn::Expr::Reference(expr_reference) => collect_guardrail_parse_candidates_from_expr(
            &expr_reference.expr,
            function_returns_guardrail,
            module_path_aliases,
            binding_names,
            in_validate_receiver,
            out,
        ),
        syn::Expr::Cast(expr_cast) => collect_guardrail_parse_candidates_from_expr(
            &expr_cast.expr,
            function_returns_guardrail,
            module_path_aliases,
            binding_names,
            in_validate_receiver,
            out,
        ),
        syn::Expr::Assign(expr_assign) => collect_guardrail_parse_candidates_from_expr(
            &expr_assign.right,
            function_returns_guardrail,
            module_path_aliases,
            binding_names,
            false,
            out,
        ),
        syn::Expr::Await(expr_await) => collect_guardrail_parse_candidates_from_expr(
            &expr_await.base,
            function_returns_guardrail,
            module_path_aliases,
            binding_names,
            in_validate_receiver,
            out,
        ),
        syn::Expr::Closure(expr_closure) => collect_guardrail_parse_candidates_from_expr(
            &expr_closure.body,
            function_returns_guardrail,
            module_path_aliases,
            binding_names,
            false,
            out,
        ),
        syn::Expr::Loop(expr_loop) => collect_guardrail_parse_candidates_from_block(
            &expr_loop.body,
            function_returns_guardrail,
            module_path_aliases,
            out,
        ),
        syn::Expr::While(expr_while) => {
            collect_guardrail_parse_candidates_from_expr(
                &expr_while.cond,
                function_returns_guardrail,
                module_path_aliases,
                binding_names,
                false,
                out,
            );
            collect_guardrail_parse_candidates_from_block(
                &expr_while.body,
                function_returns_guardrail,
                module_path_aliases,
                out,
            );
        }
        syn::Expr::ForLoop(expr_for_loop) => {
            collect_guardrail_parse_candidates_from_expr(
                &expr_for_loop.expr,
                function_returns_guardrail,
                module_path_aliases,
                binding_names,
                false,
                out,
            );
            collect_guardrail_parse_candidates_from_block(
                &expr_for_loop.body,
                function_returns_guardrail,
                module_path_aliases,
                out,
            );
        }
        syn::Expr::Async(expr_async) => collect_guardrail_parse_candidates_from_block(
            &expr_async.block,
            function_returns_guardrail,
            module_path_aliases,
            out,
        ),
        syn::Expr::Unsafe(expr_unsafe) => collect_guardrail_parse_candidates_from_block(
            &expr_unsafe.block,
            function_returns_guardrail,
            module_path_aliases,
            out,
        ),
        syn::Expr::Let(expr_let) => collect_guardrail_parse_candidates_from_expr(
            &expr_let.expr,
            function_returns_guardrail,
            module_path_aliases,
            binding_names,
            false,
            out,
        ),
        _ => {}
    }
}

fn guardrail_parse_kind_for_call(
    call: &syn::ExprCall,
    function_returns_guardrail: bool,
    module_path_aliases: &BTreeMap<String, String>,
) -> Option<GuardrailConfigParseKind> {
    let syn::Expr::Path(path) = &*call.func else {
        return None;
    };
    if !path_is_toml_from_str(&path.path, module_path_aliases) {
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
    function_returns_guardrail: bool,
) -> Option<GuardrailConfigParseKind> {
    (call.method == "try_into"
        && call.turbofish.as_ref().map_or(function_returns_guardrail, |turbofish| {
            turbofish.args.iter().any(|argument| {
                matches!(argument, syn::GenericArgument::Type(ty) if type_contains_guardrail_config(ty))
            })
        }))
    .then_some(GuardrailConfigParseKind::TryInto)
}

fn path_is_toml_from_str(path: &syn::Path, module_path_aliases: &BTreeMap<String, String>) -> bool {
    let mut segments = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    let Some(first) = segments.first().cloned() else {
        return false;
    };
    if let Some(target) = module_path_aliases.get(&first) {
        let mut resolved = target
            .split("::")
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        resolved.extend(segments.drain(1..));
        return resolved.join("::") == "toml::from_str";
    }
    segments.join("::") == "toml::from_str"
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

fn collect_binding_aliases(block: &syn::Block) -> Vec<BindingAlias> {
    let mut aliases = Vec::new();
    for statement in &block.stmts {
        match statement {
            syn::Stmt::Local(local) => {
                let Some(init) = &local.init else {
                    continue;
                };
                let alias_names = binding_names_from_pat(&local.pat);
                let source_names = binding_name_uses_from_expr(&init.expr);
                push_binding_aliases(
                    &mut aliases,
                    &alias_names,
                    &source_names,
                    fields::span_line(syn::spanned::Spanned::span(local)),
                );
            }
            syn::Stmt::Expr(expr, _) => {
                let syn::Expr::Assign(assign) = expr else {
                    continue;
                };
                let Some(alias_name) = expr_ident(&assign.left) else {
                    continue;
                };
                let mut alias_names = BTreeSet::new();
                let _ = alias_names.insert(alias_name);
                let source_names = binding_name_uses_from_expr(&assign.right);
                push_binding_aliases(
                    &mut aliases,
                    &alias_names,
                    &source_names,
                    fields::span_line(syn::spanned::Spanned::span(assign)),
                );
            }
            syn::Stmt::Item(_) | syn::Stmt::Macro(_) => {}
        }
    }
    aliases
}

fn push_binding_aliases(
    aliases: &mut Vec<BindingAlias>,
    alias_names: &BTreeSet<String>,
    source_names: &BTreeMap<String, BTreeSet<usize>>,
    line: usize,
) {
    if source_names.is_empty() {
        return;
    }
    for alias_name in alias_names {
        for (source_name, ignored_use_lines) in source_names {
            if alias_name != source_name {
                aliases.push(BindingAlias {
                    from_name: source_name.clone(),
                    alias_name: alias_name.clone(),
                    line,
                    ignored_use_lines: ignored_use_lines.clone(),
                });
            }
        }
    }
}

fn binding_name_uses_from_expr(expr: &syn::Expr) -> BTreeMap<String, BTreeSet<usize>> {
    let mut names = BTreeMap::new();
    collect_binding_name_uses_from_expr(expr, &mut names);
    names
}

fn collect_binding_name_uses_from_expr(
    expr: &syn::Expr,
    names: &mut BTreeMap<String, BTreeSet<usize>>,
) {
    match expr {
        syn::Expr::Path(path) => {
            if let Some(ident) = path.path.get_ident() {
                let _ = names
                    .entry(ident.to_string())
                    .or_default()
                    .insert(fields::span_line(syn::spanned::Spanned::span(path)));
            }
        }
        syn::Expr::Paren(expr_paren) => {
            collect_binding_name_uses_from_expr(&expr_paren.expr, names)
        }
        syn::Expr::Group(expr_group) => {
            collect_binding_name_uses_from_expr(&expr_group.expr, names)
        }
        syn::Expr::Reference(expr_reference) => {
            collect_binding_name_uses_from_expr(&expr_reference.expr, names)
        }
        syn::Expr::Try(expr_try) => collect_binding_name_uses_from_expr(&expr_try.expr, names),
        _ => {}
    }
}

fn expr_ident(expr: &syn::Expr) -> Option<String> {
    match expr {
        syn::Expr::Path(path) => path.path.get_ident().map(|ident| ident.to_string()),
        syn::Expr::Paren(expr_paren) => expr_ident(&expr_paren.expr),
        syn::Expr::Group(expr_group) => expr_ident(&expr_group.expr),
        syn::Expr::Reference(expr_reference) => expr_ident(&expr_reference.expr),
        _ => None,
    }
}

fn tracked_binding_names(
    binding_name: &str,
    parse_line: usize,
    binding_aliases: &[BindingAlias],
) -> BTreeSet<String> {
    let mut tracked = BTreeSet::from([binding_name.to_owned()]);
    let mut changed = true;
    while changed {
        changed = false;
        for alias in binding_aliases {
            if alias.line > parse_line
                && tracked.contains(&alias.from_name)
                && tracked.insert(alias.alias_name.clone())
            {
                changed = true;
            }
        }
    }
    tracked
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
