use std::collections::BTreeSet;

use super::super::helpers::attrs_enter_test_context;

pub(super) fn use_tree_matches_std_fs(
    tree: &syn::UseTree,
    std_aliases: &BTreeSet<String>,
    fs_aliases: &BTreeSet<String>,
) -> bool {
    match tree {
        syn::UseTree::Path(path)
            if path.ident == "std" || std_aliases.contains(path.ident.to_string().as_str()) =>
        {
            match &*path.tree {
                syn::UseTree::Path(inner) => {
                    inner.ident == "fs" || fs_aliases.contains(inner.ident.to_string().as_str())
                }
                syn::UseTree::Name(name) => {
                    name.ident == "fs" || fs_aliases.contains(name.ident.to_string().as_str())
                }
                syn::UseTree::Rename(rename) => {
                    rename.ident == "fs" || fs_aliases.contains(rename.ident.to_string().as_str())
                }
                syn::UseTree::Group(group) => group
                    .items
                    .iter()
                    .any(|item| use_tree_matches_std_fs_with_std_prefix(item, fs_aliases)),
                _ => false,
            }
        }
        syn::UseTree::Path(path) if fs_aliases.contains(path.ident.to_string().as_str()) => true,
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .any(|item| use_tree_matches_std_fs(item, std_aliases, fs_aliases)),
        _ => false,
    }
}

fn use_tree_matches_std_fs_with_std_prefix(
    tree: &syn::UseTree,
    fs_aliases: &BTreeSet<String>,
) -> bool {
    match tree {
        syn::UseTree::Path(path) => {
            path.ident == "fs" || fs_aliases.contains(path.ident.to_string().as_str())
        }
        syn::UseTree::Name(name) => {
            name.ident == "fs"
                || name.ident == "self"
                || fs_aliases.contains(name.ident.to_string().as_str())
        }
        syn::UseTree::Rename(rename) => {
            rename.ident == "fs"
                || rename.ident == "self"
                || fs_aliases.contains(rename.ident.to_string().as_str())
        }
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .any(|item| use_tree_matches_std_fs_with_std_prefix(item, fs_aliases)),
        syn::UseTree::Glob(_) => false,
    }
}

pub(super) fn use_tree_is_std_fs_glob(
    tree: &syn::UseTree,
    std_aliases: &BTreeSet<String>,
    fs_aliases: &BTreeSet<String>,
) -> bool {
    match tree {
        syn::UseTree::Path(std_path)
            if std_path.ident == "std"
                || std_aliases.contains(std_path.ident.to_string().as_str()) =>
        {
            match &*std_path.tree {
                syn::UseTree::Path(fs_path)
                    if fs_path.ident == "fs"
                        || fs_aliases.contains(fs_path.ident.to_string().as_str()) =>
                {
                    fs_subtree_contains_glob(&fs_path.tree)
                }
                syn::UseTree::Group(group) => group
                    .items
                    .iter()
                    .any(|item| use_tree_is_std_fs_glob_with_std_prefix(item, fs_aliases)),
                _ => false,
            }
        }
        syn::UseTree::Path(fs_path)
            if fs_aliases.contains(fs_path.ident.to_string().as_str()) =>
        {
            fs_subtree_contains_glob(&fs_path.tree)
        }
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .any(|item| use_tree_is_std_fs_glob(item, std_aliases, fs_aliases)),
        _ => false,
    }
}

fn use_tree_is_std_fs_glob_with_std_prefix(
    tree: &syn::UseTree,
    fs_aliases: &BTreeSet<String>,
) -> bool {
    match tree {
        syn::UseTree::Path(fs_path)
            if fs_path.ident == "fs" || fs_aliases.contains(fs_path.ident.to_string().as_str()) =>
        {
            fs_subtree_contains_glob(&fs_path.tree)
        }
        syn::UseTree::Glob(_) => true,
        syn::UseTree::Group(group) => group
            .items
            .iter()
            .any(|item| use_tree_is_std_fs_glob_with_std_prefix(item, fs_aliases)),
        _ => false,
    }
}

fn fs_subtree_contains_glob(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Glob(_) => true,
        syn::UseTree::Group(group) => group.items.iter().any(fs_subtree_contains_glob),
        _ => false,
    }
}

pub(super) trait TestContextAware {
    fn in_test_context_mut(&mut self) -> &mut bool;

    fn save_and_apply_test_context(&mut self, attrs: &[syn::Attribute]) -> bool {
        let was = *self.in_test_context_mut();
        *self.in_test_context_mut() |= attrs_enter_test_context(attrs);
        was
    }

    fn restore_test_context(&mut self, was: bool) {
        *self.in_test_context_mut() = was;
    }
}

pub(super) fn collect_std_aliases(
    tree: &syn::UseTree,
    std_aliases: &mut BTreeSet<String>,
    fs_aliases: &mut BTreeSet<String>,
) {
    match tree {
        syn::UseTree::Rename(rename)
            if rename.ident == "std"
                || std_aliases.contains(rename.ident.to_string().as_str()) =>
        {
            let _ = std_aliases.insert(rename.rename.to_string());
        }
        syn::UseTree::Rename(rename)
            if rename.ident == "fs"
                || fs_aliases.contains(rename.ident.to_string().as_str()) =>
        {
            let _ = fs_aliases.insert(rename.rename.to_string());
        }
        syn::UseTree::Path(path)
            if path.ident == "std" || std_aliases.contains(path.ident.to_string().as_str()) =>
        {
            collect_std_aliases_under_std(&path.tree, std_aliases, fs_aliases);
        }
        syn::UseTree::Path(path)
            if path.ident == "fs" || fs_aliases.contains(path.ident.to_string().as_str()) =>
        {
            collect_std_aliases_under_fs(&path.tree, fs_aliases);
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_std_aliases(item, std_aliases, fs_aliases);
            }
        }
        _ => {}
    }
}

pub(super) fn collect_std_extern_crate_alias(
    item: &syn::ItemExternCrate,
    aliases: &mut BTreeSet<String>,
) {
    if item.ident != "std" {
        return;
    }
    if let Some((_, rename)) = &item.rename {
        let _ = aliases.insert(rename.to_string());
    } else {
        let _ = aliases.insert(String::from("std"));
    }
}

fn collect_std_aliases_under_std(
    tree: &syn::UseTree,
    std_aliases: &mut BTreeSet<String>,
    fs_aliases: &mut BTreeSet<String>,
) {
    match tree {
        syn::UseTree::Rename(rename)
            if rename.ident == "self"
                || std_aliases.contains(rename.ident.to_string().as_str()) =>
        {
            let _ = std_aliases.insert(rename.rename.to_string());
        }
        syn::UseTree::Rename(rename)
            if rename.ident == "fs"
                || fs_aliases.contains(rename.ident.to_string().as_str()) =>
        {
            let _ = fs_aliases.insert(rename.rename.to_string());
        }
        syn::UseTree::Path(path)
            if path.ident == "self" || std_aliases.contains(path.ident.to_string().as_str()) =>
        {
            collect_std_aliases_under_std(&path.tree, std_aliases, fs_aliases);
        }
        syn::UseTree::Path(path)
            if path.ident == "fs" || fs_aliases.contains(path.ident.to_string().as_str()) =>
        {
            collect_std_aliases_under_fs(&path.tree, fs_aliases);
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_std_aliases_under_std(item, std_aliases, fs_aliases);
            }
        }
        _ => {}
    }
}

fn collect_std_aliases_under_fs(tree: &syn::UseTree, fs_aliases: &mut BTreeSet<String>) {
    match tree {
        syn::UseTree::Rename(rename)
            if rename.ident == "fs" || fs_aliases.contains(rename.ident.to_string().as_str()) =>
        {
            let _ = fs_aliases.insert(rename.rename.to_string());
        }
        syn::UseTree::Path(path)
            if path.ident == "fs" || fs_aliases.contains(path.ident.to_string().as_str()) =>
        {
            collect_std_aliases_under_fs(&path.tree, fs_aliases);
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_std_aliases_under_fs(item, fs_aliases);
            }
        }
        _ => {}
    }
}
