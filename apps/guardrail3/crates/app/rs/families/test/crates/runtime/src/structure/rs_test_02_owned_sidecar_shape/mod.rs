use std::collections::BTreeSet;

use crate::analysis::AnalyzedFile;
use crate::{CheckResult, Severity};
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;

use crate::discover::{join_under_root, parent_dir, path_is_under};
use crate::facts::{SidecarViolation, TestFileKind, TestRootFacts};
use crate::inputs::SidecarViolationInput;
use crate::parse;

const ID: &str = "RS-TEST-02";

pub(crate) fn collect(
    tree: &ProjectTree,
    root: &TestRootFacts,
    files: &[AnalyzedFile],
    scoped_files: Option<&BTreeSet<String>>,
    results: &mut Vec<CheckResult>,
) {
    let violations = collect_violations(tree, root, files, scoped_files);
    if violations.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "owned sidecar shape confirmed".to_owned(),
                format!(
                    "Root `{}` keeps test harnesses inside modules as `<module>/tests/`.",
                    root.rel_dir
                ),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
    for violation in violations {
        check(&SidecarViolationInput::new(&violation), results);
    }
}

pub fn check(input: &SidecarViolationInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        input.violation.title.clone(),
        input.violation.message.clone(),
        Some(input.violation.rel_path.clone()),
        input.violation.line,
        false,
    ));
}

fn collect_violations(
    tree: &ProjectTree,
    root: &TestRootFacts,
    files: &[AnalyzedFile],
    scoped_files: Option<&BTreeSet<String>>,
) -> Vec<SidecarViolation> {
    let mut violations = Vec::new();
    let src_roots = src_roots_for_root(root);
    let mut all_dirs: Vec<(String, String)> = Vec::new();
    for src_root in &src_roots {
        let mut stack = vec![src_root.clone()];
        while let Some(dir_rel) = stack.pop() {
            if let Some(entry) = tree.dir_contents(&dir_rel) {
                for child in entry.dirs() {
                    stack.push(ProjectTree::join_rel(&dir_rel, child));
                }
            }
            all_dirs.push((dir_rel, src_root.clone()));
        }
    }
    all_dirs.sort_by(|(a, _), (b, _)| a.cmp(b));
    for (dir_rel, src_root) in &all_dirs {
        let rel_after_src = dir_rel
            .strip_prefix(src_root.as_str())
            .and_then(|rest| rest.strip_prefix('/'))
            .unwrap_or("");
        let owner_module_rel_path = owned_sidecar_owner_rel_path(src_root, rel_after_src);
        if scoped_files.is_some_and(|paths| {
            !paths.iter().any(|path| path_is_under(path, &dir_rel))
                && !paths.contains(&format!("{dir_rel}/mod.rs"))
                && owner_module_rel_path
                    .as_ref()
                    .is_none_or(|owner_rel_path| !paths.contains(owner_rel_path))
        }) {
            continue;
        }
        // Forbid ad-hoc src/tests/ trees, but allow module/tests/ (the inside pattern).
        if rel_after_src == "tests" || rel_after_src.starts_with("tests/") {
            violations.push(SidecarViolation {
                rel_path: dir_rel.clone(),
                line: None,
                title: "ad hoc src/tests tree".to_owned(),
                message: "Internal test harnesses must live inside their module as `<module>/tests/`, not under `src/tests/`.".to_owned(),
            });
            continue;
        }

        let Some(owner_module_rel_path) = owner_module_rel_path else {
            continue;
        };
        let mod_rel_path = format!("{dir_rel}/mod.rs");
        if !tree.file_exists(&mod_rel_path) {
            violations.push(SidecarViolation {
                rel_path: dir_rel.clone(),
                line: None,
                title: "sidecar directory missing mod.rs".to_owned(),
                message:
                    "Internal sidecar harness directories must expose `mod.rs` as their entrypoint."
                        .to_owned(),
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
            TestFileKind::Source
                | TestFileKind::InternalSidecarMod
                | TestFileKind::InternalSidecarSupport
        ) {
            continue;
        }

        if is_flat_test_sidecar(&file.facts.rel_path) {
            violations.push(SidecarViolation {
                rel_path: file.facts.rel_path.clone(),
                line: None,
                title: "flat sidecar test file".to_owned(),
                message: "Internal test harnesses must live inside their module as `<module>/tests/mod.rs`, not as flat `*_tests.rs` files or `<module>_tests/` sidecars.".to_owned(),
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
                    message: "Declaration-only `#[cfg(test)]` modules must be `mod tests;` resolving to `tests/mod.rs` inside the module directory.".to_owned(),
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

    // Only pattern: `module/tests/` — owner is `module/mod.rs`.
    if dir_name == "tests" {
        let module_rel = parent_dir(rel_after_src);
        if module_rel.is_empty() {
            return None; // src/tests/ is ad-hoc, not inside-module.
        }
        return Some(format!("{src_root}/{module_rel}/mod.rs"));
    }

    // Any _tests/ directory is a violation — must use module/tests/ instead.
    if dir_name.ends_with("_tests") {
        return None;
    }

    None
}

fn is_flat_test_sidecar(rel_path: &str) -> bool {
    rel_path.ends_with("_tests.rs")
        || rel_path.ends_with("_test.rs")
        || rel_path.ends_with("/tests.rs")
}

fn cfg_test_decl_is_owned_sidecar(
    tree: &ProjectTree,
    file_rel_path: &str,
    _owner_module_name: Option<&str>,
    module: &parse::CfgTestModuleInfo,
) -> bool {
    // Only valid pattern: `#[cfg(test)] mod tests;` resolving to `tests/mod.rs`
    // inside the module's own directory. No #[path], no _tests suffix.
    if module.name != "tests" {
        return false;
    }
    if module.path_attr.is_some() {
        return false; // #[path] is forbidden.
    }

    let parent = parent_dir(file_rel_path);
    let tests_mod_rs = format!("{parent}/tests/mod.rs");
    tree.file_exists(&tests_mod_rs)
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    crate::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]

mod rs_test_02_owned_sidecar_shape_tests;
