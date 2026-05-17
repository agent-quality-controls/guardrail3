use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use astro_config_parser_types::document::{
    AstroAdapterSnapshot, AstroCallSnapshot, AstroConfigDocument, AstroConfigFileKind,
    AstroConfigParseState, AstroConfigSelectedFile, AstroConfigSnapshot, AstroIntegrationSnapshot,
    AstroOutputMode, AstroStaticObjectProperty, AstroStaticValue, AstroTrailingSlashPolicy,
};
use swc_common::{FileName, SourceMap, sync::Lrc};
use swc_ecma_ast::{
    ArrowExpr, AssignExpr, BindingIdent, BlockStmt, BlockStmtOrExpr, Callee, Decl, EsVersion, Expr,
    ExprOrSpread, KeyValueProp, Lit, MemberProp, Module, ModuleDecl, ModuleItem, ObjectLit,
    OptCall, OptChainBase, Pat, Prop, PropName, PropOrSpread, ReturnStmt, Script, Stmt, VarDecl,
    VarDeclKind, VarDeclOrExpr, VarDeclarator,
};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

pub fn parse(
    workspace_root: impl AsRef<Path>,
    config_rel_path: &str,
) -> Result<AstroConfigSnapshot, crate::error::Error> {
    let document = parse_document(workspace_root, config_rel_path)?;
    match document.typed {
        AstroConfigParseState::Parsed(snapshot) => Ok(snapshot),
        AstroConfigParseState::Invalid(reason) => Err(crate::error::Error::Parse(reason)),
    }
}

pub fn parse_document(
    workspace_root: impl AsRef<Path>,
    config_rel_path: &str,
) -> Result<AstroConfigDocument, crate::error::Error> {
    let abs_path = workspace_root.as_ref().join(config_rel_path);
    let source = crate::fs::read_to_string(&abs_path)?;
    let selected_config = AstroConfigSelectedFile {
        rel_path: config_rel_path.to_owned(),
        kind: file_kind(config_rel_path)?,
    };
    let program = parse_program(&abs_path, &source, selected_config.kind)?;
    let raw = serde_json::json!({
        "selected_config": {
            "rel_path": selected_config.rel_path,
            "kind": selected_config.kind,
        }
    });
    let typed = match normalize_snapshot(&program, &selected_config, workspace_root.as_ref()) {
        Ok(snapshot) => AstroConfigParseState::Parsed(snapshot),
        Err(reason) => AstroConfigParseState::Invalid(reason),
    };

    Ok(AstroConfigDocument { raw, typed })
}

pub fn from_path(
    workspace_root: impl AsRef<Path>,
    config_rel_path: &str,
) -> Result<AstroConfigSnapshot, crate::error::Error> {
    parse(workspace_root, config_rel_path)
}

pub fn module_has_runtime_source_import(
    workspace_root: impl AsRef<Path>,
    module_rel_path: &str,
    source_module: &str,
) -> Result<bool, crate::error::Error> {
    let abs_path = workspace_root.as_ref().join(module_rel_path);
    let source = crate::fs::read_to_string(&abs_path)?;
    let kind = file_kind(module_rel_path)?;
    let program = parse_program(&abs_path, &source, kind)?;
    let swc_ecma_ast::Program::Module(module) = program else {
        return Ok(false);
    };

    Ok(module
        .body
        .iter()
        .any(|item| module_item_has_runtime_source_import(item, source_module)))
}

fn file_kind(config_rel_path: &str) -> Result<AstroConfigFileKind, crate::error::Error> {
    match Path::new(config_rel_path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or_default()
    {
        "js" => Ok(AstroConfigFileKind::Js),
        "mjs" => Ok(AstroConfigFileKind::Mjs),
        "cjs" => Ok(AstroConfigFileKind::Cjs),
        "ts" => Ok(AstroConfigFileKind::Ts),
        "mts" => Ok(AstroConfigFileKind::Mts),
        "cts" => Ok(AstroConfigFileKind::Cts),
        _ => Err(crate::error::Error::Parse(format!(
            "unsupported Astro config file kind: {config_rel_path}"
        ))),
    }
}

fn module_item_has_runtime_source_import(item: &ModuleItem, source_module: &str) -> bool {
    match item {
        ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) => {
            !import_decl.type_only && import_decl.src.value.to_string_lossy() == source_module
        }
        ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(export_decl)) => {
            !export_decl.type_only
                && export_decl
                    .src
                    .as_ref()
                    .is_some_and(|source| source.value.to_string_lossy() == source_module)
        }
        ModuleItem::ModuleDecl(ModuleDecl::ExportAll(export_decl)) => {
            !export_decl.type_only && export_decl.src.value.to_string_lossy() == source_module
        }
        ModuleItem::ModuleDecl(_) | ModuleItem::Stmt(_) => false,
    }
}

fn parse_program(
    abs_path: &Path,
    source: &str,
    kind: AstroConfigFileKind,
) -> Result<swc_ecma_ast::Program, crate::error::Error> {
    let cm: Lrc<SourceMap> = Lrc::default();
    let fm = cm.new_source_file(
        FileName::Real(PathBuf::from(abs_path)).into(),
        source.to_owned(),
    );
    let syntax = match kind {
        AstroConfigFileKind::Ts | AstroConfigFileKind::Mts | AstroConfigFileKind::Cts => {
            Syntax::Typescript(TsSyntax {
                tsx: false,
                dts: false,
                ..Default::default()
            })
        }
        AstroConfigFileKind::Js | AstroConfigFileKind::Mjs | AstroConfigFileKind::Cjs => {
            Syntax::Es(EsSyntax {
                jsx: false,
                ..Default::default()
            })
        }
    };
    let lexer = Lexer::new(syntax, EsVersion::EsNext, StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);

    let program = match kind {
        AstroConfigFileKind::Mjs | AstroConfigFileKind::Mts => parser
            .parse_module()
            .map(swc_ecma_ast::Program::Module)
            .map_err(|err| crate::error::Error::Parse(err.kind().msg().to_string()))?,
        AstroConfigFileKind::Cjs | AstroConfigFileKind::Cts => parser
            .parse_commonjs()
            .map(swc_ecma_ast::Program::Script)
            .map_err(|err| crate::error::Error::Parse(err.kind().msg().to_string()))?,
        AstroConfigFileKind::Js | AstroConfigFileKind::Ts => parser
            .parse_program()
            .map_err(|err| crate::error::Error::Parse(err.kind().msg().to_string()))?,
    };

    if let Some(err) = parser.take_errors().into_iter().next() {
        return Err(crate::error::Error::Parse(err.kind().msg().to_string()));
    }

    Ok(program)
}

fn normalize_snapshot(
    program: &swc_ecma_ast::Program,
    selected_config: &AstroConfigSelectedFile,
    workspace_root: &Path,
) -> Result<AstroConfigSnapshot, String> {
    let mut state = AnalysisState::default();
    collect_state(program, &mut state);
    collect_local_import_static_values(workspace_root, &selected_config.rel_path, &mut state, 0);

    let export_expr = state
        .module_exports
        .first()
        .ok_or_else(|| "could not find exported Astro config expression".to_owned())?;
    let config_expr = resolve_to_config_expr(export_expr, &state, 0)
        .ok_or_else(|| "could not reduce exported Astro config to an object literal".to_owned())?;
    let config_object = as_object(config_expr, &state, 0)
        .ok_or_else(|| "Astro config export must resolve to an object literal".to_owned())?;

    Ok(AstroConfigSnapshot {
        selected_config: selected_config.clone(),
        site: property_string(config_object, "site", &state)?,
        output: property_output(config_object, &state)?,
        out_dir: property_string(config_object, "outDir", &state)?,
        trailing_slash: property_trailing_slash(config_object, &state)?,
        integrations: property_integrations(config_object, &state)?,
        adapter: property_adapter(config_object, &state)?,
    })
}

#[derive(Default)]
struct AnalysisState {
    import_bindings: BTreeMap<String, ImportBinding>,
    const_bindings: BTreeMap<String, Expr>,
    const_aliases: BTreeMap<String, BTreeSet<String>>,
    exported_const_names: BTreeMap<String, String>,
    imported_static_values: BTreeMap<String, AstroStaticValue>,
    mutated_bindings: BTreeSet<String>,
    module_exports: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ImportBinding {
    source_module: String,
    imported_name: Option<String>,
    kind: ImportBindingKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ImportBindingKind {
    Default,
    Named,
    Namespace,
}

fn collect_state(program: &swc_ecma_ast::Program, state: &mut AnalysisState) {
    match program {
        swc_ecma_ast::Program::Module(module) => collect_module_state(module, state),
        swc_ecma_ast::Program::Script(script) => collect_script_state(script, state),
    }
}

fn collect_local_import_static_values(
    workspace_root: &Path,
    module_rel_path: &str,
    state: &mut AnalysisState,
    depth: usize,
) {
    if depth > 8 {
        return;
    }

    for (local_name, binding) in state.import_bindings.clone() {
        if binding.kind != ImportBindingKind::Named || !binding.source_module.starts_with('.') {
            continue;
        }
        let Some(imported_name) = binding.imported_name.as_deref() else {
            continue;
        };
        let Some(import_rel_path) =
            resolve_local_module_rel_path(workspace_root, module_rel_path, &binding.source_module)
        else {
            continue;
        };
        let Ok(imported_state) = imported_module_state(workspace_root, &import_rel_path, depth + 1)
        else {
            continue;
        };
        let Some(local_export_name) = imported_state.exported_const_names.get(imported_name) else {
            continue;
        };
        let Some(expr) = const_binding(&imported_state, local_export_name) else {
            continue;
        };
        let Ok(value) = static_value(expr, &imported_state, 0) else {
            continue;
        };
        let _ = state.imported_static_values.insert(local_name, value);
    }
}

fn imported_module_state(
    workspace_root: &Path,
    module_rel_path: &str,
    depth: usize,
) -> Result<AnalysisState, crate::error::Error> {
    let abs_path = workspace_root.join(module_rel_path);
    let source = crate::fs::read_to_string(&abs_path)?;
    let kind = file_kind(module_rel_path)?;
    let program = parse_program(&abs_path, &source, kind)?;
    let mut state = AnalysisState::default();
    collect_state(&program, &mut state);
    collect_local_import_static_values(workspace_root, module_rel_path, &mut state, depth);

    Ok(state)
}

fn resolve_local_module_rel_path(
    workspace_root: &Path,
    from_rel_path: &str,
    source_module: &str,
) -> Option<String> {
    let from_dir = Path::new(from_rel_path)
        .parent()
        .unwrap_or_else(|| Path::new(""));
    let base_path = workspace_root.join(from_dir).join(source_module);
    let mut candidates = Vec::new();
    if base_path.extension().is_some() {
        candidates.push(base_path);
    } else {
        for extension in ["ts", "mts", "js", "mjs", "cts", "cjs"] {
            candidates.push(base_path.with_extension(extension));
        }
    }

    candidates.into_iter().find_map(|candidate| {
        candidate
            .is_file()
            .then(|| candidate.strip_prefix(workspace_root).ok())
            .flatten()
            .and_then(path_to_rel_string)
    })
}

fn path_to_rel_string(path: &Path) -> Option<String> {
    Some(path.to_str()?.replace(std::path::MAIN_SEPARATOR, "/"))
}

fn collect_module_state(module: &Module, state: &mut AnalysisState) {
    for item in &module.body {
        match item {
            ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) => {
                let source = import_decl.src.value.to_string_lossy().into_owned();
                for specifier in &import_decl.specifiers {
                    match specifier {
                        swc_ecma_ast::ImportSpecifier::Default(default_specifier) => {
                            let _ = state.import_bindings.insert(
                                default_specifier.local.sym.to_string(),
                                ImportBinding {
                                    source_module: source.clone(),
                                    imported_name: None,
                                    kind: ImportBindingKind::Default,
                                },
                            );
                        }
                        swc_ecma_ast::ImportSpecifier::Named(named_specifier) => {
                            let imported_name = named_specifier.imported.as_ref().map_or_else(
                                || named_specifier.local.sym.to_string(),
                                module_export_name,
                            );
                            let _ = state.import_bindings.insert(
                                named_specifier.local.sym.to_string(),
                                ImportBinding {
                                    source_module: source.clone(),
                                    imported_name: Some(imported_name),
                                    kind: ImportBindingKind::Named,
                                },
                            );
                        }
                        swc_ecma_ast::ImportSpecifier::Namespace(namespace_specifier) => {
                            let _ = state.import_bindings.insert(
                                namespace_specifier.local.sym.to_string(),
                                ImportBinding {
                                    source_module: source.clone(),
                                    imported_name: Some("*".to_owned()),
                                    kind: ImportBindingKind::Namespace,
                                },
                            );
                        }
                    }
                }
            }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(default_expr)) => {
                collect_expression_mutations(&default_expr.expr, state);
                state.module_exports.push((*default_expr.expr).clone());
            }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(export_decl)) => {
                if let Decl::Var(var_decl) = &export_decl.decl {
                    if var_decl.kind == VarDeclKind::Const {
                        collect_var_decl(var_decl, state);
                        collect_exported_var_decl(var_decl, state);
                    } else {
                        collect_var_decl_mutations(var_decl, state);
                    }
                }
            }
            ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(export_decl))
                if export_decl.src.is_none() =>
            {
                collect_named_exports(export_decl, state);
            }
            ModuleItem::Stmt(stmt) => collect_statement_state(stmt, state),
            ModuleItem::ModuleDecl(_) => {}
        }
    }
}

fn collect_script_state(script: &Script, state: &mut AnalysisState) {
    for stmt in &script.body {
        collect_statement_state(stmt, state);
    }
}

fn collect_statement_state(stmt: &Stmt, state: &mut AnalysisState) {
    match stmt {
        Stmt::Decl(Decl::Var(var_decl)) if var_decl.kind == VarDeclKind::Const => {
            collect_var_decl(var_decl, state);
        }
        Stmt::Decl(Decl::Var(var_decl)) => collect_var_decl_mutations(var_decl, state),
        Stmt::Expr(expr_stmt) => collect_expression_mutations(&expr_stmt.expr, state),
        Stmt::Block(block) => collect_block_statement_state(block, state),
        Stmt::Labeled(labeled) => collect_statement_state(&labeled.body, state),
        Stmt::If(if_stmt) => {
            collect_expression_mutations(&if_stmt.test, state);
            collect_statement_state(&if_stmt.cons, state);
            if let Some(alt) = &if_stmt.alt {
                collect_statement_state(alt, state);
            }
        }
        Stmt::Switch(switch_stmt) => {
            collect_expression_mutations(&switch_stmt.discriminant, state);
            for case in &switch_stmt.cases {
                if let Some(test) = &case.test {
                    collect_expression_mutations(test, state);
                }
                for stmt in &case.cons {
                    collect_statement_state(stmt, state);
                }
            }
        }
        Stmt::Try(try_stmt) => {
            collect_block_statement_state(&try_stmt.block, state);
            if let Some(handler) = &try_stmt.handler {
                collect_block_statement_state(&handler.body, state);
            }
            if let Some(finalizer) = &try_stmt.finalizer {
                collect_block_statement_state(finalizer, state);
            }
        }
        Stmt::While(while_stmt) => {
            collect_expression_mutations(&while_stmt.test, state);
            collect_statement_state(&while_stmt.body, state);
        }
        Stmt::DoWhile(do_while_stmt) => {
            collect_statement_state(&do_while_stmt.body, state);
            collect_expression_mutations(&do_while_stmt.test, state);
        }
        Stmt::For(for_stmt) => {
            if let Some(init) = &for_stmt.init {
                collect_for_init_state(init, state);
            }
            if let Some(test) = &for_stmt.test {
                collect_expression_mutations(test, state);
            }
            if let Some(update) = &for_stmt.update {
                collect_expression_mutations(update, state);
            }
            collect_statement_state(&for_stmt.body, state);
        }
        Stmt::ForIn(for_in_stmt) => collect_statement_state(&for_in_stmt.body, state),
        Stmt::ForOf(for_of_stmt) => collect_statement_state(&for_of_stmt.body, state),
        _ => {}
    }
}

fn collect_block_statement_state(block: &BlockStmt, state: &mut AnalysisState) {
    for stmt in &block.stmts {
        collect_statement_state(stmt, state);
    }
}

fn collect_for_init_state(init: &VarDeclOrExpr, state: &mut AnalysisState) {
    match init {
        VarDeclOrExpr::VarDecl(var_decl) if var_decl.kind == VarDeclKind::Const => {
            collect_var_decl(var_decl, state);
        }
        VarDeclOrExpr::VarDecl(var_decl) => collect_var_decl_mutations(var_decl, state),
        VarDeclOrExpr::Expr(expr) => collect_expression_mutations(expr, state),
    }
}

fn collect_var_decl(var_decl: &VarDecl, state: &mut AnalysisState) {
    for declarator in &var_decl.decls {
        collect_var_declarator(declarator, state);
    }
}

fn collect_exported_var_decl(var_decl: &VarDecl, state: &mut AnalysisState) {
    for declarator in &var_decl.decls {
        let Pat::Ident(BindingIdent { id, .. }) = &declarator.name else {
            continue;
        };
        let name = id.sym.to_string();
        let _ = state.exported_const_names.insert(name.clone(), name);
    }
}

fn collect_named_exports(export_decl: &swc_ecma_ast::NamedExport, state: &mut AnalysisState) {
    for specifier in &export_decl.specifiers {
        let swc_ecma_ast::ExportSpecifier::Named(named) = specifier else {
            continue;
        };
        let export_name = named
            .exported
            .as_ref()
            .map_or_else(|| module_export_name(&named.orig), module_export_name);
        let local_name = module_export_name(&named.orig);
        if const_binding(state, &local_name).is_some() {
            let _ = state.exported_const_names.insert(export_name, local_name);
        }
    }
}

fn collect_var_decl_mutations(var_decl: &VarDecl, state: &mut AnalysisState) {
    for declarator in &var_decl.decls {
        if let Some(init) = &declarator.init {
            collect_expression_mutations(init, state);
        }
    }
}

fn collect_var_declarator(declarator: &VarDeclarator, state: &mut AnalysisState) {
    let Some(init) = &declarator.init else {
        return;
    };
    collect_expression_mutations(init, state);
    let Pat::Ident(BindingIdent { id, .. }) = &declarator.name else {
        return;
    };
    let _ = state
        .const_bindings
        .insert(id.sym.to_string(), (**init).clone());
    let Some(alias_name) = expr_root_ident(init) else {
        return;
    };
    let mut aliases = binding_aliases(state, alias_name);
    let _ = aliases.insert(alias_name.to_owned());
    let _ = state.const_aliases.insert(id.sym.to_string(), aliases);
}

fn collect_expression_mutations(expr: &Expr, state: &mut AnalysisState) {
    match strip_wrappers(expr) {
        Expr::Assign(assign_expr) => {
            collect_assignment(assign_expr, state);
            collect_expression_mutations(&assign_expr.right, state);
        }
        Expr::Update(update_expr) => {
            if let Some(name) = expr_root_ident(&update_expr.arg) {
                mark_binding_mutated(state, name);
            }
        }
        Expr::Call(call_expr) => {
            collect_mutating_call(call_expr, state);
            if let Callee::Expr(callee_expr) = &call_expr.callee {
                collect_expression_mutations(callee_expr, state);
            }
            for arg in &call_expr.args {
                if arg.spread.is_none() {
                    collect_expression_mutations(&arg.expr, state);
                }
            }
        }
        Expr::OptChain(opt_chain) => match &*opt_chain.base {
            OptChainBase::Call(opt_call) => {
                collect_optional_mutating_call(opt_call, state);
                collect_expression_mutations(&opt_call.callee, state);
                for arg in &opt_call.args {
                    if arg.spread.is_none() {
                        collect_expression_mutations(&arg.expr, state);
                    } else {
                        mark_all_static_bindings_mutated(state);
                    }
                }
            }
            OptChainBase::Member(member) => collect_expression_mutations(&member.obj, state),
        },
        Expr::Arrow(arrow) => match &*arrow.body {
            BlockStmtOrExpr::Expr(expr) => collect_expression_mutations(expr, state),
            BlockStmtOrExpr::BlockStmt(block) => collect_block_statement_state(block, state),
        },
        Expr::Fn(function) => {
            if let Some(body) = &function.function.body {
                collect_block_statement_state(body, state);
            }
        }
        Expr::Array(array) => {
            for element in array.elems.iter().flatten() {
                if element.spread.is_none() {
                    collect_expression_mutations(&element.expr, state);
                }
            }
        }
        Expr::Object(object) => {
            for prop in &object.props {
                let PropOrSpread::Prop(prop) = prop else {
                    continue;
                };
                if let Prop::KeyValue(KeyValueProp { value, .. }) = &**prop {
                    collect_expression_mutations(value, state);
                }
            }
        }
        Expr::Cond(cond) => {
            collect_expression_mutations(&cond.test, state);
            collect_expression_mutations(&cond.cons, state);
            collect_expression_mutations(&cond.alt, state);
        }
        Expr::Bin(bin) => {
            collect_expression_mutations(&bin.left, state);
            collect_expression_mutations(&bin.right, state);
        }
        Expr::Unary(unary) => collect_expression_mutations(&unary.arg, state),
        Expr::Await(await_expr) => collect_expression_mutations(&await_expr.arg, state),
        Expr::Seq(seq) => {
            for expr in &seq.exprs {
                collect_expression_mutations(expr, state);
            }
        }
        _ => {}
    }
}

fn collect_assignment(assign_expr: &AssignExpr, state: &mut AnalysisState) {
    if assignment_exports_target(assign_expr) {
        collect_expression_mutations(&assign_expr.right, state);
        state.module_exports.push((*assign_expr.right).clone());
        return;
    }

    if let Some(name) = assignment_root_ident(&assign_expr.left) {
        mark_binding_mutated(state, name);
    }
}

fn collect_mutating_call(call_expr: &swc_ecma_ast::CallExpr, state: &mut AnalysisState) {
    let Callee::Expr(callee_expr) = &call_expr.callee else {
        return;
    };

    let Expr::Member(member) = strip_wrappers(callee_expr) else {
        return;
    };

    let Some(method_name) = member_property_name(&member.prop) else {
        return;
    };

    if is_mutating_member_method(&method_name) {
        if let Some(name) = expr_root_ident(&member.obj) {
            mark_binding_mutated(state, name);
        }
        return;
    }

    if is_global_mutating_call(&member.obj, &method_name) {
        let Some(first_arg) = call_expr.args.first() else {
            return;
        };
        if first_arg.spread.is_some() {
            mark_all_static_bindings_mutated(state);
            return;
        }
        if let Some(name) = expr_root_ident(&first_arg.expr) {
            mark_binding_mutated(state, name);
        }
    }
}

fn collect_optional_mutating_call(opt_call: &OptCall, state: &mut AnalysisState) {
    let Expr::Member(member) = strip_wrappers(&opt_call.callee) else {
        return;
    };

    let Some(method_name) = member_property_name(&member.prop) else {
        return;
    };

    if is_global_mutating_call(&member.obj, &method_name) {
        mark_all_static_bindings_mutated(state);
        return;
    }

    if is_mutating_member_method(&method_name) {
        if let Some(name) = expr_root_ident(&member.obj) {
            mark_binding_mutated(state, name);
        } else {
            mark_all_static_bindings_mutated(state);
        }
    }
}

fn is_mutating_member_method(method_name: &str) -> bool {
    matches!(
        method_name,
        "copyWithin"
            | "fill"
            | "pop"
            | "push"
            | "reverse"
            | "shift"
            | "sort"
            | "splice"
            | "unshift"
    )
}

fn is_global_mutating_call(object: &Expr, method_name: &str) -> bool {
    matches!(
        (strip_wrappers(object), method_name),
        (Expr::Ident(ident), "assign") if ident.sym == *"Object"
    ) || matches!(
        (strip_wrappers(object), method_name),
        (Expr::Ident(ident), "set") if ident.sym == *"Reflect"
    )
}

fn assignment_root_ident(target: &swc_ecma_ast::AssignTarget) -> Option<&str> {
    match target {
        swc_ecma_ast::AssignTarget::Simple(simple) => simple_assignment_root_ident(simple),
        swc_ecma_ast::AssignTarget::Pat(pat) => pat_assignment_root_ident(pat),
    }
}

fn simple_assignment_root_ident(target: &swc_ecma_ast::SimpleAssignTarget) -> Option<&str> {
    match target {
        swc_ecma_ast::SimpleAssignTarget::Ident(ident) => Some(ident.id.sym.as_ref()),
        swc_ecma_ast::SimpleAssignTarget::Member(member) => expr_root_ident(&member.obj),
        swc_ecma_ast::SimpleAssignTarget::Paren(paren) => expr_root_ident(&paren.expr),
        swc_ecma_ast::SimpleAssignTarget::TsAs(ts_as) => expr_root_ident(&ts_as.expr),
        swc_ecma_ast::SimpleAssignTarget::TsSatisfies(ts_satisfies) => {
            expr_root_ident(&ts_satisfies.expr)
        }
        swc_ecma_ast::SimpleAssignTarget::TsNonNull(non_null) => expr_root_ident(&non_null.expr),
        swc_ecma_ast::SimpleAssignTarget::TsTypeAssertion(assertion) => {
            expr_root_ident(&assertion.expr)
        }
        _ => None,
    }
}

const fn pat_assignment_root_ident(target: &swc_ecma_ast::AssignTargetPat) -> Option<&str> {
    match target {
        swc_ecma_ast::AssignTargetPat::Array(_)
        | swc_ecma_ast::AssignTargetPat::Object(_)
        | swc_ecma_ast::AssignTargetPat::Invalid(_) => None,
    }
}

fn expr_root_ident(expr: &Expr) -> Option<&str> {
    match strip_wrappers(expr) {
        Expr::Ident(ident) => Some(ident.sym.as_ref()),
        Expr::Member(member) => expr_root_ident(&member.obj),
        _ => None,
    }
}

fn const_binding<'a>(state: &'a AnalysisState, name: &str) -> Option<&'a Expr> {
    (!state.mutated_bindings.contains(name))
        .then(|| state.const_bindings.get(name))
        .flatten()
}

fn mark_binding_mutated(state: &mut AnalysisState, name: &str) {
    let _ = state.mutated_bindings.insert(name.to_owned());
    for alias in binding_aliases(state, name) {
        let _ = state.mutated_bindings.insert(alias);
    }
}

fn mark_all_static_bindings_mutated(state: &mut AnalysisState) {
    state
        .mutated_bindings
        .extend(state.const_bindings.keys().cloned());
}

fn binding_aliases(state: &AnalysisState, name: &str) -> BTreeSet<String> {
    let mut aliases = BTreeSet::new();
    collect_binding_aliases(state, name, &mut aliases);
    aliases
}

fn collect_binding_aliases(state: &AnalysisState, name: &str, aliases: &mut BTreeSet<String>) {
    let Some(direct_aliases) = state.const_aliases.get(name) else {
        return;
    };
    for alias in direct_aliases {
        if aliases.insert(alias.clone()) {
            collect_binding_aliases(state, alias, aliases);
        }
    }
}

fn assignment_exports_target(assign_expr: &AssignExpr) -> bool {
    let left = match &assign_expr.left {
        swc_ecma_ast::AssignTarget::Simple(simple) => simple,
        swc_ecma_ast::AssignTarget::Pat(_) => return false,
    };
    let swc_ecma_ast::SimpleAssignTarget::Member(member) = left else {
        return false;
    };

    let object_ident = match &*member.obj {
        Expr::Ident(ident) => ident.sym.as_ref(),
        _ => return false,
    };
    let property_name = match &member.prop {
        MemberProp::Ident(ident) => ident.sym.as_ref(),
        _ => return false,
    };

    (object_ident == "module" && property_name == "exports")
        || (object_ident == "exports" && property_name == "default")
}

fn resolve_to_config_expr<'a>(
    expr: &'a Expr,
    state: &'a AnalysisState,
    depth: usize,
) -> Option<&'a Expr> {
    if depth > 16 {
        return None;
    }
    match strip_wrappers(expr) {
        Expr::Object(_) => Some(strip_wrappers(expr)),
        Expr::Ident(ident) => const_binding(state, ident.sym.as_ref())
            .and_then(|binding| resolve_to_config_expr(binding, state, depth + 1)),
        Expr::Call(call_expr) => {
            let callee = match &call_expr.callee {
                Callee::Expr(callee_expr) => strip_wrappers(callee_expr),
                _ => return None,
            };
            match callee {
                Expr::Ident(ident)
                    if state
                        .import_bindings
                        .get(ident.sym.as_ref())
                        .is_some_and(|binding| binding.source_module == "astro/config") =>
                {
                    let first_arg = call_expr.args.first()?;
                    resolve_callable_config_arg(&first_arg.expr, state, depth + 1)
                }
                _ => None,
            }
        }
        Expr::Arrow(arrow) => arrow_body_to_expr(arrow)
            .and_then(|body| resolve_to_config_expr(body, state, depth + 1)),
        Expr::Fn(function) => function
            .function
            .body
            .as_ref()
            .and_then(last_return_expr)
            .and_then(|body| resolve_to_config_expr(body, state, depth + 1)),
        _ => None,
    }
}

fn resolve_callable_config_arg<'a>(
    expr: &'a Expr,
    state: &'a AnalysisState,
    depth: usize,
) -> Option<&'a Expr> {
    match strip_wrappers(expr) {
        Expr::Arrow(arrow) => arrow_body_to_expr(arrow)
            .and_then(|body| resolve_to_config_expr(body, state, depth + 1)),
        Expr::Fn(function) => function
            .function
            .body
            .as_ref()
            .and_then(last_return_expr)
            .and_then(|body| resolve_to_config_expr(body, state, depth + 1)),
        other => resolve_to_config_expr(other, state, depth + 1),
    }
}

fn arrow_body_to_expr(arrow: &ArrowExpr) -> Option<&Expr> {
    match &*arrow.body {
        BlockStmtOrExpr::Expr(expr) => Some(expr),
        BlockStmtOrExpr::BlockStmt(block) => last_return_expr(block),
    }
}

fn last_return_expr(block: &BlockStmt) -> Option<&Expr> {
    block.stmts.iter().rev().find_map(|stmt| match stmt {
        Stmt::Return(ReturnStmt { arg: Some(arg), .. }) => Some(&**arg),
        _ => None,
    })
}

fn as_object<'a>(expr: &'a Expr, state: &'a AnalysisState, depth: usize) -> Option<&'a ObjectLit> {
    if depth > 16 {
        return None;
    }
    match strip_wrappers(expr) {
        Expr::Object(object) => Some(object),
        Expr::Ident(ident) => const_binding(state, ident.sym.as_ref())
            .and_then(|binding| as_object(binding, state, depth + 1)),
        _ => None,
    }
}

fn as_array<'a>(
    expr: &'a Expr,
    state: &'a AnalysisState,
    depth: usize,
) -> Option<&'a swc_ecma_ast::ArrayLit> {
    if depth > 16 {
        return None;
    }
    match strip_wrappers(expr) {
        Expr::Array(array) => Some(array),
        Expr::Ident(ident) => const_binding(state, ident.sym.as_ref())
            .and_then(|binding| as_array(binding, state, depth + 1)),
        _ => None,
    }
}

fn property_string(
    object: &ObjectLit,
    property_name: &str,
    state: &AnalysisState,
) -> Result<Option<String>, String> {
    let Some(expr) = find_property_value(object, property_name, state)? else {
        return Ok(None);
    };
    resolve_string_expr(expr, state, 0)
        .map(Some)
        .ok_or_else(|| {
            format!("Astro config property `{property_name}` must resolve to a string literal")
        })
}

fn property_output(
    object: &ObjectLit,
    state: &AnalysisState,
) -> Result<Option<AstroOutputMode>, String> {
    let Some(expr) = find_property_value(object, "output", state)? else {
        return Ok(None);
    };
    let value = resolve_string_expr(expr, state, 0).ok_or_else(|| {
        "Astro config property `output` must resolve to a string literal".to_owned()
    })?;
    match value.as_str() {
        "static" => Ok(Some(AstroOutputMode::Static)),
        "server" => Ok(Some(AstroOutputMode::Server)),
        _ => Err(format!(
            "Astro config property `output` must be `static` or `server`, got `{value}`"
        )),
    }
}

fn property_trailing_slash(
    object: &ObjectLit,
    state: &AnalysisState,
) -> Result<Option<AstroTrailingSlashPolicy>, String> {
    let Some(expr) = find_property_value(object, "trailingSlash", state)? else {
        return Ok(None);
    };
    let value = resolve_string_expr(expr, state, 0).ok_or_else(|| {
        "Astro config property `trailingSlash` must resolve to a string literal".to_owned()
    })?;
    match value.as_str() {
        "always" => Ok(Some(AstroTrailingSlashPolicy::Always)),
        "never" => Ok(Some(AstroTrailingSlashPolicy::Never)),
        "ignore" => Ok(Some(AstroTrailingSlashPolicy::Ignore)),
        _ => Err(format!(
            "Astro config property `trailingSlash` must be `always`, `never`, or `ignore`, got `{value}`"
        )),
    }
}

fn property_integrations(
    object: &ObjectLit,
    state: &AnalysisState,
) -> Result<Vec<AstroIntegrationSnapshot>, String> {
    let Some(expr) = find_property_value(object, "integrations", state)? else {
        return Ok(Vec::new());
    };
    let array = as_array(expr, state, 0).ok_or_else(|| {
        "Astro config property `integrations` must resolve to an array literal".to_owned()
    })?;

    let mut integrations = Vec::new();
    collect_integrations_from_array(array, state, &mut integrations, 0)?;
    integrations.sort_by(|left, right| {
        (&left.source_module, &left.name, &left.imported_name).cmp(&(
            &right.source_module,
            &right.name,
            &right.imported_name,
        ))
    });
    Ok(integrations)
}

fn collect_integrations_from_array(
    array: &swc_ecma_ast::ArrayLit,
    state: &AnalysisState,
    integrations: &mut Vec<AstroIntegrationSnapshot>,
    depth: usize,
) -> Result<(), String> {
    if depth > 16 {
        return Err(
            "Astro config property `integrations` exceeded the supported spread nesting depth"
                .to_owned(),
        );
    }

    for element in &array.elems {
        let Some(ExprOrSpread { spread, expr }) = element else {
            continue;
        };

        if spread.is_some() {
            let spread_array = as_array(expr, state, depth + 1).ok_or_else(|| {
                "Astro config property `integrations` spread elements must resolve to array literals"
                    .to_owned()
            })?;
            collect_integrations_from_array(spread_array, state, integrations, depth + 1)?;
            continue;
        }

        integrations.push(expr_to_plugin_snapshot(expr, state, depth + 1)?);
    }

    Ok(())
}

fn property_adapter(
    object: &ObjectLit,
    state: &AnalysisState,
) -> Result<Option<AstroAdapterSnapshot>, String> {
    let Some(expr) = find_property_value(object, "adapter", state)? else {
        return Ok(None);
    };
    let plugin = expr_to_plugin_snapshot(expr, state, 0)?;
    Ok(Some(AstroAdapterSnapshot {
        source_module: plugin.source_module,
        name: plugin.name,
        imported_name: plugin.imported_name,
        call: plugin.call,
    }))
}

fn expr_to_plugin_snapshot(
    expr: &Expr,
    state: &AnalysisState,
    depth: usize,
) -> Result<AstroIntegrationSnapshot, String> {
    if depth > 16 {
        return Ok(AstroIntegrationSnapshot {
            source_module: None,
            name: None,
            imported_name: None,
            call: None,
        });
    }
    match strip_wrappers(expr) {
        Expr::Ident(ident) => {
            if let Some(binding) = const_binding(state, ident.sym.as_ref()) {
                return expr_to_plugin_snapshot(binding, state, depth + 1);
            }
            Ok(AstroIntegrationSnapshot {
                source_module: None,
                name: Some(ident.sym.to_string()),
                imported_name: None,
                call: None,
            })
        }
        Expr::Call(call_expr) => {
            if let Some(source_module) = require_call_source_module(call_expr) {
                return Ok(AstroIntegrationSnapshot {
                    source_module: Some(source_module),
                    name: Some("require".to_owned()),
                    imported_name: Some("require".to_owned()),
                    call: Some(AstroCallSnapshot {
                        first_arg: static_call_first_arg(call_expr, state, depth + 1)?,
                    }),
                });
            }
            let mut snapshot = match &call_expr.callee {
                Callee::Expr(callee_expr) => {
                    called_expr_to_plugin_snapshot(callee_expr, state, depth + 1)?
                }
                _ => AstroIntegrationSnapshot {
                    source_module: None,
                    name: None,
                    imported_name: None,
                    call: None,
                },
            };
            snapshot.call = Some(AstroCallSnapshot {
                first_arg: static_call_first_arg(call_expr, state, depth + 1)?,
            });
            Ok(snapshot)
        }
        Expr::Member(member) => Ok(match &*member.obj {
            Expr::Ident(ident) => AstroIntegrationSnapshot {
                source_module: None,
                name: member_property_name(&member.prop).or_else(|| Some(ident.sym.to_string())),
                imported_name: None,
                call: None,
            },
            _ => AstroIntegrationSnapshot {
                source_module: None,
                name: member_property_name(&member.prop),
                imported_name: None,
                call: None,
            },
        }),
        Expr::Object(_) => Ok(AstroIntegrationSnapshot {
            source_module: None,
            name: Some("object".to_owned()),
            imported_name: None,
            call: None,
        }),
        other => Ok(AstroIntegrationSnapshot {
            source_module: None,
            name: Some(expr_kind_name(other).to_owned()),
            imported_name: None,
            call: None,
        }),
    }
}

fn called_expr_to_plugin_snapshot(
    expr: &Expr,
    state: &AnalysisState,
    depth: usize,
) -> Result<AstroIntegrationSnapshot, String> {
    if depth > 16 {
        return Ok(AstroIntegrationSnapshot {
            source_module: None,
            name: None,
            imported_name: None,
            call: None,
        });
    }

    match strip_wrappers(expr) {
        Expr::Ident(ident) => {
            if let Some(binding) = const_binding(state, ident.sym.as_ref()) {
                return called_expr_to_plugin_snapshot(binding, state, depth + 1);
            }
            let import_binding = state.import_bindings.get(ident.sym.as_ref());
            Ok(AstroIntegrationSnapshot {
                source_module: import_binding.map(|binding| binding.source_module.clone()),
                name: Some(ident.sym.to_string()),
                imported_name: import_binding.and_then(|binding| binding.imported_name.clone()),
                call: None,
            })
        }
        Expr::Member(member) => Ok(match &*member.obj {
            Expr::Ident(ident) => {
                let import_binding = state.import_bindings.get(ident.sym.as_ref());
                AstroIntegrationSnapshot {
                    source_module: import_binding.map(|binding| binding.source_module.clone()),
                    name: member_property_name(&member.prop)
                        .or_else(|| Some(ident.sym.to_string())),
                    imported_name: import_binding
                        .and_then(|binding| {
                            (binding.kind == ImportBindingKind::Namespace).then(|| "*".to_owned())
                        })
                        .or_else(|| member_property_name(&member.prop)),
                    call: None,
                }
            }
            _ => expr_to_plugin_snapshot(expr, state, depth + 1)?,
        }),
        _ => expr_to_plugin_snapshot(expr, state, depth + 1),
    }
}

fn static_call_first_arg(
    call_expr: &swc_ecma_ast::CallExpr,
    state: &AnalysisState,
    depth: usize,
) -> Result<Option<AstroStaticValue>, String> {
    let Some(first_arg) = call_expr.args.first() else {
        return Ok(None);
    };
    if first_arg.spread.is_some() {
        return Err("Astro integration call arguments must not use spread syntax".to_owned());
    }
    static_value(&first_arg.expr, state, depth).map(Some)
}

fn static_value(
    expr: &Expr,
    state: &AnalysisState,
    depth: usize,
) -> Result<AstroStaticValue, String> {
    if depth > 16 {
        return Ok(unsupported_static_value(
            "Astro config static value exceeded supported nesting depth",
        ));
    }

    match strip_wrappers(expr) {
        Expr::Lit(Lit::Bool(value)) => Ok(AstroStaticValue::Bool(value.value)),
        Expr::Lit(Lit::Num(value)) => Ok(AstroStaticValue::Number(value.value)),
        Expr::Lit(Lit::Str(value)) => Ok(AstroStaticValue::String(
            value.value.to_string_lossy().into_owned(),
        )),
        Expr::Lit(Lit::Null(_)) => Ok(AstroStaticValue::Null),
        Expr::Tpl(_) => Ok(resolve_string_expr(expr, state, depth + 1)
            .map(AstroStaticValue::String)
            .unwrap_or_else(|| {
                unsupported_static_value("Astro template value is not statically resolvable")
            })),
        Expr::Array(array) => {
            let mut values = Vec::new();
            for element in &array.elems {
                let Some(element) = element else {
                    return Err("Astro config static arrays must not contain holes".to_owned());
                };
                if element.spread.is_some() {
                    return Err(
                        "Astro config static arrays must not contain spread elements".to_owned(),
                    );
                }
                values.push(static_value(&element.expr, state, depth + 1)?);
            }
            Ok(AstroStaticValue::Array(values))
        }
        Expr::Object(object) => {
            let mut properties = Vec::new();
            let mut seen_keys = std::collections::BTreeSet::new();
            for property in &object.props {
                let PropOrSpread::Prop(property) = property else {
                    return Err(
                        "Astro config static objects must not contain spread properties".to_owned(),
                    );
                };
                match &**property {
                    Prop::KeyValue(KeyValueProp { key, value }) => {
                        let key = prop_name(key).ok_or_else(|| {
                            "Astro config static object keys must be static identifiers or strings"
                                .to_owned()
                        })?;
                        if !seen_keys.insert(key.to_owned()) {
                            return Err(format!(
                                "Astro config static object has duplicate `{key}` property"
                            ));
                        }
                        properties.push(AstroStaticObjectProperty {
                            key: key.to_owned(),
                            value: static_value(value, state, depth + 1)?,
                        });
                    }
                    Prop::Shorthand(ident) => {
                        let key = ident.sym.to_string();
                        if !seen_keys.insert(key.clone()) {
                            return Err(format!(
                                "Astro config static object has duplicate `{key}` property"
                            ));
                        }
                        properties.push(AstroStaticObjectProperty {
                            key,
                            value: static_identifier_value(ident.sym.as_ref(), state, depth + 1)?,
                        });
                    }
                    _ => {
                        return Err(
                            "Astro config static objects must contain only key-value or shorthand properties"
                                .to_owned(),
                        );
                    }
                }
            }
            Ok(AstroStaticValue::Object(properties))
        }
        Expr::Ident(ident) => static_identifier_value(ident.sym.as_ref(), state, depth + 1),
        Expr::Member(member) => static_member_value(member, state, depth + 1),
        _ => Ok(unsupported_static_value(
            "Astro integration options must resolve to static values",
        )),
    }
}

fn unsupported_static_value(reason: &str) -> AstroStaticValue {
    AstroStaticValue::UnsupportedExpression {
        reason: reason.to_owned(),
    }
}

fn static_member_value(
    member: &swc_ecma_ast::MemberExpr,
    state: &AnalysisState,
    depth: usize,
) -> Result<AstroStaticValue, String> {
    let property_name = static_member_property_name(&member.prop)?;
    let object = static_value(&member.obj, state, depth + 1)?;
    let AstroStaticValue::Object(properties) = object else {
        return Ok(unsupported_static_value(
            "Astro config static member object must resolve to an object",
        ));
    };

    Ok(properties
        .into_iter()
        .find(|property| property.key == property_name)
        .map(|property| property.value)
        .unwrap_or_else(|| {
            unsupported_static_value(&format!(
                "Astro config static member object has no `{property_name}` property"
            ))
        }))
}

fn static_identifier_value(
    name: &str,
    state: &AnalysisState,
    depth: usize,
) -> Result<AstroStaticValue, String> {
    if let Some(binding) = const_binding(state, name) {
        return static_value(binding, state, depth + 1);
    }

    if let Some(value) = state.imported_static_values.get(name) {
        return Ok(value.clone());
    }

    Ok(state
        .import_bindings
        .get(name)
        .map(|binding| AstroStaticValue::ImportedIdentifier {
            local_name: name.to_owned(),
            source_module: Some(binding.source_module.clone()),
            imported_name: binding.imported_name.clone(),
        })
        .unwrap_or_else(|| {
            unsupported_static_value(&format!(
                "Astro config static identifier `{name}` is unresolved"
            ))
        }))
}

fn resolve_string_expr(expr: &Expr, state: &AnalysisState, depth: usize) -> Option<String> {
    if depth > 16 {
        return None;
    }
    match strip_wrappers(expr) {
        Expr::Lit(Lit::Str(value)) => Some(value.value.to_string_lossy().into_owned()),
        Expr::Tpl(template) => resolve_template_string(template, state, depth + 1),
        Expr::Ident(ident) => const_binding(state, ident.sym.as_ref())
            .and_then(|binding| resolve_string_expr(binding, state, depth + 1))
            .or_else(
                || match state.imported_static_values.get(ident.sym.as_ref()) {
                    Some(AstroStaticValue::String(value)) => Some(value.clone()),
                    _ => None,
                },
            ),
        Expr::Member(_) => match static_value(expr, state, depth + 1).ok()? {
            AstroStaticValue::String(value) => Some(value),
            _ => None,
        },
        _ => None,
    }
}

fn resolve_template_string(
    template: &swc_ecma_ast::Tpl,
    state: &AnalysisState,
    depth: usize,
) -> Option<String> {
    if template.quasis.len() != template.exprs.len() + 1 {
        return None;
    }

    let mut value = String::new();
    for (index, quasi) in template.quasis.iter().enumerate() {
        value.push_str(quasi.raw.as_ref());
        if let Some(expr) = template.exprs.get(index) {
            value.push_str(&resolve_string_expr(expr, state, depth + 1)?);
        }
    }
    Some(value)
}

fn find_property_value<'a>(
    object: &'a ObjectLit,
    property_name: &str,
    state: &'a AnalysisState,
) -> Result<Option<&'a Expr>, String> {
    let mut found = None;

    for prop in &object.props {
        let PropOrSpread::Prop(prop) = prop else {
            return Err("Astro config object must not contain spread properties".to_owned());
        };

        match &**prop {
            Prop::KeyValue(KeyValueProp { key, value }) => {
                let Some(key) = prop_name(key) else {
                    return Err(
                        "Astro config object keys must be static identifiers or strings".to_owned(),
                    );
                };
                if key == property_name {
                    if found.is_some() {
                        return Err(format!(
                            "Astro config object has duplicate `{property_name}` property"
                        ));
                    }
                    found = Some(&**value);
                }
            }
            Prop::Shorthand(ident) if ident.sym == *property_name => {
                if found.is_some() {
                    return Err(format!(
                        "Astro config object has duplicate `{property_name}` property"
                    ));
                }
                found = Some(const_binding(state, ident.sym.as_ref()).ok_or_else(|| {
                    format!("Astro config shorthand `{property_name}` must resolve to an unmutated const binding")
                })?);
            }
            Prop::Shorthand(_) => {}
            _ => {
                return Err(
                    "Astro config object must contain only key-value or shorthand properties"
                        .to_owned(),
                );
            }
        }
    }

    Ok(found)
}

fn prop_name(name: &PropName) -> Option<&str> {
    match name {
        PropName::Ident(ident) => Some(ident.sym.as_ref()),
        PropName::Str(value) => value.value.as_str(),
        _ => None,
    }
}

fn member_property_name(prop: &MemberProp) -> Option<String> {
    match prop {
        MemberProp::Ident(ident) => Some(ident.sym.to_string()),
        MemberProp::PrivateName(private) => Some(private.name.to_string()),
        MemberProp::Computed(computed) => match &*computed.expr {
            Expr::Lit(Lit::Str(value)) => Some(value.value.to_string_lossy().into_owned()),
            _ => None,
        },
    }
}

fn static_member_property_name(prop: &MemberProp) -> Result<String, String> {
    match prop {
        MemberProp::Ident(ident) => Ok(ident.sym.to_string()),
        MemberProp::PrivateName(private) => Ok(private.name.to_string()),
        MemberProp::Computed(_) => {
            Err("Astro config static member properties must not be computed".to_owned())
        }
    }
}

fn module_export_name(name: &swc_ecma_ast::ModuleExportName) -> String {
    match name {
        swc_ecma_ast::ModuleExportName::Ident(ident) => ident.sym.to_string(),
        swc_ecma_ast::ModuleExportName::Str(value) => value.value.to_string_lossy().into_owned(),
    }
}

fn require_call_source_module(call_expr: &swc_ecma_ast::CallExpr) -> Option<String> {
    let Callee::Expr(callee_expr) = &call_expr.callee else {
        return None;
    };
    let Expr::Ident(ident) = &**callee_expr else {
        return None;
    };
    if ident.sym != *"require" {
        return None;
    }
    let first_arg = call_expr.args.first()?;
    let Expr::Lit(Lit::Str(value)) = &*first_arg.expr else {
        return None;
    };
    Some(value.value.to_string_lossy().into_owned())
}

fn strip_wrappers(mut expr: &Expr) -> &Expr {
    loop {
        expr = match expr {
            Expr::Paren(paren) => &paren.expr,
            Expr::TsAs(ts_as) => &ts_as.expr,
            Expr::TsSatisfies(ts_satisfies) => &ts_satisfies.expr,
            Expr::TsNonNull(non_null) => &non_null.expr,
            Expr::TsInstantiation(instantiation) => &instantiation.expr,
            Expr::TsConstAssertion(assertion) => &assertion.expr,
            _ => return expr,
        };
    }
}

const fn expr_kind_name(expr: &Expr) -> &'static str {
    match expr {
        Expr::Arrow(_) => "arrow",
        Expr::Fn(_) => "function",
        Expr::Object(_) => "object",
        Expr::Array(_) => "array",
        Expr::Call(_) => "call",
        Expr::Member(_) => "member",
        Expr::Lit(_) => "literal",
        Expr::Ident(_) => "identifier",
        _ => "expression",
    }
}
