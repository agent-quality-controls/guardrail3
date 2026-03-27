use proc_macro2::Span;
use syn::parse::Parser;
use syn::spanned::Spanned;

pub(crate) fn span_line(span: Span) -> usize {
    span.start().line
}

pub(crate) fn span_end_line(span: Span) -> usize {
    span.end().line
}

pub(crate) fn trait_item_attrs(item: &syn::TraitItem) -> &[syn::Attribute] {
    match item {
        syn::TraitItem::Fn(f) => &f.attrs,
        syn::TraitItem::Type(t) => &t.attrs,
        syn::TraitItem::Const(c) => &c.attrs,
        _ => &[],
    }
}

pub(crate) fn impl_item_attrs(item: &syn::ImplItem) -> &[syn::Attribute] {
    match item {
        syn::ImplItem::Const(item) => &item.attrs,
        syn::ImplItem::Fn(item) => &item.attrs,
        syn::ImplItem::Macro(item) => &item.attrs,
        syn::ImplItem::Type(item) => &item.attrs,
        _ => &[],
    }
}

pub(crate) fn item_attrs(item: &syn::Item) -> &[syn::Attribute] {
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

pub(crate) fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

pub(crate) fn path_to_string_from_use_tree(tree: &syn::UseTree) -> String {
    match tree {
        syn::UseTree::Path(path) => {
            format!(
                "{}::{}",
                path.ident,
                path_to_string_from_use_tree(&path.tree)
            )
        }
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

pub(crate) fn is_cfg_test_attr(attr: &syn::Attribute) -> bool {
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

pub(crate) fn cfg_meta_requires_test(meta: &syn::Meta) -> bool {
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

pub(crate) fn attrs_have_allow_lint(attrs: &[syn::Attribute], lint_name: &str) -> bool {
    attrs.iter().any(|attr| attr_allows_lint(attr, lint_name))
}

pub(crate) fn attr_allows_lint(attr: &syn::Attribute, lint_name: &str) -> bool {
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

pub(crate) fn collect_allow_lints(attrs: &[syn::Attribute]) -> Vec<(usize, String)> {
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

pub(crate) fn collect_cfg_attr_allow_lints(attrs: &[syn::Attribute]) -> Vec<(usize, String)> {
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

pub(crate) fn collect_deny_forbid_attrs(
    attrs: &[syn::Attribute],
    crate_level_inner: bool,
    out: &mut Vec<crate::parse::DenyForbidInfo>,
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
            out.push(crate::parse::DenyForbidInfo {
                line,
                lint: path_to_string(&path),
                level: level.to_owned(),
                crate_level_inner: crate_level_inner
                    && matches!(attr.style, syn::AttrStyle::Inner(_)),
            });
        }
    }
}

pub(crate) fn collect_path_attrs(
    attrs: &[syn::Attribute],
    out: &mut Vec<crate::parse::PathAttrInfo>,
) {
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
        out.push(crate::parse::PathAttrInfo {
            line: span_line(attr.span()),
            path: path_lit.value(),
        });
    }
}

pub(crate) fn collect_always_true_cfg_attr_allows(
    attrs: &[syn::Attribute],
    out: &mut Vec<crate::parse::CfgAttrAllowInfo>,
) {
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
                out.push(crate::parse::CfgAttrAllowInfo {
                    line,
                    lint: path_to_string(&path),
                    is_always_true: true,
                });
            }
        }
    }
}

pub(crate) fn meta_is_always_true(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(_) => false,
        syn::Meta::List(list) => {
            let name = path_to_string(&list.path);
            let nested: Vec<syn::Meta> = list
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )
                .map(|punctuated| punctuated.into_iter().collect::<Vec<_>>())
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

pub(crate) fn meta_is_always_false(meta: &syn::Meta) -> bool {
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
                .map(|punctuated| punctuated.into_iter().collect::<Vec<_>>())
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

pub(crate) fn is_cfg_ident_always_false(ident: &str) -> bool {
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

pub(crate) fn is_known_exhaustive_any(nested: &[syn::Meta]) -> bool {
    let names: Vec<String> = nested
        .iter()
        .filter_map(|meta| match meta {
            syn::Meta::Path(path) => path.get_ident().map(ToString::to_string),
            _ => None,
        })
        .collect();
    names.iter().any(|name| name == "unix") && names.iter().any(|name| name == "windows")
}

pub(crate) fn macro_token_exprs(mac: &syn::Macro) -> Vec<syn::Expr> {
    if let Ok(expr) = syn::parse2::<syn::Expr>(mac.tokens.clone()) {
        return vec![expr];
    }

    syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated
        .parse2(mac.tokens.clone())
        .map(
            |args: syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>| {
                args.into_iter().collect::<Vec<_>>()
            },
        )
        .unwrap_or_default()
}

pub(crate) fn expr_contains_out_dir(expr: &syn::Expr) -> bool {
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

pub(crate) fn expr_has_path_traversal(expr: &syn::Expr) -> bool {
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

pub(crate) fn result_error_kind(ty: &syn::Type) -> Option<crate::parse::PublicResultErrorKind> {
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
        return Some(crate::parse::PublicResultErrorKind::String);
    }
    if is_box_dyn_error(err_ty) {
        return Some(crate::parse::PublicResultErrorKind::BoxDynError);
    }
    None
}

pub(crate) fn is_string_type(ty: &syn::Type) -> bool {
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

pub(crate) fn is_box_dyn_error(ty: &syn::Type) -> bool {
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
