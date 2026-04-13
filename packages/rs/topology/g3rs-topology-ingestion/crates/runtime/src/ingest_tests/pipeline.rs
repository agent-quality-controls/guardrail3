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
        .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-11")
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
        .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-11")
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
        .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-11")
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
        .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-12")
        .collect::<Vec<_>>();
    assert_eq!(rule_results.len(), 2);
    assert!(rule_results.iter().any(|result| {
        result.file() == Some("Cargo.toml")
            && result.title() == "Workspace `.` has extra member `crates/ghost`"
    }));
    assert!(rule_results.iter().any(|result| {
        result.file() == Some("crates/extra/Cargo.toml")
            && result.title() == "Workspace child `crates/extra` must be declared explicitly"
    }));
}

#[test]
fn nested_workspace_still_fires_membership_rule_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\", \"crates/nested\"]\n",
    );
    fs::create_dir_all(root.path().join("crates/api/src")).expect("api dirs");
    fs::create_dir_all(root.path().join("crates/nested/src")).expect("nested dirs");
    write(
        root.path().join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\n",
    );
    write(root.path().join("crates/api/src/lib.rs"), "pub struct Api;\n");
    write(
        root.path().join("crates/nested/Cargo.toml"),
        "[workspace]\nmembers = []\n",
    );
    write(
        root.path().join("crates/nested/src/lib.rs"),
        "pub struct Nested;\n",
    );

    let results = run_results(root.path());

    assert_eq!(
        results.iter().filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-11").count(),
        1
    );
    assert_eq!(
        results.iter().filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-12").count(),
        1
    );
    assert!(results.iter().any(|result| {
        result.id() == "RS-TOPOLOGY-FILETREE-12"
            && result.file() == Some("Cargo.toml")
            && result.title() == "Workspace `.` has extra member `crates/nested`"
    }));
}

#[test]
fn dot_slash_member_path_stays_quiet_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"./crates/api\"]\n",
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

    assert!(results.iter().all(|result| result.id() != "RS-TOPOLOGY-FILETREE-12"));
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
        .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-13")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1);
    assert_eq!(rule_results[0].file(), Some("Cargo.toml"));
}

#[test]
fn absolute_member_path_fires_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"/tmp/shared\"]\n",
    );

    let results = run_results(root.path());
    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-13")
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
        .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-16")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 2);
    assert!(rule_results.iter().all(|result| result.file() == Some("crates/api/clippy.toml")));
    assert!(rule_results.iter().any(|result| {
        result.title() == "`clippy` file `crates/api/clippy.toml` is illegally placed"
    }));
    assert!(rule_results.iter().any(|result| {
        result.title() == "`garde` file `crates/api/clippy.toml` is illegally placed"
    }));
}

#[test]
fn member_cargo_sidecar_illegal_placement_fires_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/api\"]\n",
    );
    fs::create_dir_all(root.path().join("crates/api/src")).expect("api dirs");
    fs::create_dir_all(root.path().join("crates/api/.cargo")).expect("cargo dir");
    write(
        root.path().join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\n",
    );
    write(
        root.path().join("crates/api/src/lib.rs"),
        "pub struct Api;\n",
    );
    write(
        root.path().join("crates/api/.cargo/config.toml"),
        "[alias]\n",
    );

    let results = run_results(root.path());
    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-16")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 2);
    assert!(rule_results
        .iter()
        .all(|result| result.file() == Some("crates/api/.cargo/config.toml")));
    assert!(rule_results.iter().any(|result| {
        result.title()
            == "`clippy` file `crates/api/.cargo/config.toml` is illegally placed"
    }));
    assert!(rule_results.iter().any(|result| {
        result.title()
            == "`garde` file `crates/api/.cargo/config.toml` is illegally placed"
    }));
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
        .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-16")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 2);
    assert!(rule_results.iter().all(|result| result.file() == Some("nested/clippy.toml")));
    assert!(rule_results.iter().any(|result| {
        result.title() == "`clippy` file `nested/clippy.toml` is illegally placed"
    }));
    assert!(rule_results.iter().any(|result| {
        result.title() == "`garde` file `nested/clippy.toml` is illegally placed"
    }));
}

#[test]
fn illegal_child_root_fmt_file_fires_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(root.path().join("Cargo.toml"), "[workspace]\nmembers = []\n");
    fs::create_dir_all(root.path().join("crates/api/src")).expect("api dirs");
    write(
        root.path().join("crates/api/Cargo.toml"),
        "[package]\nname = \"api\"\nversion = \"0.1.0\"\n",
    );
    write(root.path().join("crates/api/src/lib.rs"), "pub struct Api;\n");
    write(root.path().join("crates/api/rustfmt.toml"), "max_width = 100\n");

    let results = run_results(root.path());
    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-16")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1);
    assert_eq!(rule_results[0].file(), Some("crates/api/rustfmt.toml"));
    assert_eq!(
        rule_results[0].title(),
        "`fmt` file `crates/api/rustfmt.toml` is illegally placed"
    );
    assert!(rule_results[0]
        .message()
        .contains("fmt files must live at the validation root"));
}

#[test]
fn member_fmt_file_fires_end_to_end() {
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
    write(root.path().join("crates/api/src/lib.rs"), "pub struct Api;\n");
    write(root.path().join("crates/api/rustfmt.toml"), "max_width = 100\n");

    let results = run_results(root.path());
    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-16")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1);
    assert_eq!(rule_results[0].file(), Some("crates/api/rustfmt.toml"));
    assert_eq!(
        rule_results[0].title(),
        "`fmt` file `crates/api/rustfmt.toml` is illegally placed"
    );
    assert!(rule_results[0]
        .message()
        .contains("fmt files must live at the validation root"));
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

    assert!(results.is_empty());
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

    assert!(results.is_empty());
}

#[test]
fn descendant_manifest_failure_fails_closed_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"good\", \"bad\"]\n",
    );
    fs::create_dir_all(root.path().join("good/src")).expect("good dirs");
    fs::create_dir_all(root.path().join("bad/src")).expect("bad dirs");
    write(
        root.path().join("good/Cargo.toml"),
        "[package]\nname = \"good\"\nversion = \"0.1.0\"\n",
    );
    write(root.path().join("good/src/lib.rs"), "pub struct Good;\n");
    write(root.path().join("bad/Cargo.toml"), "[package");
    write(root.path().join("bad/src/lib.rs"), "pub struct Bad;\n");

    let results = run_results(root.path());
    assert_eq!(
        results.iter().filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-07").count(),
        1
    );
    assert_eq!(
        results.iter().filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-12").count(),
        0
    );
    assert!(results.iter().any(|result| {
        result.id() == "RS-TOPOLOGY-FILETREE-07" && result.file() == Some("bad/Cargo.toml")
    }));
}

#[test]
fn unreadable_descendant_manifest_fails_closed_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"good\", \"bad\"]\n",
    );
    fs::create_dir_all(root.path().join("good/src")).expect("good dirs");
    fs::create_dir_all(root.path().join("bad/src")).expect("bad dirs");
    write(
        root.path().join("good/Cargo.toml"),
        "[package]\nname = \"good\"\nversion = \"0.1.0\"\n",
    );
    write(root.path().join("good/src/lib.rs"), "pub struct Good;\n");
    let bad_manifest = root.path().join("bad/Cargo.toml");
    write(
        bad_manifest.clone(),
        "[package]\nname = \"bad\"\nversion = \"0.1.0\"\n",
    );
    write(root.path().join("bad/src/lib.rs"), "pub struct Bad;\n");
    make_unreadable(&bad_manifest);

    let results = run_results(root.path());
    restore_readable(&bad_manifest);

    assert_eq!(
        results.iter().filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-07").count(),
        1
    );
    assert_eq!(
        results.iter().filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-12").count(),
        0
    );
    assert!(results.iter().any(|result| {
        result.id() == "RS-TOPOLOGY-FILETREE-07" && result.file() == Some("bad/Cargo.toml")
    }));
}

#[test]
fn stale_read_descendant_manifest_fails_closed_end_to_end() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"good\", \"bad\"]\n",
    );
    fs::create_dir_all(root.path().join("good/src")).expect("good dirs");
    fs::create_dir_all(root.path().join("bad/src")).expect("bad dirs");
    write(
        root.path().join("good/Cargo.toml"),
        "[package]\nname = \"good\"\nversion = \"0.1.0\"\n",
    );
    write(root.path().join("good/src/lib.rs"), "pub struct Good;\n");
    let bad_manifest = root.path().join("bad/Cargo.toml");
    write(
        bad_manifest.clone(),
        "[package]\nname = \"bad\"\nversion = \"0.1.0\"\n",
    );
    write(root.path().join("bad/src/lib.rs"), "pub struct Bad;\n");

    let crawl = crawl(root.path()).expect("crawl");
    fs::remove_file(&bad_manifest).expect("remove bad manifest");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingest");
    let results = check(&input);

    assert_eq!(
        results.iter().filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-07").count(),
        1
    );
    assert_eq!(
        results.iter().filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-12").count(),
        0
    );
    assert!(results.iter().any(|result| {
        result.id() == "RS-TOPOLOGY-FILETREE-07" && result.file() == Some("bad/Cargo.toml")
    }));
}

fn write(path: std::path::PathBuf, content: &str) {
    fs::write(path, content).expect("write");
}

#[cfg(unix)]
fn make_unreadable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path).expect("metadata").permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(path, permissions).expect("set unreadable");
}

#[cfg(unix)]
fn restore_readable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path).expect("metadata").permissions();
    permissions.set_mode(0o644);
    fs::set_permissions(path, permissions).expect("restore readable");
}
