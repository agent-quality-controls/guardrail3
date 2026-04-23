use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use astro_config_parser_types::document::{
    AstroAdapterSnapshot, AstroConfigDocument, AstroConfigFileKind, AstroConfigParseState,
    AstroConfigSelectedFile, AstroConfigSnapshot, AstroIntegrationSnapshot, AstroOutputMode,
};
use swc_common::{FileName, SourceMap, sync::Lrc};
use swc_ecma_ast::{
    ArrowExpr, AssignExpr, BindingIdent, BlockStmt, BlockStmtOrExpr, Callee, Decl, EsVersion,
    Expr, ExprOrSpread, KeyValueProp, Lit, MemberProp, Module, ModuleDecl,
    ModuleItem, ObjectLit, Pat, Prop, PropName, PropOrSpread, ReturnStmt, Script, Stmt,
    VarDeclarator,
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
    let source = std::fs::read_to_string(&abs_path)?;
    let selected_config = AstroConfigSelectedFile {
        rel_path: config_rel_path.to_owned(),
        kind: file_kind(config_rel_path)?,
    };
    let program = parse_program(&abs_path, &source, selected_config.kind)?;
    let raw = serde_json::json!({
        "selected_config": {
            "rel_path": selected_config.rel_path.clone(),
            "kind": selected_config.kind,
        }
    });
    let typed = match normalize_snapshot(&program, &selected_config) {
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

fn parse_program(
    abs_path: &Path,
    source: &str,
    kind: AstroConfigFileKind,
) -> Result<swc_ecma_ast::Program, crate::error::Error> {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Real(PathBuf::from(abs_path)).into(), source.to_owned());
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
) -> Result<AstroConfigSnapshot, String> {
    let mut state = AnalysisState::default();
    collect_state(program, &mut state);

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
        integrations: property_integrations(config_object, &state)?,
        adapter: property_adapter(config_object, &state)?,
    })
}

#[derive(Default)]
struct AnalysisState {
    import_sources: BTreeMap<String, String>,
    const_bindings: BTreeMap<String, Expr>,
    module_exports: Vec<Expr>,
}

fn collect_state(program: &swc_ecma_ast::Program, state: &mut AnalysisState) {
    match program {
        swc_ecma_ast::Program::Module(module) => collect_module_state(module, state),
        swc_ecma_ast::Program::Script(script) => collect_script_state(script, state),
    }
}

fn collect_module_state(module: &Module, state: &mut AnalysisState) {
    for item in &module.body {
        match item {
            ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) => {
                let source = import_decl.src.value.to_string_lossy().into_owned();
                for specifier in &import_decl.specifiers {
                    match specifier {
                        swc_ecma_ast::ImportSpecifier::Default(default_specifier) => {
                            let _ = state
                                .import_sources
                                .insert(default_specifier.local.sym.to_string(), source.clone());
                        }
                        swc_ecma_ast::ImportSpecifier::Named(named_specifier) => {
                            let _ = state
                                .import_sources
                                .insert(named_specifier.local.sym.to_string(), source.clone());
                        }
                        swc_ecma_ast::ImportSpecifier::Namespace(namespace_specifier) => {
                            let _ = state
                                .import_sources
                                .insert(namespace_specifier.local.sym.to_string(), source.clone());
                        }
                    }
                }
            }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(default_expr)) => {
                state.module_exports.push((*default_expr.expr).clone());
            }
            ModuleItem::Stmt(stmt) => collect_statement_state(stmt, state),
            _ => {}
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
        Stmt::Decl(Decl::Var(var_decl)) => {
            for declarator in &var_decl.decls {
                collect_var_declarator(declarator, state);
            }
        }
        Stmt::Expr(expr_stmt) => {
            if let Expr::Assign(assign_expr) = &*expr_stmt.expr {
                collect_assignment(assign_expr, state);
            }
        }
        _ => {}
    }
}

fn collect_var_declarator(declarator: &VarDeclarator, state: &mut AnalysisState) {
    let Some(init) = &declarator.init else {
        return;
    };
    let Pat::Ident(BindingIdent { id, .. }) = &declarator.name else {
        return;
    };
    let _ = state
        .const_bindings
        .insert(id.sym.to_string(), (**init).clone());
}

fn collect_assignment(assign_expr: &AssignExpr, state: &mut AnalysisState) {
    if assignment_exports_target(assign_expr) {
        state.module_exports.push((*assign_expr.right).clone());
    }
}

fn assignment_exports_target(assign_expr: &AssignExpr) -> bool {
    let left = match &assign_expr.left {
        swc_ecma_ast::AssignTarget::Simple(simple) => simple,
        swc_ecma_ast::AssignTarget::Pat(_) => return false,
    };
    let member = match left {
        swc_ecma_ast::SimpleAssignTarget::Member(member) => member,
        _ => return false,
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
        Expr::Ident(ident) => state
            .const_bindings
            .get(ident.sym.as_ref())
            .and_then(|binding| resolve_to_config_expr(binding, state, depth + 1)),
        Expr::Call(call_expr) => {
            let callee = match &call_expr.callee {
                Callee::Expr(callee_expr) => strip_wrappers(callee_expr),
                _ => return None,
            };
            match callee {
                Expr::Ident(ident)
                    if state
                        .import_sources
                        .get(ident.sym.as_ref())
                        .is_some_and(|source| source == "astro/config") =>
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
        Expr::Ident(ident) => state
            .const_bindings
            .get(ident.sym.as_ref())
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
        Expr::Ident(ident) => state
            .const_bindings
            .get(ident.sym.as_ref())
            .and_then(|binding| as_array(binding, state, depth + 1)),
        _ => None,
    }
}

fn property_string(
    object: &ObjectLit,
    property_name: &str,
    state: &AnalysisState,
) -> Result<Option<String>, String> {
    let Some(expr) = find_property_value(object, property_name, state) else {
        return Ok(None);
    };
    resolve_string_expr(expr, state, 0).map(Some).ok_or_else(|| {
        format!("Astro config property `{property_name}` must resolve to a string literal")
    })
}

fn property_output(
    object: &ObjectLit,
    state: &AnalysisState,
) -> Result<Option<AstroOutputMode>, String> {
    let Some(expr) = find_property_value(object, "output", state) else {
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

fn property_integrations(
    object: &ObjectLit,
    state: &AnalysisState,
) -> Result<Vec<AstroIntegrationSnapshot>, String> {
    let Some(expr) = find_property_value(object, "integrations", state) else {
        return Ok(Vec::new());
    };
    let array = as_array(expr, state, 0).ok_or_else(|| {
        "Astro config property `integrations` must resolve to an array literal".to_owned()
    })?;

    let mut integrations = Vec::new();
    collect_integrations_from_array(array, state, &mut integrations, 0)?;
    integrations.sort();
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

        integrations.push(expr_to_plugin_snapshot(expr, state, depth + 1));
    }

    Ok(())
}

fn property_adapter(
    object: &ObjectLit,
    state: &AnalysisState,
) -> Result<Option<AstroAdapterSnapshot>, String> {
    let Some(expr) = find_property_value(object, "adapter", state) else {
        return Ok(None);
    };
    let plugin = expr_to_plugin_snapshot(expr, state, 0);
    Ok(Some(AstroAdapterSnapshot {
        source_module: plugin.source_module,
        name: plugin.name,
    }))
}

fn expr_to_plugin_snapshot(
    expr: &Expr,
    state: &AnalysisState,
    depth: usize,
) -> AstroIntegrationSnapshot {
    if depth > 16 {
        return AstroIntegrationSnapshot {
            source_module: None,
            name: None,
        };
    }
    match strip_wrappers(expr) {
        Expr::Ident(ident) => {
            if let Some(binding) = state.const_bindings.get(ident.sym.as_ref()) {
                return expr_to_plugin_snapshot(binding, state, depth + 1);
            }
            AstroIntegrationSnapshot {
                source_module: None,
                name: Some(ident.sym.to_string()),
            }
        }
        Expr::Call(call_expr) => {
            if let Some(source_module) = require_call_source_module(call_expr) {
                return AstroIntegrationSnapshot {
                    source_module: Some(source_module),
                    name: Some("require".to_owned()),
                };
            }
            match &call_expr.callee {
                Callee::Expr(callee_expr) => {
                    called_expr_to_plugin_snapshot(callee_expr, state, depth + 1)
                }
                _ => AstroIntegrationSnapshot {
                    source_module: None,
                    name: None,
                },
            }
        }
        Expr::Member(member) => match &*member.obj {
            Expr::Ident(ident) => AstroIntegrationSnapshot {
                source_module: None,
                name: member_property_name(&member.prop).or_else(|| Some(ident.sym.to_string())),
            },
            _ => AstroIntegrationSnapshot {
                source_module: None,
                name: member_property_name(&member.prop),
            },
        },
        Expr::Object(_) => AstroIntegrationSnapshot {
            source_module: None,
            name: Some("object".to_owned()),
        },
        other => AstroIntegrationSnapshot {
            source_module: None,
            name: Some(expr_kind_name(other).to_owned()),
        },
    }
}

fn called_expr_to_plugin_snapshot(
    expr: &Expr,
    state: &AnalysisState,
    depth: usize,
) -> AstroIntegrationSnapshot {
    if depth > 16 {
        return AstroIntegrationSnapshot {
            source_module: None,
            name: None,
        };
    }

    match strip_wrappers(expr) {
        Expr::Ident(ident) => {
            if let Some(binding) = state.const_bindings.get(ident.sym.as_ref()) {
                return called_expr_to_plugin_snapshot(binding, state, depth + 1);
            }
            AstroIntegrationSnapshot {
                source_module: state.import_sources.get(ident.sym.as_ref()).cloned(),
                name: Some(ident.sym.to_string()),
            }
        }
        Expr::Member(member) => match &*member.obj {
            Expr::Ident(ident) => AstroIntegrationSnapshot {
                source_module: state.import_sources.get(ident.sym.as_ref()).cloned(),
                name: member_property_name(&member.prop).or_else(|| Some(ident.sym.to_string())),
            },
            _ => expr_to_plugin_snapshot(expr, state, depth + 1),
        },
        _ => expr_to_plugin_snapshot(expr, state, depth + 1),
    }
}

fn resolve_string_expr(expr: &Expr, state: &AnalysisState, depth: usize) -> Option<String> {
    if depth > 16 {
        return None;
    }
    match strip_wrappers(expr) {
        Expr::Lit(Lit::Str(value)) => Some(value.value.to_string_lossy().into_owned()),
        Expr::Tpl(template) if template.exprs.is_empty() && template.quasis.len() == 1 => {
            template
                .quasis
                .first()
                .map(|quasi| quasi.raw.to_string())
        }
        Expr::Ident(ident) => state
            .const_bindings
            .get(ident.sym.as_ref())
            .and_then(|binding| resolve_string_expr(binding, state, depth + 1)),
        _ => None,
    }
}

fn find_property_value<'a>(
    object: &'a ObjectLit,
    property_name: &str,
    state: &'a AnalysisState,
) -> Option<&'a Expr> {
    object.props.iter().find_map(|prop| match prop {
        PropOrSpread::Prop(prop) => match &**prop {
            Prop::KeyValue(KeyValueProp { key, value }) if prop_name(key) == Some(property_name) => {
                Some(&**value)
            }
            Prop::Shorthand(ident) if ident.sym == *property_name => {
                state.const_bindings.get(ident.sym.as_ref())
            }
            _ => None,
        },
        PropOrSpread::Spread(_) => None,
    })
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

fn expr_kind_name(expr: &Expr) -> &'static str {
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

#[cfg(test)]
#[path = "parser_tests/mod.rs"]
mod parser_tests;
