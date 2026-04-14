use tempfile::tempdir;

#[test]
fn root_without_workspace_fails() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[package]\nname = \"standalone\"\nversion = \"0.1.0\"\n",
    );

    assert!(crate::ingest_for_config_checks(&super::crawl_workspace(root.path())).is_err());
    assert!(crate::ingest_for_source_checks(&super::crawl_workspace(root.path())).is_err());
}

#[test]
fn missing_workspace_member_fails_closed() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"logic/missing\"]\n",
    );

    assert!(crate::ingest_for_config_checks(&super::crawl_workspace(root.path())).is_err());
}

#[test]
fn invalid_workspace_member_glob_fails_closed() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"logic/[bad\"]\n",
    );

    assert!(crate::ingest_for_config_checks(&super::crawl_workspace(root.path())).is_err());
}

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
fn config_ingest_extracts_build_dev_and_globbed_edges() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["./types/*", "./logic/*", "./io/outbound/*"]

[workspace.dependencies]
logic-shared = { path = "logic/shared", package = "logic-shared" }
"#,
    );
    super::write(
        root.path().join("types/core/Cargo.toml"),
        r#"
[package]
name = "types-core"
version = "0.1.0"

[dev-dependencies]
logic-shared = { path = "../../logic/shared", package = "logic-shared" }
"#,
    );
    super::write(
        root.path().join("logic/shared/Cargo.toml"),
        "[package]\nname = \"logic-shared\"\nversion = \"0.1.0\"\n",
    );
    super::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        r#"
[package]
name = "db-outbound"
version = "0.1.0"

[build-dependencies]
logic-shared = { workspace = true }

[target.'cfg(unix)'.dev-dependencies]
logic-shared = { workspace = true }

[target.'cfg(unix)'.build-dependencies]
types-core = { path = "../../../types/core", package = "types-core" }
"#,
    );

    let input = crate::ingest_for_config_checks(&super::crawl_workspace(root.path())).expect("config ingest");

    assert!(input
        .crates
        .iter()
        .any(|krate| krate.cargo_rel_path == "types/core/Cargo.toml"));
    assert!(input.dependency_edges.iter().any(|edge| {
        edge.from_cargo_rel_path == "types/core/Cargo.toml"
            && edge.to_cargo_rel_path == "logic/shared/Cargo.toml"
    }));
    assert_eq!(
        input
            .dependency_edges
            .iter()
            .filter(|edge| edge.from_cargo_rel_path == "io/outbound/db/Cargo.toml"
                && edge.to_cargo_rel_path == "logic/shared/Cargo.toml")
            .count(),
        2
    );
    assert!(input.dependency_edges.iter().any(|edge| {
        edge.from_cargo_rel_path == "io/outbound/db/Cargo.toml"
            && edge.to_cargo_rel_path == "logic/shared/Cargo.toml"
            && edge.kind.label() == "build-dependencies"
    }));
    assert!(input.dependency_edges.iter().any(|edge| {
        edge.from_cargo_rel_path == "io/outbound/db/Cargo.toml"
            && edge.to_cargo_rel_path == "logic/shared/Cargo.toml"
            && edge.kind.label() == "target.*.dev-dependencies"
    }));
    assert!(input.dependency_edges.iter().any(|edge| {
        edge.from_cargo_rel_path == "io/outbound/db/Cargo.toml"
            && edge.to_cargo_rel_path == "types/core/Cargo.toml"
    }));
}

#[test]
fn config_ingest_resolves_renamed_workspace_dependencies() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["logic/service", "types/core"]

[workspace.dependencies]
contract = { path = "types/core", package = "types-core" }
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
contract = { workspace = true }
"#,
    );

    let input = crate::ingest_for_config_checks(&super::crawl_workspace(root.path())).expect("config ingest");

    assert!(input.dependency_edges.iter().any(|edge| {
        edge.from_cargo_rel_path == "logic/service/Cargo.toml"
            && edge.to_cargo_rel_path == "types/core/Cargo.toml"
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
    assert_eq!(input.public_items.len(), 1);
    assert_eq!(input.public_items[0].rel_path, "io/outbound/db/src/adapter.rs");
    assert_eq!(input.public_items[0].item_name, "DbTrait");
}

#[test]
fn source_ingest_collects_public_trait_from_private_module() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"io/outbound/db\"]\n",
    );
    super::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.path().join("io/outbound/db/src/lib.rs"), "mod adapter;\n");
    super::write(root.path().join("io/outbound/db/src/adapter.rs"), "pub trait HiddenPort {}\n");

    let input = crate::ingest_for_source_checks(&super::crawl_workspace(root.path())).expect("source ingest");

    assert!(input
        .public_items
        .iter()
        .any(|fact| fact.rel_path == "io/outbound/db/src/adapter.rs" && fact.item_name == "HiddenPort"));
}

#[test]
fn source_ingest_collects_public_trait_reexported_from_private_module() {
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
        "mod adapter;\npub use adapter::DbTrait;\n",
    );
    super::write(root.path().join("io/outbound/db/src/adapter.rs"), "pub trait DbTrait {}\n");

    let input = crate::ingest_for_source_checks(&super::crawl_workspace(root.path())).expect("source ingest");

    assert!(input
        .public_items
        .iter()
        .any(|fact| fact.rel_path == "io/outbound/db/src/adapter.rs" && fact.item_name == "DbTrait"));
}

#[test]
fn source_ingest_does_not_lose_public_trait_when_private_path_is_seen_first() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"io/outbound/db\"]\n",
    );
    super::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        r#"
[package]
name = "db-outbound"
version = "0.1.0"

[lib]
path = "src/z_lib.rs"

[[bin]]
name = "db-bin"
path = "src/a_main.rs"
"#,
    );
    super::write(
        root.path().join("io/outbound/db/src/z_lib.rs"),
        "#[path = \"shared.rs\"] pub mod shared;\n",
    );
    super::write(
        root.path().join("io/outbound/db/src/a_main.rs"),
        "#[path = \"shared.rs\"] mod shared;\nfn main() {}\n",
    );
    super::write(root.path().join("io/outbound/db/src/shared.rs"), "pub trait SharedTrait {}\n");

    let input = crate::ingest_for_source_checks(&super::crawl_workspace(root.path())).expect("source ingest");

    assert!(input
        .public_items
        .iter()
        .any(|fact| fact.rel_path == "io/outbound/db/src/shared.rs" && fact.item_name == "SharedTrait"));
}

#[test]
fn source_ingest_scans_default_entrypoint_alongside_explicit_one() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"io/outbound/db\"]\n",
    );
    super::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        r#"
[package]
name = "db-outbound"
version = "0.1.0"

[[bin]]
name = "worker"
path = "src/custom_bin.rs"
"#,
    );
    super::write(root.path().join("io/outbound/db/src/custom_bin.rs"), "fn main() {}\n");
    super::write(root.path().join("io/outbound/db/src/lib.rs"), "pub trait LibTrait {}\n");

    let input = crate::ingest_for_source_checks(&super::crawl_workspace(root.path())).expect("source ingest");

    assert!(input
        .public_items
        .iter()
        .any(|fact| fact.rel_path == "io/outbound/db/src/lib.rs" && fact.item_name == "LibTrait"));
}

#[test]
fn source_ingest_missing_declared_module_fails_closed() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"io/outbound/db\"]\n",
    );
    super::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.path().join("io/outbound/db/src/lib.rs"), "pub mod adapter;\n");

    assert!(crate::ingest_for_source_checks(&super::crawl_workspace(root.path())).is_err());
}

#[test]
fn source_ingest_ignores_file_level_cfg_test_module() {
    let root = tempdir().expect("tempdir");
    super::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"io/outbound/db\"]\n",
    );
    super::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        "[package]\nname = \"db-outbound\"\nversion = \"0.1.0\"\n",
    );
    super::write(root.path().join("io/outbound/db/src/lib.rs"), "pub mod adapter;\n");
    super::write(
        root.path().join("io/outbound/db/src/adapter.rs"),
        "#![cfg(test)]\npub trait TestOnlyTrait {}\n",
    );

    let input = crate::ingest_for_source_checks(&super::crawl_workspace(root.path())).expect("source ingest");

    assert!(input.public_items.is_empty());
}
