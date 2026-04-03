use std::collections::{BTreeMap, BTreeSet};

use super::helpers::{DirEntry, ProjectTree, run_tree};
use guardrail3_adapters_outbound_fs::RealFileSystem;
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_app_core::project_walker::walk_project;
use guardrail3_app_rs_family_code_assertions::lint_policy::rs_code_30_input_failures::{
    assert_files, assert_guardrail_policy_parse_failure, assert_message_fragment_failure,
    assert_no_hits, assert_source_parse_failure,
};
use test_support::{create_dir_all, create_temp_dir, write_path};

fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry::new(
        dirs.iter().map(|dir| (*dir).to_owned()).collect(),
        files.iter().map(|file| (*file).to_owned()).collect(),
        Vec::new(),
        Vec::new(),
    )
}

fn project_tree(
    root: &std::path::Path,
    structure: BTreeMap<String, DirEntry>,
    content: BTreeMap<String, String>,
) -> ProjectTree {
    ProjectTree::build(root.to_path_buf(), &structure, &content, &["".to_owned()], &[], &[], None, &[])
}

#[test]
fn family_surfaces_source_parse_failures_with_exact_owned_hit_set() {
    let root = create_temp_dir("rs-code-30-source-parse-failure");
    let source_rel = "src/lib.rs";
    let source_abs = root.path().join(source_rel);
    create_dir_all(source_abs.parent().unwrap_or(root.path()));
    write_path(&source_abs, "fn broken( {");

    let tree = project_tree(
        root.path(),
        BTreeMap::from([
            (String::new(), dir_entry(&["src"], &["Cargo.toml"])),
            ("src".to_owned(), dir_entry(&[], &["lib.rs"])),
        ]),
        BTreeMap::from([(
            "Cargo.toml".to_owned(),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n".to_owned(),
        )]),
    );

    let results = run_tree(&tree);

    assert_files(&results, BTreeSet::from([source_rel.to_owned()]));
    assert_source_parse_failure(&results, source_rel);
}

#[test]
fn family_surfaces_guardrail_policy_parse_failures_with_exact_owned_hit_set() {
    let root = create_temp_dir("rs-code-30-guardrail-parse-failure");
    let source_rel = "src/lib.rs";
    let source_abs = root.path().join(source_rel);
    create_dir_all(source_abs.parent().unwrap_or(root.path()));
    write_path(
        &source_abs,
        "pub fn parse() -> Result<(), String> { Ok(()) }",
    );

    let tree = project_tree(
        root.path(),
        BTreeMap::from([
            (
                String::new(),
                dir_entry(&["src"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("src".to_owned(), dir_entry(&[], &["lib.rs"])),
        ]),
        BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n".to_owned(),
            ),
            (
                "guardrail3.toml".to_owned(),
                "[rust.packages\n type = \"library\"\n".to_owned(),
            ),
        ]),
    );

    let results = run_tree(&tree);

    assert_files(&results, BTreeSet::from(["guardrail3.toml".to_owned()]));
    assert_guardrail_policy_parse_failure(&results, "guardrail3.toml");
}

#[test]
fn family_surfaces_unreadable_root_cargo_for_code_root_discovery() {
    let root = create_temp_dir("rs-code-30-cargo-read-failure");
    let source_rel = "src/lib.rs";
    let source_abs = root.path().join(source_rel);
    create_dir_all(source_abs.parent().unwrap_or(root.path()));
    write_path(&source_abs, "pub fn parse() {}\n");

    let tree = project_tree(
        root.path(),
        BTreeMap::from([
            (String::new(), dir_entry(&["src"], &["Cargo.toml"])),
            ("src".to_owned(), dir_entry(&[], &["lib.rs"])),
        ]),
        BTreeMap::new(),
    );

    let results = run_tree(&tree);

    assert_files(&results, BTreeSet::from(["Cargo.toml".to_owned()]));
    assert_message_fragment_failure(
        &results,
        "Cargo.toml",
        "Failed to read Cargo.toml for code-family root discovery.",
    );
}

#[test]
fn family_surfaces_unreadable_guardrail_policy_input() {
    let root = create_temp_dir("rs-code-30-guardrail-read-failure");
    let source_rel = "src/lib.rs";
    let source_abs = root.path().join(source_rel);
    create_dir_all(source_abs.parent().unwrap_or(root.path()));
    write_path(&source_abs, "pub fn parse() {}\n");

    let tree = project_tree(
        root.path(),
        BTreeMap::from([
            (
                String::new(),
                dir_entry(&["src"], &["Cargo.toml", "guardrail3.toml"]),
            ),
            ("src".to_owned(), dir_entry(&[], &["lib.rs"])),
        ]),
        BTreeMap::from([(
            "Cargo.toml".to_owned(),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n".to_owned(),
        )]),
    );

    let results = run_tree(&tree);

    assert_files(&results, BTreeSet::from(["guardrail3.toml".to_owned()]));
    assert_message_fragment_failure(
        &results,
        "guardrail3.toml",
        "Failed to read guardrail3.toml for code-family policy resolution.",
    );
}

#[test]
fn family_surfaces_unreadable_active_config_for_exception_comment_discovery() {
    let root = create_temp_dir("rs-code-30-config-comment-read-failure");
    let source_rel = "apps/backend/src/lib.rs";
    let source_abs = root.path().join(source_rel);
    create_dir_all(source_abs.parent().unwrap_or(root.path()));
    write_path(&source_abs, "pub fn parse() {}\n");

    let tree = project_tree(
        root.path(),
        BTreeMap::from([
            (String::new(), dir_entry(&["apps"], &["guardrail3.toml"])),
            ("apps".to_owned(), dir_entry(&["backend"], &[])),
            (
                "apps/backend".to_owned(),
                dir_entry(&["src"], &["Cargo.toml", "rustfmt.toml"]),
            ),
            ("apps/backend/src".to_owned(), dir_entry(&[], &["lib.rs"])),
        ]),
        BTreeMap::from([
            (
                "apps/backend/Cargo.toml".to_owned(),
                "[workspace]\nmembers = []\nresolver = \"2\"\n".to_owned(),
            ),
            (
                "guardrail3.toml".to_owned(),
                "[rust.checks]\ncode = true\n".to_owned(),
            ),
        ]),
    );

    let results = run_tree(&tree);

    assert_files(
        &results,
        BTreeSet::from(["apps/backend/rustfmt.toml".to_owned()]),
    );
    assert_message_fragment_failure(
        &results,
        "apps/backend/rustfmt.toml",
        "Failed to read config file for exception-comment discovery.",
    );
}

#[test]
fn stays_quiet_when_validation_scope_selects_zero_code_roots() {
    let root = create_temp_dir("rs-code-30-zero-active-roots");
    write_path(
        &root.path().join("guardrail3.toml"),
        "[rust.checks\ncode = true\n",
    );
    write_path(
        &root.path().join("apps/backend/Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    write_path(
        &root.path().join("apps/backend/src/lib.rs"),
        "pub fn ready() {}\n",
    );
    write_path(
        &root.path().join("docs/readme.md"),
        "# scope outside rust roots\n",
    );

    let tree = walk_project(&RealFileSystem, root.path());
    let structure = guardrail3_app_rs_structure::collect(tree.clone(), &[]);
        let legality = guardrail3_app_rs_legality::collect(structure);
    let selected = guardrail3_validation_model::RustFamilySelection::new(BTreeSet::from([
        guardrail3_validation_model::RustValidateFamily::Code,
    ]));
    let route =
        guardrail3_app_rs_family_mapper::FamilyMapper::from_legality(&legality, None, &selected, None)
            .with_validation_scope(Some("docs"))
            .map_rs_code();

    let surface = FamilyView::build(
        tree.root().clone(), tree.structure(), tree.content(),
        &["".to_owned()], &[], &[], None, &[],
    );
    let results = crate::check(&surface, &route);
    assert_no_hits(&results);
}
