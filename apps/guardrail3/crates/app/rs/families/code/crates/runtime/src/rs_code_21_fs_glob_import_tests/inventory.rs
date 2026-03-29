use std::collections::BTreeSet;

use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_21_fs_glob_import::{
    RuleFinding, Severity, assert_files, assert_findings,
};
use test_support::write_file;

#[test]
fn attacks_std_fs_glob_imports_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/adapters/outbound/postgres/src/lib.rs";
    let app_rel = "apps/backend/crates/app/queries/src/lib.rs";

    let backend_content = test_support::read_file(root, backend_rel);
    let app_content = test_support::read_file(root, app_rel);

    write_file(
        root,
        backend_rel,
        &format!("use std::fs::*;\n{backend_content}"),
    );
    write_file(
        root,
        app_rel,
        &format!("use std::{{fs::*, path::PathBuf}};\n{app_content}"),
    );

    let results = run_family(root);
    assert_files(
        &results,
        BTreeSet::from([backend_rel.to_owned(), app_rel.to_owned()]),
    );
    assert_findings(
        &results,
        &[
            RuleFinding {
                severity: Severity::Error,
                title: "std::fs glob import",
                message: "Direct `use std::fs::*` glob import bypasses clippy method bans.",
                file: Some(backend_rel),
                line: Some(1),
                inventory: false,
            },
            RuleFinding {
                severity: Severity::Error,
                title: "std::fs glob import",
                message: "Direct `use std::fs::*` glob import bypasses clippy method bans.",
                file: Some(app_rel),
                line: Some(1),
                inventory: false,
            },
        ],
    );
}

#[test]
fn attacks_grouped_glob_bypass_across_golden_tree() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let target_rel = "apps/backend/crates/adapters/outbound/postgres/src/lib.rs";
    let target_content = test_support::read_file(root, target_rel);

    write_file(
        root,
        target_rel,
        &format!("use std::fs::{{self, *}};\n{target_content}"),
    );

    let results = run_family(root);

    assert_files(&results, BTreeSet::from([target_rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding {
            severity: Severity::Error,
            title: "std::fs glob import",
            message: "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            file: Some(target_rel),
            line: Some(1),
            inventory: false,
        }],
    );
}

#[test]
fn attacks_inline_module_glob_across_golden_tree() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let target_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let target_content = test_support::read_file(root, target_rel);

    let mutated = format!(
        "{target_content}\nmod bypass {{\n    use std::fs::*;\n    pub fn probe() {{}}\n}}\n"
    );
    write_file(root, target_rel, &mutated);

    let results = run_family(root);
    let inline_line = mutated
        .lines()
        .position(|line| line.contains("use std::fs::*;"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_files(&results, BTreeSet::from([target_rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding {
            severity: Severity::Error,
            title: "std::fs glob import",
            message: "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            file: Some(target_rel),
            line: Some(inline_line),
            inventory: false,
        }],
    );
}

#[test]
fn attacks_alias_then_glob_across_owned_file() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let target_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let target_content = test_support::read_file(root, target_rel);

    write_file(
        root,
        target_rel,
        &format!("use std::fs::{{File as Alias, *}};\n{target_content}"),
    );

    let results = run_family(root);

    assert_files(&results, BTreeSet::from([target_rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding {
            severity: Severity::Error,
            title: "std::fs glob import",
            message: "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            file: Some(target_rel),
            line: Some(1),
            inventory: false,
        }],
    );
}

#[test]
fn attacks_function_local_glob_import_in_owned_file() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let target_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let target_content = test_support::read_file(root, target_rel);
    let mutated = format!(
        "{target_content}\nfn glob_probe() {{\n    use std::fs::*;\n    let _ = read_to_string(\"fixture.txt\");\n}}\n"
    );
    write_file(root, target_rel, &mutated);

    let results = run_family(root);
    let glob_line = mutated
        .lines()
        .position(|line| line.contains("use std::fs::*;"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_files(&results, BTreeSet::from([target_rel.to_owned()]));
    assert_findings(
        &results,
        &[RuleFinding {
            severity: Severity::Error,
            title: "std::fs glob import",
            message: "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            file: Some(target_rel),
            line: Some(glob_line),
            inventory: false,
        }],
    );
}
