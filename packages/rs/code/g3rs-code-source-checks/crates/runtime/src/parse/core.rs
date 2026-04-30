#[cfg(test)]
pub(crate) fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content))
}

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

fn count_use_tree_imports(tree: &syn::UseTree) -> usize {
    match tree {
        syn::UseTree::Path(path) => count_use_tree_imports(&path.tree),
        syn::UseTree::Group(group) => group.items.iter().map(count_use_tree_imports).sum(),
        syn::UseTree::Name(_) | syn::UseTree::Rename(_) | syn::UseTree::Glob(_) => 1,
    }
}
