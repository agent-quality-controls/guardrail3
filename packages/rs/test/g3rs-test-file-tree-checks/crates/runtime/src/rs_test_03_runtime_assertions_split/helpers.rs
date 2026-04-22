use std::collections::BTreeSet;

use g3rs_test_types::G3RsTestComponentFileTreeFacts;
use g3rs_test_types::ast::UseBinding;

pub(super) fn import_uses_external_runtime_boundary(binding: &UseBinding) -> bool {
    binding
        .path_segments
        .first()
        .is_some_and(|segment| matches!(segment.as_str(), "crate" | "super"))
}

pub(super) fn import_uses_local_boundary(binding: &UseBinding) -> bool {
    binding
        .path_segments
        .first()
        .is_some_and(|segment| matches!(segment.as_str(), "crate" | "self" | "super"))
}

pub(super) fn assertions_call_runtime_check_test_tree(
    imports: &[UseBinding],
    call_paths: &[Vec<String>],
    runtime_package_name: Option<&str>,
) -> bool {
    let Some(runtime_package_name) = runtime_package_name else {
        return false;
    };
    let mut runtime_roots = BTreeSet::from([
        runtime_package_name.to_owned(),
        runtime_package_name.replace('-', "_"),
    ]);
    let mut imported_check_test_tree = BTreeSet::new();

    for binding in imports {
        if binding
            .path_segments
            .first()
            .is_none_or(|first| !runtime_roots.contains(first))
        {
            continue;
        }
        if let Some(local_name) = binding.local_name.as_ref() {
            if binding.path_segments.len() == 1 {
                let _ = runtime_roots.insert(local_name.clone());
            } else if binding
                .path_segments
                .last()
                .is_some_and(|segment| segment == "check_test_tree")
            {
                let _ = imported_check_test_tree.insert(local_name.clone());
            }
        }
    }

    call_paths.iter().any(|path| match path.as_slice() {
        [single] => imported_check_test_tree.contains(single),
        [first, second, ..] => runtime_roots.contains(first) && second == "check_test_tree",
        _ => false,
    })
}

pub(super) fn import_hits_sibling_module(
    binding: &UseBinding,
    owner_module_name: &str,
    local_module_names: &BTreeSet<String>,
) -> bool {
    sibling_module_target(
        &binding.path_segments,
        owner_module_name,
        local_module_names,
    )
    .is_some()
}

pub(super) fn sibling_module_target<'a>(
    path_segments: &'a [String],
    owner_module_name: &str,
    local_module_names: &BTreeSet<String>,
) -> Option<&'a str> {
    let imported = first_local_module_target(path_segments)?;
    if !local_module_names.contains(imported) {
        return None;
    }
    let owner_tests_module_name = format!("{owner_module_name}_tests");
    if imported == owner_module_name || imported == owner_tests_module_name {
        return None;
    }
    Some(imported)
}

pub(super) fn disallowed_sidecar_local_boundary_target(
    path_segments: &[String],
    file_kind: &g3rs_test_types::G3RsTestFileKind,
    owner_module_name: &str,
    local_module_names: &BTreeSet<String>,
) -> Option<String> {
    let Some(first) = path_segments.first() else {
        return None;
    };
    let owner_tests_module_name = format!("{owner_module_name}_tests");
    match first.as_str() {
        "crate" => {
            let target = path_segments.get(1)?;
            if target == owner_module_name
                || target == &owner_tests_module_name
                || local_module_names.contains(target)
            {
                None
            } else {
                Some(target.clone())
            }
        }
        "self" | "super" => {
            let boundary_depth = path_segments
                .iter()
                .take_while(|segment| matches!(segment.as_str(), "self" | "super"))
                .count();
            let max_depth = match file_kind {
                g3rs_test_types::G3RsTestFileKind::InternalSidecarMod => 1,
                g3rs_test_types::G3RsTestFileKind::InternalSidecarSupport => 2,
                _ => 0,
            };
            (boundary_depth > max_depth).then(|| {
                path_segments
                    .get(boundary_depth)
                    .cloned()
                    .unwrap_or_else(|| "<crate-root>".to_owned())
            })
        }
        _ => None,
    }
}

fn first_local_module_target(path_segments: &[String]) -> Option<&str> {
    let first = path_segments.first()?;
    match first.as_str() {
        "crate" => path_segments.get(1).map(String::as_str),
        "self" | "super" => path_segments
            .iter()
            .skip(1)
            .find(|segment| !matches!(segment.as_str(), "self" | "super"))
            .map(String::as_str),
        _ => None,
    }
}

pub(super) fn allowed_external_harness_packages(
    component: &G3RsTestComponentFileTreeFacts,
) -> BTreeSet<String> {
    let mut allowed = BTreeSet::from(["test_support".to_owned()]);
    if let Some(runtime_package_name) = component.runtime_package_name.as_ref() {
        let _ = allowed.insert(runtime_package_name.clone());
    }
    if let Some(assertions_package_name) = component.assertions_package_name.as_ref() {
        let _ = allowed.insert(assertions_package_name.clone());
    }
    allowed
}

pub(super) fn allowed_sidecar_packages(
    component: &G3RsTestComponentFileTreeFacts,
) -> BTreeSet<String> {
    let mut allowed = BTreeSet::from(["test_support".to_owned()]);
    if let Some(assertions_package_name) = component.assertions_package_name.as_ref() {
        let _ = allowed.insert(assertions_package_name.clone());
    }
    allowed
}

pub(super) fn allowed_assertions_packages(
    component: &G3RsTestComponentFileTreeFacts,
) -> BTreeSet<String> {
    let mut allowed = BTreeSet::from([
        "test_support".to_owned(),
        "guardrail3_domain_report".to_owned(),
    ]);
    if let Some(runtime_package_name) = component.runtime_package_name.as_ref() {
        let _ = allowed.insert(runtime_package_name.clone());
    }
    if let Some(assertions_package_name) = component.assertions_package_name.as_ref() {
        let _ = allowed.insert(format!("{assertions_package_name}_common"));
    }
    allowed
}

pub(super) fn first_disallowed_local_package<'a>(
    path: &'a [String],
    local_package_names: &'a BTreeSet<String>,
    allowed_local_packages: &'a BTreeSet<String>,
) -> Option<&'a str> {
    let root = path.first()?;
    if !local_package_names.contains(root) || allowed_local_packages.contains(root) {
        return None;
    }
    Some(root.as_str())
}

pub(super) fn foreign_assertions_module_target<'a>(
    path_segments: &'a [String],
    assertions_package_name: Option<&str>,
    sidecar_rel_path: &str,
    owner_module_name: &str,
) -> Option<&'a str> {
    let assertions_package_name = assertions_package_name?;
    let [first, second, ..] = path_segments else {
        return None;
    };
    if first != assertions_package_name {
        return None;
    }

    let expected_prefix = expected_assertions_module_prefix(sidecar_rel_path, owner_module_name);
    if expected_prefix.is_empty() {
        return (path_segments.len() > 2).then_some(second.as_str());
    }

    let actual_prefix = &path_segments[1..];
    if actual_prefix.starts_with(&expected_prefix) {
        return None;
    }

    Some(second.as_str())
}

fn expected_assertions_module_prefix(
    sidecar_rel_path: &str,
    owner_module_name: &str,
) -> Vec<String> {
    let rel_after_src = sidecar_rel_path
        .split_once("/src/")
        .map_or(sidecar_rel_path, |(_, suffix)| suffix);

    let fabricated_assertions_rel =
        if let Some((before_rule_tests, _)) = rel_after_src.split_once("/rule_tests/") {
            format!("{before_rule_tests}/{owner_module_name}.rs")
        } else if let Some((before_flat_tests, _)) = rel_after_src.split_once("_tests/") {
            format!("{before_flat_tests}.rs")
        } else {
            return Vec::new();
        };

    assertions_module_prefix_from_rel_path(&fabricated_assertions_rel)
}

fn assertions_module_prefix_from_rel_path(rel_path: &str) -> Vec<String> {
    let rel_without_ext = rel_path.strip_suffix(".rs").unwrap_or(rel_path);
    let mut segments = rel_without_ext
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if segments.last().is_some_and(|segment| segment == "lib") {
        let _ = segments.pop();
    } else if segments.last().is_some_and(|segment| segment == "mod") {
        let _ = segments.pop();
    }
    segments
}
