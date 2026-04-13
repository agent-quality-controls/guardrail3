use tempfile::tempdir;

#[test]
fn config_ingest_extracts_layers_and_internal_edges() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        r#"
[package]
name = "root-bin"
version = "0.1.0"

[workspace]
members = ["types/core", "logic/service", "io/outbound/db"]

[workspace.dependencies]
logic-service = { path = "logic/service", package = "logic-service" }
"#,
    );
    super::write(
        root.path().join("types/core/Cargo.toml"),
        "[package]\nname = \"types-core\"\nversion = \"0.1.0\"\n",
    );
    super::write(
        root.path().join("logic/service/Cargo.toml"),
        r#"
[package]
name = "logic-service"
version = "0.1.0"

[dependencies]
types-core = { path = "../types/core", package = "types-core" }
"#,
    );
    super::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        r#"
[package]
name = "db-outbound"
version = "0.1.0"

[dependencies]
logic-service = { workspace = true }
"#,
    );

    let input = crate::ingest_for_config_checks(&super::crawl_workspace(root.path())).expect("config ingest");

    assert_eq!(input.crates.len(), 4);
    assert!(input
        .crates
        .iter()
        .any(|krate| krate.cargo_rel_path == "logic/service/Cargo.toml" && krate.layer.is_some()));
    assert!(input.dependency_edges.iter().any(|edge| {
        edge.from_cargo_rel_path == "logic/service/Cargo.toml"
            && edge.to_cargo_rel_path == "types/core/Cargo.toml"
    }));
    assert!(input.dependency_edges.iter().any(|edge| {
        edge.from_cargo_rel_path == "io/outbound/db/Cargo.toml"
            && edge.to_cargo_rel_path == "logic/service/Cargo.toml"
    }));
}

#[test]
fn source_ingest_collects_public_traits_from_nested_modules() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"io/outbound/db\"]\n",
    );
    super::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );
    super::write(
        root.path().join("io/outbound/db/src/lib.rs"),
        "pub mod adapter;\n",
    );
    super::write(
        root.path().join("io/outbound/db/src/adapter.rs"),
        "pub trait DbTrait {}\n#[cfg(test)] pub trait HiddenTrait {}\n",
    );

    let input = crate::ingest_for_source_checks(&super::crawl_workspace(root.path())).expect("source ingest");

    assert_eq!(input.crates.len(), 1);
    assert_eq!(input.public_traits.len(), 1);
    assert_eq!(input.public_traits[0].rel_path, "io/outbound/db/src/adapter.rs");
    assert_eq!(input.public_traits[0].trait_name, "DbTrait");
}
