use guardrail3_app_rs_family_fmt_assertions::rs_fmt_01_exists as assertions;

use super::{run_check, run_family};

#[test]
fn inventories_when_root_rustfmt_config_exists() {
    let results = run_check(Some("rustfmt.toml"));

    assertions::assert_no_findings(&results);
}

#[test]
fn accepts_root_dot_rustfmt_toml() {
    let fixture = tempfile::tempdir().expect("create tempdir");
    let root = fixture.path();

    std::fs::write(root.join(".rustfmt.toml"), "edition = \"2024\"\n").expect("write rustfmt");
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("write Cargo.toml");
    std::fs::write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    )
    .expect("write rust-toolchain.toml");

    let results = run_family(root);

    assertions::assert_no_findings(&results);
}

#[test]
fn errors_when_root_rustfmt_config_is_missing() {
    let results = run_check(None);

    assertions::assert_missing_root_config(&results);
}
