use g3rs_clippy_ingestion_assertions::run as assertions;
use tempfile::tempdir;

use super::helpers::{git_init, write};

#[test]
fn pipeline_reports_bad_line_threshold() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("clippy.toml"),
        "max-struct-bools = 3\nmax-fn-params-bools = 2\ntoo-many-lines-threshold = 1\ntoo-many-arguments-threshold = 3\nexcessive-nesting-threshold = 4\nallow-dbg-in-tests = false\nallow-expect-in-tests = true\nallow-panic-in-tests = false\nallow-print-in-tests = false\nallow-unwrap-in-tests = false\ncognitive-complexity-threshold = 25\ntype-complexity-threshold = 250\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_clippy_config_checks::check(&input);

    assertions::assert_bad_line_threshold(&results);
}

#[test]
fn filetree_pipeline_prefers_dotfile_and_reports_plain_same_root_conflict() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");
    write(root.join(".clippy.toml"), "msrv = \"1.80\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        crate::run::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_clippy_filetree_checks::check(&input);

    assertions::assert_same_root_conflict(&results);
}

#[test]
fn pipeline_warns_when_library_profile_cannot_prove_published_library_policy() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[workspace]\nnot = [valid");
    write(root.join("guardrail3-rs.toml"), "profile = \"library\"\n");
    write(
        root.join("clippy.toml"),
        "avoid-breaking-exported-api = true\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("malformed root Cargo.toml should not abort clippy ingestion");
    let results = g3rs_clippy_config_checks::check(&input);

    assertions::assert_library_profile_warning(&results);
}

#[test]
fn pipeline_reports_malformed_clippy_conf_dir_override_surface() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "");
    write(root.join(".cargo/config.toml"), "env = []\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_clippy_config_checks::check(&input);

    assertions::assert_override_surface_parse_error(&results);
}

#[test]
fn pipeline_still_runs_config_checks_for_raw_parseable_typed_invalid_clippy() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "totally_fake_field = true\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_clippy_config_checks::check(&input);

    assertions::assert_config_parse_error_contains(&results, "unknown field `totally_fake_field`");
}
