use syn::parse::Parser;
use syn::spanned::Spanned;

use super::types::CfgPredicateTruth;

pub(crate) fn walk_cfg_attr_payloads(
    attr: &syn::Attribute,
    mut visit: impl FnMut(usize, CfgPredicateTruth, &syn::Meta),
) {
    if !attr.path().is_ident("cfg_attr") {
        return;
    }
    let syn::Meta::List(list) = &attr.meta else {
        return;
    };
    let Ok(args) = list.parse_args_with(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
    ) else {
        return;
    };
    let mut args = args.into_iter();
    let Some(condition) = args.next() else {
        return;
    };
    let line = super::helpers::span_end_line(attr.span());
    let truth = classify_cfg_predicate(&condition);
    for meta in args {
        walk_cfg_attr_meta(line, truth, &meta, &mut visit);
    }
}

fn walk_cfg_attr_meta(
    line: usize,
    inherited_truth: CfgPredicateTruth,
    meta: &syn::Meta,
    visit: &mut impl FnMut(usize, CfgPredicateTruth, &syn::Meta),
) {
    let syn::Meta::List(list) = meta else {
        visit(line, inherited_truth, meta);
        return;
    };
    if !list.path.is_ident("cfg_attr") {
        visit(line, inherited_truth, meta);
        return;
    }
    let Ok(args) = list.parse_args_with(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
    ) else {
        return;
    };
    let mut args = args.into_iter();
    let Some(condition) = args.next() else {
        return;
    };
    let truth = combine_cfg_truth(inherited_truth, classify_cfg_predicate(&condition));
    for nested in args {
        walk_cfg_attr_meta(line, truth, &nested, visit);
    }
}

pub(crate) fn classify_cfg_predicate(meta: &syn::Meta) -> CfgPredicateTruth {
    match meta {
        syn::Meta::Path(_) | syn::Meta::NameValue(_) => CfgPredicateTruth::Unknown,
        syn::Meta::List(list) => {
            let name = super::helpers::path_to_string(&list.path);
            let nested: Vec<syn::Meta> = list
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )
                .map(|punctuated| punctuated.into_iter().collect::<Vec<_>>())
                .unwrap_or_default();
            match name.as_str() {
                "all" => {
                    if nested.is_empty() {
                        CfgPredicateTruth::KnownTrue
                    } else {
                        fold_all_truths(nested.iter().map(classify_cfg_predicate))
                    }
                }
                "any" => {
                    if nested.is_empty() {
                        CfgPredicateTruth::KnownFalse
                    } else {
                        fold_any_truths(nested.iter().map(classify_cfg_predicate))
                    }
                }
                "not" if nested.len() == 1 => invert_cfg_truth(classify_cfg_predicate(&nested[0])),
                _ => CfgPredicateTruth::Unknown,
            }
        }
    }
}

fn fold_all_truths(truths: impl Iterator<Item = CfgPredicateTruth>) -> CfgPredicateTruth {
    let mut saw_unknown = false;
    for truth in truths {
        match truth {
            CfgPredicateTruth::KnownFalse => return CfgPredicateTruth::KnownFalse,
            CfgPredicateTruth::Unknown => saw_unknown = true,
            CfgPredicateTruth::KnownTrue => {}
        }
    }
    if saw_unknown {
        CfgPredicateTruth::Unknown
    } else {
        CfgPredicateTruth::KnownTrue
    }
}

fn fold_any_truths(truths: impl Iterator<Item = CfgPredicateTruth>) -> CfgPredicateTruth {
    let mut saw_unknown = false;
    for truth in truths {
        match truth {
            CfgPredicateTruth::KnownTrue => return CfgPredicateTruth::KnownTrue,
            CfgPredicateTruth::Unknown => saw_unknown = true,
            CfgPredicateTruth::KnownFalse => {}
        }
    }
    if saw_unknown {
        CfgPredicateTruth::Unknown
    } else {
        CfgPredicateTruth::KnownFalse
    }
}

fn invert_cfg_truth(truth: CfgPredicateTruth) -> CfgPredicateTruth {
    match truth {
        CfgPredicateTruth::KnownTrue => CfgPredicateTruth::KnownFalse,
        CfgPredicateTruth::KnownFalse => CfgPredicateTruth::KnownTrue,
        CfgPredicateTruth::Unknown => CfgPredicateTruth::Unknown,
    }
}

pub(crate) fn combine_cfg_truth(
    outer: CfgPredicateTruth,
    inner: CfgPredicateTruth,
) -> CfgPredicateTruth {
    match (outer, inner) {
        (CfgPredicateTruth::KnownFalse, _) | (_, CfgPredicateTruth::KnownFalse) => {
            CfgPredicateTruth::KnownFalse
        }
        (CfgPredicateTruth::KnownTrue, truth) | (truth, CfgPredicateTruth::KnownTrue) => truth,
        _ => CfgPredicateTruth::Unknown,
    }
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

pub(crate) fn expr_is_out_dir_concat(expr: &syn::Expr) -> bool {
    let syn::Expr::Macro(expr_macro) = expr else {
        return false;
    };
    let name = super::helpers::path_to_string(&expr_macro.mac.path);
    if !name.ends_with("concat") {
        return false;
    }
    let args = macro_token_exprs(&expr_macro.mac);
    if args.len() < 2 {
        return false;
    }
    expr_is_env_out_dir(&args[0])
}

fn expr_is_env_out_dir(expr: &syn::Expr) -> bool {
    let syn::Expr::Macro(expr_macro) = expr else {
        return false;
    };
    let name = super::helpers::path_to_string(&expr_macro.mac.path);
    if !name.ends_with("env") {
        return false;
    }
    expr_macro.mac.tokens.to_string().contains("\"OUT_DIR\"")
}

pub(crate) fn expr_has_path_traversal(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Str(value) => path_string_has_parent_segment(&value.value()),
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

fn path_string_has_parent_segment(path: &str) -> bool {
    path.split('/').any(|segment| segment == "..")
        || path.split('\\').any(|segment| segment == "..")
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
    if is_str_ref_type(err_ty) {
        return Some(crate::parse::PublicResultErrorKind::StrRef);
    }
    if is_anyhow_error_type(err_ty) {
        return Some(crate::parse::PublicResultErrorKind::AnyhowError);
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

fn is_str_ref_type(ty: &syn::Type) -> bool {
    let syn::Type::Reference(reference) = ty else {
        return false;
    };
    let syn::Type::Path(type_path) = &*reference.elem else {
        return false;
    };
    type_path
        .path
        .segments
        .iter()
        .next_back()
        .is_some_and(|segment| segment.ident == "str")
}

fn is_anyhow_error_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(type_path) = ty else {
        return false;
    };
    let mut segments = type_path
        .path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string());
    matches!(
        (
            segments.next().as_deref(),
            segments.next().as_deref(),
            segments.next()
        ),
        (Some("anyhow"), Some("Error"), None)
    )
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
