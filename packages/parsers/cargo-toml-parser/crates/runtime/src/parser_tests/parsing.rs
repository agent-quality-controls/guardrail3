#![allow(
    clippy::expect_used,
    reason = "parser tests use panic-based assertions to prove file-shape coverage"
)]

use cargo_toml_parser_runtime_assertions::parser as assertions;

use super::helpers::{parse_fixture, parse_from_tempfile};

#[test]
fn empty_string_yields_empty_manifest() {
    let manifest = parse_fixture("");
    assertions::assert_manifest_empty(&manifest);
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "one realistic fixture is the clearest way to prove manifest coverage"
)]
fn realistic_manifest_parses_known_sections() {
    let manifest = parse_fixture(
        r#"
cargo-features = ["profile-rustflags"]
future-root = "keep-me"

[package]
name = "demo"
version = { workspace = true }
edition = "2024"
rust-version = { workspace = true }
authors = ["A", "B"]
build = ["build.rs", "build-extra.rs"]
metabuild = ["foo", "bar"]
default-target = "wasm32-unknown-unknown"
forced-target = "x86_64-unknown-linux-gnu"
links = "native-demo"
exclude = ["fixtures/**"]
include = { workspace = true }
publish = ["internal"]
workspace = "../"
autolib = true
autobins = false
autoexamples = false
autotests = false
autobenches = false
default-run = "demo"
description = "demo crate"
homepage = { workspace = true }
documentation = "https://docs.example.com/demo"
readme = { workspace = true }
keywords = ["guardrail", "cargo"]
categories = ["development-tools"]
license = "MIT"
license-file = "LICENSE"
repository = { workspace = true }
resolver = "2"

[package.metadata.guardrail3]
tier = "core"

[project]
name = "legacy-alias"

[badges]
maintenance = { status = "actively-developed" }

[features]
default = ["std"]
std = []

[lib]
name = "demo_lib"
crate-type = ["rlib", "cdylib"]
path = "src/lib.rs"
filename = "demo-lib"
doc-scrape-examples = true

[[bin]]
name = "demo"
path = "src/main.rs"
required-features = ["std"]
edition = "2024"

[dependencies]
serde = "1"
internal = { version = "0.3", registry-index = "https://example.com/index", path = "crates/internal", base = "workspace", package = "internal-real", optional = true, default-features = false, features = ["derive"], public = true, artifact = ["bin:codegen", "cdylib"], lib = true, target = "x86_64-unknown-linux-gnu", custom = "kept" }

[dev-dependencies]
insta = { workspace = true }

[build-dependencies]
cc = { version = "1", default_features = false }

[target.'cfg(unix)'.dependencies]
libc = "0.2"
[target.'cfg(unix)'.build_dependencies]
pkg-config = "0.3"
[target.'cfg(unix)'.dev-dependencies]
tempfile = "3"
[target.'cfg(unix)'.future]
enabled = true

[lints]
workspace = true

[lints.rust]
unsafe_code = "forbid"
unexpected_cfgs = { level = "warn", priority = 2, check-cfg = ['cfg(loom)'] }

[lints.clippy]
unwrap_used = "deny"

[hints]
mostly-unused = { level = "allow" }
future-hint = "kept"

[workspace]
members = ["crates/*"]
exclude = ["legacy"]
default-members = ["crates/app"]
resolver = "3"

[workspace.metadata.guardrail3]
enabled = true

[workspace.package]
version = "0.2.0"
authors = ["Workspace Author"]
description = "workspace package defaults"
homepage = "https://example.com"
documentation = "https://docs.example.com"
readme = "README.md"
keywords = ["workspace"]
categories = ["development-tools"]
license = "MIT"
license-file = "LICENSE"
repository = "https://example.com/repo"
publish = ["internal"]
edition = "2024"
rust-version = "1.85"
[workspace.package.badges]
maintenance = { status = "actively-developed" }

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }

[workspace.lints.rust]
unsafe_code = "forbid"

[profile.dev]
opt-level = 1
lto = false
codegen-backend = "cranelift"
codegen-units = 8
debug = 1
split-debuginfo = "packed"
debug-assertions = true
rpath = false
panic = "abort"
overflow-checks = true
incremental = true
dir-name = "dev-custom"
inherits = "release"
strip = "none"
rustflags = ["-Dwarnings"]
trim-paths = ["diagnostics", "object"]
hint-mostly-unused = true
frame-pointers = "always"
[profile.dev.package.image]
opt-level = "z"
[profile.dev.build-override]
incremental = false

[patch.crates-io]
serde = { version = "1.0.210", path = "vendor/serde" }

[replace]
"foo:0.1.0" = { path = "vendor/foo" }
"#,
    );

    assert_eq!(
        manifest.cargo_features,
        vec!["profile-rustflags".to_owned()]
    );
    assertions::assert_realistic_manifest(&manifest);
}

#[test]
fn alternative_known_multi_shape_fields_parse() {
    let manifest = parse_fixture(
        r#"
[package]
name = "demo"
version = "0.1.0"
build = false
publish = false
readme = false

[dependencies]
tool = { version = "1", artifact = "bin:codegen" }

[profile.release]
trim-paths = true

[profile.dev]
trim-paths = "none"

[profile.test]
trim-paths = "macro"
"#,
    );

    assertions::assert_alternative_known_multi_shape_fields(&manifest);
}

#[test]
fn string_build_and_bool_readme_parse() {
    let manifest = parse_fixture(
        r#"
[package]
name = "demo"
version = "0.1.0"
build = "build.rs"
readme = "README.md"
"#,
    );

    assertions::assert_string_build_and_readme(&manifest);
}

#[test]
fn invalid_trim_paths_value_is_rejected() {
    let err = super::super::parse(
        r#"
[profile.dev]
trim-paths = "symbols"
"#,
    )
    .expect_err("invalid trim-paths value should fail");

    assertions::assert_parse_error(err);
}

#[test]
fn from_path_reads_and_parses_file() {
    let manifest = parse_from_tempfile(
        r#"
[package]
name = "demo"
edition = "2024"
"#,
    );

    assertions::assert_package_name(&manifest, "demo");
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = super::super::parse("this is not [[[valid toml")
        .expect_err("invalid Cargo.toml should fail");
    assertions::assert_parse_error(err);
}
