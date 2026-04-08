use proc_macro2::Span;

pub(crate) fn span_line(span: Span) -> usize {
    span.start().line
}

pub(crate) fn path_to_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
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
    cfg_meta_mentions_test(&meta, true)
}

fn attr_is_direct_test(attr: &syn::Attribute) -> bool {
    if attr.path().is_ident("test") {
        return true;
    }
    let segments = &attr.path().segments;
    segments.len() == 2 && segments[0].ident == "tokio" && segments[1].ident == "test"
}

fn cfg_meta_mentions_test(meta: &syn::Meta, positive: bool) -> bool {
    match meta {
        syn::Meta::Path(path) => positive && path.is_ident("test"),
        syn::Meta::List(list) if list.path.is_ident("all") || list.path.is_ident("any") => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|items| {
                items
                    .iter()
                    .any(|item| cfg_meta_mentions_test(item, positive))
            }),
        syn::Meta::List(list) if list.path.is_ident("not") => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|items| {
                items
                    .iter()
                    .any(|item| cfg_meta_mentions_test(item, !positive))
            }),
        _ => false,
    }
}
