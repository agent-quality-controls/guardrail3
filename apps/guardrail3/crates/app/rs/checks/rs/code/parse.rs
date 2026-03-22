use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::visit::Visit;

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
    ast.items
        .iter()
        .filter_map(|item| {
            let syn::Item::Use(use_item) = item else {
                return None;
            };
            if use_item.attrs.iter().any(is_cfg_test_attr) {
                return None;
            }
            if use_tree_matches_std_fs(&use_item.tree) {
                Some(span_line(use_item.span()))
            } else {
                None
            }
        })
        .collect()
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

type NumberedLine = (usize, String);

fn filter_non_comment_lines(content: &str) -> Vec<NumberedLine> {
    let mut result = Vec::new();
    let mut in_block_comment = false;

    #[allow(clippy::string_slice)] // reason: block comment parsing operates on known ASCII delimiters
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
                let rest = &line[line.char_indices().nth(j).map_or(line.len(), |(idx, _)| idx)..];
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
    list.tokens.to_string().replace(' ', "") == "test"
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
                _ => false,
            }
        }
        _ => false,
    }
}

struct ForbiddenMacroVisitor {
    out: Vec<(usize, String)>,
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

#[derive(Default)]
struct UnwrapExpectVisitor {
    out: Vec<(usize, String)>,
    unwrap_allowed: bool,
    expect_allowed: bool,
}

impl UnwrapExpectVisitor {
    fn save_and_apply(&mut self, attrs: &[syn::Attribute]) -> (bool, bool) {
        let was = (self.unwrap_allowed, self.expect_allowed);
        self.unwrap_allowed |= attrs_have_allow_lint(attrs, "unwrap_used");
        self.expect_allowed |= attrs_have_allow_lint(attrs, "expect_used");
        was
    }

    fn restore(&mut self, was: (bool, bool)) {
        self.unwrap_allowed = was.0;
        self.expect_allowed = was.1;
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
            "unwrap" => self.unwrap_allowed,
            "expect" => self.expect_allowed,
            _ => true,
        };
        if !skip {
            self.out.push((span_line(method_call.method.span()), method));
        }
        syn::visit::visit_expr_method_call(self, method_call);
    }
}

struct InlineStdFsVisitor {
    out: Vec<usize>,
    in_cfg_test: bool,
    in_allowed_scope: bool,
}

struct LargeTypeVisitor {
    out: Vec<LargeTypeItem>,
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

impl InlineStdFsVisitor {
    #[allow(clippy::indexing_slicing)] // reason: guarded by len >= 3
    fn path_is_std_fs_call(path: &syn::Path) -> bool {
        let segments: Vec<_> = path.segments.iter().map(|segment| segment.ident.to_string()).collect();
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
        let was = self.in_allowed_scope;
        self.in_allowed_scope |= attrs_have_allow_lint(&item_fn.attrs, "disallowed_methods");
        syn::visit::visit_impl_item_fn(self, item_fn);
        self.in_allowed_scope = was;
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

    fn visit_expr_path(&mut self, expr_path: &'ast syn::ExprPath) {
        if !self.in_cfg_test && !self.in_allowed_scope && Self::path_is_std_fs_call(&expr_path.path) {
            let line = span_line(expr_path.path.span());
            if !self.out.contains(&line) {
                self.out.push(line);
            }
        }
        syn::visit::visit_expr_path(self, expr_path);
    }
}
