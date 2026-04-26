use std::collections::BTreeMap;

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::attrs::find_public_struct_field_bags;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-31";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_public_struct_field_bags(input.source) {
        if crate::support::has_matching_waiver(
            input,
            ID,
            &format!("struct:{}", info.struct_name),
        ) {
            continue;
        }

        let has_inherent_impl = struct_has_inherent_impl(input.source, &info.qualified_name);
        if input.is_shared_crate && info.all_fields_public && !has_inherent_impl {
            continue;
        }

        let severity = if info.public_field_count >= 5 {
            G3Severity::Error
        } else {
            G3Severity::Warn
        };
        let message = if input.is_shared_crate && has_inherent_impl {
            format!(
                "Shared-crate struct `{}` exposes {} named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
                info.struct_name, info.public_field_count
            )
        } else if input.is_shared_crate && !info.all_fields_public {
            format!(
                "Shared-crate struct `{}` exposes {} named `pub` fields but also hides some fields. In shared crates, either make this a plain data struct with all fields `pub`, or make the fields private and expose an API. Mixed visibility hides part of the shared data contract.",
                info.struct_name, info.public_field_count
            )
        } else {
            format!(
                "Struct `{}` exposes {} named `pub` fields. Make the fields private and expose constructors or getters instead, so callers use one API instead of reaching into raw state.",
                info.struct_name, info.public_field_count
            )
        };

        results.push(G3CheckResult::new(
            ID.to_owned(),
            severity,
            "public struct exposes named public fields".to_owned(),
            message,
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}

fn struct_has_inherent_impl(source: &syn::File, qualified_name: &str) -> bool {
    let mut module_struct_bindings = BTreeMap::new();
    collect_module_struct_bindings(&source.items, &mut Vec::new(), &mut module_struct_bindings);
    let mut module_bindings_by_path = module_struct_bindings.clone();
    collect_module_reexport_bindings(
        &source.items,
        &mut Vec::new(),
        &module_struct_bindings,
        &mut module_bindings_by_path,
        &BTreeMap::new(),
    );
    items_have_inherent_impl(
        &source.items,
        &mut Vec::new(),
        &module_bindings_by_path,
        qualified_name,
    )
}

fn items_have_inherent_impl(
    items: &[syn::Item],
    module_path: &mut Vec<String>,
    module_bindings_by_path: &BTreeMap<Vec<String>, BTreeMap<String, Vec<String>>>,
    qualified_name: &str,
) -> bool {
    let local_type_bindings =
        collect_local_type_bindings(items, module_path, module_bindings_by_path);
    items.iter().any(|item| match item {
        syn::Item::Mod(item_mod) => {
            let Some((_, nested_items)) = &item_mod.content else {
                return false;
            };
            module_path.push(item_mod.ident.to_string());
            let found = items_have_inherent_impl(
                nested_items,
                module_path,
                module_bindings_by_path,
                qualified_name,
            );
            let _ = module_path.pop();
            found
        }
        syn::Item::Impl(item_impl) => {
            if item_impl.trait_.is_some() {
                return false;
            }
            let syn::Type::Path(type_path) = item_impl.self_ty.as_ref() else {
                return false;
            };
            normalize_impl_self_type_path(module_path, type_path, &local_type_bindings)
                .is_some_and(|path| path == qualified_name)
        }
        _ => false,
    })
}

fn normalize_impl_self_type_path(
    module_path: &[String],
    type_path: &syn::TypePath,
    local_type_bindings: &BTreeMap<String, Vec<String>>,
) -> Option<String> {
    if type_path.qself.is_some() {
        return None;
    }

    let segments = type_path
        .path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>();
    if segments.is_empty() {
        return None;
    }

    let normalized = normalize_type_path_with_bindings(
        module_path,
        type_path.path.leading_colon.is_some(),
        &segments,
        local_type_bindings,
    )?;

    Some(normalized.join("::"))
}

fn normalize_type_path_with_bindings(
    current_module_path: &[String],
    leading_colon: bool,
    segments: &[String],
    local_type_bindings: &BTreeMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    let first = segments.first()?;

    if let Some(binding) = local_type_bindings.get(first) {
        let mut resolved = binding.clone();
        resolved.extend_from_slice(segments.get(1..).unwrap_or(&[]));
        return Some(resolved);
    }

    let normalized = if leading_colon {
        match first.as_str() {
            "crate" => segments.get(1..).unwrap_or(&[]).to_vec(),
            _ => segments.to_vec(),
        }
    } else {
        match first.as_str() {
            "crate" => segments.get(1..).unwrap_or(&[]).to_vec(),
            "self" => {
                let mut resolved = current_module_path.to_vec();
                resolved.extend_from_slice(segments.get(1..).unwrap_or(&[]));
                resolved
            }
            "super" => resolve_super_relative_path(current_module_path, segments)?,
            _ => {
                let mut resolved = current_module_path.to_vec();
                resolved.extend_from_slice(segments);
                resolved
            }
        }
    };

    Some(normalized)
}

fn resolve_super_relative_path(
    current_module_path: &[String],
    segments: &[String],
) -> Option<Vec<String>> {
    let mut resolved = current_module_path.to_vec();
    let mut rest = segments;

    while matches!(rest.first().map(String::as_str), Some("super")) {
        let _ = resolved.pop()?;
        rest = rest.get(1..).unwrap_or(&[]);
    }

    resolved.extend_from_slice(rest);
    Some(resolved)
}

fn resolve_binding_target(
    target: Vec<String>,
    module_bindings_by_path: &BTreeMap<Vec<String>, BTreeMap<String, Vec<String>>>,
) -> Vec<String> {
    let mut resolved = target;
    loop {
        let Some((name, module_path)) = resolved.split_last() else {
            break;
        };
        let Some(bindings) = module_bindings_by_path.get(module_path) else {
            break;
        };
        let Some(next) = bindings.get(name) else {
            break;
        };
        if *next == resolved {
            break;
        }
        resolved = next.clone();
    }
    resolved
}

fn collect_local_type_bindings(
    items: &[syn::Item],
    module_path: &[String],
    module_bindings_by_path: &BTreeMap<Vec<String>, BTreeMap<String, Vec<String>>>,
) -> BTreeMap<String, Vec<String>> {
    let mut bindings = module_bindings_by_path
        .get(module_path)
        .cloned()
        .unwrap_or_default();
    for item in items {
        let syn::Item::Use(item_use) = item else {
            continue;
        };
        collect_local_type_bindings_from_use_tree(
            &item_use.tree,
            &mut Vec::new(),
            module_path,
            module_bindings_by_path,
            &mut bindings,
        );
    }
    bindings
}

fn collect_local_type_bindings_from_use_tree(
    tree: &syn::UseTree,
    prefix: &mut Vec<String>,
    module_path: &[String],
    module_bindings_by_path: &BTreeMap<Vec<String>, BTreeMap<String, Vec<String>>>,
    bindings: &mut BTreeMap<String, Vec<String>>,
) {
    match tree {
        syn::UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_local_type_bindings_from_use_tree(
                &path.tree,
                prefix,
                module_path,
                module_bindings_by_path,
                bindings,
            );
            let _ = prefix.pop();
        }
        syn::UseTree::Name(name) => {
            let mut segments = prefix.clone();
            segments.push(name.ident.to_string());
            if let Some(target) =
                normalize_type_path_with_bindings(module_path, false, &segments, bindings)
            {
                let target = resolve_binding_target(target, module_bindings_by_path);
                let _ = bindings.insert(name.ident.to_string(), target);
            }
        }
        syn::UseTree::Rename(rename) => {
            let mut segments = prefix.clone();
            segments.push(rename.ident.to_string());
            if let Some(target) =
                normalize_type_path_with_bindings(module_path, false, &segments, bindings)
            {
                let target = resolve_binding_target(target, module_bindings_by_path);
                let _ = bindings.insert(rename.rename.to_string(), target);
            }
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_local_type_bindings_from_use_tree(
                    item,
                    prefix,
                    module_path,
                    module_bindings_by_path,
                    bindings,
                );
            }
        }
        syn::UseTree::Glob(_) => {
            if let Some(target_module_path) =
                normalize_type_path_with_bindings(module_path, false, prefix, bindings)
            {
                let target_module_path =
                    resolve_binding_target(target_module_path, module_bindings_by_path);
                if let Some(target_bindings) = module_bindings_by_path.get(&target_module_path) {
                    for (name, target) in target_bindings {
                        let _ = bindings.entry(name.clone()).or_insert_with(|| target.clone());
                    }
                }
            }
        }
    }
}

fn collect_module_struct_bindings(
    items: &[syn::Item],
    module_path: &mut Vec<String>,
    out: &mut BTreeMap<Vec<String>, BTreeMap<String, Vec<String>>>,
) {
    let mut bindings = BTreeMap::new();
    for item in items {
        match item {
            syn::Item::Struct(item_struct) => {
                let mut qualified = module_path.to_vec();
                qualified.push(item_struct.ident.to_string());
                let _ = bindings.insert(item_struct.ident.to_string(), qualified);
            }
            syn::Item::Mod(item_mod) => {
                let Some((_, nested_items)) = &item_mod.content else {
                    continue;
                };
                module_path.push(item_mod.ident.to_string());
                collect_module_struct_bindings(nested_items, module_path, out);
                let _ = module_path.pop();
            }
            _ => {}
        }
    }
    let _ = out.insert(module_path.to_vec(), bindings);
}

fn collect_module_reexport_bindings(
    items: &[syn::Item],
    module_path: &mut Vec<String>,
    direct_module_struct_bindings: &BTreeMap<Vec<String>, BTreeMap<String, Vec<String>>>,
    out: &mut BTreeMap<Vec<String>, BTreeMap<String, Vec<String>>>,
    parent_visible_bindings: &BTreeMap<String, Vec<String>>,
) {
    let mut visible_bindings = parent_visible_bindings.clone();
    if let Some(direct_bindings) = direct_module_struct_bindings.get(module_path) {
        visible_bindings.extend(direct_bindings.clone());
    }

    loop {
        let mut changed = false;
        for item in items {
            let syn::Item::Use(item_use) = item else {
                continue;
            };
            changed |= collect_reexport_aliases_from_use_tree(
                &item_use.tree,
                &mut Vec::new(),
                module_path,
                direct_module_struct_bindings,
                &mut visible_bindings,
            );
        }
        if !changed {
            break;
        }
    }

    let _ = out.insert(module_path.to_vec(), visible_bindings.clone());

    for item in items {
        let syn::Item::Mod(item_mod) = item else {
            continue;
        };
        let Some((_, nested_items)) = &item_mod.content else {
            continue;
        };
        module_path.push(item_mod.ident.to_string());
        collect_module_reexport_bindings(
            nested_items,
            module_path,
            direct_module_struct_bindings,
            out,
            &visible_bindings,
        );
        let _ = module_path.pop();
    }
}

fn collect_reexport_aliases_from_use_tree(
    tree: &syn::UseTree,
    prefix: &mut Vec<String>,
    module_path: &[String],
    direct_module_struct_bindings: &BTreeMap<Vec<String>, BTreeMap<String, Vec<String>>>,
    visible_bindings: &mut BTreeMap<String, Vec<String>>,
) -> bool {
    match tree {
        syn::UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            let changed = collect_reexport_aliases_from_use_tree(
                &path.tree,
                prefix,
                module_path,
                direct_module_struct_bindings,
                visible_bindings,
            );
            let _ = prefix.pop();
            changed
        }
        syn::UseTree::Name(name) => {
            let mut segments = prefix.clone();
            segments.push(name.ident.to_string());
            bind_reexport_alias(
                &name.ident.to_string(),
                &segments,
                module_path,
                direct_module_struct_bindings,
                visible_bindings,
            )
        }
        syn::UseTree::Rename(rename) => {
            let mut segments = prefix.clone();
            segments.push(rename.ident.to_string());
            bind_reexport_alias(
                &rename.rename.to_string(),
                &segments,
                module_path,
                direct_module_struct_bindings,
                visible_bindings,
            )
        }
        syn::UseTree::Group(group) => group.items.iter().any(|item| {
            collect_reexport_aliases_from_use_tree(
                item,
                prefix,
                module_path,
                direct_module_struct_bindings,
                visible_bindings,
            )
        }),
        syn::UseTree::Glob(_) => false,
    }
}

fn bind_reexport_alias(
    alias: &str,
    segments: &[String],
    module_path: &[String],
    direct_module_struct_bindings: &BTreeMap<Vec<String>, BTreeMap<String, Vec<String>>>,
    visible_bindings: &mut BTreeMap<String, Vec<String>>,
) -> bool {
    let Some(target) = normalize_type_path_with_bindings(module_path, false, segments, visible_bindings)
    else {
        return false;
    };
    if !is_known_struct_binding(&target, direct_module_struct_bindings) {
        return false;
    }

    let previous = visible_bindings.insert(alias.to_owned(), target.clone());
    match previous {
        Some(existing) if existing == target => false,
        _ => true,
    }
}

fn is_known_struct_binding(
    target: &[String],
    direct_module_struct_bindings: &BTreeMap<Vec<String>, BTreeMap<String, Vec<String>>>,
) -> bool {
    let Some((name, module_path)) = target.split_last() else {
        return false;
    };
    direct_module_struct_bindings
        .get(module_path)
        .is_some_and(|bindings| bindings.contains_key(name))
}

#[cfg(test)]
pub(super) fn check_source(
    rel_path: &str,
    content: &str,
    is_test: bool,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    check_source_with_waivers(rel_path, content, is_test, false, &[])
}

#[cfg(test)]
pub(super) fn check_source_with_shared(
    rel_path: &str,
    content: &str,
    is_test: bool,
    is_shared_crate: bool,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    check_source_with_waivers(rel_path, content, is_test, is_shared_crate, &[])
}

#[cfg(test)]
pub(super) fn check_source_with_waivers(
    rel_path: &str,
    content: &str,
    is_test: bool,
    is_shared_crate: bool,
    waivers: &[(&str, &str, &str, &str)],
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let source = crate::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let parsed = crate::support::G3RsCodeSourceFileAst {
        source_file: g3rs_code_types::G3RsSourceFile {
            rel_path: rel_path.to_owned(),
            content: content.to_owned(),
            is_test,
            profile_name: None,
            is_library_root: false,
        },
        source,
    };
    let input = crate::support::CodeSourceRuleInput {
        is_shared_crate,
        waivers: &waivers
            .iter()
            .map(|(rule, file, selector, reason)| g3rs_code_types::G3RsCodeWaiver {
                rule: (*rule).to_owned(),
                file: (*file).to_owned(),
                selector: (*selector).to_owned(),
                reason: (*reason).to_owned(),
            })
            .collect::<Vec<_>>(),
        ..crate::support::CodeSourceRuleInput::from(&parsed)
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
