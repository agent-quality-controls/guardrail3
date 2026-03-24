use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::app::rs::checks::hooks::rs::check;
use crate::app::rs::checks::hooks::rs::test_support::StubToolChecker;
use crate::domain::project_tree::{DirEntry, ProjectTree};

fn hook_tree(pre_commit: &str) -> ProjectTree {
    ProjectTree {
        root: PathBuf::from("/tmp/project"),
        structure: BTreeMap::from([(
            ".githooks".to_owned(),
            DirEntry {
                dirs: Vec::new(),
                files: vec!["pre-commit".to_owned()],
                symlink_dirs: Vec::new(),
                symlink_files: Vec::new(),
            },
        )]),
        content: BTreeMap::from([(".githooks/pre-commit".to_owned(), pre_commit.to_owned())]),
    }
}

#[test]
fn orchestrator_skips_hook_rs_14_when_guardrail_validation_is_not_expected() {
    let tree = hook_tree("echo noop\n");

    let results = check(&tree, &StubToolChecker::new(&[]));

    assert!(results.iter().all(|result| result.id != "HOOK-RS-14"));
}

#[test]
fn orchestrator_marks_hook_rs_14_present_for_path_qualified_guardrail_validation() {
    let tree = hook_tree("/usr/local/bin/guardrail3 rs validate --staged .\n");

    let results = check(&tree, &StubToolChecker::new(&[]));
    let result = results
        .iter()
        .find(|result| result.id == "HOOK-RS-14")
        .expect("expected HOOK-RS-14 result");

    assert!(result.inventory);
}

#[test]
fn orchestrator_skips_hook_rs_15_when_cargo_dupes_is_not_required() {
    let tree = hook_tree("echo noop\n");

    let results = check(&tree, &StubToolChecker::new(&[]));

    assert!(results.iter().all(|result| result.id != "HOOK-RS-15"));
}

#[test]
fn orchestrator_marks_hook_rs_15_present_for_path_qualified_cargo_dupes() {
    let tree = hook_tree("/usr/local/bin/cargo-dupes check --exclude-tests\n");

    let results = check(&tree, &StubToolChecker::new(&[]));
    let result = results
        .iter()
        .find(|result| result.id == "HOOK-RS-15")
        .expect("expected HOOK-RS-15 result");

    assert!(result.inventory);
}

#[test]
fn orchestrator_marks_hook_rs_15_present_for_wrapped_path_qualified_cargo_dupes() {
    let tree = hook_tree("exec /usr/local/bin/cargo-dupes check --exclude-tests\n");

    let results = check(&tree, &StubToolChecker::new(&[]));
    let result = results
        .iter()
        .find(|result| result.id == "HOOK-RS-15")
        .expect("expected HOOK-RS-15 result");

    assert!(result.inventory);
}

#[test]
fn orchestrator_does_not_treat_wrapper_prose_as_path_qualified_cargo_dupes() {
    let tree = hook_tree(
        "cargo dupes check --exclude-tests\nbash -lc 'echo /usr/local/bin/cargo-dupes check --exclude-tests'\n",
    );

    let results = check(&tree, &StubToolChecker::new(&[]));
    let result = results
        .iter()
        .find(|result| result.id == "HOOK-RS-15")
        .expect("expected HOOK-RS-15 result");

    assert!(!result.inventory);
}
