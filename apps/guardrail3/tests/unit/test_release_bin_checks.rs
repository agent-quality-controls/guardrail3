use std::path::PathBuf;

use guardrail3::app::rs::validate::release_bin_checks::{
    check_binary_linux_target, check_binary_release_workflow, check_binstall_metadata,
};
use guardrail3::app::rs::validate::release_checks::CrateInfo;

// --- R-BIN-01 ---

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn bin01_neg_no_release_workflow() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_bin01_neg");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
    let _ = std::fs::write(tmp.join(".github/workflows/ci.yml"), "name: CI\n");

    let mut r = Vec::new();
    check_binary_release_workflow(&fs, &tmp, "mybin", &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-BIN-01" && c.title.contains("no binary release"))
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn bin01_pos_has_release_workflow() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_bin01_pos");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
    let _ = std::fs::write(
        tmp.join(".github/workflows/release.yml"),
        "cargo build --release\nuses: action-gh-release\n",
    );

    let mut r = Vec::new();
    check_binary_release_workflow(&fs, &tmp, "mybin", &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-BIN-01" && c.title.contains("binary release workflow found"))
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

// --- R-BIN-02 ---

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn bin02_neg_no_linux() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_bin02_neg");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
    let _ = std::fs::write(
        tmp.join(".github/workflows/ci.yml"),
        "name: CI\nruns-on: macos-latest\n",
    );

    let mut r = Vec::new();
    check_binary_linux_target(&fs, &tmp, "mybin", &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-BIN-02" && c.title.contains("no linux"))
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
#[allow(clippy::disallowed_methods)] // reason: tests need direct fs access for temp dirs
fn bin02_pos_has_linux() {
    let fs = guardrail3::adapters::outbound::fs::RealFileSystem;
    let tmp = std::env::temp_dir().join("guardrail3_bin02_pos");
    let _ = std::fs::remove_dir_all(&tmp);
    let _ = std::fs::create_dir_all(tmp.join(".github/workflows"));
    let _ = std::fs::write(
        tmp.join(".github/workflows/release.yml"),
        "runs-on: ubuntu-latest\ntarget: x86_64-unknown-linux-gnu\n",
    );

    let mut r = Vec::new();
    check_binary_linux_target(&fs, &tmp, "mybin", &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-BIN-02" && c.title.contains("linux target"))
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

// --- R-BIN-03 ---

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn bin03_neg_no_binstall() {
    let t: toml::Value = "[package]\nname = \"x\"\nversion = \"0.1.0\""
        .parse()
        .expect("parse"); // reason: test
    let krate = CrateInfo {
        name: "x".to_owned(),
        cargo_toml_path: PathBuf::from("Cargo.toml"),
        dir: PathBuf::from("."),
        publishable: true,
        is_binary: true,
        table: t,
    };
    let mut r = Vec::new();
    check_binstall_metadata(&krate, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-BIN-03" && c.title.contains("no binstall"))
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test assertion
fn bin03_pos_has_binstall() {
    let t: toml::Value =
        "[package]\nname = \"x\"\nversion = \"0.1.0\"\n[package.metadata.binstall]\npkg-url = \"https://example.com\""
            .parse()
            .expect("parse"); // reason: test
    let krate = CrateInfo {
        name: "x".to_owned(),
        cargo_toml_path: PathBuf::from("Cargo.toml"),
        dir: PathBuf::from("."),
        publishable: true,
        is_binary: true,
        table: t,
    };
    let mut r = Vec::new();
    check_binstall_metadata(&krate, &mut r);
    assert!(
        r.iter()
            .any(|c| c.id == "R-BIN-03" && c.title.contains("binstall metadata present"))
    );
}
