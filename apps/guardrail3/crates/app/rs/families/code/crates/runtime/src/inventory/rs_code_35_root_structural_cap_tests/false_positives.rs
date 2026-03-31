use std::collections::BTreeSet;

use super::super::run_family;
use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_app_rs_family_code_assertions::rs_code_35_root_structural_cap::assert_no_hits;
use guardrail3_app_rs_family_mapper::RsProjectSurface;
use test_support::{create_temp_dir, write_file};

#[test]
fn stays_quiet_at_exact_thresholds() {
    let tmp = create_temp_dir("rs-code-35-threshold");
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(tmp.path(), "src/lib.rs", "");
    for index in 0..11 {
        write_file(tmp.path(), &format!("src/dir{index}/mod.rs"), "");
    }
    for index in 0..19 {
        write_file(tmp.path(), &format!("src/file{index}.rs"), "");
    }
    write_file(tmp.path(), "src/a/b/c/d/e/mod.rs", "");

    assert_no_hits(&run_family(tmp.path()));
}

#[test]
fn stays_quiet_when_root_is_scoped_to_one_file() {
    let tmp = create_temp_dir("rs-code-35-file-scoped");
    write_file(
        tmp.path(),
        "Cargo.toml",
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(tmp.path(), "src/lib.rs", "");
    for index in 0..13 {
        write_file(tmp.path(), &format!("src/dir{index}/mod.rs"), "");
    }
    for index in 0..21 {
        write_file(tmp.path(), &format!("src/file{index}.rs"), "");
    }
    write_file(tmp.path(), "src/a/b/c/d/e/f/mod.rs", "");

    let tree = walk_project(&RealFileSystem, tmp.path());
    let scope = guardrail3_app_rs_structure::collect(&tree);
    let selected = guardrail3_validation_model::RustFamilySelection::new(BTreeSet::from([
        guardrail3_validation_model::RustValidateFamily::Code,
    ]));
    let scoped_files = BTreeSet::from(["src/lib.rs".to_owned()]);
    let route = guardrail3_app_rs_family_mapper::FamilyMapper::new(
        &tree,
        &scope,
        None,
        &selected,
        Some(&scoped_files),
    )
    .map_rs_code();

    let results = crate::check(&RsProjectSurface::from_tree(&tree), &route);
    assert_no_hits(&results);
}
