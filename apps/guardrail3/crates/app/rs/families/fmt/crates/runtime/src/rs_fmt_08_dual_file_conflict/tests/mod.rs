mod helpers;
use guardrail3_app_rs_family_fmt_assertions::rs_fmt_08_dual_file_conflict as assertions;
use test_support::{tempdir, write_file};

use super::run_check;

#[test]
fn reports_dual_root_config_conflicts() {
    let results = run_check("");

    assertions::assert_conflict(&results, "rustfmt.toml");
}

#[test]
fn reports_nested_dual_config_conflicts_at_nested_path() {
    let results = run_check("nested");

    assertions::assert_conflict(&results, "nested/rustfmt.toml");
}

#[test]
fn discovers_only_root_dual_file_conflicts_from_family_walk() {
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
        "edition = \"2024\"\nstyle_edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\n",
    );
    write_file(
        root,
        ".rustfmt.toml",
        "edition = \"2024\"\nstyle_edition = \"2024\"\n",
    );
    write_file(root, "nested/rustfmt.toml", "max_width = 120\n");
    write_file(root, "nested/.rustfmt.toml", "max_width = 120\n");

    let results = super::run_family_check(root);

    assertions::assert_findings(
        &results,
        &[assertions::Finding {
            severity: assertions::Severity::Warn,
            title: "Conflicting rustfmt config files",
            message: "Both rustfmt.toml and .rustfmt.toml exist in the same directory",
            file: Some("rustfmt.toml"),
            inventory: false,
        }],
    );
}
