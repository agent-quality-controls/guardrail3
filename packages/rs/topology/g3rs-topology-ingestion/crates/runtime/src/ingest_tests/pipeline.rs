use std::fs;

use g3rs_topology_file_tree_checks::check;
use g3rs_workspace_crawl::crawl;
use tempfile::tempdir;

fn run_results(root: &std::path::Path) -> Vec<guardrail3_check_types::G3CheckResult> {
    let crawl = crawl(root).expect("crawl");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingest");
    check(&input)
}

#[test]
fn nested_workspace_fires_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\"]\n",
    );
    fs::create_dir_all(root.path().join("crates/api/src")).expect("api dirs");
    fs::create_dir_all(root.path().join("crates/api/nested/src")).expect("nested dirs");
    write(
        root.path().join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\n",
    );
    write(
        root.path().join("crates/api/src/lib.rs"),
        "pub struct Api;\n",
    );
    write(
        root.path().join("crates/api/nested/Cargo.toml"),
        "[workspace]\nmembers = []\n",
    );
    write(
        root.path().join("crates/api/nested/src/lib.rs"),
        "pub struct Nested;\n",
    );

    let results = run_results(root.path());
    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-TOPOLOGY-11")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1);
    assert_eq!(rule_results[0].file(), Some("crates/api/nested/Cargo.toml"));
}

#[test]
fn excluded_nested_workspace_still_fires_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\"]\nexclude = [\"crates/api/nested\"]\n",
    );
    fs::create_dir_all(root.path().join("crates/api/src")).expect("api dirs");
    fs::create_dir_all(root.path().join("crates/api/nested/src")).expect("nested dirs");
    write(
        root.path().join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\n",
    );
    write(
        root.path().join("crates/api/src/lib.rs"),
        "pub struct Api;\n",
    );
    write(
        root.path().join("crates/api/nested/Cargo.toml"),
        "[workspace]\nmembers = []\n",
    );
    write(
        root.path().join("crates/api/nested/src/lib.rs"),
        "pub struct Nested;\n",
    );

    let results = run_results(root.path());
    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-TOPOLOGY-11")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1);
    assert_eq!(rule_results[0].file(), Some("crates/api/nested/Cargo.toml"));
}

#[test]
fn unreferenced_nested_workspace_still_fires_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\"]\n",
    );
    fs::create_dir_all(root.path().join("crates/api/src")).expect("api dirs");
    fs::create_dir_all(root.path().join("crates/stray/src")).expect("stray dirs");
    write(
        root.path().join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\n",
    );
    write(
        root.path().join("crates/api/src/lib.rs"),
        "pub struct Api;\n",
    );
    write(
        root.path().join("crates/stray/Cargo.toml"),
        "[workspace]\nmembers = []\n",
    );
    write(
        root.path().join("crates/stray/src/lib.rs"),
        "pub struct Stray;\n",
    );

    let results = run_results(root.path());
    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-TOPOLOGY-11")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1);
    assert_eq!(rule_results[0].file(), Some("crates/stray/Cargo.toml"));
}

#[test]
fn exact_membership_fires_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\", \"crates/ghost\"]\n",
    );
    fs::create_dir_all(root.path().join("crates/api/src")).expect("api dirs");
    fs::create_dir_all(root.path().join("crates/extra/src")).expect("extra dirs");
    write(
        root.path().join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\n",
    );
    write(
        root.path().join("crates/api/src/lib.rs"),
        "pub struct Api;\n",
    );
    write(
        root.path().join("crates/extra/Cargo.toml"),
        "[package]\nname = \"extra\"\nversion = \"0.1.0\"\n",
    );
    write(
        root.path().join("crates/extra/src/lib.rs"),
        "pub struct Extra;\n",
    );

    let results = run_results(root.path());

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-TOPOLOGY-12")
        .collect::<Vec<_>>();
    assert_eq!(rule_results.len(), 2);
    assert!(
        rule_results
            .iter()
            .any(|result| result.file() == Some("Cargo.toml"))
    );
    assert!(
        rule_results
            .iter()
            .any(|result| result.file() == Some("crates/extra/Cargo.toml"))
    );
}

#[test]
fn escaping_member_path_fires_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\", \"../shared\"]\n",
    );
    fs::create_dir_all(root.path().join("crates/api/src")).expect("api dirs");
    write(
        root.path().join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\n",
    );
    write(
        root.path().join("crates/api/src/lib.rs"),
        "pub struct Api;\n",
    );

    let results = run_results(root.path());
    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-TOPOLOGY-13")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1);
    assert_eq!(rule_results[0].file(), Some("Cargo.toml"));
}

#[test]
fn illegal_family_file_placement_fires_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\"]\n",
    );
    fs::create_dir_all(root.path().join("crates/api/src")).expect("api dirs");
    write(
        root.path().join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\n",
    );
    write(
        root.path().join("crates/api/src/lib.rs"),
        "pub struct Api;\n",
    );
    write(
        root.path().join("crates/api/clippy.toml"),
        "msrv = \"1.85\"\n",
    );

    let results = run_results(root.path());
    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-TOPOLOGY-16")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 2);
    assert!(
        rule_results
            .iter()
            .all(|result| result.file() == Some("crates/api/clippy.toml"))
    );
}

#[test]
fn illegal_root_nested_family_file_placement_fires_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\"]\n",
    );
    fs::create_dir_all(root.path().join("crates/api/src")).expect("api dirs");
    fs::create_dir_all(root.path().join("nested")).expect("nested dir");
    write(
        root.path().join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\n",
    );
    write(
        root.path().join("crates/api/src/lib.rs"),
        "pub struct Api;\n",
    );
    write(root.path().join("nested/clippy.toml"), "msrv = \"1.85\"\n");

    let results = run_results(root.path());
    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-TOPOLOGY-16")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 2);
    assert!(
        rule_results
            .iter()
            .all(|result| result.file() == Some("nested/clippy.toml"))
    );
}

#[test]
fn legal_workspace_stays_quiet() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\"]\n",
    );
    fs::create_dir_all(root.path().join("crates/api/src")).expect("api dirs");
    write(
        root.path().join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\n",
    );
    write(
        root.path().join("crates/api/src/lib.rs"),
        "pub struct Api;\n",
    );
    write(root.path().join("clippy.toml"), "msrv = \"1.85\"\n");
    write(
        root.path().join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );

    let results = run_results(root.path());

    assert!(results.iter().all(|result| !matches!(
        result.id(),
        "RS-TOPOLOGY-11" | "RS-TOPOLOGY-12" | "RS-TOPOLOGY-13" | "RS-TOPOLOGY-16"
    )));
}

#[test]
fn legal_root_sidecar_configs_stay_quiet() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\"]\n",
    );
    fs::create_dir_all(root.path().join("crates/api/src")).expect("api dirs");
    fs::create_dir_all(root.path().join(".cargo")).expect("cargo dir");
    fs::create_dir_all(root.path().join(".config")).expect("config dir");
    write(
        root.path().join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\n",
    );
    write(
        root.path().join("crates/api/src/lib.rs"),
        "pub struct Api;\n",
    );
    write(root.path().join(".cargo/config.toml"), "[alias]\n");
    write(
        root.path().join(".cargo/mutants.toml"),
        "timeout_multiplier = 2.0\n",
    );
    write(
        root.path().join(".config/nextest.toml"),
        "[profile.default]\n",
    );

    let results = run_results(root.path());

    assert!(results.iter().all(|result| result.id() != "RS-TOPOLOGY-16"));
}

fn write(path: std::path::PathBuf, content: &str) {
    fs::write(path, content).expect("write");
}
