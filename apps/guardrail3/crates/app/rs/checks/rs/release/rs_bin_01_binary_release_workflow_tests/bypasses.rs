use std::collections::BTreeSet;

use super::super::super::check as run_family;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};
use crate::domain::report::Severity;

use super::super::super::test_support::{crate_facts, crate_input, repo_facts, workflow_from_yaml};
use super::super::check;

#[test]
fn reports_absence_when_release_build_only_appears_in_echo_line() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/fake-binary.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: echo cargo build --release
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo.clone()], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].title.contains("no binary release workflow"));
    assert_eq!(
        results[0].message,
        "No workflow builds a release binary and publishes it via GitHub Releases."
    );
}

#[test]
fn does_not_emit_for_non_binary_publishable_crates() {
    let krate = crate_facts("lib");
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary.yml",
        r#"
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo.clone()], &mut results);

    assert!(results.is_empty());

    let mut non_publishable_binary = crate_facts("bin");
    non_publishable_binary.is_binary = true;
    non_publishable_binary.publishable = false;
    let non_publishable_binary_input = crate_input(&non_publishable_binary);
    let mut non_publishable_binary_results = Vec::new();

    check(
        &non_publishable_binary_input,
        &[repo.clone()],
        &mut non_publishable_binary_results,
    );

    assert!(non_publishable_binary_results.is_empty());
}

#[test]
fn should_not_count_release_action_when_real_release_build_is_missing() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/fake-binary.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: echo cargo build --release
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo.clone()], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].title.contains("no binary release workflow"));
}

#[test]
fn should_not_join_unrelated_build_and_release_jobs_into_a_pass() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/split.yml",
        r#"
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
  announce:
    runs-on: ubuntu-latest
    steps:
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo.clone()], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].title.contains("no binary release workflow"));
}

#[test]
fn reports_absence_when_repo_has_no_workflows() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let repo = repo_facts();
    let mut results = Vec::new();

    check(&input, &[repo.clone()], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert_eq!(results[0].title, "bin: no binary release workflow");
}

#[test]
fn reports_absence_when_release_action_is_missing() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/build-only.yml",
        r#"
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo.clone()], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert_eq!(results[0].title, "bin: no binary release workflow");
}

#[test]
fn does_not_emit_for_autobins_disabled_package_with_src_main() {
    let root = temp_root("release-autobins-disabled-bin-01");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&[".github", "src"], &["Cargo.toml", "README.md"]),
            ),
            (".github", dir_entry(&["workflows"], &[])),
            (".github/workflows", dir_entry(&[], &["binary-release.yml"])),
            ("src", dir_entry(&[], &["main.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "not-a-bin"
version = "0.1.0"
edition = "2024"
description = "not-a-bin"
license = "MIT"
repository = "https://example.com/not-a-bin"
autobins = false
"#,
            ),
            ("README.md", "# Not A Bin\n\nREADME\n"),
            ("src/main.rs", "fn main() {}\n"),
            (
                ".github/workflows/binary-release.yml",
                r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
      - uses: softprops/action-gh-release@v2
"#,
            ),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(
        results.iter().all(|result| result.id != "RS-BIN-01"),
        "autobins=false package should stay out of RS-BIN-01: {results:#?}"
    );
}

#[test]
fn reports_absence_when_workflow_targets_different_binary_crate() {
    let mut krate = crate_facts("worker");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/cli-only.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release -p cli
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert_eq!(results[0].title, "worker: no binary release workflow");
}

#[test]
fn reports_absence_when_generic_release_build_cannot_be_linked_to_current_crate() {
    let mut krate = crate_facts("worker");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.publishable_binary_crate_names = BTreeSet::from(["cli".to_owned(), "worker".to_owned()]);
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/generic-build.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "worker: no binary release workflow");
}

#[test]
fn reports_absence_when_workflow_targets_different_binary_by_bin_name() {
    let mut krate = crate_facts("worker-package");
    krate.is_binary = true;
    krate.binary_target_names = BTreeSet::from(["worker-cli".to_owned()]);
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/cli-only-bin.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release --bin cli
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert!(results[0].inventory);
    assert_eq!(
        results[0].title,
        "worker-package: no binary release workflow"
    );
}

#[test]
fn does_not_emit_for_autobins_disabled_package_with_src_bin() {
    let root = temp_root("release-autobins-disabled-src-bin-01");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&[".github", "src"], &["Cargo.toml", "README.md"]),
            ),
            (".github", dir_entry(&["workflows"], &[])),
            (".github/workflows", dir_entry(&[], &["binary-release.yml"])),
            ("src", dir_entry(&["bin"], &[])),
            ("src/bin", dir_entry(&[], &["cli.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "not-a-bin"
version = "0.1.0"
edition = "2024"
description = "not-a-bin"
license = "MIT"
repository = "https://example.com/not-a-bin"
autobins = false
"#,
            ),
            ("README.md", "# Not A Bin\n\nREADME\n"),
            ("src/bin/cli.rs", "fn main() {}\n"),
            (
                ".github/workflows/binary-release.yml",
                r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
      - uses: softprops/action-gh-release@v2
"#,
            ),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(
        results.iter().all(|result| result.id != "RS-BIN-01"),
        "autobins=false src/bin package should stay out of RS-BIN-01: {results:#?}"
    );
}

#[test]
fn reports_absence_when_explicit_bin_manifest_targets_different_bin_name() {
    let root = temp_root("release-explicit-bin-by-name-negative-01");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&[".github", "src"], &["Cargo.toml", "README.md"]),
            ),
            (".github", dir_entry(&["workflows"], &[])),
            (".github/workflows", dir_entry(&[], &["binary-release.yml"])),
            ("src", dir_entry(&[], &["main.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "my-package"
version = "0.1.0"
edition = "2024"
description = "bin"
license = "MIT"
repository = "https://example.com/bin"
autobins = false

[[bin]]
name = "worker"
path = "src/main.rs"
"#,
            ),
            ("README.md", "# Bin\n\nREADME\n"),
            ("src/main.rs", "fn main() {}\n"),
            (
                ".github/workflows/binary-release.yml",
                r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release --bin cli
      - uses: softprops/action-gh-release@v2
"#,
            ),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(results.iter().any(|result| {
        result.id == "RS-BIN-01"
            && result.inventory
            && result.file.as_deref() == Some("Cargo.toml")
            && result.title.contains("no binary release workflow")
    }));
}

#[test]
fn reports_absence_when_needs_chain_targets_different_binary_crate() {
    let mut krate = crate_facts("worker");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/cli-only-needs.yml",
        r#"
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release -p cli
  publish:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "worker: no binary release workflow");
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
}

#[test]
fn reports_absence_when_manifest_path_targets_different_crate() {
    let mut krate = crate_facts("worker");
    krate.is_binary = true;
    krate.cargo_rel_path = "crates/worker/Cargo.toml".to_owned();
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/cli-manifest-path.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release --manifest-path crates/cli/Cargo.toml
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "worker: no binary release workflow");
    assert_eq!(results[0].file.as_deref(), Some("crates/worker/Cargo.toml"));
}

#[test]
fn reports_absence_when_release_job_does_not_need_real_build_job_from_array() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/needs-array-near-miss.yml",
        r#"
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
  test:
    runs-on: ubuntu-latest
    steps:
      - run: cargo test
  publish:
    needs: [test]
    runs-on: ubuntu-latest
    steps:
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "bin: no binary release workflow");
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
}

#[test]
fn reports_absence_for_action_name_that_only_contains_release_action_substring() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/fake-release-action.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
      - uses: owner/action-gh-release-wrapper@v1
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "bin: no binary release workflow");
}

#[test]
fn reports_absence_for_action_name_that_only_contains_alternate_release_action_substring() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/fake-alt-release-action.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
      - uses: owner/release-action-wrapper@v1
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "bin: no binary release workflow");
}
