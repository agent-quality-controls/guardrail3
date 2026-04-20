use g3rs_arch_ingestion_assertions::file_tree as assertions;
use guardrail3_check_types::G3Severity;

use super::helpers::{
    file_tree_input, file_tree_results, make_dir, temp_workspace_root, write_file,
};

#[test]
fn file_tree_ingestion_threads_structural_split_waivers_from_rust_policy() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crate_a\"]\n",
    );
    write_file(
        &root,
        "guardrail3-rs.toml",
        "[[waivers]]\nrule = \"RS-ARCH-FILETREE-07\"\nfile = \"crate_a/Cargo.toml\"\nselector = \"structural-split\"\nreason = \"Rule runtime crate intentionally aggregates one rule per file and is the package boundary by design.\"\n",
    );
    make_dir(&root, "crate_a/src/deep/a/b/c");
    write_file(
        &root,
        "crate_a/Cargo.toml",
        "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "crate_a/src/lib.rs", "pub mod api;\n");

    let input = file_tree_input(&root);
    assertions::assert_parsed_rust_policy(
        &input.rust_policy,
        "RS-ARCH-FILETREE-07",
        "crate_a/Cargo.toml",
        "structural-split",
    );
}

#[test]
fn file_tree_pipeline_reports_missing_facade_and_complexity() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crate_a\"]\n",
    );
    make_dir(&root, "crate_a/src/deep/a/b/c");
    write_file(
        &root,
        "crate_a/Cargo.toml",
        "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n\n[dependencies]\none = \"1\"\ntwo = \"1\"\nthree = \"1\"\nfour = \"1\"\nfive = \"1\"\nsix = \"1\"\nseven = \"1\"\neight = \"1\"\nnine = \"1\"\nten = \"1\"\neleven = \"1\"\ntwelve = \"1\"\nthirteen = \"1\"\n",
    );
    write_file(&root, "crate_a/src/api.rs", "pub struct Api;\n");
    write_file(&root, "crate_a/src/one.rs", "pub struct One;\n");
    write_file(&root, "crate_a/src/two.rs", "pub struct Two;\n");
    write_file(&root, "crate_a/src/three.rs", "pub struct Three;\n");
    write_file(&root, "crate_a/src/four.rs", "pub struct Four;\n");
    write_file(&root, "crate_a/src/five.rs", "pub struct Five;\n");
    write_file(&root, "crate_a/src/six.rs", "pub struct Six;\n");
    write_file(&root, "crate_a/src/seven.rs", "pub struct Seven;\n");
    write_file(&root, "crate_a/src/eight.rs", "pub struct Eight;\n");
    write_file(&root, "crate_a/src/nine.rs", "pub struct Nine;\n");
    write_file(&root, "crate_a/src/ten.rs", "pub struct Ten;\n");
    write_file(&root, "crate_a/src/eleven.rs", "pub struct Eleven;\n");
    write_file(&root, "crate_a/src/deep/a/b/c/mod.rs", "pub struct Deep;\n");

    let results = file_tree_results(&root);
    assertions::assert_has_result(
        &results,
        "RS-ARCH-FILETREE-01",
        G3Severity::Error,
        Some("crate_a/Cargo.toml"),
    );
    assertions::assert_has_result(
        &results,
        "RS-ARCH-FILETREE-07",
        G3Severity::Error,
        Some("crate_a/Cargo.toml"),
    );
}

#[test]
fn file_tree_pipeline_reports_missing_mod_rs() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crate_a\"]\n",
    );
    make_dir(&root, "crate_a/src/nested");
    write_file(
        &root,
        "crate_a/Cargo.toml",
        "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "crate_a/src/lib.rs", "pub mod nested;\n");
    write_file(&root, "crate_a/src/nested/thing.rs", "pub struct Thing;\n");

    let results = file_tree_results(&root);
    assertions::assert_has_result(
        &results,
        "RS-ARCH-FILETREE-03",
        G3Severity::Error,
        Some("crate_a/src/lib.rs"),
    );
}

#[test]
fn file_tree_ingestion_stays_inside_the_pointed_workspace() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crate_a\"]\n",
    );
    make_dir(&root, "crate_a/src/nested");
    make_dir(&root, "foreign/src/nested");
    write_file(
        &root,
        "crate_a/Cargo.toml",
        "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "crate_a/src/lib.rs", "pub mod nested;\n");
    write_file(&root, "crate_a/src/nested/thing.rs", "pub struct Thing;\n");
    write_file(
        &root,
        "foreign/Cargo.toml",
        "[package]\nname = \"foreign\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "foreign/src/lib.rs", "pub mod nested;\n");
    write_file(&root, "foreign/src/nested/thing.rs", "pub struct Thing;\n");

    let input = file_tree_input(&root);

    assert_eq!(input.crates.len(), 1);
    assert_eq!(input.crates[0].rel_dir, "crate_a");
    assert!(
        input
            .module_dirs
            .iter()
            .all(|module_dir| module_dir.dir_rel.starts_with("crate_a/"))
    );
}

#[test]
fn file_tree_complexity_ignores_excluded_nested_crates_for_root_level_layouts() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"pkg\", \"pkg/crates/inner\"]\nexclude = [\"pkg/crates/inner\"]\n",
    );
    make_dir(&root, "pkg/crates/inner/src/deep/a/b/c");
    write_file(
        &root,
        "pkg/Cargo.toml",
        "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\n\n[lib]\npath = \"lib.rs\"\n",
    );
    write_file(&root, "pkg/lib.rs", "pub mod api;\n");
    write_file(&root, "pkg/api.rs", "pub struct Api;\n");
    write_file(
        &root,
        "pkg/crates/inner/Cargo.toml",
        "[package]\nname = \"inner\"\nversion = \"0.1.0\"\n",
    );
    write_file(
        &root,
        "pkg/crates/inner/src/deep/a/b/c/mod.rs",
        "pub struct Deep;\n",
    );

    let results = file_tree_results(&root);
    assertions::assert_missing_result(&results, "RS-ARCH-FILETREE-07");
}

#[test]
fn file_tree_pipeline_reports_nested_rs_file_piles() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crate_a\"]\n",
    );
    make_dir(&root, "crate_a/src/nested");
    write_file(
        &root,
        "crate_a/Cargo.toml",
        "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "crate_a/src/lib.rs", "pub mod nested;\n");
    write_file(&root, "crate_a/src/nested/mod.rs", "pub struct Root;\n");
    for name in [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
    ] {
        write_file(
            &root,
            &format!("crate_a/src/nested/{name}.rs"),
            "pub struct Item;\n",
        );
    }

    let results = file_tree_results(&root);
    assertions::assert_has_result(
        &results,
        "RS-ARCH-FILETREE-07",
        G3Severity::Error,
        Some("crate_a/Cargo.toml"),
    );
}

#[test]
fn file_tree_pipeline_reports_nested_directory_piles() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crate_a\"]\n",
    );
    make_dir(&root, "crate_a/src/nested");
    write_file(
        &root,
        "crate_a/Cargo.toml",
        "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "crate_a/src/lib.rs", "pub mod nested;\n");
    write_file(&root, "crate_a/src/nested/mod.rs", "pub struct Root;\n");
    for name in ["a", "b", "c", "d", "e"] {
        make_dir(&root, &format!("crate_a/src/nested/{name}"));
        write_file(
            &root,
            &format!("crate_a/src/nested/{name}/mod.rs"),
            "pub struct Item;\n",
        );
    }

    let results = file_tree_results(&root);
    assertions::assert_has_result(
        &results,
        "RS-ARCH-FILETREE-07",
        G3Severity::Error,
        Some("crate_a/Cargo.toml"),
    );
}
