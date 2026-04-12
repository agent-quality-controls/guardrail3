use std::fs;

use g3rs_arch_config_checks::check as check_config;
use g3rs_arch_file_tree_checks::check as check_file_tree;
use g3rs_arch_source_checks::check as check_source;
use g3rs_workspace_crawl::crawl;
use tempfile::tempdir;

#[test]
fn source_pipeline_reports_lib_body_logic_and_path_attr() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crate_a"]
"#,
    )
    .expect("root cargo");
    fs::create_dir_all(root.path().join("crate_a/src/nested")).expect("crate dirs");
    fs::write(
        root.path().join("crate_a/Cargo.toml"),
        r#"
[package]
name = "crate_a"
version = "0.1.0"

[features]
all = ["api"]
default = ["all"]
api = []
"#,
    )
    .expect("crate cargo");
    fs::write(
        root.path().join("crate_a/src/lib.rs"),
        r#"
pub fn leaked() {}

#[path = "nested/custom.rs"]
pub mod nested;
"#,
    )
    .expect("lib");
    fs::write(root.path().join("crate_a/src/nested/custom.rs"), "pub struct Value;")
        .expect("custom");

    let crawl = crawl(root.path()).expect("crawl");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");
    let results = check_source(&inputs[0]);

    assert!(results.iter().any(|result| result.id() == "RS-ARCH-SOURCE-02"));
    assert!(results.iter().any(|result| result.id() == "RS-ARCH-SOURCE-09"));
}

#[test]
fn source_pipeline_reports_only_source_half_of_feature_gating_rule() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crate_a"]
"#,
    )
    .expect("root cargo");
    fs::create_dir_all(root.path().join("crate_a/src")).expect("crate dirs");
    fs::write(
        root.path().join("crate_a/Cargo.toml"),
        r#"
[package]
name = "crate_a"
version = "0.1.0"
"#,
    )
    .expect("crate cargo");
    fs::write(
        root.path().join("crate_a/src/lib.rs"),
        r#"
pub mod api;
"#,
    )
    .expect("lib");

    let crawl = crawl(root.path()).expect("crawl");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");
    let results = check_source(&inputs[0]);

    assert!(results.iter().any(|result| result.id() == "RS-ARCH-SOURCE-08"));
    assert!(!results.iter().any(|result| result.id() == "RS-ARCH-CONFIG-08"));
}

#[test]
fn config_pipeline_reports_boundary_crossing_missing_shared_and_split_rules() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["pkg", "pkg/crates/inner", "other"]
"#,
    )
    .expect("root cargo");

    fs::create_dir_all(root.path().join("pkg/src")).expect("pkg src");
    fs::create_dir_all(root.path().join("pkg/crates/inner/src")).expect("inner src");
    fs::create_dir_all(root.path().join("other/src")).expect("other src");

    fs::write(
        root.path().join("pkg/Cargo.toml"),
        r#"
[package]
name = "pkg"
version = "0.1.0"

[features]
api = []

[dependencies]
inner = { path = "crates/inner" }
other = { path = "../other" }
"#,
    )
    .expect("pkg cargo");
    fs::write(root.path().join("pkg/src/lib.rs"), "pub mod api;").expect("pkg lib");

    fs::write(
        root.path().join("pkg/crates/inner/Cargo.toml"),
        r#"
[package]
name = "inner"
version = "0.1.0"
"#,
    )
    .expect("inner cargo");
    fs::write(root.path().join("pkg/crates/inner/src/lib.rs"), "pub struct Inner;")
        .expect("inner lib");

    fs::write(
        root.path().join("other/Cargo.toml"),
        r#"
[package]
name = "other"
version = "0.1.0"
"#,
    )
    .expect("other cargo");
    fs::write(root.path().join("other/src/lib.rs"), "pub struct Other;").expect("other lib");

    let crawl = crawl(root.path()).expect("crawl");
    let inputs = crate::ingest_for_config_checks(&crawl).expect("config ingest");
    let results = check_config(&inputs[0]);

    assert!(results.iter().any(|result| result.id() == "RS-ARCH-CONFIG-05"));
    assert!(results.iter().any(|result| result.id() == "RS-ARCH-CONFIG-06"));
    assert!(results.iter().any(|result| result.id() == "RS-ARCH-CONFIG-08"));
}

#[test]
fn source_ingestion_stays_inside_the_pointed_workspace() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crate_a"]
"#,
    )
    .expect("root cargo");
    fs::create_dir_all(root.path().join("crate_a/src")).expect("crate dirs");
    fs::create_dir_all(root.path().join("foreign/src")).expect("foreign dirs");
    fs::write(
        root.path().join("crate_a/Cargo.toml"),
        r#"
[package]
name = "crate_a"
version = "0.1.0"
"#,
    )
    .expect("crate cargo");
    fs::write(root.path().join("crate_a/src/lib.rs"), "pub fn leaked() {}\n").expect("crate lib");
    fs::write(
        root.path().join("foreign/Cargo.toml"),
        r#"
[package]
name = "foreign"
version = "0.1.0"
"#,
    )
    .expect("foreign cargo");
    fs::write(root.path().join("foreign/src/lib.rs"), "pub fn stray() {}\n").expect("foreign lib");

    let crawl = crawl(root.path()).expect("crawl");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].crates.len(), 1);
    assert_eq!(inputs[0].crates[0].rel_dir, "crate_a");
    assert!(inputs[0]
        .source_files
        .iter()
        .all(|file| file.rel_path.starts_with("crate_a/")));
}

#[test]
fn config_ingestion_stays_inside_the_pointed_workspace() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crate_a"]
"#,
    )
    .expect("root cargo");
    fs::create_dir_all(root.path().join("crate_a/src")).expect("crate dirs");
    fs::create_dir_all(root.path().join("foreign/src")).expect("foreign dirs");
    fs::write(
        root.path().join("crate_a/Cargo.toml"),
        r#"
[package]
name = "crate_a"
version = "0.1.0"
"#,
    )
    .expect("crate cargo");
    fs::write(root.path().join("crate_a/src/lib.rs"), "pub mod api;\n").expect("crate lib");
    fs::write(
        root.path().join("foreign/Cargo.toml"),
        r#"
[package]
name = "foreign"
version = "0.1.0"

[dependencies]
crate_a = { path = "../crate_a" }
"#,
    )
    .expect("foreign cargo");
    fs::write(root.path().join("foreign/src/lib.rs"), "pub fn stray() {}\n").expect("foreign lib");

    let crawl = crawl(root.path()).expect("crawl");
    let inputs = crate::ingest_for_config_checks(&crawl).expect("config ingest");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].crates.len(), 1);
    assert_eq!(inputs[0].crates[0].rel_dir, "crate_a");
    assert!(inputs[0]
        .dependency_edges
        .iter()
        .all(|edge| edge.source_rel_dir == "crate_a"));
}

#[test]
fn source_ingestion_does_not_recurse_into_excluded_nested_crates() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["pkg", "pkg/crates/inner"]
exclude = ["pkg/crates/inner"]
"#,
    )
    .expect("root cargo");
    fs::create_dir_all(root.path().join("pkg/src")).expect("pkg src");
    fs::create_dir_all(root.path().join("pkg/crates/inner/src")).expect("inner src");
    fs::write(
        root.path().join("pkg/Cargo.toml"),
        r#"
[package]
name = "pkg"
version = "0.1.0"
"#,
    )
    .expect("pkg cargo");
    fs::write(
        root.path().join("pkg/src/lib.rs"),
        "pub mod api;\n",
    )
    .expect("pkg lib");
    fs::write(
        root.path().join("pkg/crates/inner/Cargo.toml"),
        r#"
[package]
name = "inner"
version = "0.1.0"
"#,
    )
    .expect("inner cargo");
    fs::write(
        root.path().join("pkg/crates/inner/src/lib.rs"),
        "pub fn leaked_from_excluded_child() {}\n",
    )
    .expect("inner lib");

    let crawl = crawl(root.path()).expect("crawl");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("source ingest");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].crates.len(), 1);
    assert_eq!(inputs[0].crates[0].rel_dir, "pkg");
    assert!(inputs[0]
        .source_files
        .iter()
        .all(|file| !file.rel_path.starts_with("pkg/crates/inner/")));
    assert!(inputs[0]
        .facade_surfaces
        .iter()
        .all(|surface| !surface.rel_path.starts_with("pkg/crates/inner/")));
}

#[test]
fn config_ingestion_respects_workspace_exclude() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["pkg", "pkg/crates/inner"]
exclude = ["pkg/crates/inner"]
"#,
    )
    .expect("root cargo");
    fs::create_dir_all(root.path().join("pkg/src")).expect("pkg src");
    fs::create_dir_all(root.path().join("pkg/crates/inner/src")).expect("inner src");

    fs::write(
        root.path().join("pkg/Cargo.toml"),
        r#"
[package]
name = "pkg"
version = "0.1.0"

[dependencies]
inner = { path = "crates/inner" }
"#,
    )
    .expect("pkg cargo");
    fs::write(root.path().join("pkg/src/lib.rs"), "pub mod api;\n").expect("pkg lib");

    fs::write(
        root.path().join("pkg/crates/inner/Cargo.toml"),
        r#"
[package]
name = "inner"
version = "0.1.0"
"#,
    )
    .expect("inner cargo");
    fs::write(root.path().join("pkg/crates/inner/src/lib.rs"), "pub struct Inner;")
        .expect("inner lib");

    let crawl = crawl(root.path()).expect("crawl");
    let inputs = crate::ingest_for_config_checks(&crawl).expect("config ingest");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].crates.len(), 1);
    assert_eq!(inputs[0].crates[0].rel_dir, "pkg");
    assert!(inputs[0].dependency_edges.is_empty());
}

#[test]
fn file_tree_pipeline_reports_missing_facade_and_complexity() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crate_a"]
"#,
    )
    .expect("root cargo");
    fs::create_dir_all(root.path().join("crate_a/src/deep/a/b/c")).expect("crate dirs");
    fs::write(
        root.path().join("crate_a/Cargo.toml"),
        r#"
[package]
name = "crate_a"
version = "0.1.0"

[dependencies]
one = "1"
two = "1"
three = "1"
four = "1"
five = "1"
six = "1"
seven = "1"
eight = "1"
nine = "1"
ten = "1"
eleven = "1"
twelve = "1"
thirteen = "1"
"#,
    )
    .expect("crate cargo");
    fs::write(root.path().join("crate_a/src/api.rs"), "pub struct Api;\n").expect("api");
    fs::write(root.path().join("crate_a/src/one.rs"), "pub struct One;\n").expect("one");
    fs::write(root.path().join("crate_a/src/two.rs"), "pub struct Two;\n").expect("two");
    fs::write(root.path().join("crate_a/src/three.rs"), "pub struct Three;\n").expect("three");
    fs::write(root.path().join("crate_a/src/four.rs"), "pub struct Four;\n").expect("four");
    fs::write(root.path().join("crate_a/src/five.rs"), "pub struct Five;\n").expect("five");
    fs::write(root.path().join("crate_a/src/six.rs"), "pub struct Six;\n").expect("six");
    fs::write(root.path().join("crate_a/src/seven.rs"), "pub struct Seven;\n").expect("seven");
    fs::write(root.path().join("crate_a/src/eight.rs"), "pub struct Eight;\n").expect("eight");
    fs::write(root.path().join("crate_a/src/nine.rs"), "pub struct Nine;\n").expect("nine");
    fs::write(root.path().join("crate_a/src/ten.rs"), "pub struct Ten;\n").expect("ten");
    fs::write(root.path().join("crate_a/src/eleven.rs"), "pub struct Eleven;\n").expect("eleven");
    fs::write(root.path().join("crate_a/src/deep/a/b/c/mod.rs"), "pub struct Deep;\n")
        .expect("deep");

    let crawl = crawl(root.path()).expect("crawl");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingest");
    let results = check_file_tree(&input);

    let rule_01 = results
        .iter()
        .filter(|result| result.id() == "RS-ARCH-FILETREE-01")
        .collect::<Vec<_>>();
    let rule_07 = results
        .iter()
        .filter(|result| result.id() == "RS-ARCH-FILETREE-07")
        .collect::<Vec<_>>();

    assert_eq!(rule_01.len(), 1);
    assert_eq!(rule_01[0].file(), Some("crate_a/Cargo.toml"));
    assert_eq!(rule_01[0].inventory(), false);

    assert_eq!(rule_07.len(), 1);
    assert_eq!(rule_07[0].file(), Some("crate_a/Cargo.toml"));
    assert_eq!(rule_07[0].inventory(), false);
}

#[test]
fn file_tree_pipeline_reports_missing_mod_rs() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crate_a"]
"#,
    )
    .expect("root cargo");
    fs::create_dir_all(root.path().join("crate_a/src/nested")).expect("crate dirs");
    fs::write(
        root.path().join("crate_a/Cargo.toml"),
        r#"
[package]
name = "crate_a"
version = "0.1.0"
"#,
    )
    .expect("crate cargo");
    fs::write(root.path().join("crate_a/src/lib.rs"), "pub mod nested;\n").expect("lib");
    fs::write(
        root.path().join("crate_a/src/nested/thing.rs"),
        "pub struct Thing;\n",
    )
    .expect("thing");

    let crawl = crawl(root.path()).expect("crawl");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingest");
    let results = check_file_tree(&input);

    let rule_03 = results
        .iter()
        .filter(|result| result.id() == "RS-ARCH-FILETREE-03")
        .collect::<Vec<_>>();

    assert_eq!(rule_03.len(), 1);
    assert_eq!(rule_03[0].file(), Some("crate_a/src/lib.rs"));
    assert_eq!(rule_03[0].inventory(), false);
}

#[test]
fn file_tree_ingestion_stays_inside_the_pointed_workspace() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crate_a"]
"#,
    )
    .expect("root cargo");
    fs::create_dir_all(root.path().join("crate_a/src/nested")).expect("crate dirs");
    fs::create_dir_all(root.path().join("foreign/src/nested")).expect("foreign dirs");
    fs::write(
        root.path().join("crate_a/Cargo.toml"),
        r#"
[package]
name = "crate_a"
version = "0.1.0"
"#,
    )
    .expect("crate cargo");
    fs::write(root.path().join("crate_a/src/lib.rs"), "pub mod nested;\n").expect("crate lib");
    fs::write(
        root.path().join("crate_a/src/nested/thing.rs"),
        "pub struct Thing;\n",
    )
    .expect("crate thing");
    fs::write(
        root.path().join("foreign/Cargo.toml"),
        r#"
[package]
name = "foreign"
version = "0.1.0"
"#,
    )
    .expect("foreign cargo");
    fs::write(root.path().join("foreign/src/lib.rs"), "pub mod nested;\n").expect("foreign lib");
    fs::write(
        root.path().join("foreign/src/nested/thing.rs"),
        "pub struct Thing;\n",
    )
    .expect("foreign thing");

    let crawl = crawl(root.path()).expect("crawl");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingest");

    assert_eq!(input.crates.len(), 1);
    assert_eq!(input.crates[0].rel_dir, "crate_a");
    assert!(input
        .module_dirs
        .iter()
        .all(|module_dir| module_dir.dir_rel.starts_with("crate_a/")));
}

#[test]
fn file_tree_complexity_ignores_excluded_nested_crates_for_root_level_layouts() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["pkg", "pkg/crates/inner"]
exclude = ["pkg/crates/inner"]
"#,
    )
    .expect("root cargo");
    fs::create_dir_all(root.path().join("pkg/crates/inner/src/deep/a/b/c"))
        .expect("inner dirs");

    fs::write(
        root.path().join("pkg/Cargo.toml"),
        r#"
[package]
name = "pkg"
version = "0.1.0"

[lib]
path = "lib.rs"
"#,
    )
    .expect("pkg cargo");
    fs::write(root.path().join("pkg/lib.rs"), "pub mod api;\n").expect("pkg lib");
    fs::write(root.path().join("pkg/api.rs"), "pub struct Api;\n").expect("pkg api");

    fs::write(
        root.path().join("pkg/crates/inner/Cargo.toml"),
        r#"
[package]
name = "inner"
version = "0.1.0"
"#,
    )
    .expect("inner cargo");
    fs::write(
        root.path().join("pkg/crates/inner/src/deep/a/b/c/mod.rs"),
        "pub struct Deep;\n",
    )
    .expect("inner deep");

    let crawl = crawl(root.path()).expect("crawl");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingest");
    let results = check_file_tree(&input);

    assert!(!results.iter().any(|result| result.id() == "RS-ARCH-FILETREE-07"));
}

#[test]
fn split_rule_pipeline_routes_dependency_threshold_to_config_only() {
    let root = tempdir().expect("tempdir");

    fs::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crate_a"]
"#,
    )
    .expect("root cargo");
    fs::create_dir_all(root.path().join("crate_a/src")).expect("crate dirs");
    fs::write(
        root.path().join("crate_a/Cargo.toml"),
        r#"
[package]
name = "crate_a"
version = "0.1.0"

[dependencies]
one = "1"
two = "1"
three = "1"
four = "1"
five = "1"
six = "1"
seven = "1"
eight = "1"
nine = "1"
ten = "1"
eleven = "1"
twelve = "1"
thirteen = "1"
"#,
    )
    .expect("crate cargo");
    fs::write(root.path().join("crate_a/src/lib.rs"), "pub mod api;\n").expect("lib");

    let crawl = crawl(root.path()).expect("crawl");
    let config_inputs = crate::ingest_for_config_checks(&crawl).expect("config ingest");
    let config_results = check_config(&config_inputs[0]);
    let file_tree_input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingest");
    let file_tree_results = check_file_tree(&file_tree_input);

    assert!(config_results
        .iter()
        .any(|result| result.id() == "RS-ARCH-CONFIG-07"));
    assert!(!file_tree_results
        .iter()
        .any(|result| result.id() == "RS-ARCH-FILETREE-07"));
}
