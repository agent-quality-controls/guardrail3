use g3rs_fmt_ingestion_assertions::run as assertions;
use tempfile::tempdir;

use super::helpers::{git_init, write};

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
    let input = crate::run::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_fmt_config_checks::check(&input);

    assertions::assert_nightly_rustfmt_keys_on_stable(&results);
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
    let input = crate::run::ingest_for_config_checks(&crawl).expect(
        "ingestion should preserve missing toolchain for RS-FMT-CONFIG-03 blocker reporting",
    );
    let results = g3rs_fmt_config_checks::check(&input);

    assertions::assert_nightly_key_blocker_when_toolchain_is_missing(&results);
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
    let input = crate::run::ingest_for_config_checks(&crawl).expect(
        "ingestion should preserve missing Cargo.toml for RS-FMT-CONFIG-04 blocker reporting",
    );
    let results = g3rs_fmt_config_checks::check(&input);

    assertions::assert_edition_blocker_when_cargo_is_missing(&results);
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
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve rustfmt parse failures for RS-FMT-CONFIG-01");
    let results = g3rs_fmt_config_checks::check(&input);

    assertions::assert_rustfmt_parse_error(&results);
}

#[test]
fn pipeline_reports_rustfmt_ignore_waiver_from_guardrail3_rs_toml() {
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
        root.join("guardrail3-rs.toml"),
        "[[waivers]]\nrule = \"RS-FMT-CONFIG-07\"\nfile = \"rustfmt.toml\"\nselector = \"ignore\"\nreason = \"Generated code rewrites break formatter stability.\"\n",
    );
    write(
        root.join("guardrail3.toml"),
        "[[escape_hatches]]\nfamily = \"fmt\"\nfile = \"rustfmt.toml\"\nkind = \"ignore\"\nselector = \"ignore\"\nreason = \"legacy dead config\"\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_fmt_config_checks::check(&input);

    assertions::assert_rustfmt_ignore_waiver(&results);
}

#[test]
fn pipeline_uses_root_dot_rustfmt_toml_for_config_checks() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join(".rustfmt.toml"),
        "group_imports = \"StdExternalCrate\"\n",
    );
    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(input.rustfmt_rel_path, ".rustfmt.toml");

    let results = g3rs_fmt_config_checks::check(&input);

    assertions::assert_root_dot_rustfmt_toml_for_config_checks(&results);
}

#[test]
fn pipeline_keeps_config_01_active_when_cargo_is_parse_error() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rustfmt.toml"),
        "edition = \"2024\"\nstyle_edition = \"2024\"\nmax_width = 90\ntab_spaces = 4\nuse_field_init_shorthand = true\nuse_try_shorthand = true\nreorder_imports = true\nreorder_modules = true\n",
    );
    write(root.join("Cargo.toml"), "[package]\nname = [\n");
    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_fmt_config_checks::check(&input);

    assertions::assert_keeps_config_01_active_when_cargo_is_parse_error(&results);
}
