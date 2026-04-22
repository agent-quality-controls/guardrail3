use std::collections::BTreeMap;

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::attrs::find_public_struct_field_bags;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-31";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_public_struct_field_bags(input.source) {
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
    items_have_inherent_impl(&source.items, &mut Vec::new(), qualified_name)
}

fn items_have_inherent_impl(
    items: &[syn::Item],
    module_path: &mut Vec<String>,
    qualified_name: &str,
) -> bool {
    let local_type_bindings = collect_local_type_bindings(items, module_path);
    items.iter().any(|item| match item {
        syn::Item::Mod(item_mod) => {
            let Some((_, nested_items)) = &item_mod.content else {
                return false;
            };
            module_path.push(item_mod.ident.to_string());
            let found = items_have_inherent_impl(nested_items, module_path, qualified_name);
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
            "super" => {
                let mut resolved = current_module_path.to_vec();
                if resolved.pop().is_none() {
                    return None;
                }
                resolved.extend_from_slice(segments.get(1..).unwrap_or(&[]));
                resolved
            }
            _ => {
                let mut resolved = current_module_path.to_vec();
                resolved.extend_from_slice(segments);
                resolved
            }
        }
    };

    Some(normalized)
}

fn collect_local_type_bindings(
    items: &[syn::Item],
    module_path: &[String],
) -> BTreeMap<String, Vec<String>> {
    let mut bindings = BTreeMap::new();
    for item in items {
        let syn::Item::Use(item_use) = item else {
            continue;
        };
        collect_local_type_bindings_from_use_tree(
            &item_use.tree,
            &mut Vec::new(),
            module_path,
            &mut bindings,
        );
    }
    bindings
}

fn collect_local_type_bindings_from_use_tree(
    tree: &syn::UseTree,
    prefix: &mut Vec<String>,
    module_path: &[String],
    bindings: &mut BTreeMap<String, Vec<String>>,
) {
    match tree {
        syn::UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_local_type_bindings_from_use_tree(&path.tree, prefix, module_path, bindings);
            let _ = prefix.pop();
        }
        syn::UseTree::Name(name) => {
            let mut segments = prefix.clone();
            segments.push(name.ident.to_string());
            if let Some(target) =
                normalize_type_path_with_bindings(module_path, false, &segments, &BTreeMap::new())
            {
                let _ = bindings.insert(name.ident.to_string(), target);
            }
        }
        syn::UseTree::Rename(rename) => {
            let mut segments = prefix.clone();
            segments.push(rename.ident.to_string());
            if let Some(target) =
                normalize_type_path_with_bindings(module_path, false, &segments, &BTreeMap::new())
            {
                let _ = bindings.insert(rename.rename.to_string(), target);
            }
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_local_type_bindings_from_use_tree(item, prefix, module_path, bindings);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}

#[cfg(test)]
pub(super) fn check_source(
    rel_path: &str,
    content: &str,
    is_test: bool,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    check_source_with_shared(rel_path, content, is_test, false)
}

#[cfg(test)]
pub(super) fn check_source_with_shared(
    rel_path: &str,
    content: &str,
    is_test: bool,
    is_shared_crate: bool,
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
        ..crate::support::CodeSourceRuleInput::from(&parsed)
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
