use g3rs_apparch_types as apparch;

use super::error::G3RsApparchIngestionError;
use crate::view::CrawlView;

pub(crate) fn collect_public_use_items(
    view: &CrawlView<'_>,
    rel_path: &str,
    cargo_rel_path: &str,
    items: &[syn::Item],
    tree: &syn::UseTree,
    include_behavior_reexports: bool,
    public_items: &mut Vec<apparch::G3RsApparchPublicItem>,
) -> Result<(), G3RsApparchIngestionError> {
    collect_public_use_items_with_prefix(
        view,
        rel_path,
        cargo_rel_path,
        items,
        tree,
        &mut Vec::new(),
        include_behavior_reexports,
        public_items,
    )
}

fn collect_public_use_items_with_prefix(
    view: &CrawlView<'_>,
    rel_path: &str,
    cargo_rel_path: &str,
    items: &[syn::Item],
    tree: &syn::UseTree,
    prefix: &mut Vec<String>,
    include_behavior_reexports: bool,
    public_items: &mut Vec<apparch::G3RsApparchPublicItem>,
) -> Result<(), G3RsApparchIngestionError> {
    match tree {
        syn::UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_public_use_items_with_prefix(
                view,
                rel_path,
                cargo_rel_path,
                items,
                &path.tree,
                prefix,
                include_behavior_reexports,
                public_items,
            )?;
            let _ = prefix.pop();
        }
        syn::UseTree::Name(name) => {
            let mut segments = prefix.clone();
            segments.push(name.ident.to_string());
            collect_public_use_target(
                view,
                rel_path,
                cargo_rel_path,
                items,
                &segments,
                &name.ident.to_string(),
                include_behavior_reexports,
                public_items,
            )?;
        }
        syn::UseTree::Rename(rename) => {
            let mut segments = prefix.clone();
            segments.push(rename.ident.to_string());
            collect_public_use_target(
                view,
                rel_path,
                cargo_rel_path,
                items,
                &segments,
                &rename.rename.to_string(),
                include_behavior_reexports,
                public_items,
            )?;
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_public_use_items_with_prefix(
                    view,
                    rel_path,
                    cargo_rel_path,
                    items,
                    item,
                    prefix,
                    include_behavior_reexports,
                    public_items,
                )?;
            }
        }
        syn::UseTree::Glob(_) => {}
    }
    Ok(())
}

fn collect_public_use_target(
    view: &CrawlView<'_>,
    rel_path: &str,
    cargo_rel_path: &str,
    items: &[syn::Item],
    segments: &[String],
    exported_name: &str,
    include_behavior_reexports: bool,
    public_items: &mut Vec<apparch::G3RsApparchPublicItem>,
) -> Result<(), G3RsApparchIngestionError> {
    let Some((head, tail)) = segments.split_first() else {
        return Ok(());
    };

    if tail.is_empty() {
        for item in items {
            if is_cfg_test_only(item.attrs()) {
                continue;
            }
            match item {
                syn::Item::Trait(item_trait) if item_trait.ident == *head => {
                    push_public_item(
                        public_items,
                        apparch::G3RsApparchPublicItem {
                            cargo_rel_path: cargo_rel_path.to_owned(),
                            rel_path: rel_path.to_owned(),
                            item_name: exported_name.to_owned(),
                            owner_name: None,
                            kind: apparch::G3RsApparchPublicItemKind::Trait,
                        },
                    );
                    return Ok(());
                }
                syn::Item::Fn(item_fn)
                    if include_behavior_reexports
                        && matches!(item_fn.vis, syn::Visibility::Public(_))
                        && item_fn.sig.ident == *head =>
                {
                    push_public_item(
                        public_items,
                        apparch::G3RsApparchPublicItem {
                            cargo_rel_path: cargo_rel_path.to_owned(),
                            rel_path: rel_path.to_owned(),
                            item_name: exported_name.to_owned(),
                            owner_name: None,
                            kind: apparch::G3RsApparchPublicItemKind::FreeFunction,
                        },
                    );
                    return Ok(());
                }
                syn::Item::Impl(item_impl) if include_behavior_reexports => {
                    let owner_name = self_type_name(item_impl.self_ty.as_ref());
                    for impl_item in &item_impl.items {
                        let syn::ImplItem::Fn(method) = impl_item else {
                            continue;
                        };
                        if !matches!(method.vis, syn::Visibility::Public(_))
                            || method.sig.ident != *head
                        {
                            continue;
                        }
                        push_public_item(
                            public_items,
                            apparch::G3RsApparchPublicItem {
                                cargo_rel_path: cargo_rel_path.to_owned(),
                                rel_path: rel_path.to_owned(),
                                item_name: exported_name.to_owned(),
                                owner_name: owner_name.clone(),
                                kind: apparch::G3RsApparchPublicItemKind::InherentMethod,
                            },
                        );
                        return Ok(());
                    }
                }
                _ => {}
            }
        }
        return Ok(());
    }

    let Some(item_mod) = items.iter().find_map(|item| match item {
        syn::Item::Mod(item_mod) if item_mod.ident == *head => Some(item_mod),
        _ => None,
    }) else {
        return Ok(());
    };

    if let Some((_, nested_items)) = &item_mod.content {
        return collect_public_use_target(
            view,
            rel_path,
            cargo_rel_path,
            nested_items,
            tail,
            exported_name,
            include_behavior_reexports,
            public_items,
        );
    }

    let module_path = resolve_module_path(view, rel_path, item_mod)?;
    let Some(entry) = view.entry(&module_path) else {
        return Ok(());
    };
    if !entry.readable {
        return Ok(());
    }
    let content = view.read_file(&module_path).map_err(|error| {
        G3RsApparchIngestionError::Unreadable {
            path: entry.path.abs_path.clone(),
            reason: error.to_string(),
        }
    })?;
    let parsed = syn::parse_file(&content).map_err(|error| G3RsApparchIngestionError::ParseFailed {
        path: entry.path.abs_path.clone(),
        reason: error.to_string(),
    })?;
    if is_cfg_test_only(&parsed.attrs) {
        return Ok(());
    }
    collect_public_use_target(
        view,
        &module_path,
        cargo_rel_path,
        &parsed.items,
        tail,
        exported_name,
        include_behavior_reexports,
        public_items,
    )
}

pub(crate) fn push_public_item(
    public_items: &mut Vec<apparch::G3RsApparchPublicItem>,
    item: apparch::G3RsApparchPublicItem,
) {
    if !public_items.iter().any(|existing| existing == &item) {
        public_items.push(item);
    }
}

pub(crate) fn self_type_name(self_ty: &syn::Type) -> Option<String> {
    match self_ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        _ => None,
    }
}

pub(crate) fn resolve_module_path(
    view: &CrawlView<'_>,
    rel_path: &str,
    item_mod: &syn::ItemMod,
) -> Result<String, G3RsApparchIngestionError> {
    let file_dir = rel_path.rsplit_once('/').map_or("", |(dir, _)| dir);
    if let Some(path_attr) = module_path_attr(item_mod) {
        let candidate = CrawlView::join_rel(file_dir, &path_attr);
        return view.file_exists(&candidate).then_some(candidate).ok_or_else(|| {
            G3RsApparchIngestionError::NormalizationFailed {
                path: std::path::PathBuf::from(rel_path),
                reason: format!(
                    "declared module `{}` points to missing file `{}`",
                    item_mod.ident, path_attr
                ),
            }
        });
    }

    let module_name = item_mod.ident.to_string();
    let file_name = rel_path.rsplit('/').next().unwrap_or(rel_path);
    let stem = file_name.strip_suffix(".rs").unwrap_or(file_name);
    let module_dir = if matches!(stem, "lib" | "main" | "mod") {
        file_dir.to_owned()
    } else {
        CrawlView::join_rel(file_dir, stem)
    };

    let direct = CrawlView::join_rel(&module_dir, &format!("{module_name}.rs"));
    if view.file_exists(&direct) {
        return Ok(direct);
    }

    let nested = CrawlView::join_rel(&module_dir, &format!("{module_name}/mod.rs"));
    if view.file_exists(&nested) {
        return Ok(nested);
    }

    Err(G3RsApparchIngestionError::NormalizationFailed {
        path: std::path::PathBuf::from(rel_path),
        reason: format!(
            "declared module `{}` has no backing file under `{}`",
            item_mod.ident, file_dir
        ),
    })
}

fn module_path_attr(item_mod: &syn::ItemMod) -> Option<String> {
    item_mod.attrs.iter().find_map(|attr| {
        if !attr.path().is_ident("path") {
            return None;
        }
        match &attr.meta {
            syn::Meta::NameValue(name_value) => match &name_value.value {
                syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                    syn::Lit::Str(value) => Some(value.value()),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    })
}

pub(crate) fn is_cfg_test_only(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("cfg") {
            return false;
        }
        let Ok(meta) = attr.parse_args::<syn::Meta>() else {
            return false;
        };
        cfg_meta_is_test_only(&meta)
    })
}

fn cfg_meta_is_test_only(meta: &syn::Meta) -> bool {
    match meta {
        syn::Meta::Path(path) => path.is_ident("test"),
        syn::Meta::List(list) => {
            let Some(ident) = list.path.get_ident().map(|ident| ident.to_string()) else {
                return false;
            };
            let nested = list
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )
                .ok();
            match (ident.as_str(), nested) {
                ("all", Some(items)) | ("any", Some(items)) => {
                    !items.is_empty() && items.iter().all(cfg_meta_is_test_only)
                }
                _ => false,
            }
        }
        syn::Meta::NameValue(_) => false,
    }
}

pub(crate) trait ItemAttrs {
    fn attrs(&self) -> &[syn::Attribute];
}

impl ItemAttrs for syn::Item {
    fn attrs(&self) -> &[syn::Attribute] {
        match self {
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
}
