use std::collections::BTreeSet;

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_domain_project_tree::ProjectTree;

use super::discover::{join_under_root, parent_dir, path_is_under};
use super::facts::{SidecarViolation, TestFileKind, TestRootFacts};
use super::inputs::SidecarViolationInput;
use super::{AnalyzedFile, parse};

const ID: &str = "RS-TEST-02";

pub(crate) fn collect(
    tree: &ProjectTree,
    root: &TestRootFacts,
    files: &[AnalyzedFile],
    scoped_files: Option<&BTreeSet<String>>,
    results: &mut Vec<CheckResult>,
) {
    for violation in collect_violations(tree, root, files, scoped_files) {
        check(&SidecarViolationInput::new(&violation), results);
    }
}

pub fn check(input: &SidecarViolationInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: input.violation.title.clone(),
        message: input.violation.message.clone(),
        file: Some(input.violation.rel_path.clone()),
        line: input.violation.line,
        inventory: false,
    });
}

fn collect_violations(
    tree: &ProjectTree,
    root: &TestRootFacts,
    files: &[AnalyzedFile],
    scoped_files: Option<&BTreeSet<String>>,
) -> Vec<SidecarViolation> {
    let mut violations = Vec::new();
    let src_roots = src_roots_for_root(root);
    for dir_rel in tree.all_dir_rels() {
        let Some(src_root) = src_roots
            .iter()
            .find(|src_root| path_is_under(&dir_rel, src_root))
        else {
            continue;
        };
        let rel_after_src = dir_rel
            .strip_prefix(src_root)
            .and_then(|rest| rest.strip_prefix('/'))
            .unwrap_or("");
        if rel_after_src == "tests" || rel_after_src.starts_with("tests/") {
            violations.push(SidecarViolation {
                rel_path: dir_rel.clone(),
                line: None,
                title: "ad hoc src/tests tree".to_owned(),
                message: "Internal test harnesses must live in owned `<module>_tests/` directories, not under `src/tests/`.".to_owned(),
            });
            continue;
        }

        let Some(owner_module_rel_path) = owned_sidecar_owner_rel_path(src_root, rel_after_src) else {
            continue;
        };
        let mod_rel_path = format!("{dir_rel}/mod.rs");
        if !tree.file_exists(&mod_rel_path) {
            violations.push(SidecarViolation {
                rel_path: dir_rel.clone(),
                line: None,
                title: "sidecar directory missing mod.rs".to_owned(),
                message: "Internal sidecar harness directories must expose `mod.rs` as their entrypoint.".to_owned(),
            });
            continue;
        }

        if !tree.file_exists(&owner_module_rel_path) {
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

    for file in files {
        if scoped_files.is_some_and(|paths| !paths.contains(&file.facts.rel_path)) {
            continue;
        }
        if !matches!(
            file.facts.kind,
            TestFileKind::Source | TestFileKind::InternalSidecarMod | TestFileKind::InternalSidecarSupport
        ) {
            continue;
        }

        if is_flat_test_sidecar(&file.facts.rel_path) {
            violations.push(SidecarViolation {
                rel_path: file.facts.rel_path.clone(),
                line: None,
                title: "flat sidecar test file".to_owned(),
                message: "Internal sidecar harnesses must use `<module>_tests/mod.rs`, not flat `*_tests.rs` files.".to_owned(),
            });
        }

        if matches!(file.facts.kind, TestFileKind::Source) {
            for module in &file.parsed.cfg_test_modules {
                if module.has_body
                    || cfg_test_decl_is_owned_sidecar(
                        tree,
                        &file.facts.rel_path,
                        file.facts.owner_module_name.as_deref(),
                        module,
                    )
                {
                    continue;
                }
                violations.push(SidecarViolation {
                    rel_path: file.facts.rel_path.clone(),
                    line: Some(module.line),
                    title: "ad hoc cfg(test) module declaration".to_owned(),
                    message: "Declaration-only `#[cfg(test)]` modules must resolve to the owned `<module>_tests/mod.rs` sidecar shape.".to_owned(),
                });
            }
        }
    }

    violations.sort_by(|left, right| left.rel_path.cmp(&right.rel_path));
    violations
}

fn src_roots_for_root(root: &TestRootFacts) -> Vec<String> {
    let mut roots = vec![join_under_root(&root.rel_dir, "src")];
    roots.extend(
        root.components
            .iter()
            .map(|component| format!("{}/src", component.runtime_rel_dir)),
    );
    roots
}

fn owned_sidecar_owner_rel_path(src_root: &str, rel_after_src: &str) -> Option<String> {
    let dir_name = rel_after_src.rsplit('/').next()?;
    let owner_module_name = dir_name.strip_suffix("_tests")?;
    let relative_parent = parent_dir(rel_after_src);
    let owner_rel = if relative_parent.is_empty() {
        format!("{owner_module_name}.rs")
    } else {
        format!("{relative_parent}/{owner_module_name}.rs")
    };
    Some(format!("{src_root}/{owner_rel}"))
}

fn is_flat_test_sidecar(rel_path: &str) -> bool {
    rel_path.ends_with("_tests.rs")
        || rel_path.ends_with("_test.rs")
        || rel_path.ends_with("/tests.rs")
}

fn cfg_test_decl_is_owned_sidecar(
    tree: &ProjectTree,
    file_rel_path: &str,
    owner_module_name: Option<&str>,
    module: &parse::CfgTestModuleInfo,
) -> bool {
    let Some(owner_module_name) = owner_module_name else {
        return false;
    };
    let expected_module_name = format!("{owner_module_name}_tests");
    if module.name != expected_module_name {
        return false;
    }

    let parent = parent_dir(file_rel_path);
    let expected_path = format!("{expected_module_name}/mod.rs");
    if let Some(path_attr) = module.path_attr.as_deref() {
        return path_attr == expected_path && tree.file_exists(&format!("{parent}/{expected_path}"));
    }

    let sidecar_dir = format!("{parent}/{expected_module_name}");
    tree.file_exists(&format!("{sidecar_dir}/mod.rs"))
        && !tree.file_exists(&format!("{parent}/{expected_module_name}.rs"))
}

#[cfg(test)]
#[path = "rs_test_02_owned_sidecar_shape_tests/mod.rs"]
mod rs_test_02_owned_sidecar_shape_tests;
