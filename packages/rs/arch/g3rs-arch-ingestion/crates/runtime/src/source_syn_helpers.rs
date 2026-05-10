//! Small `syn` helpers shared by the source-tier ingestion routines.

use proc_macro2::Span;

/// Returns `true` when `visibility` is exactly `pub`.
pub(super) const fn is_pub(visibility: &syn::Visibility) -> bool {
    matches!(visibility, syn::Visibility::Public(_))
}

/// Returns the 1-based start line for `span`.
pub(super) fn span_line(span: Span) -> usize {
    span.start().line
}

/// Builds a human readable name for a `use` tree, recursing into path segments.
pub(super) fn use_tree_name(tree: &syn::UseTree) -> String {
    match tree {
        syn::UseTree::Path(path) => {
            format!("{}::{}", path.ident, use_tree_name(&path.tree))
        }
        syn::UseTree::Name(name) => name.ident.to_string(),
        syn::UseTree::Rename(rename) => rename.ident.to_string(),
        syn::UseTree::Glob(_) => "*".to_owned(),
        syn::UseTree::Group(_) => "{...}".to_owned(),
    }
}

/// Returns `true` when the `use` tree ends in a glob (`*`).
pub(super) fn is_broad_reexport(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Glob(_) => true,
        syn::UseTree::Path(path) => is_broad_reexport(&path.tree),
        syn::UseTree::Name(_) | syn::UseTree::Rename(_) | syn::UseTree::Group(_) => false,
    }
}

/// Extracts the `#[cfg(feature = "...")]` literal that gates `item`, if any.
pub(super) fn extract_feature_gate(item: &syn::Item) -> Option<String> {
    let attrs = match item {
        syn::Item::Mod(module) => &module.attrs,
        syn::Item::Use(item_use) => &item_use.attrs,
        syn::Item::Fn(item_fn) => &item_fn.attrs,
        syn::Item::Impl(item_impl) => &item_impl.attrs,
        syn::Item::ExternCrate(extern_crate) => &extern_crate.attrs,
        syn::Item::Static(item_static) => &item_static.attrs,
        syn::Item::ForeignMod(foreign_mod) => &foreign_mod.attrs,
        syn::Item::Macro(item_macro) => &item_macro.attrs,
        syn::Item::Const(_)
        | syn::Item::Enum(_)
        | syn::Item::Struct(_)
        | syn::Item::Trait(_)
        | syn::Item::TraitAlias(_)
        | syn::Item::Type(_)
        | syn::Item::Union(_)
        | syn::Item::Verbatim(_)
        | _ => return None,
    };

    for attr in attrs {
        if !attr.path().is_ident("cfg") {
            continue;
        }
        let Ok(expr) = attr.parse_args::<syn::Expr>() else {
            continue;
        };
        if let Some(feature) = extract_feature_expr(&expr) {
            return Some(feature);
        }
    }
    None
}

/// Walks one `cfg(...)` expression looking for a `feature = "..."` clause.
fn extract_feature_expr(expr: &syn::Expr) -> Option<String> {
    match expr {
        syn::Expr::Assign(assign) => {
            let syn::Expr::Path(path) = &*assign.left else {
                return None;
            };
            if path.path.is_ident("feature") {
                let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(value),
                    ..
                }) = &*assign.right
                else {
                    return None;
                };
                return Some(value.value());
            }
            None
        }
        syn::Expr::Call(call) => call.args.iter().find_map(extract_feature_expr),
        syn::Expr::Array(_)
        | syn::Expr::Async(_)
        | syn::Expr::Await(_)
        | syn::Expr::Binary(_)
        | syn::Expr::Block(_)
        | syn::Expr::Break(_)
        | syn::Expr::Cast(_)
        | syn::Expr::Closure(_)
        | syn::Expr::Const(_)
        | syn::Expr::Continue(_)
        | syn::Expr::Field(_)
        | syn::Expr::ForLoop(_)
        | syn::Expr::Group(_)
        | syn::Expr::If(_)
        | syn::Expr::Index(_)
        | syn::Expr::Infer(_)
        | syn::Expr::Let(_)
        | syn::Expr::Lit(_)
        | syn::Expr::Loop(_)
        | syn::Expr::Macro(_)
        | syn::Expr::Match(_)
        | syn::Expr::MethodCall(_)
        | syn::Expr::Paren(_)
        | syn::Expr::Path(_)
        | syn::Expr::Range(_)
        | syn::Expr::RawAddr(_)
        | syn::Expr::Reference(_)
        | syn::Expr::Repeat(_)
        | syn::Expr::Return(_)
        | syn::Expr::Struct(_)
        | syn::Expr::Try(_)
        | syn::Expr::TryBlock(_)
        | syn::Expr::Tuple(_)
        | syn::Expr::Unary(_)
        | syn::Expr::Unsafe(_)
        | syn::Expr::Verbatim(_)
        | syn::Expr::While(_)
        | syn::Expr::Yield(_)
        | _ => None,
    }
}

/// Extracts the literal value of a `#[path = "..."]` attribute.
pub(super) fn extract_path_attr_value(attr: &syn::Attribute) -> Option<String> {
    let syn::Meta::NameValue(name_value) = &attr.meta else {
        return None;
    };
    let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(value),
        ..
    }) = &name_value.value
    else {
        return None;
    };
    Some(value.value())
}

/// Returns `true` when `attr` is `#[cfg(test)]` (or any cfg whose truth implies `test`).
pub(super) fn attr_is_cfg_test(attr: &syn::Attribute) -> bool {
    if !attr.path().is_ident("cfg") {
        return false;
    }
    let syn::Meta::List(list) = &attr.meta else {
        return false;
    };
    let Ok(meta) = list.parse_args::<syn::Meta>() else {
        return false;
    };
    cfg_meta_is_test(&meta, true)
}

/// Recursively inspects a `cfg(...)` meta tree to decide whether `test` is satisfied.
fn cfg_meta_is_test(meta: &syn::Meta, positive: bool) -> bool {
    match meta {
        syn::Meta::Path(path) => positive && path.is_ident("test"),
        syn::Meta::List(list) if list.path.is_ident("all") || list.path.is_ident("any") => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|items| {
                items
                    .iter()
                    .any(|nested| cfg_meta_is_test(nested, positive))
            }),
        syn::Meta::List(list) if list.path.is_ident("not") => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|items| {
                items
                    .iter()
                    .any(|nested| cfg_meta_is_test(nested, !positive))
            }),
        syn::Meta::List(_) | syn::Meta::NameValue(_) => false,
    }
}
