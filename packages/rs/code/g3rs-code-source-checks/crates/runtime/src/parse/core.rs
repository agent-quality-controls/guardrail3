#[cfg(test)]
pub(crate) fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content))
}

pub(crate) fn count_top_level_use_imports(source: &syn::File) -> usize {
    source
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Use(item_use) => Some(count_use_tree_imports(&item_use.tree)),
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
