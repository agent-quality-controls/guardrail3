use std::collections::BTreeSet;

use super::super::super::test_support::run_tree as run_family;
use super::super::super::test_support::{StubToolChecker, dir_entry, project_tree, temp_root};
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{crate_facts, crate_input, repo_facts, workflow_from_yaml};
use super::super::check;

#[test]
fn inventories_real_binary_release_workflow_from_yaml() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary.yml",
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
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/binary.yml")
    );
    assert!(results[0].title.contains("binary release workflow present"));
    assert!(
        results[0]
            .message
            .contains("builds release binaries and uses a GitHub release action")
    );
    assert!(results[0].message.contains(".github/workflows/binary.yml"));
}

#[test]
fn inventories_real_binary_release_workflow_from_alternate_release_action() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary-alt-action.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
      - uses: ncipollo/release-action@v1
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/binary-alt-action.yml")
    );
    assert!(results[0].title.contains("binary release workflow present"));
}

#[test]
fn inventories_split_build_and_release_jobs_when_release_job_needs_build_job() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary-needs.yml",
        r#"
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
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
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/binary-needs.yml")
    );
    assert!(results[0].title.contains("binary release workflow present"));
}

#[test]
fn inventories_when_release_job_needs_build_job_via_array() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary-needs-array.yml",
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
    needs: [build, test]
    runs-on: ubuntu-latest
    steps:
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/binary-needs-array.yml")
    );
    assert_eq!(results[0].title, "bin: binary release workflow present");
    assert_eq!(
        results[0].message,
        "Workflow `.github/workflows/binary-needs-array.yml` builds release binaries and uses a GitHub release action."
    );
}

#[test]
fn inventories_when_release_job_reaches_build_job_transitively() {
    let mut krate = crate_facts("bin");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary-needs-transitive.yml",
        r#"
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release
  package:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - run: echo package
  publish:
    needs: package
    runs-on: ubuntu-latest
    steps:
      - uses: softprops/action-gh-release@v2
"#,
    ));
    let mut results = Vec::new();

    check(&input, &[repo], &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-BIN-01");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/binary-needs-transitive.yml")
    );
}

#[test]
fn inventories_when_workflow_targets_current_binary_crate_by_package() {
    let mut krate = crate_facts("cli");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary-targeted.yml",
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
        Some(".github/workflows/binary-targeted.yml")
    );
}

#[test]
fn inventories_when_workflow_targets_current_binary_crate_by_package_equals_syntax() {
    let mut krate = crate_facts("cli");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary-targeted-equals.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release --package=cli
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
        Some(".github/workflows/binary-targeted-equals.yml")
    );
}

#[test]
fn inventories_when_workflow_targets_current_binary_crate_by_bin_name() {
    let mut krate = crate_facts("my-package");
    krate.is_binary = true;
    krate.binary_target_names = BTreeSet::from(["cli".to_owned()]);
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary-targeted-bin.yml",
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
        results[0].file.as_deref(),
        Some(".github/workflows/binary-targeted-bin.yml")
    );
}

#[test]
fn inventories_when_workflow_targets_current_binary_crate_by_bin_equals_syntax() {
    let mut krate = crate_facts("my-package");
    krate.is_binary = true;
    krate.binary_target_names = BTreeSet::from(["cli".to_owned()]);
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary-targeted-bin-equals.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: cargo build --release --bin=cli
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
        Some(".github/workflows/binary-targeted-bin-equals.yml")
    );
}

#[test]
fn inventories_when_workflow_targets_current_binary_crate_by_manifest_path() {
    let mut krate = crate_facts("cli");
    krate.is_binary = true;
    krate.cargo_rel_path = "crates/cli/Cargo.toml".to_owned();
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary-targeted-manifest.yml",
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
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/binary-targeted-manifest.yml")
    );
}

#[test]
fn inventories_when_build_runs_through_shell_wrapper() {
    let mut krate = crate_facts("cli");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary-shell-wrapper.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: bash -lc 'cargo build --release -p cli'
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
        Some(".github/workflows/binary-shell-wrapper.yml")
    );
}

#[test]
fn inventories_when_build_uses_toolchain_and_absolute_cargo_path() {
    let mut krate = crate_facts("cli");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary-toolchain-wrapper.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: /home/runner/.cargo/bin/cargo +nightly build --release -p cli
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
        Some(".github/workflows/binary-toolchain-wrapper.yml")
    );
}

#[test]
fn inventories_when_build_runs_through_env_wrapper() {
    let mut krate = crate_facts("cli");
    krate.is_binary = true;
    let input = crate_input(&krate);
    let mut repo = repo_facts();
    repo.workflows.push(workflow_from_yaml(
        ".github/workflows/binary-env-wrapper.yml",
        r#"
jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - run: env FOO=1 cargo build --release --package=cli
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
        Some(".github/workflows/binary-env-wrapper.yml")
    );
}

#[test]
fn inventories_when_manifest_has_autodiscovered_src_bin_binary() {
    let root = temp_root("release-autodiscovered-src-bin-01");
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
name = "bin"
version = "0.1.0"
edition = "2024"
description = "bin"
license = "MIT"
repository = "https://example.com/bin"
"#,
            ),
            ("README.md", "# Bin\n\nREADME\n"),
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

    assert!(results.iter().any(|result| {
        result.id == "RS-BIN-01"
            && result.inventory
            && result.file.as_deref() == Some(".github/workflows/binary-release.yml")
            && result.title.contains("binary release workflow present")
    }));
}

#[test]
fn inventories_when_manifest_has_nested_autodiscovered_src_bin_binary() {
    let root = temp_root("release-autodiscovered-src-bin-nested-01");
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&[".github", "src"], &["Cargo.toml", "README.md"]),
            ),
            (".github", dir_entry(&["workflows"], &[])),
            (".github/workflows", dir_entry(&[], &["binary-release.yml"])),
            ("src", dir_entry(&["bin"], &[])),
            ("src/bin", dir_entry(&["cli"], &[])),
            ("src/bin/cli", dir_entry(&[], &["main.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "bin"
version = "0.1.0"
edition = "2024"
description = "bin"
license = "MIT"
repository = "https://example.com/bin"
"#,
            ),
            ("README.md", "# Bin\n\nREADME\n"),
            ("src/bin/cli/main.rs", "fn main() {}\n"),
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

    assert!(results.iter().any(|result| {
        result.id == "RS-BIN-01"
            && result.inventory
            && result.file.as_deref() == Some(".github/workflows/binary-release.yml")
            && result.title.contains("binary release workflow present")
    }));
}

#[test]
fn inventories_when_autobins_disabled_but_explicit_bin_exists() {
    let root = temp_root("release-explicit-bin-autobins-disabled-01");
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
name = "bin"
version = "0.1.0"
edition = "2024"
description = "bin"
license = "MIT"
repository = "https://example.com/bin"
autobins = false

[[bin]]
name = "bin"
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
      - run: cargo build --release -p bin
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
            && result.file.as_deref() == Some(".github/workflows/binary-release.yml")
    }));
}

#[test]
fn inventories_when_explicit_bin_manifest_is_targeted_by_bin_name() {
    let root = temp_root("release-explicit-bin-by-name-positive-01");
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
name = "cli"
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
            && result.file.as_deref() == Some(".github/workflows/binary-release.yml")
            && result.title.contains("binary release workflow present")
    }));
}
