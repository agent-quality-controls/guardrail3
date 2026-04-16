#[test]
fn root_without_workspace_fails() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[package]\nname = \"standalone\"\nversion = \"0.1.0\"\n",
    );

    assert!(super::super::ingest_for_config_checks(&super::helpers::crawl_workspace(root.path())).is_err());
}

#[test]
fn missing_workspace_member_fails_closed() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"logic/missing\"]\n",
    );

    assert!(super::super::ingest_for_config_checks(&super::helpers::crawl_workspace(root.path())).is_err());
}

#[test]
fn invalid_workspace_member_glob_fails_closed() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"logic/[bad\"]\n",
    );

    assert!(super::super::ingest_for_config_checks(&super::helpers::crawl_workspace(root.path())).is_err());
}

#[test]
fn config_ingest_extracts_layers_and_internal_edges() {
    let root = super::helpers::temp_workspace();
    super::helpers::write(
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
    super::helpers::write(
        root.path().join("types/core/Cargo.toml"),
        "[package]\nname = \"types-core\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(
        root.path().join("logic/service/Cargo.toml"),
        r#"
[package]
name = "logic-service"
version = "0.1.0"

[dependencies]
types-core = { path = "../types/core", package = "types-core" }
"#,
    );
    super::helpers::write(
        root.path().join("io/outbound/db/Cargo.toml"),
        r#"
[package]
name = "db-outbound"
version = "0.1.0"

[dependencies]
logic-service = { workspace = true }
"#,
    );

    let input = super::helpers::config_input(root.path());

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
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["./types/*", "./logic/*", "./io/outbound/*"]

[workspace.dependencies]
logic-shared = { path = "logic/shared", package = "logic-shared" }
"#,
    );
    super::helpers::write(
        root.path().join("types/core/Cargo.toml"),
        r#"
[package]
name = "types-core"
version = "0.1.0"

[dev-dependencies]
logic-shared = { path = "../../logic/shared", package = "logic-shared" }
"#,
    );
    super::helpers::write(
        root.path().join("logic/shared/Cargo.toml"),
        "[package]\nname = \"logic-shared\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(
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

    let input = super::helpers::config_input(root.path());

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
    let root = super::helpers::temp_workspace();
    super::helpers::write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["logic/service", "types/core"]

[workspace.dependencies]
contract = { path = "types/core", package = "types-core" }
"#,
    );
    super::helpers::write(
        root.path().join("types/core/Cargo.toml"),
        "[package]\nname = \"types-core\"\nversion = \"0.1.0\"\n",
    );
    super::helpers::write(
        root.path().join("logic/service/Cargo.toml"),
        r#"
[package]
name = "logic-service"
version = "0.1.0"

[dependencies]
contract = { workspace = true }
"#,
    );

    let input = super::helpers::config_input(root.path());

    assert!(input.dependency_edges.iter().any(|edge| {
        edge.from_cargo_rel_path == "logic/service/Cargo.toml"
            && edge.to_cargo_rel_path == "types/core/Cargo.toml"
    }));
}
