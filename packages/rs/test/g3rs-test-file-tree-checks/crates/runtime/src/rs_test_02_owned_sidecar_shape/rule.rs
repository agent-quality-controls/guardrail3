use std::collections::{BTreeMap, BTreeSet};

use g3rs_test_types::G3RsTestAnalyzedSourceFile;
use g3rs_test_types::G3RsTestFileKind;
use g3rs_test_types::G3RsTestFileTreeChecksInput;
use g3rs_test_types::ast::CfgTestModuleInfo;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-TEST-FILETREE-02";

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

pub(crate) fn collect(
    input: &G3RsTestFileTreeChecksInput,
    files: &[G3RsTestAnalyzedSourceFile],
    results: &mut Vec<G3CheckResult>,
) {
    let violations = collect_violations(input, files);
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

fn collect_violations(
    input: &G3RsTestFileTreeChecksInput,
    files: &[G3RsTestAnalyzedSourceFile],
) -> Vec<SidecarViolation> {
    let file_set = files
        .iter()
        .map(|file| file.rel_path.clone())
        .collect::<BTreeSet<_>>();
    let src_roots = collect_src_roots(input, files);
    let sidecar_dirs = collect_sidecar_dirs(files, &src_roots);
    let mut violations = Vec::new();

    for (dir_rel, src_root) in sidecar_dirs {
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

        let Some(owner_module_rel_path) = owned_sidecar_owner_rel_path(&src_root, rel_after_src)
        else {
            continue;
        };
        let mod_rel_path = format!("{dir_rel}/mod.rs");
        if !file_set.contains(&mod_rel_path) {
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
        if !file_set.contains(&owner_module_rel_path) {
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
            if cfg_test_decl_is_owned_sidecar(&file.rel_path, module, &file_set) {
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

fn collect_src_roots(
    input: &G3RsTestFileTreeChecksInput,
    files: &[G3RsTestAnalyzedSourceFile],
) -> BTreeSet<String> {
    let mut roots = input
        .components
        .iter()
        .map(|component| join_under_root(&component.runtime_rel_dir, "src"))
        .collect::<BTreeSet<_>>();

    let root_src = join_under_root(&input.root_rel_dir, "src");
    if files
        .iter()
        .any(|file| file.component_rel_dir.is_none() && path_is_under(&file.rel_path, &root_src))
    {
        let _ = roots.insert(root_src);
    }

    roots
}

fn collect_sidecar_dirs(
    files: &[G3RsTestAnalyzedSourceFile],
    src_roots: &BTreeSet<String>,
) -> BTreeMap<String, String> {
    let mut dirs = BTreeMap::new();

    for file in files {
        for src_root in src_roots {
            if !path_is_under(&file.rel_path, src_root) {
                continue;
            }
            let mut current = parent_dir(&file.rel_path).to_owned();
            while !current.is_empty() && current != *src_root {
                let _ = dirs.insert(current.clone(), src_root.clone());
                current = parent_dir(&current).to_owned();
            }
        }
    }

    dirs
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
    file_set: &BTreeSet<String>,
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
    if !file_set.contains(&expected_mod_rel) {
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
