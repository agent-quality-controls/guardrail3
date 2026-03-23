use crate::domain::report::Severity;

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

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-02");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
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

    check(&input, &[repo], &mut results);

    assert!(results.is_empty());
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

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-02");
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
}
