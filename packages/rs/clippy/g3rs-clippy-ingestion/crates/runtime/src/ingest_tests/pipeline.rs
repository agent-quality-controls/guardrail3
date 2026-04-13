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
fn pipeline_reports_bad_line_threshold() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("clippy.toml"),
        "max-struct-bools = 3\nmax-fn-params-bools = 2\ntoo-many-lines-threshold = 1\ntoo-many-arguments-threshold = 3\nexcessive-nesting-threshold = 4\nallow-dbg-in-tests = false\nallow-expect-in-tests = true\nallow-panic-in-tests = false\nallow-print-in-tests = false\nallow-unwrap-in-tests = false\ncognitive-complexity-threshold = 25\ntype-complexity-threshold = 250\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_clippy_config_checks::check(&input);
    let summarized = results
        .iter()
        .filter(|result| result.id() == "RS-CLIPPY-CONFIG-03")
        .map(|result| {
            (
                result.id().to_owned(),
                format!("{:?}", result.severity()),
                result.title().to_owned(),
                result.message().to_owned(),
                result.file().map(str::to_owned),
                result.inventory(),
            )
        })
        .collect::<Vec<_>>();

    assert!(
        summarized
            == vec![(
                "RS-CLIPPY-CONFIG-03".to_owned(),
                "Error".to_owned(),
                "too-many-lines-threshold wrong value".to_owned(),
                "Expected 75, got 1. Set `too-many-lines-threshold = 75` in clippy.toml."
                    .to_owned(),
                Some("clippy.toml".to_owned()),
                false,
            )],
        "{results:#?}"
    );
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
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_clippy_filetree_checks::check(&input);
    let mut summarized = results
        .iter()
        .map(|result| {
            (
                result.id().to_owned(),
                format!("{:?}", result.severity()),
                result.title().to_owned(),
                result.file().map(str::to_owned),
                result.inventory(),
            )
        })
        .collect::<Vec<_>>();
    summarized.sort();

    assert_eq!(
        summarized,
        vec![
            (
                "RS-CLIPPY-FILETREE-01".to_owned(),
                "Info".to_owned(),
                "workspace root covered by clippy config".to_owned(),
                Some(".clippy.toml".to_owned()),
                true,
            ),
            (
                "RS-CLIPPY-FILETREE-02".to_owned(),
                "Error".to_owned(),
                "same-root clippy config conflict".to_owned(),
                Some("clippy.toml".to_owned()),
                false,
            ),
        ],
        "{results:#?}"
    );
}

#[test]
fn pipeline_warns_when_library_profile_cannot_prove_published_library_policy() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[workspace]\nnot = [valid");
    write(root.join("guardrail3.toml"), "[profile]\nname = \"library\"\n");
    write(root.join("clippy.toml"), "avoid-breaking-exported-api = true\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl)
        .expect("malformed root Cargo.toml should not abort clippy ingestion");
    let results = g3rs_clippy_config_checks::check(&input);
    let summarized = results
        .iter()
        .filter(|result| result.id() == "RS-CLIPPY-CONFIG-15")
        .map(|result| {
            (
                result.id().to_owned(),
                format!("{:?}", result.severity()),
                result.title().to_owned(),
                result.file().map(str::to_owned),
                result.inventory(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        summarized,
        vec![(
            "RS-CLIPPY-CONFIG-15".to_owned(),
            "Warn".to_owned(),
            "avoid-breaking-exported-api enabled".to_owned(),
            Some("clippy.toml".to_owned()),
            false,
        )],
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_malformed_clippy_conf_dir_override_surface() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "");
    write(root.join(".cargo/config.toml"), "env = []\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_clippy_config_checks::check(&input);
    let summarized = results
        .iter()
        .filter(|result| result.id() == "RS-CLIPPY-CONFIG-20")
        .map(|result| {
            (
                result.id().to_owned(),
                format!("{:?}", result.severity()),
                result.title().to_owned(),
                result.file().map(str::to_owned),
                result.inventory(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        summarized,
        vec![(
            "RS-CLIPPY-CONFIG-20".to_owned(),
            "Error".to_owned(),
            "cargo config override surface is not parseable".to_owned(),
            Some(".cargo/config.toml".to_owned()),
            false,
        )],
        "{results:#?}"
    );
}

#[test]
fn pipeline_still_runs_config_checks_for_raw_parseable_typed_invalid_clippy() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("clippy.toml"),
        "totally_fake_field = true\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_clippy_config_checks::check(&input);
    let summarized = results
        .iter()
        .filter(|result| result.id() == "RS-CLIPPY-CONFIG-21")
        .map(|result| {
            (
                result.id().to_owned(),
                format!("{:?}", result.severity()),
                result.title().to_owned(),
                result.file().map(str::to_owned),
                result.inventory(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        summarized,
        vec![(
            "RS-CLIPPY-CONFIG-21".to_owned(),
            "Error".to_owned(),
            "clippy.toml parse error".to_owned(),
            Some("clippy.toml".to_owned()),
            false,
        )],
        "{results:#?}"
    );
}
