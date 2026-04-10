use std::collections::{BTreeMap, BTreeSet};

use quote::ToTokens;

pub(crate) fn self_ty_name(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(type_path) => Some(
            type_path
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join("::"),
        ),
        _ => None,
    }
}

pub(crate) fn collect_use_aliases(
    tree: &syn::UseTree,
    boundary_derive_aliases: &mut BTreeSet<String>,
    deserialize_aliases: &mut BTreeSet<String>,
    validate_aliases: &mut BTreeSet<String>,
    query_as_aliases: &mut BTreeSet<String>,
    module_path_aliases: &mut BTreeMap<String, String>,
) {
    collect_use_aliases_with_prefix(
        tree,
        &[],
        boundary_derive_aliases,
        deserialize_aliases,
        validate_aliases,
        query_as_aliases,
        module_path_aliases,
    );
}

fn collect_use_aliases_with_prefix(
    tree: &syn::UseTree,
    prefix: &[String],
    boundary_derive_aliases: &mut BTreeSet<String>,
    deserialize_aliases: &mut BTreeSet<String>,
    validate_aliases: &mut BTreeSet<String>,
    query_as_aliases: &mut BTreeSet<String>,
    module_path_aliases: &mut BTreeMap<String, String>,
) {
    match tree {
        syn::UseTree::Path(path) => {
            let mut next_prefix = prefix.to_vec();
            next_prefix.push(path.ident.to_string());
            collect_use_aliases_with_prefix(
                &path.tree,
                &next_prefix,
                boundary_derive_aliases,
                deserialize_aliases,
                validate_aliases,
                query_as_aliases,
                module_path_aliases,
            );
        }
        syn::UseTree::Name(name) => {
            let ident = name.ident.to_string();
            register_canonical_alias(
                prefix,
                &ident,
                &ident,
                boundary_derive_aliases,
                deserialize_aliases,
                validate_aliases,
                query_as_aliases,
                module_path_aliases,
            );
        }
        syn::UseTree::Rename(rename) => {
            let target = rename.ident.to_string();
            let alias = rename.rename.to_string();
            register_canonical_alias(
                prefix,
                &target,
                &alias,
                boundary_derive_aliases,
                deserialize_aliases,
                validate_aliases,
                query_as_aliases,
                module_path_aliases,
            );
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_use_aliases_with_prefix(
                    item,
                    prefix,
                    boundary_derive_aliases,
                    deserialize_aliases,
                    validate_aliases,
                    query_as_aliases,
                    module_path_aliases,
                );
            }
        }
        _ => {}
    }
}

fn register_canonical_alias(
    prefix: &[String],
    target: &str,
    alias: &str,
    boundary_derive_aliases: &mut BTreeSet<String>,
    deserialize_aliases: &mut BTreeSet<String>,
    validate_aliases: &mut BTreeSet<String>,
    query_as_aliases: &mut BTreeSet<String>,
    module_path_aliases: &mut BTreeMap<String, String>,
) {
    let path = qualified_use_target(prefix, target);
    if path != alias {
        let _ = module_path_aliases.insert(alias.to_owned(), path.clone());
    }
    match path.as_str() {
        "serde::Deserialize" => {
            let _ = boundary_derive_aliases.insert(alias.to_owned());
            let _ = deserialize_aliases.insert(alias.to_owned());
        }
        "clap::Parser" | "clap::Args" | "sqlx::FromRow" => {
            let _ = boundary_derive_aliases.insert(alias.to_owned());
        }
        "garde::Validate" => {
            let _ = validate_aliases.insert(alias.to_owned());
        }
        "sqlx::query_as" | "sqlx::query_as_unchecked" => {
            let _ = query_as_aliases.insert(alias.to_owned());
        }
        _ => {}
    }
}

fn qualified_use_target(prefix: &[String], target: &str) -> String {
    if prefix.is_empty() {
        target.to_owned()
    } else {
        format!("{}::{target}", prefix.join("::"))
    }
}

pub(crate) fn is_deserialize_trait_path(
    path: &syn::Path,
    aliases: &BTreeSet<String>,
    module_path_aliases: &BTreeMap<String, String>,
) -> bool {
    is_canonical_or_aliased_path(path, aliases, module_path_aliases, "serde::Deserialize")
}

pub(crate) fn is_validate_trait_path(
    path: &syn::Path,
    aliases: &BTreeSet<String>,
    module_path_aliases: &BTreeMap<String, String>,
) -> bool {
    is_canonical_or_aliased_path(path, aliases, module_path_aliases, "garde::Validate")
}

fn is_canonical_or_aliased_path(
    path: &syn::Path,
    aliases: &BTreeSet<String>,
    module_path_aliases: &BTreeMap<String, String>,
    canonical: &str,
) -> bool {
    let normalized = normalized_path_idents(path, module_path_aliases).join("::");
    if normalized == canonical {
        return true;
    }
    path.segments.len() == 1
        && path
            .segments
            .last()
            .is_some_and(|segment| aliases.contains(&segment.ident.to_string()))
}

pub(crate) fn is_sqlx_query_as_macro_path(
    path: &syn::Path,
    module_path_aliases: &BTreeMap<String, String>,
) -> bool {
    matches!(
        normalized_path_idents(path, module_path_aliases)
            .join("::")
            .as_str(),
        "sqlx::query_as" | "sqlx::query_as_unchecked"
    )
}

pub(crate) fn resolve_path_string_aliases(
    path: &str,
    module_path_aliases: &BTreeMap<String, String>,
) -> String {
    let mut segments = path
        .trim_start_matches(':')
        .split("::")
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let Some(first) = segments.first().cloned() else {
        return path.trim_start_matches(':').to_owned();
    };
    let Some(target) = module_path_aliases.get(&first) else {
        return path.trim_start_matches(':').to_owned();
    };

    let mut resolved = target
        .split("::")
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    resolved.extend(segments.drain(1..));
    resolved.join("::")
}

pub(crate) fn path_to_string(path: &syn::Path) -> String {
    path.to_token_stream().to_string().replace(' ', "")
}

fn normalized_path_idents(
    path: &syn::Path,
    module_path_aliases: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut segments = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    let Some(first) = segments.first().cloned() else {
        return segments;
    };
    let Some(target) = module_path_aliases.get(&first) else {
        return segments;
    };
    let mut resolved = target
        .split("::")
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    resolved.extend(segments.drain(1..));
    resolved
}

pub(crate) fn token_stream_uses_ctx_variable(stream: proc_macro2::TokenStream) -> bool {
    let tokens: Vec<_> = stream.into_iter().collect();
    for (index, token) in tokens.iter().enumerate() {
        match token {
            proc_macro2::TokenTree::Ident(found) if found == "ctx" => {
                if !ctx_token_is_path_segment(&tokens, index) {
                    return true;
                }
            }
            proc_macro2::TokenTree::Group(group)
                if token_stream_uses_ctx_variable(group.stream()) =>
            {
                return true;
            }
            _ => {}
        }
    }
    false
}

fn ctx_token_is_path_segment(tokens: &[proc_macro2::TokenTree], index: usize) -> bool {
    preceding_double_colon(tokens, index)
        || following_double_colon(tokens, index)
        || matches!(
            tokens.get(index.saturating_sub(1)),
            Some(proc_macro2::TokenTree::Punct(punct)) if punct.as_char() == '.'
        )
}

fn preceding_double_colon(tokens: &[proc_macro2::TokenTree], index: usize) -> bool {
    matches!(
        (index.checked_sub(2).and_then(|i| tokens.get(i)), index.checked_sub(1).and_then(|i| tokens.get(i))),
        (
            Some(proc_macro2::TokenTree::Punct(first)),
            Some(proc_macro2::TokenTree::Punct(second))
        ) if first.as_char() == ':' && second.as_char() == ':'
    )
}

fn following_double_colon(tokens: &[proc_macro2::TokenTree], index: usize) -> bool {
    matches!(
        (tokens.get(index + 1), tokens.get(index + 2)),
        (
            Some(proc_macro2::TokenTree::Punct(first)),
            Some(proc_macro2::TokenTree::Punct(second))
        ) if first.as_char() == ':' && second.as_char() == ':'
    )
}
