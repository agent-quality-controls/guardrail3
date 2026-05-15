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

pub(crate) fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content))
}

/// Implements `count top level use imports`.
pub(crate) fn count_top_level_use_imports(source: &syn::File) -> usize {
    let facade_only = source.items.iter().all(|item| match item {
        syn::Item::Use(item_use) => matches!(item_use.vis, syn::Visibility::Public(_)),
        syn::Item::Mod(_) => true,
        _ => false,
    });
    source
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Use(item_use)
                if !facade_only || !matches!(item_use.vis, syn::Visibility::Public(_)) =>
            {
                Some(count_use_tree_imports(&item_use.tree))
            }
            _ => None,
        })
        .sum()
}

/// Implements `count use tree imports`.
fn count_use_tree_imports(tree: &syn::UseTree) -> usize {
    match tree {
        syn::UseTree::Path(path) => count_use_tree_imports(&path.tree),
        syn::UseTree::Group(group) => group.items.iter().map(count_use_tree_imports).sum(),
        syn::UseTree::Name(_) | syn::UseTree::Rename(_) | syn::UseTree::Glob(_) => 1,
    }
}
