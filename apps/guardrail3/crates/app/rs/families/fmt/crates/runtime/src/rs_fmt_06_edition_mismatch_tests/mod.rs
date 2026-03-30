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
fn fails_closed_when_root_cargo_manifest_is_malformed() {
    let fixture =
        tempfile::tempdir().expect("RS-FMT-06 fixture setup should create a temporary directory");
    let root = fixture.path();

    std::fs::write(root.join("Cargo.toml"), "[workspace")
        .expect("RS-FMT-06 fixture setup should write malformed Cargo.toml");
    std::fs::write(
        root.join("rustfmt.toml"),
        "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\n",
    )
    .expect("RS-FMT-06 fixture setup should write rustfmt.toml");
    std::fs::write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    )
    .expect("RS-FMT-06 fixture setup should write rust-toolchain.toml");

    let results = run_family(root);

    assertions::assert_error(
        &results,
        "Cargo.toml parse error",
        "rustfmt edition checks require a parseable root Cargo.toml.",
    );
}

#[test]
fn fails_closed_when_root_cargo_manifest_has_no_edition() {
    let fixture =
        tempfile::tempdir().expect("RS-FMT-06 fixture setup should create a temporary directory");
    let root = fixture.path();

    std::fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    )
    .expect("RS-FMT-06 fixture setup should write Cargo.toml without edition");
    std::fs::write(
        root.join("rustfmt.toml"),
        "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\n",
    )
    .expect("RS-FMT-06 fixture setup should write rustfmt.toml");
    std::fs::write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    )
    .expect("RS-FMT-06 fixture setup should write rust-toolchain.toml");

    let results = run_family(root);

    assertions::assert_error(
        &results,
        "Cargo.toml edition missing",
        "rustfmt edition checks require `[workspace.package].edition` or `[package].edition` in root Cargo.toml.",
    );
}

#[test]
fn fails_closed_when_root_cargo_manifest_is_missing() {
    let fixture =
        tempfile::tempdir().expect("RS-FMT-06 fixture setup should create a temporary directory");
    let root = fixture.path();

    std::fs::write(
        root.join("rustfmt.toml"),
        "edition = \"2024\"\nmax_width = 100\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\n",
    )
    .expect("RS-FMT-06 fixture setup should write rustfmt.toml");
    std::fs::write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    )
    .expect("RS-FMT-06 fixture setup should write rust-toolchain.toml");

    let results = run_family(root);

    assertions::assert_error(
        &results,
        "Cargo.toml missing",
        "rustfmt edition checks require a root Cargo.toml with workspace or package edition.",
    );
}
