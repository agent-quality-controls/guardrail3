use proc_macro2::Span;
use syn::spanned::Spanned;

use super::types::{CfgAttrLintInfo, LintPolicyInfo, LintPolicyKind};

pub(crate) fn span_line(span: Span) -> usize {
    span.start().line
}

pub(crate) fn span_end_line(span: Span) -> usize {
    span.end().line
}

pub(crate) fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
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

pub(crate) fn attrs_enter_test_context(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(attr_enters_test_context) || attrs.iter().any(attr_is_direct_test)
}

fn attr_enters_test_context(attr: &syn::Attribute) -> bool {
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

fn attr_is_direct_test(attr: &syn::Attribute) -> bool {
    if attr.path().is_ident("test") {
        return true;
    }
    let segments = &attr.path().segments;
    segments.len() == 2 && segments[0].ident == "tokio" && segments[1].ident == "test"
}

fn cfg_meta_requires_test(meta: &syn::Meta) -> bool {
    !cfg_meta_eval_without_test(meta).can_be_true
}

struct CfgEvalWithoutTest {
    can_be_true: bool,
    can_be_false: bool,
}

fn cfg_meta_eval_without_test(meta: &syn::Meta) -> CfgEvalWithoutTest {
    match meta {
        syn::Meta::Path(path) if path.is_ident("test") => CfgEvalWithoutTest {
            can_be_true: false,
            can_be_false: true,
        },
        syn::Meta::Path(_) | syn::Meta::NameValue(_) => CfgEvalWithoutTest {
            can_be_true: true,
            can_be_false: true,
        },
        syn::Meta::List(list) if list.path.is_ident("all") => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .map(|items| CfgEvalWithoutTest {
                can_be_true: items
                    .iter()
                    .all(|item| cfg_meta_eval_without_test(item).can_be_true),
                can_be_false: items
                    .iter()
                    .any(|item| cfg_meta_eval_without_test(item).can_be_false),
            })
            .unwrap_or(CfgEvalWithoutTest {
                can_be_true: true,
                can_be_false: true,
            }),
        syn::Meta::List(list) if list.path.is_ident("any") => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .map(|items| CfgEvalWithoutTest {
                can_be_true: items
                    .iter()
                    .any(|item| cfg_meta_eval_without_test(item).can_be_true),
                can_be_false: items
                    .iter()
                    .all(|item| cfg_meta_eval_without_test(item).can_be_false),
            })
            .unwrap_or(CfgEvalWithoutTest {
                can_be_true: true,
                can_be_false: true,
            }),
        syn::Meta::List(list) if list.path.is_ident("not") => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .ok()
            .and_then(|items| items.first().cloned())
            .map(|item| {
                let inner = cfg_meta_eval_without_test(&item);
                CfgEvalWithoutTest {
                    can_be_true: inner.can_be_false,
                    can_be_false: inner.can_be_true,
                }
            })
            .unwrap_or(CfgEvalWithoutTest {
                can_be_true: true,
                can_be_false: true,
            }),
        syn::Meta::List(_) => CfgEvalWithoutTest {
            can_be_true: true,
            can_be_false: true,
        },
    }
}

fn lint_policy_kind(path: &syn::Path) -> Option<LintPolicyKind> {
    if path.is_ident("allow") {
        Some(LintPolicyKind::Allow)
    } else if path.is_ident("expect") {
        Some(LintPolicyKind::Expect)
    } else {
        None
    }
}

pub(crate) fn collect_item_lint_policies(attrs: &[syn::Attribute]) -> Vec<LintPolicyInfo> {
    let mut out = Vec::new();
    for attr in attrs {
        let Some(kind) = lint_policy_kind(attr.path()) else {
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
            out.push(LintPolicyInfo {
                line,
                lint: path_to_string(&path),
                kind,
            });
        }
    }
    out
}

pub(crate) fn collect_cfg_attr_lint_policies(
    attrs: &[syn::Attribute],
    out: &mut Vec<CfgAttrLintInfo>,
) {
    for attr in attrs {
        if !attr.path().is_ident("cfg_attr") {
            continue;
        }
        super::analysis_helpers::walk_cfg_attr_payloads(attr, |line, truth, meta| {
            let syn::Meta::List(inner) = meta else {
                return;
            };
            let Some(kind) = lint_policy_kind(&inner.path) else {
                return;
            };
            let Ok(paths) = inner.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            ) else {
                return;
            };
            for path in paths {
                out.push(CfgAttrLintInfo {
                    line,
                    lint: path_to_string(&path),
                    kind,
                    truth,
                });
            }
        });
    }
}

pub(crate) fn collect_deny_forbid_attrs(
    attrs: &[syn::Attribute],
    crate_level_inner: bool,
    out: &mut Vec<super::types::DenyForbidInfo>,
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
            out.push(super::types::DenyForbidInfo {
                line,
                lint: path_to_string(&path),
                level: level.to_owned(),
                crate_level_inner: crate_level_inner
                    && matches!(attr.style, syn::AttrStyle::Inner(_)),
                cfg_truth: super::types::CfgPredicateTruth::KnownTrue,
            });
        }
    }
}

pub(crate) fn collect_cfg_attr_deny_forbid_attrs(
    attrs: &[syn::Attribute],
    crate_level_inner: bool,
    out: &mut Vec<super::types::DenyForbidInfo>,
) {
    for attr in attrs {
        if !attr.path().is_ident("cfg_attr") {
            continue;
        }
        super::analysis_helpers::walk_cfg_attr_payloads(attr, |line, truth, meta| {
            let syn::Meta::List(inner) = meta else {
                return;
            };
            let level = if inner.path.is_ident("deny") {
                "deny"
            } else if inner.path.is_ident("forbid") {
                "forbid"
            } else {
                return;
            };
            let Ok(paths) = inner.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            ) else {
                return;
            };
            for path in paths {
                out.push(super::types::DenyForbidInfo {
                    line,
                    lint: path_to_string(&path),
                    level: level.to_owned(),
                    crate_level_inner: crate_level_inner
                        && matches!(attr.style, syn::AttrStyle::Inner(_)),
                    cfg_truth: truth,
                });
            }
        });
    }
}
