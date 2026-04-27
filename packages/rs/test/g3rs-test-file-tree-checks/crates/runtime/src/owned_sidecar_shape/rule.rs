use std::collections::BTreeSet;

use g3rs_test_types::G3RsTestComponentFileTreeFacts;
use g3rs_test_types::G3RsTestFileKind;
use g3rs_test_types::G3RsTestFileTreeChecksInput;
use g3rs_test_types::ast::CfgTestModuleInfo;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-test/owned-sidecar-shape";

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;

#[derive(Debug, Clone, PartialEq, Eq)]
struct SidecarViolation {
    rel_path: String,
    line: Option<usize>,
    title: String,
    message: String,
}

pub(crate) fn collect(input: &G3RsTestFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    let violations = collect_violations(input);
    if violations.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "owned sidecar shape confirmed".to_owned(),
                format!(
                    "Root `{}` keeps internal harnesses inside module-owned `*_tests/` sidecars.",
                    input.root_rel_dir
                ),
                Some(input.cargo_rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
        return;
    }

    for violation in violations {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            violation.title,
            violation.message,
            Some(violation.rel_path),
            violation.line,
        ));
    }
}

fn collect_violations(input: &G3RsTestFileTreeChecksInput) -> Vec<SidecarViolation> {
    let prebound_sidecar_mod_paths = input
        .components
        .iter()
        .flat_map(|component| {
            component
                .sidecars
                .iter()
                .map(|sidecar| sidecar.mod_rel_path.clone())
        })
        .collect::<BTreeSet<_>>();
    let mut violations = Vec::new();

    collect_ad_hoc_src_tests_tree_violations(input, &mut violations);

    for component in &input.components {
        collect_component_sidecar_violations(component, &mut violations);
    }

    for file in &input.files {
        if !matches!(
            file.kind,
            G3RsTestFileKind::Source
                | G3RsTestFileKind::InternalSidecarMod
                | G3RsTestFileKind::InternalSidecarSupport
        ) {
            continue;
        }

        if is_flat_test_sidecar(&file.rel_path) {
            violations.push(SidecarViolation {
                rel_path: file.rel_path.clone(),
                line: None,
                title: "flat sidecar test file".to_owned(),
                message: "Internal test harnesses must live in `*_tests/mod.rs`, not as flat `*_tests.rs`, `*_test.rs`, or `tests.rs` files.".to_owned(),
            });
        }

        if !matches!(file.kind, G3RsTestFileKind::Source) {
            continue;
        }
        for module in &file.parsed.cfg_test_modules {
            if cfg_test_decl_is_owned_sidecar(&file.rel_path, module, &prebound_sidecar_mod_paths) {
                continue;
            }
            let expected = owned_sidecar_contract(&file.rel_path);
            violations.push(SidecarViolation {
                rel_path: file.rel_path.clone(),
                line: Some(module.line),
                title: "ad hoc cfg(test) module declaration".to_owned(),
                message: match expected {
                    Some((module_name, path_attr)) => format!(
                        "File `{}` declares `#[cfg(test)] mod {};` without the owned sidecar declaration `#[path = \"{}\"] mod {};`. Use that exact file-owned sidecar shape, so this module's internal tests live in one sidecar directory.",
                        file.rel_path, module.name, path_attr, module_name,
                    ),
                    None => format!(
                        "Facade file `{}` must not declare internal test sidecars. Move the tests onto a real sibling `x.rs` file and use `#[path = \"x_tests/mod.rs\"] mod x_tests;` there.",
                        file.rel_path,
                    ),
                },
            });
        }
    }

    violations.sort_by(|left, right| {
        left.rel_path
            .cmp(&right.rel_path)
            .then_with(|| left.line.cmp(&right.line))
            .then_with(|| left.title.cmp(&right.title))
    });
    violations
}

fn collect_ad_hoc_src_tests_tree_violations(
    input: &G3RsTestFileTreeChecksInput,
    violations: &mut Vec<SidecarViolation>,
) {
    let mut src_roots = input
        .components
        .iter()
        .map(|component| join_under_root(&component.runtime_rel_dir, "src"))
        .collect::<BTreeSet<_>>();
    let root_src = join_under_root(&input.root_rel_dir, "src");
    if input
        .files
        .iter()
        .any(|file| file.component_rel_dir.is_none() && path_is_under(&file.rel_path, &root_src))
    {
        let _ = src_roots.insert(root_src);
    }

    let mut offending_dirs = BTreeSet::new();
    for file in &input.files {
        for src_root in &src_roots {
            if !path_is_under(&file.rel_path, src_root) {
                continue;
            }
            let mut current = parent_dir(&file.rel_path).to_owned();
            while !current.is_empty() && current != *src_root {
                let rel_after_src = current
                    .strip_prefix(src_root.as_str())
                    .and_then(|rest| rest.strip_prefix('/'))
                    .unwrap_or("");
                if rel_after_src == "tests"
                    || rel_after_src.starts_with("tests/")
                    || rel_after_src.ends_with("/tests")
                    || rel_after_src.contains("/tests/")
                {
                    let _ = offending_dirs.insert(current.clone());
                }
                current = parent_dir(&current).to_owned();
            }
        }
    }

    for dir_rel in offending_dirs {
        violations.push(SidecarViolation {
            rel_path: dir_rel,
            line: None,
            title: "ad hoc src/tests tree".to_owned(),
            message: "Internal test harnesses must live in module-owned `*_tests/` sidecars, not under `src/**/tests/`.".to_owned(),
        });
    }
}

fn collect_component_sidecar_violations(
    component: &G3RsTestComponentFileTreeFacts,
    violations: &mut Vec<SidecarViolation>,
) {
    let src_root = join_under_root(&component.runtime_rel_dir, "src");
    let declared_sidecar_mods = component
        .sidecars
        .iter()
        .map(|sidecar| sidecar.mod_rel_path.as_str())
        .collect::<BTreeSet<_>>();
    let sidecar_dirs = component
        .sidecar_files
        .iter()
        .map(|file| parent_dir(&file.rel_path).to_owned())
        .collect::<BTreeSet<_>>();

    for dir_rel in sidecar_dirs {
        let rel_after_src = dir_rel
            .strip_prefix(src_root.as_str())
            .and_then(|rest| rest.strip_prefix('/'))
            .unwrap_or("");
        if rel_after_src == "tests"
            || rel_after_src.starts_with("tests/")
            || rel_after_src.ends_with("/tests")
            || rel_after_src.contains("/tests/")
        {
            violations.push(SidecarViolation {
                rel_path: dir_rel,
                line: None,
                title: "ad hoc src/tests tree".to_owned(),
                message: "Internal test harnesses must live in module-owned `*_tests/` sidecars, not under `src/**/tests/`.".to_owned(),
            });
            continue;
        }

        let mod_rel_path = format!("{dir_rel}/mod.rs");
        if !declared_sidecar_mods.contains(mod_rel_path.as_str()) {
            violations.push(SidecarViolation {
                rel_path: dir_rel,
                line: None,
                title: "sidecar directory missing mod.rs".to_owned(),
                message:
                    "Internal sidecar harness directories must expose `mod.rs` as their entrypoint."
                        .to_owned(),
            });
            continue;
        }

        let Some(owner_module_rel_path) = owned_sidecar_owner_rel_path(&src_root, rel_after_src)
        else {
            continue;
        };
        let Some(owner_module_name) = owner_module_rel_path
            .rsplit('/')
            .next()
            .and_then(|name| name.strip_suffix(".rs"))
        else {
            continue;
        };
        if !component.source_module_names.contains(owner_module_name) {
            violations.push(SidecarViolation {
                rel_path: mod_rel_path,
                line: None,
                title: "orphaned sidecar harness".to_owned(),
                message: format!(
                    "Owned sidecar `{dir_rel}/mod.rs` requires matching production module `{owner_module_rel_path}`."
                ),
            });
        }
    }
}

fn owned_sidecar_owner_rel_path(src_root: &str, rel_after_src: &str) -> Option<String> {
    let dir_name = rel_after_src.rsplit('/').next()?;
    let owner_module_name = dir_name.strip_suffix("_tests")?;
    if owner_module_name.is_empty() {
        return None;
    }
    let relative_parent = parent_dir(rel_after_src);
    if relative_parent.is_empty() {
        Some(format!("{src_root}/{owner_module_name}.rs"))
    } else {
        Some(format!(
            "{src_root}/{relative_parent}/{owner_module_name}.rs"
        ))
    }
}

fn is_flat_test_sidecar(rel_path: &str) -> bool {
    rel_path.ends_with("_tests.rs")
        || rel_path.ends_with("_test.rs")
        || rel_path.ends_with("/tests.rs")
}

fn cfg_test_decl_is_owned_sidecar(
    file_rel_path: &str,
    module: &CfgTestModuleInfo,
    prebound_sidecar_mod_paths: &BTreeSet<String>,
) -> bool {
    if module.has_body {
        return false;
    }

    let Some((expected_module_name, expected_path_attr)) = owned_sidecar_contract(file_rel_path)
    else {
        return false;
    };
    if module.name != expected_module_name {
        return false;
    }
    let parent = parent_dir(file_rel_path);
    let expected_mod_rel = format!("{parent}/{expected_module_name}/mod.rs");
    if !prebound_sidecar_mod_paths.contains(&expected_mod_rel) {
        return false;
    }

    module.path_attr.as_deref() == Some(expected_path_attr.as_str())
}

fn owned_sidecar_contract(file_rel_path: &str) -> Option<(String, String)> {
    let file_name = file_rel_path.rsplit('/').next()?;
    let stem = file_name.strip_suffix(".rs")?;
    if stem == "mod" || stem.is_empty() {
        return None;
    }
    let module_name = format!("{stem}_tests");
    Some((module_name.clone(), format!("{module_name}/mod.rs")))
}

fn join_under_root(root_rel_dir: &str, child_rel: &str) -> String {
    if root_rel_dir.is_empty() {
        child_rel.to_owned()
    } else {
        format!("{root_rel_dir}/{child_rel}")
    }
}

fn parent_dir(rel_path: &str) -> &str {
    rel_path.rsplit_once('/').map_or("", |(parent, _)| parent)
}

fn path_is_under(rel_path: &str, prefix: &str) -> bool {
    rel_path == prefix
        || rel_path
            .strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with('/'))
}
