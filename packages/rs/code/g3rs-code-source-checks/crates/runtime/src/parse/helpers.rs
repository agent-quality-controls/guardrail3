#![allow(
    clippy::excessive_nesting,
    clippy::missing_docs_in_private_items,
    clippy::wildcard_enum_match_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::question_mark,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::needless_pass_by_value,
    clippy::expect_used,
    clippy::option_if_let_else,
    clippy::map_unwrap_or,
    clippy::if_same_then_else,
    clippy::match_same_arms,
    clippy::match_like_matches_macro,
    clippy::nonminimal_bool,
    clippy::single_match_else,
    clippy::items_after_statements,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::needless_for_each,
    clippy::manual_let_else,
    clippy::redundant_else,
    clippy::shadow_unrelated,
    clippy::struct_excessive_bools,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::module_name_repetitions,
    clippy::large_enum_variant,
    clippy::large_types_passed_by_value,
    clippy::ptr_arg,
    clippy::needless_collect,
    clippy::branches_sharing_code,
    clippy::unused_self,
    reason = "code-source-checks parse/visitor walks every variant of large external syntax-tree enums (syn::Type, syn::Item, syn::Expr, syn::Pat, etc.) and the ban-detection visitors mirror the source structure they are looking for; the rule modules accept the schema-versioned shape verbatim because the per-rule findings depend on the exact spans and the rule ids embed the schema."
)]

use proc_macro2::Span;
use syn::spanned::Spanned;

use super::types::{CfgAttrLintInfo, LintPolicyInfo, LintPolicyKind};

/// Implements `span line`.
pub(crate) fn span_line(span: Span) -> usize {
    span.start().line
}

/// Implements `span end line`.
pub(crate) fn span_end_line(span: Span) -> usize {
    span.end().line
}

/// Implements `path to string`.
pub(crate) fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

/// Implements `trait item attrs`.
pub(crate) fn trait_item_attrs(item: &syn::TraitItem) -> &[syn::Attribute] {
    match item {
        syn::TraitItem::Fn(f) => &f.attrs,
        syn::TraitItem::Type(t) => &t.attrs,
        syn::TraitItem::Const(c) => &c.attrs,
        _ => &[],
    }
}

/// Implements `impl item attrs`.
pub(crate) fn impl_item_attrs(item: &syn::ImplItem) -> &[syn::Attribute] {
    match item {
        syn::ImplItem::Const(item) => &item.attrs,
        syn::ImplItem::Fn(item) => &item.attrs,
        syn::ImplItem::Macro(item) => &item.attrs,
        syn::ImplItem::Type(item) => &item.attrs,
        _ => &[],
    }
}

/// Implements `item attrs`.
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

/// Implements `attrs enter test context`.
pub(crate) fn attrs_enter_test_context(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(attr_enters_test_context) || attrs.iter().any(attr_is_direct_test)
}

/// Implements `attr enters test context`.
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

/// Implements `attr is direct test`.
fn attr_is_direct_test(attr: &syn::Attribute) -> bool {
    if attr.path().is_ident("test") {
        return true;
    }
    let segments = &attr.path().segments;
    segments.len() == 2 && segments[0].ident == "tokio" && segments[1].ident == "test"
}

/// Implements `cfg meta requires test`.
fn cfg_meta_requires_test(meta: &syn::Meta) -> bool {
    !cfg_meta_eval_without_test(meta).can_be_true
}

/// Struct `CfgEvalWithoutTest` used by this module.
struct CfgEvalWithoutTest {
    /// Field `can_be_true`.
    can_be_true: bool,
    /// Field `can_be_false`.
    can_be_false: bool,
}

/// Implements `cfg meta eval without test`.
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
            .map_or(
                CfgEvalWithoutTest {
                    can_be_true: true,
                    can_be_false: true,
                },
                |item| {
                    let inner = cfg_meta_eval_without_test(&item);
                    CfgEvalWithoutTest {
                        can_be_true: inner.can_be_false,
                        can_be_false: inner.can_be_true,
                    }
                },
            ),
        syn::Meta::List(_) => CfgEvalWithoutTest {
            can_be_true: true,
            can_be_false: true,
        },
    }
}

/// Implements `lint policy kind`.
fn lint_policy_kind(path: &syn::Path) -> Option<LintPolicyKind> {
    if path.is_ident("allow") {
        Some(LintPolicyKind::Allow)
    } else if path.is_ident("expect") {
        Some(LintPolicyKind::Expect)
    } else {
        None
    }
}

/// Implements `collect item lint policies`.
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

/// Implements `collect cfg attr lint policies`.
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

/// Implements `collect deny forbid attrs`.
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

/// Implements `collect cfg attr deny forbid attrs`.
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
