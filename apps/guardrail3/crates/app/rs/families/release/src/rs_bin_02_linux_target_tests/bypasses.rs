use std::collections::BTreeSet;

use super::super::super::test_support::run_tree as run_family;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{crate_facts, crate_input, repo_facts, workflow_from_yaml};
use super::super::check;

#[test]
fn reports_absence_when_linux_only_appears_in_display_text() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/macos.yml",
        r#"
name: linux release notes
jobs:
  build:
    runs-on: macos-latest
    steps:
      - name: linux target someday
        run: cargo build --release
  publish:
    runs-on: macos-latest
    steps:
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo.clone()], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-02");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].title.contains("no linux release target"));
    assert_eq!(
        results[0].message,
        "No workflow includes a Linux target for binary release."
    );
}

#[test]
fn reports_absence_when_linux_only_appears_in_echo_output() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/linux-echo.yml",
        r#"
jobs:
  publish:
    runs-on: macos-latest
    steps:
      - run: echo x86_64-unknown-linux-gnu && cargo build --release
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-02");
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "bin: no linux release target");
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
fn should_not_count_linux_target_from_release_action_job_without_linux_build() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/fake-linux.yml",
        r#"
jobs:
  build:
    runs-on: macos-latest
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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-02");
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].title.contains("no linux release target"));
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
    assert_eq!(results[0].id, "RS-BIN-02");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert_eq!(results[0].title, "bin: no linux release target");
}

#[test]
fn reports_absence_when_workflow_targets_only_non_linux_platforms() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/non-linux.yml",
        r#"
jobs:
  publish:
    strategy:
      matrix:
        os: [macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - run: cargo build --release --target x86_64-apple-darwin
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo.clone()], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-02");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert_eq!(results[0].title, "bin: no linux release target");
}

#[test]
fn reports_absence_when_target_dir_only_mentions_linux() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/linux-target-dir.yml",
        r#"
jobs:
  publish:
    runs-on: macos-latest
    steps:
      - run: cargo build --release --target-dir target/linux-cache
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-02");
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "bin: no linux release target");
}

#[test]
fn does_not_emit_for_autobins_disabled_package_with_src_main() {
    let root = temp_root("release-autobins-disabled-bin-02");
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
      - run: cargo build --release --target x86_64-unknown-linux-gnu
      - uses: softprops/action-gh-release@v2
"#,
            ),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(
        results.iter().all(|result| result.id != "RS-BIN-02"),
        "autobins=false package should stay out of RS-BIN-02: {results:#?}"
    );
}

#[test]
fn reports_absence_when_linux_workflow_targets_different_binary_crate() {
    let mut krate = crate_facts("worker");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/cli-linux-only.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release -p cli --target x86_64-unknown-linux-gnu
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-02");
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert_eq!(results[0].title, "worker: no linux release target");
}

#[test]
fn reports_absence_when_linux_build_targets_different_binary_crate_through_needs() {
    let mut krate = crate_facts("worker");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/cli-linux-needs.yml",
        r#"
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release -p cli --target x86_64-unknown-linux-gnu
  publish:
    needs: build
    runs-on: macos-latest
    steps:
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-02");
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "worker: no linux release target");
}

#[test]
fn does_not_emit_for_autobins_disabled_package_with_src_bin() {
    let root = temp_root("release-autobins-disabled-src-bin-02");
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
      - run: cargo build --release --target x86_64-unknown-linux-gnu
      - uses: softprops/action-gh-release@v2
"#,
            ),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(
        results.iter().all(|result| result.id != "RS-BIN-02"),
        "autobins=false src/bin package should stay out of RS-BIN-02: {results:#?}"
    );
}

#[test]
fn reports_absence_when_release_job_does_not_need_linux_build_from_array() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/linux-needs-array-near-miss.yml",
        r#"
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
  test:
    runs-on: macos-latest
    steps:
      - run: cargo test
  publish:
    needs: [test]
    runs-on: macos-latest
    steps:
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-02");
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "bin: no linux release target");
}

#[test]
fn reports_absence_when_generic_linux_release_build_cannot_be_linked_to_current_crate() {
    let mut krate = crate_facts("worker");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.publishable_binary_crate_names = BTreeSet::from(["cli".to_owned(), "worker".to_owned()]);
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/generic-linux-build.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release --target x86_64-unknown-linux-gnu
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-02");
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "worker: no linux release target");
}

#[test]
fn reports_absence_when_linux_target_step_builds_different_crate_in_same_job() {
    let mut krate = crate_facts("worker");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/mixed-linux-build-steps.yml",
        r#"
jobs:
  publish:
    runs-on: macos-latest
    steps:
      - run: cargo build --release -p worker
      - run: cargo build --release -p cli --target x86_64-unknown-linux-gnu
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-02");
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "worker: no linux release target");
}
