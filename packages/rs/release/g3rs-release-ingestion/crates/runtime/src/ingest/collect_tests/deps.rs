use g3rs_release_ingestion_assertions::ingest::collect as assertions;
use tempfile::tempdir;

use super::support::{crawl, git_init, write};

#[test]
fn config_pipeline_reports_publish_integrity_edges_and_binary_workflow() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/public", "crates/internal", "crates/cli"]
resolver = "2"
"#,
    );
    write(
        root.join("crates/public/Cargo.toml"),
        r#"
[package]
name = "public"
version = "0.1.0"
edition = "2024"
publish = true
description = "public crate"
license = "MIT"
repository = "https://example.com/public"
readme = "README.md"

[dependencies]
internal = { path = "../internal", version = "0.1.0" }
"#,
    );
    write(root.join("crates/public/src/lib.rs"), "pub fn public() {}\n");
    write(root.join("crates/public/README.md"), "# Public\n\ncrate\n");

    write(
        root.join("crates/internal/Cargo.toml"),
        r#"
[package]
name = "internal"
version = "0.1.0"
edition = "2024"
publish = false
"#,
    );
    write(root.join("crates/internal/src/lib.rs"), "pub fn internal() {}\n");

    write(
        root.join("crates/cli/Cargo.toml"),
        r#"
[package]
name = "cli"
version = "0.1.0"
edition = "2024"
publish = true
description = "cli crate"
license = "MIT"
repository = "https://example.com/cli"
readme = "README.md"
include = ["src/**", "Cargo.toml", "README.md"]

[dependencies]
public = { path = "../public", version = "0.2.0" }
"#,
    );
    write(root.join("crates/cli/src/main.rs"), "fn main() {}\n");
    write(root.join("crates/cli/README.md"), "# Cli\n\ncrate\n");

    write(
        root.join(".github/workflows/release-binaries.yml"),
        r#"
name: release-binaries
on:
  push:
    tags: ["v*"]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release --manifest-path crates/cli/Cargo.toml --target x86_64-unknown-linux-gnu
  release:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: softprops/action-gh-release@v2
"#,
    );

    let crawl = crawl(root);
    let input = super::super::config_result(&crawl).expect("config ingestion should succeed");
    let results = g3rs_release_config_checks::check(&input);

    assertions::assert_present(
        &results,
        "RS-RELEASE-CONFIG-19",
        "public: path dep to non-publishable crate",
        Some("crates/public/Cargo.toml"),
        false,
    );
    assertions::assert_present(
        &results,
        "RS-RELEASE-CONFIG-20",
        "cli: version mismatch with public",
        Some("crates/cli/Cargo.toml"),
        false,
    );
    assertions::assert_present(
        &results,
        "RS-RELEASE-CONFIG-22",
        "public: include/exclude missing",
        Some("crates/public/Cargo.toml"),
        false,
    );
    assertions::assert_present(
        &results,
        "RS-RELEASE-CONFIG-23",
        "cli: binary release workflow present",
        Some("crates/cli/Cargo.toml"),
        true,
    );
    assertions::assert_present(
        &results,
        "RS-RELEASE-CONFIG-24",
        "cli: linux release target present",
        Some("crates/cli/Cargo.toml"),
        true,
    );
}

#[test]
fn config_pipeline_warns_for_path_dependency_that_escapes_workspace() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/public"]
resolver = "2"
"#,
    );
    write(
        root.join("crates/public/Cargo.toml"),
        r#"
[package]
name = "public"
version = "0.1.0"
edition = "2024"
publish = true
description = "public crate"
license = "MIT"
repository = "https://example.com/public"
readme = "README.md"

[dependencies]
tooling = { path = "../../../vendor/tooling", version = "0.1.0" }
"#,
    );
    write(root.join("crates/public/src/lib.rs"), "pub fn public() {}\n");
    write(root.join("crates/public/README.md"), "# Public\n\ncrate\n");

    let crawl = crawl(root);
    let input = super::super::config_result(&crawl).expect("config ingestion should succeed");
    let results = g3rs_release_config_checks::check(&input);

    assertions::assert_present(
        &results,
        "RS-RELEASE-CONFIG-19",
        "public: path dep escapes workspace",
        Some("crates/public/Cargo.toml"),
        false,
    );
}
