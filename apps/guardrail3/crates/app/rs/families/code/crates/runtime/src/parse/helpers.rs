use proc_macro2::Span;
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

pub(crate) fn collect_allow_lints(attrs: &[syn::Attribute]) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("allow") {
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
        let line = span_end_line(attr.span());
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

pub(crate) fn collect_cfg_attr_deny_forbid_attrs(
    attrs: &[syn::Attribute],
    crate_level_inner: bool,
    out: &mut Vec<crate::parse::DenyForbidInfo>,
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
        let Some(_) = args.next() else {
            continue;
        };
        let line = span_end_line(attr.span());
        for meta in args {
            let syn::Meta::List(inner) = meta else {
                continue;
            };
            let level = if inner.path.is_ident("deny") {
                "deny"
            } else if inner.path.is_ident("forbid") {
                "forbid"
            } else {
                continue;
            };
            let Ok(paths) = inner.parse_args_with(
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
            line: span_end_line(attr.span()),
            path: path_lit.value(),
            via_cfg_attr: false,
        });
    }
}

pub(crate) fn collect_cfg_attr_path_attrs(
    attrs: &[syn::Attribute],
    out: &mut Vec<crate::parse::PathAttrInfo>,
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
        let Some(_) = args.next() else {
            continue;
        };
        let line = span_end_line(attr.span());
        for meta in args {
            let syn::Meta::NameValue(name_value) = meta else {
                continue;
            };
            if !name_value.path.is_ident("path") {
                continue;
            }
            let syn::Expr::Lit(expr_lit) = &name_value.value else {
                continue;
            };
            let syn::Lit::Str(path_lit) = &expr_lit.lit else {
                continue;
            };
            out.push(crate::parse::PathAttrInfo {
                line,
                path: path_lit.value(),
                via_cfg_attr: true,
            });
        }
    }
}
