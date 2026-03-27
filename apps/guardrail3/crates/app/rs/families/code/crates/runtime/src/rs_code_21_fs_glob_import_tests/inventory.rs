use std::collections::BTreeSet;

use super::super::super::test_support::{copy_fixture, files_for_rule, run_family, write_file};
use guardrail3_domain_report::Severity;

#[test]
fn attacks_std_fs_glob_imports_across_multiple_owned_files() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/adapters/outbound/postgres/src/lib.rs";
    let devctl_rel = "apps/devctl/crates/adapters/outbound/fs/src/lib.rs";

    let backend_content =
        std::fs::read_to_string(root.join(backend_rel)).expect("read backend source");
    let devctl_content =
        std::fs::read_to_string(root.join(devctl_rel)).expect("read devctl source");

    write_file(
        root,
        backend_rel,
        &format!("use std::fs::*;\n{backend_content}"),
    );
    write_file(
        root,
        devctl_rel,
        &format!("use std::{{fs::*, path::PathBuf}};\n{devctl_content}"),
    );

    let results = run_family(root);
    let mut rs_code_21_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-21")
        .map(|result| {
            (
                result.file.clone(),
                result.line,
                format!("{:?}", result.severity),
                result.title.clone(),
                result.message.clone(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();
    rs_code_21_results.sort();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-21"),
        BTreeSet::from([backend_rel.to_owned(), devctl_rel.to_owned()])
    );
    assert_eq!(
        rs_code_21_results,
        vec![
            (
                Some(backend_rel.to_owned()),
                Some(1),
                format!("{:?}", Severity::Error),
                "std::fs glob import".to_owned(),
                "Direct `use std::fs::*` glob import bypasses clippy method bans.".to_owned(),
                false,
            ),
            (
                Some(devctl_rel.to_owned()),
                Some(1),
                format!("{:?}", Severity::Error),
                "std::fs glob import".to_owned(),
                "Direct `use std::fs::*` glob import bypasses clippy method bans.".to_owned(),
                false,
            ),
        ]
    );
}

#[test]
fn attacks_grouped_glob_bypass_across_golden_tree() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let target_rel = "apps/backend/crates/adapters/outbound/postgres/src/lib.rs";
    let target_content = std::fs::read_to_string(root.join(target_rel)).expect("read source");

    write_file(
        root,
        target_rel,
        &format!("use std::fs::{{self, *}};\n{target_content}"),
    );

    let results = run_family(root);
    let rs_code_21_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-21")
        .map(|result| {
            (
                result.file.clone(),
                result.line,
                result.severity,
                result.title.clone(),
                result.message.clone(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-21"),
        BTreeSet::from([target_rel.to_owned()])
    );
    assert_eq!(
        rs_code_21_results,
        vec![(
            Some(target_rel.to_owned()),
            Some(1),
            Severity::Error,
            "std::fs glob import".to_owned(),
            "Direct `use std::fs::*` glob import bypasses clippy method bans.".to_owned(),
            false,
        )]
    );
}

#[test]
fn attacks_inline_module_glob_across_golden_tree() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let target_rel = "apps/devctl/crates/adapters/outbound/fs/src/lib.rs";
    let target_content = std::fs::read_to_string(root.join(target_rel)).expect("read source");

    let mutated = format!(
        "{target_content}\nmod bypass {{\n    use std::fs::*;\n    pub fn probe() {{}}\n}}\n"
    );
    write_file(root, target_rel, &mutated);

    let results = run_family(root);
    let inline_line = mutated
        .lines()
        .position(|line| line.contains("use std::fs::*;"))
        .expect("inline glob line")
        + 1;
    let rs_code_21_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-21")
        .map(|result| {
            (
                result.file.clone(),
                result.line,
                result.severity,
                result.title.clone(),
                result.message.clone(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-21"),
        BTreeSet::from([target_rel.to_owned()])
    );
    assert_eq!(
        rs_code_21_results,
        vec![(
            Some(target_rel.to_owned()),
            Some(inline_line),
            Severity::Error,
            "std::fs glob import".to_owned(),
            "Direct `use std::fs::*` glob import bypasses clippy method bans.".to_owned(),
            false,
        )]
    );
}

#[test]
fn attacks_function_local_glob_import_in_owned_file() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let target_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let target_content = std::fs::read_to_string(root.join(target_rel)).expect("read source");
    let mutated = format!(
        "{target_content}\nfn glob_probe() {{\n    use std::fs::*;\n    let _ = read_to_string(\"fixture.txt\");\n}}\n"
    );
    write_file(root, target_rel, &mutated);

    let results = run_family(root);
    let glob_line = mutated
        .lines()
        .position(|line| line.contains("use std::fs::*;"))
        .expect("function-local glob line")
        + 1;
    let rs_code_21_results = results
        .iter()
        .filter(|result| result.id == "RS-CODE-21")
        .map(|result| {
            (
                result.file.clone(),
                result.line,
                result.severity,
                result.title.clone(),
                result.message.clone(),
                result.inventory,
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        files_for_rule(&results, "RS-CODE-21"),
        BTreeSet::from([target_rel.to_owned()])
    );
    assert_eq!(
        rs_code_21_results,
        vec![(
            Some(target_rel.to_owned()),
            Some(glob_line),
            Severity::Error,
            "std::fs glob import".to_owned(),
            "Direct `use std::fs::*` glob import bypasses clippy method bans.".to_owned(),
            false,
        )]
    );
}
