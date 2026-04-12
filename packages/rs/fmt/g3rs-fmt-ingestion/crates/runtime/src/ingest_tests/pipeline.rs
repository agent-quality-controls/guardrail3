use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn git_init(path: &Path) {
    let _ = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write fixture file");
}

#[test]
fn pipeline_reports_nightly_rustfmt_keys_on_stable() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );
    write(
        root.join("rustfmt.toml"),
        "group_imports = \"StdExternalCrate\"\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_fmt_config_checks::check(&input);

    assert!(
        results
            .iter()
            .any(|result| result.id() == "RS-FMT-CONFIG-03"),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_nightly_key_blocker_when_toolchain_is_missing() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("rustfmt.toml"),
        "group_imports = \"StdExternalCrate\"\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl).expect(
        "ingestion should preserve missing toolchain for RS-FMT-CONFIG-03 blocker reporting",
    );
    let results = g3rs_fmt_config_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-FMT-CONFIG-03"
                && result.title() == "rust-toolchain.toml missing"
                && result.file() == Some("rust-toolchain.toml")
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_edition_blocker_when_cargo_is_missing() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );
    write(root.join("rustfmt.toml"), "edition = \"2024\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl).expect(
        "ingestion should preserve missing Cargo.toml for RS-FMT-CONFIG-04 blocker reporting",
    );
    let results = g3rs_fmt_config_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-FMT-CONFIG-04"
                && result.title() == "Cargo.toml missing"
                && result.file() == Some("Cargo.toml")
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_rustfmt_parse_error_via_config_rule() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );
    write(root.join("rustfmt.toml"), "edition = [\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve rustfmt parse failures for RS-FMT-CONFIG-01");
    let results = g3rs_fmt_config_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-FMT-CONFIG-01"
                && result.title() == "rustfmt config parse error"
                && result.file() == Some("rustfmt.toml")
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_rustfmt_ignore_escape_hatch() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );
    write(root.join("rustfmt.toml"), "ignore = [\"generated/**\"]\n");
    write(
        root.join("guardrail3.toml"),
        "[[escape_hatches]]\nfamily = \"fmt\"\nfile = \"rustfmt.toml\"\nkind = \"ignore\"\nselector = \"ignore\"\nreason = \"Generated code rewrites break formatter stability.\"\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_fmt_config_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-FMT-CONFIG-07"
                && result.title() == "rustfmt ignore escape hatch"
                && result.file() == Some("rustfmt.toml")
        }),
        "{results:#?}"
    );
}
