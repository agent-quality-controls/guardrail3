#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    clippy::too_many_lines,
    clippy::missing_docs_in_private_items,
    reason = "single-call realistic-manifest assertion intentionally enumerates every Cargo.toml table inline so the fixture-vs-output diff stays in one place"
)]

use cargo_toml_parser_runtime::types::{CargoToml, Dependency, Value};

pub use crate::parser_deps::{assert_detailed_dep_version, assert_lint_level, assert_simple_dep};
pub use crate::parser_manifest::assert_top_level_extra_string as assert_extra_string;

pub fn assert_realistic_manifest(manifest: &CargoToml) {
    use cargo_toml_parser_runtime::types::{
        InheritableValue, IntegerOrString, PackageBuildValue, StringOrVec, TomlTrimPaths,
        TomlTrimPathsValue, VecStringOrBool,
    };

    assert_extra_string(manifest, "future-root", "keep-me");

    let package = manifest.package.as_ref();
    assert!(package.is_some(), "package should exist");
    let Some(package) = package else {
        return;
    };
    assert!(
        matches!(package.version, Some(InheritableValue::Inherit(_))),
        "package.version should inherit from workspace",
    );
    assert_eq!(
        package.edition.as_ref(),
        Some(&InheritableValue::Value("2024".to_owned())),
        "package.edition mismatch",
    );
    assert!(
        matches!(
            package.build.as_ref(),
            Some(PackageBuildValue::MultipleScript(scripts))
                if scripts.as_slice() == ["build.rs".to_owned(), "build-extra.rs".to_owned()]
        ),
        "package.build should keep multiple scripts",
    );
    assert_eq!(
        package.publish,
        Some(InheritableValue::Value(VecStringOrBool::VecString(vec![
            "internal".to_owned()
        ]))),
        "package.publish mismatch",
    );
    assert!(
        matches!(package.readme, Some(InheritableValue::Inherit(_))),
        "package.readme should inherit from workspace",
    );
    assert_eq!(
        package
            .metadata
            .as_ref()
            .and_then(|value| value.get("guardrail3"))
            .and_then(|value| value.get("tier"))
            .and_then(Value::as_str),
        Some("core"),
        "package.metadata.guardrail3.tier mismatch",
    );

    assert_eq!(
        manifest
            .project
            .as_ref()
            .and_then(|project| project.name.as_deref()),
        Some("legacy-alias"),
        "project.name mismatch",
    );
    assert_eq!(
        manifest
            .badges
            .get("maintenance")
            .and_then(|badge| badge.get("status"))
            .and_then(Value::as_str),
        Some("actively-developed"),
        "badges.maintenance.status mismatch",
    );
    assert_eq!(
        manifest.features.get("default"),
        Some(&vec!["std".to_owned()]),
        "features.default mismatch",
    );

    let lib = manifest.lib.as_ref();
    assert!(lib.is_some(), "lib target should exist");
    let Some(lib) = lib else {
        return;
    };
    assert_eq!(
        lib.crate_type,
        vec!["rlib".to_owned(), "cdylib".to_owned()],
        "lib.crate_type mismatch",
    );
    assert_eq!(
        lib.filename.as_deref(),
        Some("demo-lib"),
        "lib.filename mismatch"
    );
    assert_eq!(
        lib.doc_scrape_examples,
        Some(true),
        "lib.doc-scrape-examples mismatch",
    );

    let bin = manifest.bin.first();
    assert!(bin.is_some(), "bin target should exist");
    let Some(bin) = bin else {
        return;
    };
    assert_eq!(
        bin.required_features,
        vec!["std".to_owned()],
        "bin.required-features mismatch",
    );

    assert_simple_dep(
        manifest.dependencies.get("serde"),
        "1",
        "dependencies.serde",
    );
    let detailed = manifest.dependencies.get("internal");
    assert!(
        matches!(detailed, Some(Dependency::Detailed(_))),
        "dependencies.internal should be detailed",
    );
    let Some(Dependency::Detailed(detailed)) = detailed else {
        return;
    };
    assert_eq!(
        detailed.registry_index.as_deref(),
        Some("https://example.com/index"),
        "dependencies.internal.registry-index mismatch",
    );
    assert_eq!(
        detailed.base.as_deref(),
        Some("workspace"),
        "dependencies.internal.base mismatch"
    );
    assert_eq!(
        detailed.package.as_deref(),
        Some("internal-real"),
        "dependencies.internal.package mismatch",
    );
    assert_eq!(
        detailed.optional,
        Some(true),
        "dependencies.internal.optional mismatch"
    );
    assert_eq!(
        detailed.default_features,
        Some(false),
        "dependencies.internal.default-features mismatch",
    );
    assert_eq!(
        detailed.features,
        vec!["derive".to_owned()],
        "dependencies.internal.features mismatch",
    );
    assert_eq!(
        detailed.public,
        Some(true),
        "dependencies.internal.public mismatch"
    );
    assert_eq!(
        detailed.artifact.as_ref(),
        Some(&StringOrVec::Vec(vec![
            "bin:codegen".to_owned(),
            "cdylib".to_owned(),
        ])),
        "dependencies.internal.artifact mismatch",
    );
    assert_eq!(
        detailed.lib,
        Some(true),
        "dependencies.internal.lib mismatch"
    );
    assert_eq!(
        detailed.target.as_deref(),
        Some("x86_64-unknown-linux-gnu"),
        "dependencies.internal.target mismatch",
    );
    assert_eq!(
        detailed.extra.get("custom").and_then(Value::as_str),
        Some("kept"),
        "dependencies.internal.custom mismatch",
    );

    let build_dep = manifest.build_dependencies.get("cc");
    assert!(
        matches!(build_dep, Some(Dependency::Detailed(_))),
        "build-dependencies.cc should be detailed",
    );
    let Some(Dependency::Detailed(build_dep)) = build_dep else {
        return;
    };
    assert_eq!(
        build_dep.default_features,
        Some(false),
        "build-dependencies.cc.default-features mismatch",
    );

    assert_simple_dep(
        manifest
            .target
            .get("cfg(unix)")
            .and_then(|target| target.dependencies.get("libc")),
        "0.2",
        "target.cfg(unix).dependencies.libc",
    );
    assert_simple_dep(
        manifest
            .target
            .get("cfg(unix)")
            .and_then(|target| target.build_dependencies.get("pkg-config")),
        "0.3",
        "target.cfg(unix).build-dependencies.pkg-config",
    );
    assert_simple_dep(
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
            .and_then(Value::as_bool),
        Some(true),
        "target.cfg(unix).future.enabled mismatch",
    );

    let lints = manifest.lints.as_ref();
    assert!(lints.is_some(), "lints should exist");
    let Some(lints) = lints else {
        return;
    };
    assert_eq!(lints.workspace, Some(true), "lints.workspace mismatch");
    assert_lint_level(
        lints
            .tools
            .get("rust")
            .and_then(|tool| tool.get("unsafe_code")),
        "forbid",
        "lints.rust.unsafe_code",
    );
    let unexpected_cfgs = lints
        .tools
        .get("rust")
        .and_then(|tool| tool.get("unexpected_cfgs"));
    assert!(
        matches!(
            unexpected_cfgs,
            Some(cargo_toml_parser_runtime::types::LintValue::Detailed(_))
        ),
        "lints.rust.unexpected_cfgs should be detailed",
    );
    let Some(cargo_toml_parser_runtime::types::LintValue::Detailed(detail)) = unexpected_cfgs
    else {
        return;
    };
    assert_eq!(
        detail.level, "warn",
        "lints.rust.unexpected_cfgs.level mismatch"
    );
    assert_eq!(
        detail.priority,
        Some(2),
        "lints.rust.unexpected_cfgs.priority mismatch",
    );
    assert_eq!(
        detail
            .extra
            .get("check-cfg")
            .and_then(Value::as_array)
            .map(Vec::len),
        Some(1),
        "lints.rust.unexpected_cfgs.check-cfg mismatch",
    );

    assert_eq!(
        manifest
            .hints
            .as_ref()
            .and_then(|hints| hints.extra.get("future-hint"))
            .and_then(Value::as_str),
        Some("kept"),
        "hints.future-hint mismatch",
    );

    let workspace = manifest.workspace.as_ref();
    assert!(workspace.is_some(), "workspace should exist");
    let Some(workspace) = workspace else {
        return;
    };
    assert_eq!(
        workspace.members,
        vec!["crates/*".to_owned()],
        "workspace.members mismatch"
    );
    assert_eq!(
        workspace
            .metadata
            .as_ref()
            .and_then(|value| value.get("guardrail3"))
            .and_then(|value| value.get("enabled"))
            .and_then(Value::as_bool),
        Some(true),
        "workspace.metadata.guardrail3.enabled mismatch",
    );
    assert_eq!(
        workspace
            .package
            .as_ref()
            .and_then(|workspace_package| workspace_package.rust_version.as_deref()),
        Some("1.85"),
        "workspace.package.rust-version mismatch",
    );

    let profile = manifest.profile.get("dev");
    assert!(profile.is_some(), "profile.dev should exist");
    let Some(profile) = profile else {
        return;
    };
    assert_eq!(
        profile.codegen_backend.as_deref(),
        Some("cranelift"),
        "profile.dev.codegen-backend mismatch",
    );
    assert_eq!(
        profile.rustflags,
        vec!["-Dwarnings".to_owned()],
        "profile.dev.rustflags mismatch",
    );
    assert_eq!(
        profile.trim_paths,
        Some(TomlTrimPaths::Values(vec![
            TomlTrimPathsValue::Diagnostics,
            TomlTrimPathsValue::Object,
        ])),
        "profile.dev.trim-paths mismatch",
    );
    assert_eq!(
        profile
            .build_override
            .as_ref()
            .and_then(|override_| override_.incremental),
        Some(false),
        "profile.dev.build-override.incremental mismatch",
    );
    assert_eq!(
        profile
            .package
            .get("image")
            .and_then(|pkg| pkg.opt_level.as_ref()),
        Some(&IntegerOrString::String("z".to_owned())),
        "profile.dev.package.image.opt-level mismatch",
    );

    assert_detailed_dep_version(
        manifest
            .patch
            .get("crates-io")
            .and_then(|table| table.get("serde")),
        "1.0.210",
        "patch.crates-io.serde",
    );
    let replace = manifest.replace.get("foo:0.1.0");
    assert!(
        matches!(replace, Some(Dependency::Detailed(_))),
        "replace.foo:0.1.0 should be detailed",
    );
    let Some(Dependency::Detailed(replace)) = replace else {
        return;
    };
    assert_eq!(
        replace.path.as_deref(),
        Some("vendor/foo"),
        "replace.foo:0.1.0.path mismatch",
    );
}

pub fn assert_alternative_known_multi_shape_fields(manifest: &CargoToml) {
    use cargo_toml_parser_runtime::types::{
        InheritableValue, PackageBuildValue, StringOrBool, StringOrVec, TomlTrimPaths,
        TomlTrimPathsValue, VecStringOrBool,
    };

    let package = manifest.package.as_ref();
    assert!(package.is_some(), "package should exist");
    let Some(package) = package else {
        return;
    };
    assert_eq!(
        package.build,
        Some(PackageBuildValue::Auto(false)),
        "package.build mismatch"
    );
    assert_eq!(
        package.publish,
        Some(InheritableValue::Value(VecStringOrBool::Bool(false))),
        "package.publish mismatch",
    );
    assert_eq!(
        package.readme,
        Some(InheritableValue::Value(StringOrBool::Bool(false))),
        "package.readme mismatch",
    );

    let tool = manifest.dependencies.get("tool");
    assert!(
        matches!(tool, Some(Dependency::Detailed(_))),
        "tool dependency should be detailed",
    );
    let Some(Dependency::Detailed(tool)) = tool else {
        return;
    };
    assert_eq!(
        tool.artifact,
        Some(StringOrVec::String("bin:codegen".to_owned())),
        "tool.artifact mismatch",
    );

    assert_eq!(
        manifest
            .profile
            .get("release")
            .and_then(|profile| profile.trim_paths.clone()),
        Some(TomlTrimPaths::All),
        "profile.release.trim-paths mismatch",
    );
    assert_eq!(
        manifest
            .profile
            .get("dev")
            .and_then(|profile| profile.trim_paths.clone()),
        Some(TomlTrimPaths::Values(Vec::new())),
        "profile.dev.trim-paths mismatch",
    );
    assert_eq!(
        manifest
            .profile
            .get("test")
            .and_then(|profile| profile.trim_paths.clone()),
        Some(TomlTrimPaths::Values(vec![TomlTrimPathsValue::Macro])),
        "profile.test.trim-paths mismatch",
    );
}
