#![allow(
    clippy::expect_used,
    clippy::manual_let_else,
    clippy::panic,
    reason = "parser tests use panic-based assertions to prove file-shape coverage"
)]

use crate::{
    Dependency, InheritableValue, IntegerOrString, LintValue, PackageBuildValue, StringOrBool,
    StringOrVec, TomlTrimPaths, TomlTrimPathsValue, VecStringOrBool,
};
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
    assertions::assert_top_level_extra_string(&manifest, "future-root", "keep-me");

    let package = manifest.package.as_ref().expect("package should exist");
    assert!(matches!(
        package.version,
        Some(InheritableValue::Inherit(_))
    ));
    assert_eq!(
        package.edition.as_ref(),
        Some(&InheritableValue::Value("2024".to_owned()))
    );
    assert!(matches!(
        package.build.as_ref(),
        Some(PackageBuildValue::MultipleScript(scripts)) if scripts == &vec!["build.rs".to_owned(), "build-extra.rs".to_owned()]
    ));
    assert_eq!(
        package.publish,
        Some(InheritableValue::Value(VecStringOrBool::VecString(vec![
            "internal".to_owned()
        ]))),
    );
    assert_eq!(
        package.readme,
        Some(InheritableValue::Inherit(crate::WorkspaceInheritance {
            workspace: true,
        })),
    );
    assert_eq!(
        package
            .metadata
            .as_ref()
            .and_then(|value| value.get("guardrail3"))
            .and_then(|value| value.get("tier"))
            .and_then(toml::Value::as_str),
        Some("core"),
    );

    assert_eq!(
        manifest
            .project
            .as_ref()
            .and_then(|project| project.name.as_deref()),
        Some("legacy-alias"),
    );
    assert_eq!(
        manifest
            .badges
            .get("maintenance")
            .and_then(|badge| badge.get("status"))
            .and_then(toml::Value::as_str),
        Some("actively-developed"),
    );
    assert_eq!(
        manifest.features.get("default"),
        Some(&vec!["std".to_owned()])
    );

    let lib = manifest.lib.as_ref().expect("lib target should exist");
    assert_eq!(lib.crate_type, vec!["rlib".to_owned(), "cdylib".to_owned()]);
    assert_eq!(lib.filename.as_deref(), Some("demo-lib"));
    assert_eq!(lib.doc_scrape_examples, Some(true));

    let bin = manifest.bin.first().expect("bin target should exist");
    assert_eq!(bin.required_features, vec!["std".to_owned()]);

    assertions::assert_simple_dep(
        manifest.dependencies.get("serde"),
        "1",
        "dependencies.serde",
    );
    let detailed = match manifest.dependencies.get("internal") {
        Some(Dependency::Detailed(detail)) => detail,
        Some(Dependency::Simple(_)) => panic!("dependencies.internal should be detailed"),
        None => panic!("dependencies.internal should exist"),
    };
    assert_eq!(
        detailed.registry_index.as_deref(),
        Some("https://example.com/index")
    );
    assert_eq!(detailed.base.as_deref(), Some("workspace"));
    assert_eq!(detailed.package.as_deref(), Some("internal-real"));
    assert_eq!(detailed.optional, Some(true));
    assert_eq!(detailed.default_features, Some(false));
    assert_eq!(detailed.features, vec!["derive".to_owned()]);
    assert_eq!(detailed.public, Some(true));
    assert_eq!(
        detailed.artifact.as_ref(),
        Some(&StringOrVec::Vec(vec![
            "bin:codegen".to_owned(),
            "cdylib".to_owned(),
        ])),
    );
    assert_eq!(detailed.lib, Some(true));
    assert_eq!(detailed.target.as_deref(), Some("x86_64-unknown-linux-gnu"));
    assert_eq!(
        detailed.extra.get("custom").and_then(toml::Value::as_str),
        Some("kept"),
    );

    let build_dep = match manifest.build_dependencies.get("cc") {
        Some(Dependency::Detailed(detail)) => detail,
        _ => panic!("build-dependencies.cc should be a detailed dependency"),
    };
    assert_eq!(build_dep.default_features, Some(false));

    assertions::assert_simple_dep(
        manifest
            .target
            .get("cfg(unix)")
            .and_then(|target| target.dependencies.get("libc")),
        "0.2",
        "target.cfg(unix).dependencies.libc",
    );
    assertions::assert_simple_dep(
        manifest
            .target
            .get("cfg(unix)")
            .and_then(|target| target.build_dependencies.get("pkg-config")),
        "0.3",
        "target.cfg(unix).build-dependencies.pkg-config",
    );
    assertions::assert_simple_dep(
        manifest
            .target
            .get("cfg(unix)")
            .and_then(|target| target.dev_dependencies.get("tempfile")),
        "3",
        "target.cfg(unix).dev-dependencies.tempfile",
    );
    assert_eq!(
        manifest
            .target
            .get("cfg(unix)")
            .and_then(|target| target.extra.get("future"))
            .and_then(|value| value.get("enabled"))
            .and_then(toml::Value::as_bool),
        Some(true),
    );

    let lints = manifest.lints.as_ref().expect("lints should exist");
    assert_eq!(lints.workspace, Some(true));
    assertions::assert_lint_level(
        lints
            .tools
            .get("rust")
            .and_then(|tool| tool.get("unsafe_code")),
        "forbid",
        "lints.rust.unsafe_code",
    );
    match lints
        .tools
        .get("rust")
        .and_then(|tool| tool.get("unexpected_cfgs"))
    {
        Some(LintValue::Detailed(detail)) => {
            assert_eq!(detail.level, "warn");
            assert_eq!(detail.priority, Some(2));
            assert_eq!(
                detail
                    .extra
                    .get("check-cfg")
                    .and_then(toml::Value::as_array)
                    .map(Vec::len),
                Some(1),
            );
        }
        _ => panic!("lints.rust.unexpected_cfgs should be a detailed lint"),
    }

    assert!(manifest.hints.is_some(), "hints should exist");
    assert_eq!(
        manifest
            .hints
            .as_ref()
            .and_then(|hints| hints.extra.get("future-hint"))
            .and_then(toml::Value::as_str),
        Some("kept"),
    );

    let workspace = manifest.workspace.as_ref().expect("workspace should exist");
    assert_eq!(workspace.members, vec!["crates/*".to_owned()]);
    assert_eq!(
        workspace
            .metadata
            .as_ref()
            .and_then(|value| value.get("guardrail3"))
            .and_then(|value| value.get("enabled"))
            .and_then(toml::Value::as_bool),
        Some(true),
    );
    assert_eq!(
        workspace
            .package
            .as_ref()
            .and_then(|workspace_package| workspace_package.rust_version.as_deref()),
        Some("1.85"),
    );

    let profile = manifest
        .profile
        .get("dev")
        .expect("profile.dev should exist");
    assert_eq!(profile.codegen_backend.as_deref(), Some("cranelift"));
    assert_eq!(profile.rustflags, vec!["-Dwarnings".to_owned()]);
    assert_eq!(
        profile.trim_paths,
        Some(TomlTrimPaths::Values(vec![
            TomlTrimPathsValue::Diagnostics,
            TomlTrimPathsValue::Object,
        ])),
    );
    assert_eq!(
        profile
            .build_override
            .as_ref()
            .and_then(|override_| override_.incremental),
        Some(false),
    );
    assert_eq!(
        profile
            .package
            .get("image")
            .and_then(|pkg| pkg.opt_level.as_ref()),
        Some(&IntegerOrString::String("z".to_owned())),
    );

    assertions::assert_detailed_dep_version(
        manifest
            .patch
            .get("crates-io")
            .and_then(|table| table.get("serde")),
        "1.0.210",
        "patch.crates-io.serde",
    );
    match manifest.replace.get("foo:0.1.0") {
        Some(Dependency::Detailed(detail)) => {
            assert_eq!(detail.path.as_deref(), Some("vendor/foo"));
        }
        _ => panic!("replace.foo:0.1.0 should be a detailed dependency"),
    }
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

    let package = manifest.package.as_ref().expect("package should exist");
    assert_eq!(package.build, Some(PackageBuildValue::Auto(false)));
    assert_eq!(
        package.publish,
        Some(InheritableValue::Value(VecStringOrBool::Bool(false))),
    );
    assert_eq!(
        package.readme,
        Some(InheritableValue::Value(StringOrBool::Bool(false))),
    );

    let tool = match manifest.dependencies.get("tool") {
        Some(Dependency::Detailed(detail)) => detail,
        _ => panic!("tool dependency should be detailed"),
    };
    assert_eq!(
        tool.artifact,
        Some(StringOrVec::String("bin:codegen".to_owned())),
    );

    assert_eq!(
        manifest
            .profile
            .get("release")
            .and_then(|profile| profile.trim_paths.clone()),
        Some(TomlTrimPaths::All),
    );
    assert_eq!(
        manifest
            .profile
            .get("dev")
            .and_then(|profile| profile.trim_paths.clone()),
        Some(TomlTrimPaths::Values(Vec::new())),
    );
    assert_eq!(
        manifest
            .profile
            .get("test")
            .and_then(|profile| profile.trim_paths.clone()),
        Some(TomlTrimPaths::Values(vec![TomlTrimPathsValue::Macro])),
    );
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

    let package = manifest.package.as_ref().expect("package should exist");
    assert_eq!(
        package.build,
        Some(PackageBuildValue::SingleScript("build.rs".to_owned())),
    );
    assert_eq!(
        package.readme,
        Some(InheritableValue::Value(StringOrBool::String(
            "README.md".to_owned(),
        ))),
    );
}

#[test]
fn invalid_trim_paths_value_is_rejected() {
    let err = crate::parse(
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

    assert_eq!(
        manifest
            .package
            .as_ref()
            .and_then(|package| package.name.as_deref()),
        Some("demo"),
    );
}

#[test]
fn parse_error_on_invalid_toml() {
    let err =
        crate::parse("this is not [[[valid toml").expect_err("invalid Cargo.toml should fail");
    assertions::assert_parse_error(err);
}
