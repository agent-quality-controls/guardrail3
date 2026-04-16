use g3rs_arch_ingestion_assertions::config as assertions;
use guardrail3_check_types::G3Severity;

use super::helpers::{config_inputs, config_results, make_dir, temp_workspace_root, write_file};

#[test]
fn config_pipeline_reports_missing_shared_and_feature_contract_rules() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"pkg\", \"pkg/crates/inner\", \"other\"]\n",
    );
    make_dir(&root, "pkg/src");
    make_dir(&root, "pkg/crates/inner/src");
    make_dir(&root, "other/src");
    write_file(
        &root,
        "pkg/Cargo.toml",
        "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\n\n[features]\napi = []\n\n[dependencies]\ninner = { path = \"crates/inner\" }\nother = { path = \"../other\" }\n",
    );
    write_file(&root, "pkg/src/lib.rs", "pub mod api;");
    write_file(
        &root,
        "pkg/crates/inner/Cargo.toml",
        "[package]\nname = \"inner\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "pkg/crates/inner/src/lib.rs", "pub struct Inner;");
    write_file(
        &root,
        "other/Cargo.toml",
        "[package]\nname = \"other\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "other/src/lib.rs", "pub struct Other;");

    let results = config_results(&root);
    assertions::assert_has_result(
        &results,
        "RS-ARCH-CONFIG-05",
        G3Severity::Info,
        Some("pkg/Cargo.toml"),
    );
    assertions::assert_has_result(
        &results,
        "RS-ARCH-CONFIG-06",
        G3Severity::Error,
        Some("pkg/Cargo.toml"),
    );
    assertions::assert_has_result(
        &results,
        "RS-ARCH-CONFIG-08",
        G3Severity::Error,
        Some("pkg/Cargo.toml"),
    );
}

#[test]
fn config_ingestion_stays_inside_the_pointed_workspace() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crate_a\"]\n",
    );
    make_dir(&root, "crate_a/src");
    make_dir(&root, "foreign/src");
    write_file(
        &root,
        "crate_a/Cargo.toml",
        "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "crate_a/src/lib.rs", "pub mod api;\n");
    write_file(
        &root,
        "foreign/Cargo.toml",
        "[package]\nname = \"foreign\"\nversion = \"0.1.0\"\n\n[dependencies]\ncrate_a = { path = \"../crate_a\" }\n",
    );
    write_file(&root, "foreign/src/lib.rs", "pub fn stray() {}\n");

    let inputs = config_inputs(&root);

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].crates.len(), 1);
    assert_eq!(inputs[0].crates[0].rel_dir, "crate_a");
    assert!(
        inputs[0]
            .dependency_edges
            .iter()
            .all(|edge| edge.source_rel_dir == "crate_a")
    );
}

#[test]
fn config_ingestion_respects_workspace_exclude() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"pkg\", \"pkg/crates/inner\"]\nexclude = [\"pkg/crates/inner\"]\n",
    );
    make_dir(&root, "pkg/src");
    make_dir(&root, "pkg/crates/inner/src");
    write_file(
        &root,
        "pkg/Cargo.toml",
        "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\n\n[dependencies]\ninner = { path = \"crates/inner\" }\n",
    );
    write_file(&root, "pkg/src/lib.rs", "pub mod api;\n");
    write_file(
        &root,
        "pkg/crates/inner/Cargo.toml",
        "[package]\nname = \"inner\"\nversion = \"0.1.0\"\n",
    );
    write_file(&root, "pkg/crates/inner/src/lib.rs", "pub struct Inner;");

    let inputs = config_inputs(&root);

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].crates.len(), 1);
    assert_eq!(inputs[0].crates[0].rel_dir, "pkg");
    assert!(inputs[0].dependency_edges.is_empty());
}

#[test]
fn split_rule_pipeline_routes_dependency_threshold_to_config_only() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crate_a\"]\n",
    );
    make_dir(&root, "crate_a/src");
    write_file(
        &root,
        "crate_a/Cargo.toml",
        "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n\n[dependencies]\none = \"1\"\ntwo = \"1\"\nthree = \"1\"\nfour = \"1\"\nfive = \"1\"\nsix = \"1\"\nseven = \"1\"\neight = \"1\"\nnine = \"1\"\nten = \"1\"\neleven = \"1\"\ntwelve = \"1\"\nthirteen = \"1\"\n",
    );
    write_file(&root, "crate_a/src/lib.rs", "pub mod api;\n");

    let results = config_results(&root);
    assertions::assert_has_result(
        &results,
        "RS-ARCH-CONFIG-07",
        G3Severity::Error,
        Some("crate_a/Cargo.toml"),
    );
}

#[test]
fn split_rule_ignores_dev_dependencies_in_hard_config_cap() {
    let root = temp_workspace_root();

    write_file(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"crate_a\"]\n",
    );
    make_dir(&root, "crate_a/src");
    write_file(
        &root,
        "crate_a/Cargo.toml",
        "[package]\nname = \"crate_a\"\nversion = \"0.1.0\"\n\n[dependencies]\none = \"1\"\ntwo = \"1\"\nthree = \"1\"\nfour = \"1\"\nfive = \"1\"\nsix = \"1\"\nseven = \"1\"\neight = \"1\"\nnine = \"1\"\nten = \"1\"\neleven = \"1\"\ntwelve = \"1\"\n\n[dev-dependencies]\ntempfile = \"3\"\nserde_json = \"1\"\n",
    );
    write_file(&root, "crate_a/src/lib.rs", "pub mod api;\n");

    let inputs = config_inputs(&root);
    let config_crate = &inputs[0].crates[0];
    let results = config_results(&root);

    assert_eq!(config_crate.production_dependency_count, 12);
    assert_eq!(config_crate.dev_dependency_count, 2);
    assertions::assert_missing_result(&results, "RS-ARCH-CONFIG-07");
}
