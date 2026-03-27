use guardrail3_app_rs_ast::ast_helpers;
use proc_macro2::Span;
use syn::parse::Parser;
use syn::spanned::Spanned;
use syn::visit::Visit;

pub use ast_helpers::{CfgAttrAllowInfo, GardeSkipInfo, InlineModAllow};

pub fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content))
}

pub fn effective_non_comment_line_count(content: &str) -> usize {
    filter_non_comment_lines(content).len()
}

pub fn count_top_level_use_statements(ast: &syn::File) -> usize {
    ast.items
        .iter()
        .filter(|item| matches!(item, syn::Item::Use(_)))
        .count()
}

pub fn find_forbidden_macros(ast: &syn::File) -> Vec<(usize, String)> {
    let mut visitor = ForbiddenMacroVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_unwrap_expect(ast: &syn::File) -> Vec<(usize, String)> {
    let mut visitor = UnwrapExpectVisitor::default();
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_std_fs_import_lines(ast: &syn::File) -> Vec<usize> {
    let mut visitor = StdFsImportVisitor::default();
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_inline_std_fs_call_lines(ast: &syn::File) -> Vec<usize> {
    let mut visitor = InlineStdFsVisitor {
        out: Vec::new(),
        in_cfg_test: false,
        in_allowed_scope: false,
    };
    visitor.visit_file(ast);
    visitor.out
}

pub fn line_text<'a>(content: &'a str, line: usize) -> &'a str {
    content
        .lines()
        .nth(line.saturating_sub(1))
        .unwrap_or("")
        .trim()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LargeTypeItem {
    Struct {
        line: usize,
        name: String,
        field_count: usize,
    },
    Enum {
        line: usize,
        name: String,
        variant_count: usize,
    },
}

pub fn find_large_type_items(ast: &syn::File) -> Vec<LargeTypeItem> {
    let mut visitor = LargeTypeVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_crate_level_allows(ast: &syn::File) -> Vec<(usize, String)> {
    ast_helpers::find_crate_level_allows(ast)
}

pub fn find_inline_mod_allows(ast: &syn::File) -> Vec<InlineModAllow> {
    ast_helpers::find_inline_mod_allows(ast)
}

pub fn find_item_allows(ast: &syn::File) -> Vec<(usize, String)> {
    let mut visitor = ItemOnlyAllowVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_cfg_attr_allows(ast: &syn::File) -> Vec<CfgAttrAllowInfo> {
    let always_true = find_always_true_cfg_attr_allows(ast)
        .into_iter()
        .map(|info| (info.line, info.lint))
        .collect::<std::collections::BTreeSet<_>>();
    ast_helpers::find_cfg_attr_allows(ast)
        .into_iter()
        .map(|mut info| {
            if always_true.contains(&(info.line, info.lint.clone())) {
                info.is_always_true = true;
            }
            info
        })
        .collect()
}

pub fn find_garde_skips_with_types(ast: &syn::File) -> Vec<GardeSkipInfo> {
    ast_helpers::find_garde_skips_with_types(ast)
}

pub fn same_line_reason(content: &str, line: usize) -> Option<String> {
    content
        .lines()
        .nth(line.saturating_sub(1))
        .and_then(|source_line| source_line.split("//").nth(1))
        .and_then(|comment| {
            let trimmed = comment.trim();
            let lower = trimmed.to_ascii_lowercase();
            if !lower.starts_with("reason:") {
                return None;
            }
            let reason = trimmed.get("reason:".len()..)?.trim();
            if reason.is_empty() {
                None
            } else {
                Some(reason.to_owned())
            }
        })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplAllowInfo {
    pub line: usize,
    pub lint: String,
    pub method_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenyForbidInfo {
    pub line: usize,
    pub lint: String,
    pub level: String,
    pub crate_level_inner: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeMacroInfo {
    pub line: usize,
    pub macro_name: String,
    pub build_script_pattern: bool,
    pub path_traversal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathAttrInfo {
    pub line: usize,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicResultErrorKind {
    String,
    BoxDynError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicResultErrorInfo {
    pub line: usize,
    pub fn_name: String,
    pub kind: PublicResultErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacadeBodyItemInfo {
    pub line: usize,
    pub kind: &'static str,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitMethodCountInfo {
    pub line: usize,
    pub trait_name: String,
    pub method_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignModAllowInfo {
    pub line: usize,
    pub lint: String,
    pub via_cfg_attr: bool,
}

pub fn find_impl_block_allows(ast: &syn::File) -> Vec<ImplAllowInfo> {
    let mut visitor = ImplAllowVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_always_true_cfg_attr_allows(ast: &syn::File) -> Vec<CfgAttrAllowInfo> {
    let mut out = Vec::new();
    collect_always_true_cfg_attr_allows(&ast.attrs, &mut out);
    let mut visitor = AlwaysTrueCfgAttrVisitor { out: &mut out };
    visitor.visit_file(ast);
    out
}

pub fn find_foreign_mod_allows(ast: &syn::File) -> Vec<ForeignModAllowInfo> {
    let mut visitor = ForeignModAllowVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_std_fs_glob_import_lines(ast: &syn::File) -> Vec<usize> {
    let mut visitor = StdFsGlobImportVisitor::default();
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_deny_forbid_attrs(ast: &syn::File) -> Vec<DenyForbidInfo> {
    let mut out = Vec::new();
    collect_deny_forbid_attrs(&ast.attrs, true, &mut out);
    let mut visitor = DenyForbidVisitor { out: &mut out };
    visitor.visit_file(ast);
    out
}

pub fn find_include_macros(ast: &syn::File) -> Vec<IncludeMacroInfo> {
    let mut visitor = IncludeMacroVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_path_attrs(ast: &syn::File) -> Vec<PathAttrInfo> {
    let mut out = Vec::new();
    collect_path_attrs(&ast.attrs, &mut out);
    let mut visitor = PathAttrVisitor { out: &mut out };
    visitor.visit_file(ast);
    out
}

pub fn find_public_result_error_types(ast: &syn::File) -> Vec<PublicResultErrorInfo> {
    let mut visitor = PublicResultErrorVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

pub fn find_pub_use_glob_reexports(ast: &syn::File) -> Vec<(usize, String)> {
    ast.items
        .iter()
        .filter_map(|item| {
            let syn::Item::Use(use_item) = item else {
                return None;
            };
            if !matches!(use_item.vis, syn::Visibility::Public(_)) {
                return None;
            }
            glob_reexport_target(&use_item.tree).map(|target| (span_line(use_item.span()), target))
        })
        .collect()
}

pub fn find_facade_body_items(ast: &syn::File) -> Vec<FacadeBodyItemInfo> {
    ast.items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Fn(item_fn) => Some(FacadeBodyItemInfo {
                line: span_line(item_fn.sig.ident.span()),
                kind: "function",
                name: item_fn.sig.ident.to_string(),
            }),
            syn::Item::Impl(item_impl) => Some(FacadeBodyItemInfo {
                line: span_line(item_impl.impl_token.span()),
                kind: "impl",
                name: "impl".to_owned(),
            }),
            syn::Item::Use(item_use) if !matches!(item_use.vis, syn::Visibility::Public(_)) => {
                Some(FacadeBodyItemInfo {
                    line: span_line(item_use.span()),
                    kind: "private use",
                    name: path_to_string_from_use_tree(&item_use.tree),
                })
            }
            syn::Item::ExternCrate(item) => Some(FacadeBodyItemInfo {
                line: span_line(item.span()),
                kind: "extern crate",
                name: item.ident.to_string(),
            }),
            syn::Item::Static(item) => Some(FacadeBodyItemInfo {
                line: span_line(item.ident.span()),
                kind: "static",
                name: item.ident.to_string(),
            }),
            syn::Item::ForeignMod(item) => Some(FacadeBodyItemInfo {
                line: span_line(item.abi.extern_token.span()),
                kind: "extern block",
                name: "extern".to_owned(),
            }),
            syn::Item::Macro(item) => Some(FacadeBodyItemInfo {
                line: span_line(item.mac.path.span()),
                kind: "macro item",
                name: path_to_string(&item.mac.path),
            }),
            _ => None,
        })
        .collect()
}

pub fn find_inline_public_modules(ast: &syn::File) -> Vec<(usize, String)> {
    ast.items
        .iter()
        .filter_map(|item| {
            let syn::Item::Mod(item_mod) = item else {
                return None;
            };
            if item_mod.content.is_none() || !matches!(item_mod.vis, syn::Visibility::Public(_)) {
                return None;
            }
            Some((span_line(item_mod.ident.span()), item_mod.ident.to_string()))
        })
        .collect()
}

pub fn find_large_traits(ast: &syn::File) -> Vec<TraitMethodCountInfo> {
    let mut visitor = LargeTraitVisitor { out: Vec::new() };
    visitor.visit_file(ast);
    visitor.out
}

type NumberedLine = (usize, String);

fn filter_non_comment_lines(content: &str) -> Vec<NumberedLine> {
    let mut result = Vec::new();
    let mut in_block_comment = false;

    #[allow(clippy::string_slice)]
    // reason: block comment parsing operates on known ASCII delimiters
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim().to_owned();
        let for_comment_check = strip_string_literals(&trimmed);

        if in_block_comment {
            if let Some(end_pos) = for_comment_check.find("*/") {
                let after = trimmed[end_pos.saturating_add(2)..].trim().to_owned();
                let after_for_check = strip_string_literals(&after);
                if after_for_check.contains("/*") {
                    in_block_comment = true;
                    if let Some(new_open) = after_for_check.find("/*") {
                        let before_new = after[..new_open].trim().to_owned();
                        if !before_new.is_empty() && !before_new.starts_with("//") {
                            result.push((line_num, before_new));
                        }
                    }
                } else {
                    in_block_comment = false;
                    if !after.is_empty() && !after.starts_with("//") {
                        result.push((line_num, after));
                    }
                }
            }
            continue;
        }

        let processed = strip_inline_block_comments(&trimmed);
        let processed_for_check = strip_string_literals(&processed);
        if let Some(open_pos) = processed_for_check.find("/*") {
            let before = processed[..open_pos].trim().to_owned();
            in_block_comment = true;
            if !before.is_empty() && !before.starts_with("//") {
                result.push((line_num, before));
            }
            continue;
        }

        let final_trimmed = processed.trim().to_owned();
        if final_trimmed.is_empty()
            || final_trimmed.starts_with("//")
            || final_trimmed.starts_with("///")
        {
            continue;
        }

        result.push((line_num, final_trimmed));
    }

    result
}

#[allow(clippy::string_slice)] // reason: inline comment stripping uses known ASCII delimiters
fn strip_inline_block_comments(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let mut remaining = line;

    loop {
        let remaining_for_check = strip_string_literals(remaining);
        match remaining_for_check.find("/*") {
            Some(start) => {
                result.push_str(&remaining[..start]);
                let check_rest = strip_string_literals(&remaining[start..]);
                match check_rest.find("*/") {
                    Some(end) => {
                        remaining = &remaining[start.saturating_add(end).saturating_add(2)..];
                    }
                    None => {
                        result.push_str(&remaining[start..]);
                        break;
                    }
                }
            }
            None => {
                result.push_str(remaining);
                break;
            }
        }
    }

    result
}

#[allow(clippy::indexing_slicing)] // reason: char indices are bounds-checked against len
#[allow(clippy::string_slice)] // reason: positions from str::find are valid UTF-8 boundaries
fn strip_string_literals(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars[i] == 'r' {
            let mut hashes = 0usize;
            let mut j = i.saturating_add(1);
            while j < len && chars[j] == '#' {
                hashes = hashes.saturating_add(1);
                j = j.saturating_add(1);
            }
            if j < len && chars[j] == '"' && (hashes > 0 || j == i.saturating_add(1)) {
                if i > 0 && (chars[i.saturating_sub(1)].is_alphanumeric() || chars[i - 1] == '_') {
                    result.push(chars[i]);
                    i = i.saturating_add(1);
                    continue;
                }
                let terminator = format!("\"{}", "#".repeat(hashes));
                let rest = &line[line
                    .char_indices()
                    .nth(j)
                    .map_or(line.len(), |(idx, _)| idx)..];
                if let Some(end) = rest.find(&terminator) {
                    i = j.saturating_add(rest[..end].chars().count() + terminator.chars().count());
                } else {
                    break;
                }
                continue;
            }
        }

        if chars[i] == '"' {
            i = i.saturating_add(1);
            let mut escaped = false;
            while i < len {
                let ch = chars[i];
                if escaped {
                    escaped = false;
                } else if ch == '\\' {
                    escaped = true;
                } else if ch == '"' {
                    i = i.saturating_add(1);
                    break;
                }
                i = i.saturating_add(1);
            }
            continue;
        }

        result.push(chars[i]);
        i = i.saturating_add(1);
    }

    result
}

fn span_line(span: Span) -> usize {
    span.start().line
}

fn span_end_line(span: Span) -> usize {
    span.end().line
}

fn trait_item_attrs(item: &syn::TraitItem) -> &[syn::Attribute] {
    match item {
        syn::TraitItem::Fn(f) => &f.attrs,
        syn::TraitItem::Type(t) => &t.attrs,
        syn::TraitItem::Const(c) => &c.attrs,
        _ => &[],
    }
}

fn collect_outer_allows(attrs: &[syn::Attribute], out: &mut Vec<(usize, String)>) {
    out.extend(
        attrs
            .iter()
            .filter(|attr| !matches!(attr.style, syn::AttrStyle::Inner(_)))
            .flat_map(|attr| collect_allow_lints(std::slice::from_ref(attr))),
    );
}

struct ItemOnlyAllowVisitor {
    out: Vec<(usize, String)>,
}

impl<'ast> Visit<'ast> for ItemOnlyAllowVisitor {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if !matches!(item, syn::Item::ForeignMod(_)) {
            collect_outer_allows(item_attrs(item), &mut self.out);
        }
        syn::visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        collect_outer_allows(impl_item_attrs(item), &mut self.out);
        syn::visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        collect_outer_allows(trait_item_attrs(item), &mut self.out);
        syn::visit::visit_trait_item(self, item);
    }
}

fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

fn is_cfg_test_attr(attr: &syn::Attribute) -> bool {
    if !attr.path().is_ident("cfg") {
        return false;
    }
    let syn::Meta::List(list) = &attr.meta else {
        return false;
    };
    let Ok(meta) = list.parse_args::<syn::Meta>() else {
        return false;
    };
    cfg_meta_requires_test(&meta)
}

fn cfg_meta_requires_test(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(path) => path.is_ident("test"),
        syn::Meta::List(list) if list.path.is_ident("all") => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|items| items.iter().any(cfg_meta_requires_test)),
        _ => false,
    }
}

fn attrs_have_allow_lint(attrs: &[syn::Attribute], lint_name: &str) -> bool {
    attrs.iter().any(|attr| attr_allows_lint(attr, lint_name))
}

fn attr_allows_lint(attr: &syn::Attribute, lint_name: &str) -> bool {
    if !attr.path().is_ident("allow") {
        return false;
    }
    let syn::Meta::List(list) = &attr.meta else {
        return false;
    };
    let Ok(paths) = list.parse_args_with(
        syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
    ) else {
        return false;
    };
    paths.iter().any(|path| {
        path.segments
            .iter()
            .next_back()
            .is_some_and(|segment| segment.ident == lint_name)
    })
}

fn item_attrs(item: &syn::Item) -> &[syn::Attribute] {
    match item {
        syn::Item::Const(item) => &item.attrs,
        syn::Item::Enum(item) => &item.attrs,
        syn::Item::ExternCrate(item) => &item.attrs,
        syn::Item::Fn(item) => &item.attrs,
        syn::Item::ForeignMod(item) => &item.attrs,
        syn::Item::Impl(item) => &item.attrs,
        syn::Item::Macro(item) => &item.attrs,
        syn::Item::Mod(item) => &item.attrs,
        syn::Item::Static(item) => &item.attrs,
        syn::Item::Struct(item) => &item.attrs,
        syn::Item::Trait(item) => &item.attrs,
        syn::Item::TraitAlias(item) => &item.attrs,
        syn::Item::Type(item) => &item.attrs,
        syn::Item::Union(item) => &item.attrs,
        syn::Item::Use(item) => &item.attrs,
        _ => &[],
    }
}

fn impl_item_attrs(item: &syn::ImplItem) -> &[syn::Attribute] {
    match item {
        syn::ImplItem::Const(item) => &item.attrs,
        syn::ImplItem::Fn(item) => &item.attrs,
        syn::ImplItem::Macro(item) => &item.attrs,
        syn::ImplItem::Type(item) => &item.attrs,
        _ => &[],
    }
}

fn collect_allow_lints(attrs: &[syn::Attribute]) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("allow") {
            let line = span_line(attr.span());
            let syn::Meta::List(list) = &attr.meta else {
                continue;
            };
            let Ok(paths) = list.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            ) else {
                continue;
            };
            for path in paths {
                out.push((line, path_to_string(&path)));
            }
        }
    }
    out
}

fn collect_cfg_attr_allow_lints(attrs: &[syn::Attribute]) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("cfg_attr") {
            continue;
        }
        let syn::Meta::List(list) = &attr.meta else {
            continue;
        };
        let Ok(args) = list.parse_args_with(
            syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
        ) else {
            continue;
        };
        let mut args = args.into_iter();
        let Some(_) = args.next() else {
            continue;
        };
        let line = span_line(attr.span());
        for meta in args {
            let syn::Meta::List(inner) = meta else {
                continue;
            };
            if !inner.path.is_ident("allow") {
                continue;
            }
            let Ok(paths) = inner.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            ) else {
                continue;
            };
            for path in paths {
                out.push((line, path_to_string(&path)));
            }
        }
    }
    out
}

fn collect_deny_forbid_attrs(
    attrs: &[syn::Attribute],
    crate_level_inner: bool,
    out: &mut Vec<DenyForbidInfo>,
) {
    for attr in attrs {
        let level = if attr.path().is_ident("deny") {
            "deny"
        } else if attr.path().is_ident("forbid") {
            "forbid"
        } else {
            continue;
        };
        let line = span_end_line(attr.span());
        let syn::Meta::List(list) = &attr.meta else {
            continue;
        };
        let Ok(paths) = list.parse_args_with(
            syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
        ) else {
            continue;
        };
        for path in paths {
            out.push(DenyForbidInfo {
                line,
                lint: path_to_string(&path),
                level: level.to_owned(),
                crate_level_inner: crate_level_inner
                    && matches!(attr.style, syn::AttrStyle::Inner(_)),
            });
        }
    }
}

fn collect_path_attrs(attrs: &[syn::Attribute], out: &mut Vec<PathAttrInfo>) {
    for attr in attrs {
        if !attr.path().is_ident("path") {
            continue;
        }
        let syn::Meta::NameValue(name_value) = &attr.meta else {
            continue;
        };
        let syn::Expr::Lit(expr_lit) = &name_value.value else {
            continue;
        };
        let syn::Lit::Str(path_lit) = &expr_lit.lit else {
            continue;
        };
        out.push(PathAttrInfo {
            line: span_line(attr.span()),
            path: path_lit.value(),
        });
    }
}

fn collect_always_true_cfg_attr_allows(attrs: &[syn::Attribute], out: &mut Vec<CfgAttrAllowInfo>) {
    for attr in attrs {
        if !attr.path().is_ident("cfg_attr") {
            continue;
        }
        let syn::Meta::List(list) = &attr.meta else {
            continue;
        };
        let Ok(args) = list.parse_args_with(
            syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
        ) else {
            continue;
        };
        let mut args = args.into_iter();
        let Some(condition) = args.next() else {
            continue;
        };
        if !meta_is_always_true(&condition) {
            continue;
        }
        let line = span_line(attr.span());
        for meta in args {
            let syn::Meta::List(inner) = meta else {
                continue;
            };
            if !inner.path.is_ident("allow") {
                continue;
            }
            let Ok(paths) = inner.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            ) else {
                continue;
            };
            for path in paths {
                out.push(CfgAttrAllowInfo {
                    line,
                    lint: path_to_string(&path),
                    is_always_true: true,
                });
            }
        }
    }
}

fn use_tree_matches_std_fs(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Path(path) => {
            if path.ident != "std" {
                return false;
            }
            match &*path.tree {
                syn::UseTree::Path(inner) => inner.ident == "fs",
                syn::UseTree::Name(name) => name.ident == "fs",
                syn::UseTree::Rename(rename) => rename.ident == "fs",
                syn::UseTree::Group(group) => group
                    .items
                    .iter()
                    .any(use_tree_matches_std_fs_with_std_prefix),
                _ => false,
            }
        }
        syn::UseTree::Group(group) => group.items.iter().any(use_tree_matches_std_fs),
        _ => false,
    }
}

fn use_tree_matches_std_fs_with_std_prefix(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Path(path) => path.ident == "fs",
        syn::UseTree::Name(name) => name.ident == "fs",
        syn::UseTree::Rename(rename) => rename.ident == "fs",
        _ => false,
    }
}

fn use_tree_is_std_fs_glob(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Path(std_path) if std_path.ident == "std" => match &*std_path.tree {
            syn::UseTree::Path(fs_path) if fs_path.ident == "fs" => {
                fs_subtree_contains_glob(&fs_path.tree)
            }
            syn::UseTree::Group(group) => group
                .items
                .iter()
                .any(use_tree_is_std_fs_glob_with_std_prefix),
            _ => false,
        },
        syn::UseTree::Group(group) => group.items.iter().any(use_tree_is_std_fs_glob),
        _ => false,
    }
}

fn use_tree_is_std_fs_glob_with_std_prefix(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Path(fs_path) if fs_path.ident == "fs" => {
            fs_subtree_contains_glob(&fs_path.tree)
        }
        _ => false,
    }
}

fn fs_subtree_contains_glob(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Glob(_) => true,
        syn::UseTree::Group(group) => group.items.iter().any(fs_subtree_contains_glob),
        _ => false,
    }
}

fn glob_reexport_target(tree: &syn::UseTree) -> Option<String> {
    match tree {
        syn::UseTree::Path(path) => {
            glob_reexport_target(&path.tree).map(|target| format!("{}::{target}", path.ident))
        }
        syn::UseTree::Glob(_) => Some("*".to_owned()),
        _ => None,
    }
}

fn meta_is_always_true(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(_) => false,
        syn::Meta::List(list) => {
            let name = path_to_string(&list.path);
            let nested: Vec<syn::Meta> = list
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )
                .map(|punctuated| punctuated.into_iter().collect())
                .unwrap_or_default();
            match name.as_str() {
                "all" => nested.iter().all(meta_is_always_true),
                "any" => {
                    if nested.is_empty() {
                        return false;
                    }
                    if nested.iter().any(meta_is_always_true) {
                        return true;
                    }
                    is_known_exhaustive_any(&nested)
                }
                "not" if nested.len() == 1 => meta_is_always_false(&nested[0]),
                _ => false,
            }
        }
        syn::Meta::NameValue(_) => false,
    }
}

fn meta_is_always_false(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(path) => path
            .get_ident()
            .is_some_and(|ident| is_cfg_ident_always_false(ident.to_string().as_str())),
        syn::Meta::List(list) => {
            let name = path_to_string(&list.path);
            let nested: Vec<syn::Meta> = list
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )
                .map(|punctuated| punctuated.into_iter().collect())
                .unwrap_or_default();
            match name.as_str() {
                "all" => nested.iter().any(meta_is_always_false),
                "any" => nested.iter().all(meta_is_always_false),
                "not" if nested.len() == 1 => meta_is_always_true(&nested[0]),
                _ => false,
            }
        }
        syn::Meta::NameValue(_) => false,
    }
}

fn is_cfg_ident_always_false(ident: &str) -> bool {
    !matches!(
        ident,
        "unix"
            | "windows"
            | "target_os"
            | "target_family"
            | "target_arch"
            | "target_env"
            | "target_vendor"
            | "target_pointer_width"
            | "target_endian"
            | "debug_assertions"
            | "test"
            | "feature"
            | "proc_macro"
            | "panic"
    )
}

fn is_known_exhaustive_any(nested: &[syn::Meta]) -> bool {
    let names: Vec<String> = nested
        .iter()
        .filter_map(|meta| match meta {
            syn::Meta::Path(path) => path.get_ident().map(ToString::to_string),
            _ => None,
        })
        .collect();
    names.iter().any(|name| name == "unix") && names.iter().any(|name| name == "windows")
}

fn macro_token_exprs(mac: &syn::Macro) -> Vec<syn::Expr> {
    if let Ok(expr) = syn::parse2::<syn::Expr>(mac.tokens.clone()) {
        return vec![expr];
    }

    syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
        .parse2(mac.tokens.clone())
        .map(|args| args.into_iter().collect())
        .unwrap_or_default()
}

fn expr_contains_out_dir(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Macro(expr_macro) => {
            let name = path_to_string(&expr_macro.mac.path);
            if name.ends_with("env") {
                return expr_macro.mac.tokens.to_string().contains("\"OUT_DIR\"");
            }
            if name.ends_with("concat") {
                return macro_token_exprs(&expr_macro.mac)
                    .iter()
                    .any(expr_contains_out_dir);
            }
            macro_token_exprs(&expr_macro.mac)
                .iter()
                .any(expr_contains_out_dir)
        }
        syn::Expr::Call(call) => {
            expr_contains_out_dir(&call.func) || call.args.iter().any(expr_contains_out_dir)
        }
        syn::Expr::Path(_) => false,
        _ => false,
    }
}

fn expr_has_path_traversal(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Str(value) => value.value().contains(".."),
            _ => false,
        },
        syn::Expr::Macro(expr_macro) => macro_token_exprs(&expr_macro.mac)
            .iter()
            .any(expr_has_path_traversal),
        syn::Expr::Call(call) => {
            expr_has_path_traversal(&call.func) || call.args.iter().any(expr_has_path_traversal)
        }
        syn::Expr::Array(array) => array.elems.iter().any(expr_has_path_traversal),
        _ => false,
    }
}

fn result_error_kind(ty: &syn::Type) -> Option<PublicResultErrorKind> {
    let syn::Type::Path(type_path) = ty else {
        return None;
    };
    let last = type_path.path.segments.iter().next_back()?;
    if last.ident != "Result" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &last.arguments else {
        return None;
    };
    let second = args.args.iter().nth(1)?;
    let syn::GenericArgument::Type(err_ty) = second else {
        return None;
    };
    if is_string_type(err_ty) {
        return Some(PublicResultErrorKind::String);
    }
    if is_box_dyn_error(err_ty) {
        return Some(PublicResultErrorKind::BoxDynError);
    }
    None
}

fn is_string_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(type_path) = ty else {
        return false;
    };
    type_path
        .path
        .segments
        .iter()
        .next_back()
        .is_some_and(|segment| segment.ident == "String")
}

fn is_box_dyn_error(ty: &syn::Type) -> bool {
    let syn::Type::Path(type_path) = ty else {
        return false;
    };
    let Some(last) = type_path.path.segments.iter().next_back() else {
        return false;
    };
    if last.ident != "Box" {
        return false;
    }
    let syn::PathArguments::AngleBracketed(args) = &last.arguments else {
        return false;
    };
    let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() else {
        return false;
    };
    let syn::Type::TraitObject(trait_object) = inner_ty else {
        return false;
    };
    trait_object.bounds.iter().any(|bound| match bound {
        syn::TypeParamBound::Trait(trait_bound) => trait_bound
            .path
            .segments
            .iter()
            .next_back()
            .is_some_and(|segment| segment.ident == "Error"),
        _ => false,
    })
}

struct ForbiddenMacroVisitor {
    out: Vec<(usize, String)>,
}

struct ImplAllowVisitor {
    out: Vec<ImplAllowInfo>,
}

struct DenyForbidVisitor<'a> {
    out: &'a mut Vec<DenyForbidInfo>,
}

struct ForeignModAllowVisitor {
    out: Vec<ForeignModAllowInfo>,
}

struct IncludeMacroVisitor {
    out: Vec<IncludeMacroInfo>,
}

struct AlwaysTrueCfgAttrVisitor<'a> {
    out: &'a mut Vec<CfgAttrAllowInfo>,
}

struct PathAttrVisitor<'a> {
    out: &'a mut Vec<PathAttrInfo>,
}

struct PublicResultErrorVisitor {
    out: Vec<PublicResultErrorInfo>,
}

impl<'ast> Visit<'ast> for ForbiddenMacroVisitor {
    fn visit_macro(&mut self, macro_call: &'ast syn::Macro) {
        let name = path_to_string(&macro_call.path);
        let base = name.rsplit("::").next().unwrap_or(&name);
        if matches!(base, "todo" | "unimplemented" | "unreachable" | "panic") {
            self.out.push((span_line(macro_call.path.span()), name));
        }
        syn::visit::visit_macro(self, macro_call);
    }
}

impl<'ast> Visit<'ast> for ImplAllowVisitor {
    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        let method_count = item_impl
            .items
            .iter()
            .filter(|item| matches!(item, syn::ImplItem::Fn(_)))
            .count();
        if method_count > 3 {
            for (line, lint) in collect_allow_lints(&item_impl.attrs) {
                self.out.push(ImplAllowInfo {
                    line,
                    lint,
                    method_count,
                });
            }
        }
        syn::visit::visit_item_impl(self, item_impl);
    }
}

impl<'ast> Visit<'ast> for DenyForbidVisitor<'ast> {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        collect_deny_forbid_attrs(item_attrs(item), false, self.out);
        syn::visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        collect_deny_forbid_attrs(impl_item_attrs(item), false, self.out);
        syn::visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        collect_deny_forbid_attrs(trait_item_attrs(item), false, self.out);
        syn::visit::visit_trait_item(self, item);
    }
}

impl<'ast> Visit<'ast> for ForeignModAllowVisitor {
    fn visit_item_foreign_mod(&mut self, item: &'ast syn::ItemForeignMod) {
        for (line, lint) in collect_allow_lints(&item.attrs) {
            self.out.push(ForeignModAllowInfo {
                line,
                lint,
                via_cfg_attr: false,
            });
        }
        for (line, lint) in collect_cfg_attr_allow_lints(&item.attrs) {
            self.out.push(ForeignModAllowInfo {
                line,
                lint,
                via_cfg_attr: true,
            });
        }
        syn::visit::visit_item_foreign_mod(self, item);
    }
}

impl<'ast> Visit<'ast> for IncludeMacroVisitor {
    fn visit_macro(&mut self, macro_call: &'ast syn::Macro) {
        let name = path_to_string(&macro_call.path);
        let base = name.rsplit("::").next().unwrap_or(&name);
        if matches!(base, "include" | "include_str" | "include_bytes") {
            let exprs = macro_token_exprs(macro_call);
            let build_script_pattern = exprs.iter().any(expr_contains_out_dir);
            let path_traversal = exprs.iter().any(expr_has_path_traversal);
            self.out.push(IncludeMacroInfo {
                line: span_line(macro_call.path.span()),
                macro_name: base.to_owned(),
                build_script_pattern,
                path_traversal,
            });
        }
        syn::visit::visit_macro(self, macro_call);
    }
}

fn path_to_string_from_use_tree(tree: &syn::UseTree) -> String {
    match tree {
        syn::UseTree::Path(path) => format!(
            "{}::{}",
            path.ident,
            path_to_string_from_use_tree(&path.tree)
        ),
        syn::UseTree::Name(name) => name.ident.to_string(),
        syn::UseTree::Rename(rename) => rename.ident.to_string(),
        syn::UseTree::Glob(_) => "*".to_owned(),
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .map(path_to_string_from_use_tree)
            .collect::<Vec<_>>()
            .join(", "),
    }
}

impl<'ast> Visit<'ast> for AlwaysTrueCfgAttrVisitor<'ast> {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if !matches!(item, syn::Item::ForeignMod(_)) {
            collect_always_true_cfg_attr_allows(item_attrs(item), self.out);
        }
        syn::visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        collect_always_true_cfg_attr_allows(impl_item_attrs(item), self.out);
        syn::visit::visit_impl_item(self, item);
    }
}

impl<'ast> Visit<'ast> for PathAttrVisitor<'ast> {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        collect_path_attrs(item_attrs(item), self.out);
        syn::visit::visit_item(self, item);
    }
}

impl<'ast> Visit<'ast> for PublicResultErrorVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        if !matches!(item_fn.vis, syn::Visibility::Public(_)) {
            return syn::visit::visit_item_fn(self, item_fn);
        }
        if let syn::ReturnType::Type(_, ty) = &item_fn.sig.output {
            if let Some(kind) = result_error_kind(ty) {
                self.out.push(PublicResultErrorInfo {
                    line: span_line(item_fn.sig.ident.span()),
                    fn_name: item_fn.sig.ident.to_string(),
                    kind,
                });
            }
        }
        syn::visit::visit_item_fn(self, item_fn);
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'ast syn::ImplItemFn) {
        if !matches!(item_fn.vis, syn::Visibility::Public(_)) {
            return syn::visit::visit_impl_item_fn(self, item_fn);
        }
        if let syn::ReturnType::Type(_, ty) = &item_fn.sig.output {
            if let Some(kind) = result_error_kind(ty) {
                self.out.push(PublicResultErrorInfo {
                    line: span_line(item_fn.sig.ident.span()),
                    fn_name: item_fn.sig.ident.to_string(),
                    kind,
                });
            }
        }
        syn::visit::visit_impl_item_fn(self, item_fn);
    }
}

#[derive(Default)]
struct UnwrapExpectVisitor {
    out: Vec<(usize, String)>,
    in_cfg_test: bool,
    unwrap_allowed: bool,
    expect_allowed: bool,
}

impl UnwrapExpectVisitor {
    fn save_and_apply(&mut self, attrs: &[syn::Attribute]) -> (bool, bool, bool) {
        let was = (self.in_cfg_test, self.unwrap_allowed, self.expect_allowed);
        self.in_cfg_test |= attrs.iter().any(is_cfg_test_attr);
        self.unwrap_allowed |= attrs_have_allow_lint(attrs, "unwrap_used");
        self.expect_allowed |= attrs_have_allow_lint(attrs, "expect_used");
        was
    }

    fn restore(&mut self, was: (bool, bool, bool)) {
        self.in_cfg_test = was.0;
        self.unwrap_allowed = was.1;
        self.expect_allowed = was.2;
    }
}

impl<'ast> Visit<'ast> for UnwrapExpectVisitor {
    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let was = self.save_and_apply(&item_fn.attrs);
        syn::visit::visit_item_fn(self, item_fn);
        self.restore(was);
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'ast syn::ImplItemFn) {
        let was = self.save_and_apply(&item_fn.attrs);
        syn::visit::visit_impl_item_fn(self, item_fn);
        self.restore(was);
    }

    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let was = self.save_and_apply(&item_mod.attrs);
        syn::visit::visit_item_mod(self, item_mod);
        self.restore(was);
    }

    fn visit_local(&mut self, local: &'ast syn::Local) {
        let was = self.save_and_apply(&local.attrs);
        syn::visit::visit_local(self, local);
        self.restore(was);
    }

    fn visit_expr_method_call(&mut self, method_call: &'ast syn::ExprMethodCall) {
        let method = method_call.method.to_string();
        let skip = match method.as_str() {
            "unwrap" => self.in_cfg_test || self.unwrap_allowed,
            "expect" => self.in_cfg_test || self.expect_allowed,
            _ => true,
        };
        if !skip {
            self.out
                .push((span_line(method_call.method.span()), method));
        }
        syn::visit::visit_expr_method_call(self, method_call);
    }
}

struct InlineStdFsVisitor {
    out: Vec<usize>,
    in_cfg_test: bool,
    in_allowed_scope: bool,
}

#[derive(Default)]
struct StdFsGlobImportVisitor {
    out: Vec<usize>,
    in_cfg_test: bool,
}

#[derive(Default)]
struct StdFsImportVisitor {
    out: Vec<usize>,
    in_cfg_test: bool,
}

struct LargeTypeVisitor {
    out: Vec<LargeTypeItem>,
}

struct LargeTraitVisitor {
    out: Vec<TraitMethodCountInfo>,
}

impl<'ast> Visit<'ast> for LargeTypeVisitor {
    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        let field_count = match &item_struct.fields {
            syn::Fields::Named(fields) => fields.named.len(),
            syn::Fields::Unnamed(fields) => fields.unnamed.len(),
            syn::Fields::Unit => 0,
        };
        if field_count > 15 {
            self.out.push(LargeTypeItem::Struct {
                line: span_line(item_struct.ident.span()),
                name: item_struct.ident.to_string(),
                field_count,
            });
        }
        syn::visit::visit_item_struct(self, item_struct);
    }

    fn visit_item_enum(&mut self, item_enum: &'ast syn::ItemEnum) {
        let variant_count = item_enum.variants.len();
        if variant_count > 20 {
            self.out.push(LargeTypeItem::Enum {
                line: span_line(item_enum.ident.span()),
                name: item_enum.ident.to_string(),
                variant_count,
            });
        }
        syn::visit::visit_item_enum(self, item_enum);
    }
}

impl<'ast> Visit<'ast> for LargeTraitVisitor {
    fn visit_item_trait(&mut self, item_trait: &'ast syn::ItemTrait) {
        let method_count = item_trait
            .items
            .iter()
            .filter(|item| matches!(item, syn::TraitItem::Fn(_)))
            .count();
        if method_count > 8 {
            self.out.push(TraitMethodCountInfo {
                line: span_line(item_trait.ident.span()),
                trait_name: item_trait.ident.to_string(),
                method_count,
            });
        }
        syn::visit::visit_item_trait(self, item_trait);
    }
}

impl InlineStdFsVisitor {
    #[allow(clippy::indexing_slicing)] // reason: guarded by len >= 3
    fn path_is_std_fs_call(path: &syn::Path) -> bool {
        let segments: Vec<_> = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();
        segments.len() >= 3 && segments[0] == "std" && segments[1] == "fs"
    }
}

impl<'ast> Visit<'ast> for InlineStdFsVisitor {
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let was = (self.in_cfg_test, self.in_allowed_scope);
        self.in_cfg_test |= item_mod.attrs.iter().any(is_cfg_test_attr);
        self.in_allowed_scope |= attrs_have_allow_lint(&item_mod.attrs, "disallowed_methods");
        syn::visit::visit_item_mod(self, item_mod);
        (self.in_cfg_test, self.in_allowed_scope) = was;
    }

    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let was = (self.in_cfg_test, self.in_allowed_scope);
        self.in_cfg_test |= item_fn.attrs.iter().any(is_cfg_test_attr);
        self.in_allowed_scope |= attrs_have_allow_lint(&item_fn.attrs, "disallowed_methods");
        syn::visit::visit_item_fn(self, item_fn);
        (self.in_cfg_test, self.in_allowed_scope) = was;
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'ast syn::ImplItemFn) {
        let was = (self.in_cfg_test, self.in_allowed_scope);
        self.in_cfg_test |= item_fn.attrs.iter().any(is_cfg_test_attr);
        self.in_allowed_scope |= attrs_have_allow_lint(&item_fn.attrs, "disallowed_methods");
        syn::visit::visit_impl_item_fn(self, item_fn);
        (self.in_cfg_test, self.in_allowed_scope) = was;
    }

    fn visit_local(&mut self, local: &'ast syn::Local) {
        let was = (self.in_cfg_test, self.in_allowed_scope);
        self.in_cfg_test |= local.attrs.iter().any(is_cfg_test_attr);
        self.in_allowed_scope |= attrs_have_allow_lint(&local.attrs, "disallowed_methods");
        syn::visit::visit_local(self, local);
        (self.in_cfg_test, self.in_allowed_scope) = was;
    }

    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if !self.in_cfg_test && !self.in_allowed_scope {
            if let syn::Expr::Path(expr_path) = &*expr_call.func {
                if Self::path_is_std_fs_call(&expr_path.path) {
                    self.out.push(span_line(expr_path.path.span()));
                }
            }
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
}

impl<'ast> Visit<'ast> for StdFsGlobImportVisitor {
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let was = self.in_cfg_test;
        self.in_cfg_test |= item_mod.attrs.iter().any(is_cfg_test_attr);
        syn::visit::visit_item_mod(self, item_mod);
        self.in_cfg_test = was;
    }

    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let was = self.in_cfg_test;
        self.in_cfg_test |= item_fn.attrs.iter().any(is_cfg_test_attr);
        syn::visit::visit_item_fn(self, item_fn);
        self.in_cfg_test = was;
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'ast syn::ImplItemFn) {
        let was = self.in_cfg_test;
        self.in_cfg_test |= item_fn.attrs.iter().any(is_cfg_test_attr);
        syn::visit::visit_impl_item_fn(self, item_fn);
        self.in_cfg_test = was;
    }

    fn visit_item_use(&mut self, use_item: &'ast syn::ItemUse) {
        if !self.in_cfg_test
            && !use_item.attrs.iter().any(is_cfg_test_attr)
            && use_tree_is_std_fs_glob(&use_item.tree)
        {
            self.out.push(span_line(use_item.span()));
        }
        syn::visit::visit_item_use(self, use_item);
    }
}

impl<'ast> Visit<'ast> for StdFsImportVisitor {
    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let was = self.in_cfg_test;
        self.in_cfg_test |= item_mod.attrs.iter().any(is_cfg_test_attr);
        syn::visit::visit_item_mod(self, item_mod);
        self.in_cfg_test = was;
    }

    fn visit_item_fn(&mut self, item_fn: &'ast syn::ItemFn) {
        let was = self.in_cfg_test;
        self.in_cfg_test |= item_fn.attrs.iter().any(is_cfg_test_attr);
        syn::visit::visit_item_fn(self, item_fn);
        self.in_cfg_test = was;
    }

    fn visit_impl_item_fn(&mut self, item_fn: &'ast syn::ImplItemFn) {
        let was = self.in_cfg_test;
        self.in_cfg_test |= item_fn.attrs.iter().any(is_cfg_test_attr);
        syn::visit::visit_impl_item_fn(self, item_fn);
        self.in_cfg_test = was;
    }

    fn visit_item_use(&mut self, use_item: &'ast syn::ItemUse) {
        if !self.in_cfg_test
            && !use_item.attrs.iter().any(is_cfg_test_attr)
            && use_tree_matches_std_fs(&use_item.tree)
        {
            self.out.push(span_line(use_item.span()));
        }
        syn::visit::visit_item_use(self, use_item);
    }
}
