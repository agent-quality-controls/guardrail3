pub fn parse_rust_file(content: &str) -> Result<syn::File, syn::Error> {
    syn::parse_file(content.strip_prefix('\u{feff}').unwrap_or(content))
}

pub fn count_top_level_use_statements(ast: &syn::File) -> usize {
    ast.items
        .iter()
        .filter(|item| matches!(item, syn::Item::Use(_)))
        .count()
}
