use guardrail3_app_rs_family_fmt_assertions::rs_fmt_05_per_crate_override as assertions;
use test_support::{tempdir, write_file};

use super::run_check;

#[test]
fn reports_extra_per_crate_rustfmt_configs() {
    let results = run_check(
        "crates/core/.rustfmt.toml",
        super::super::facts::RustfmtConfigKind::DotRustfmtToml,
    );

    assertions::assert_override(
        &results,
        ".rustfmt.toml below workspace root overrides root formatting policy",
        "crates/core/.rustfmt.toml",
    );
}

#[test]
fn reports_plain_nested_rustfmt_toml_overrides() {
    let results = run_check(
        "crates/core/rustfmt.toml",
        super::super::facts::RustfmtConfigKind::RustfmtToml,
    );

    assertions::assert_override(
        &results,
        "rustfmt.toml below workspace root overrides root formatting policy",
        "crates/core/rustfmt.toml",
    );
}

#[test]
fn discovers_nested_override_files_from_family_walk() {
    let fixture = tempdir();
    let root = fixture.path();

    write_file(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = []\nresolver = \"2\"\n\n[workspace.package]\nedition = \"2024\"\n",
    );
    write_file(
        root,
        "rust-toolchain.toml",
        "[toolchain]\nchannel = \"stable\"\n",
    );
    write_file(
        root,
        "rustfmt.toml",
        "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\n",
    );
    write_file(root, "nested/rustfmt.toml", "max_width = 120\n");
    write_file(root, "nested/.rustfmt.toml", "max_width = 120\n");

    let results = super::run_family_check(root);

    assertions::assert_findings(
        &results,
        &[
            assertions::Finding {
                severity: assertions::Severity::Warn,
                title: "Per-crate rustfmt override",
                message: ".rustfmt.toml below workspace root overrides root formatting policy",
                file: Some("nested/.rustfmt.toml"),
                inventory: false,
            },
            assertions::Finding {
                severity: assertions::Severity::Warn,
                title: "Per-crate rustfmt override",
                message: "rustfmt.toml below workspace root overrides root formatting policy",
                file: Some("nested/rustfmt.toml"),
                inventory: false,
            },
        ],
    );
}
