use syn::spanned::Spanned;
use syn::parse::Parser;

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
        let line = super::helpers::span_line(attr.span());
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
                    lint: super::helpers::path_to_string(&path),
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
            let name = super::helpers::path_to_string(&list.path);
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
            let name = super::helpers::path_to_string(&list.path);
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
            let name = super::helpers::path_to_string(&expr_macro.mac.path);
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
