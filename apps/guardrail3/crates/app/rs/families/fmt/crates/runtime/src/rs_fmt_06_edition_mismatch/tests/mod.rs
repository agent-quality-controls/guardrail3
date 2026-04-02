use guardrail3_app_rs_family_fmt_assertions::rs_fmt_06_edition_mismatch as assertions;

use super::{TestCargoEditionState, run_check, run_family};

#[test]
fn reports_rustfmt_edition_mismatch() {
    let results = run_check(TestCargoEditionState::Edition("2024"), "2021");

    assertions::assert_mismatch(&results, "2021", "2024");
}

#[test]
fn emits_no_result_when_editions_match() {
    let results = run_check(TestCargoEditionState::Edition("2024"), "2024");

    assertions::assert_no_findings(&results);
}

#[test]
fn reads_package_edition_when_workspace_package_edition_is_absent() {
    let fixture = tempfile::tempdir().expect("fixture setup should create a temporary directory");
    let root = fixture.path();

    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .expect("fixture setup should write package Cargo.toml");
    std::fs::write(
        root.join("rustfmt.toml"),
        "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\n",
    )
    .expect("fixture setup should write rustfmt.toml");
    std::fs::write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    )
    .expect("fixture setup should write rust-toolchain.toml");

    let results = run_family(root);

    assertions::assert_mismatch(&results, "2024", "2021");
}

#[test]
fn fails_closed_when_root_cargo_manifest_is_malformed() {
    let fixture = tempfile::tempdir().expect("fixture setup should create a temporary directory");
    let root = fixture.path();

    std::fs::write(root.join("Cargo.toml"), "[workspace")
        .expect("fixture setup should write malformed Cargo.toml");
    std::fs::write(
        root.join("rustfmt.toml"),
        "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\n",
    )
    .expect("fixture setup should write rustfmt.toml");
    std::fs::write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    )
    .expect("fixture setup should write rust-toolchain.toml");

    let results = run_family(root);

    assertions::assert_malformed_root_manifest_error(&results);
}

#[test]
fn fails_closed_when_root_cargo_manifest_has_no_edition() {
    let fixture = tempfile::tempdir().expect("fixture setup should create a temporary directory");
    let root = fixture.path();

    std::fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    )
    .expect("fixture setup should write Cargo.toml without edition");
    std::fs::write(
        root.join("rustfmt.toml"),
        "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\n",
    )
    .expect("fixture setup should write rustfmt.toml");
    std::fs::write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    )
    .expect("fixture setup should write rust-toolchain.toml");

    let results = run_family(root);

    assertions::assert_missing_root_edition_error(&results);
}

#[test]
fn fails_closed_when_root_cargo_manifest_is_missing() {
    let fixture = tempfile::tempdir().expect("fixture setup should create a temporary directory");
    let root = fixture.path();

    std::fs::write(
        root.join("rustfmt.toml"),
        "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\n",
    )
    .expect("fixture setup should write rustfmt.toml");
    std::fs::write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    )
    .expect("fixture setup should write rust-toolchain.toml");

    let results = run_family(root);

    assertions::assert_missing_root_manifest_error(&results);
}
