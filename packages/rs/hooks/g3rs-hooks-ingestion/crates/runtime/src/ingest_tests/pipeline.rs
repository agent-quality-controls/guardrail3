use std::fs;
use std::process::Command;

use tempfile::tempdir;

fn write(path: impl AsRef<std::path::Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write fixture");
}

fn git_init(path: &std::path::Path) {
    let status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
    assert!(status.success(), "git init should exit successfully");
}

fn repo_root(temp_dir: &tempfile::TempDir) -> &std::path::Path {
    let root = temp_dir.path();
    git_init(root);
    root
}

fn git_config_hooks_path(path: &std::path::Path, hooks_path: &str) {
    let status = Command::new("git")
        .args(["config", "core.hooksPath", hooks_path])
        .current_dir(path)
        .status()
        .expect("git config should succeed");
    assert!(status.success(), "git config should exit successfully");
}

#[cfg(unix)]
fn make_executable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt as _;

    let mut permissions = fs::metadata(path).expect("metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("set executable bit");
}

#[test]
fn pipeline_reports_fmt_step_when_real_command_exists() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "cargo fmt --check\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-SOURCE-03")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1, "{results:#?}");
    assert!(
        rule_results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-03"
                && result.file() == Some(".githooks/pre-commit")
                && result.title() == "cargo fmt --check step present"
                && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_keeps_hook_rs_10_quiet_for_single_crate_repo() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join(".githooks/pre-commit"), "cargo test\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-SOURCE-11")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1, "{results:#?}");
    assert!(
        rule_results.iter().any(|result| {
            result.title() == "cargo test workspace scope not required" && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_keeps_echoed_fmt_text_as_missing_step() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "echo \"cargo fmt --check\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-SOURCE-03")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1, "{results:#?}");
    assert!(
        rule_results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-03"
                && result.file() == Some(".githooks/pre-commit")
                && result.title() == "cargo fmt --check step missing"
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_works_through_hooks_pre_commit_fallback() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("hooks/pre-commit"), "cargo fmt --check\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-SOURCE-03")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1, "{results:#?}");
    assert!(
        rule_results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-03"
                && result.file() == Some("hooks/pre-commit")
                && result.title() == "cargo fmt --check step present"
                && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_fallback_hook_stays_quiet_about_inactive_modular_layout() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("hooks/pre-commit"), "cargo fmt --check\n");
    write(root.join(".githooks/pre-commit.d/10-rust.sh"), "echo cargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert!(
        results.iter().all(|result| result.file() != Some(".githooks/pre-commit.d/10-rust.sh")),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_rs_config_trigger_for_guardrail3_rs_toml() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(
        root.join(".githooks/pre-commit"),
        "if echo \"$STAGED_FILES\" | grep -qE '(guardrail3-rs\\.toml|clippy\\.toml|\\.clippy\\.toml|deny\\.toml|\\.deny\\.toml|rustfmt\\.toml|\\.rustfmt\\.toml|rust-toolchain\\.toml)$'; then\n    g3rs validate --path .\nfi\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-SOURCE-15")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1, "{results:#?}");
    assert!(
        rule_results.iter().any(|result| {
            result.title() == "Rust config changes trigger hook validation" && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_top_level_g3rs_validate_step_inventory() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "g3rs validate --path .\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let matching = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-SOURCE-09")
        .collect::<Vec<_>>();

    assert_eq!(matching.len(), 1, "{results:#?}");
    assert!(
        matching.iter().any(|result| {
            result.file() == Some(".githooks/pre-commit")
                && result.title() == "Rust guardrail validate step present"
                && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_stays_clean_for_valid_githooks_setup() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    fs::create_dir_all(root.join(".githooks/pre-commit.d")).expect("create modular dir");
    write(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\nrun-parts .githooks/pre-commit.d\ncargo fmt --check\ncargo clippy -- -D warnings\ncargo deny check\ncargo test --workspace\ncargo machete\ngitleaks protect --staged --no-banner\ncargo dupes check --exclude-tests\ng3rs validate --path .\npnpm install --frozen-lockfile\nrg '^(<<<<<<<|=======|>>>>>>>)' .\nstat -c%s Cargo.toml >/dev/null\nif echo \"$STAGED_FILES\" | grep -qE '(guardrail3-rs\\.toml|clippy\\.toml|\\.clippy\\.toml|deny\\.toml|\\.deny\\.toml|rustfmt\\.toml|\\.rustfmt\\.toml|rust-toolchain\\.toml)$'; then\n    g3rs validate --path .\nfi\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .filter(|result| !result.inventory())
        .collect::<Vec<_>>();

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn pipeline_reports_inventory_for_valid_rust_hook_steps() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\ncargo fmt --check\ncargo clippy -- -D warnings\ncargo deny check\ncargo test --workspace\ncargo machete\ncargo dupes check --exclude-tests\ngitleaks protect --staged --no-banner\ng3rs validate --path .\nrg '^(<<<<<<<|=======|>>>>>>>)' .\nstat -c%s Cargo.toml >/dev/null\npnpm install --frozen-lockfile\nif echo \"$STAGED_FILES\" | grep -qE '(guardrail3-rs\\.toml|clippy\\.toml|\\.clippy\\.toml|deny\\.toml|\\.deny\\.toml|rustfmt\\.toml|\\.rustfmt\\.toml|rust-toolchain\\.toml)$'; then\n    g3rs validate --path .\nfi\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        ("RS-HOOKS-SOURCE-04", "cargo clippy step present"),
        ("RS-HOOKS-SOURCE-05", "cargo deny check step present"),
        ("RS-HOOKS-SOURCE-06", "cargo test step present"),
        ("RS-HOOKS-SOURCE-07", "cargo machete step present"),
        (
            "RS-HOOKS-SOURCE-08",
            "cargo-dupes selected for Rust duplication checks",
        ),
        ("RS-HOOKS-SOURCE-10", "cargo clippy denies warnings"),
        ("RS-HOOKS-SOURCE-12", "gitleaks step present"),
        ("RS-HOOKS-SOURCE-13", "cargo dupes step present"),
        ("RS-HOOKS-SOURCE-14", "cargo-dupes excludes tests"),
    ] {
        let matching = results
            .iter()
            .filter(|result| result.id() == rule_id)
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 1, "{rule_id}: {results:#?}");
        assert!(
            matching.iter().any(|result| {
                result.file() == Some(".githooks/pre-commit")
                    && result.title() == title
                    && result.inventory()
            }),
            "{rule_id}: {results:#?}"
        );
    }
}

#[test]
fn pipeline_reports_inventory_for_command_substitution_and_binary_aliases() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\nOUT=\"$(g3rs validate --path .)\"\ncargo-deny check\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        ("RS-HOOKS-SOURCE-05", "cargo deny check step present"),
        ("RS-HOOKS-SOURCE-09", "Rust guardrail validate step present"),
    ] {
        let matching = results
            .iter()
            .filter(|result| result.id() == rule_id)
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 1, "{rule_id}: {results:#?}");
        assert!(
            matching.iter().any(|result| {
                result.file() == Some(".githooks/pre-commit")
                    && result.title() == title
                    && result.inventory()
            }),
            "{rule_id}: {results:#?}"
        );
    }
}

#[test]
fn pipeline_reports_inventory_for_called_function_commands() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nrun_guardrails() {\n    g3rs validate --path .\n}\nrun_guardrails\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let matching = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-SOURCE-09")
        .collect::<Vec<_>>();

    assert_eq!(matching.len(), 1, "{results:#?}");
    assert!(
        matching.iter().any(|result| {
            result.file() == Some(".githooks/pre-commit")
                && result.title() == "Rust guardrail validate step present"
                && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_inventory_for_executed_subshell_commands() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\n(g3rs validate --path .)\n(cargo-deny check)\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        ("RS-HOOKS-SOURCE-05", "cargo deny check step present"),
        ("RS-HOOKS-SOURCE-09", "Rust guardrail validate step present"),
    ] {
        let matching = results
            .iter()
            .filter(|result| result.id() == rule_id)
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 1, "{rule_id}: {results:#?}");
        assert!(
            matching.iter().any(|result| {
                result.file() == Some(".githooks/pre-commit")
                    && result.title() == title
                    && result.inventory()
            }),
            "{rule_id}: {results:#?}"
        );
    }
}

#[test]
fn pipeline_reports_inventory_for_valid_shell_hook_steps() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\nrg '^(<<<<<<<|=======|>>>>>>>)' .\nstat -c%s Cargo.toml >/dev/null\npnpm install --frozen-lockfile\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        ("RS-HOOKS-SOURCE-20", "merge-conflict check step present"),
        ("RS-HOOKS-SOURCE-21", "file-size check step present"),
        (
            "RS-HOOKS-SOURCE-23",
            "concrete lockfile integrity command present",
        ),
    ] {
        let matching = results
            .iter()
            .filter(|result| result.id() == rule_id)
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 1, "{rule_id}: {results:#?}");
        assert!(
            matching.iter().any(|result| {
                result.file() == Some(".githooks/pre-commit")
                    && result.title() == title
                    && result.inventory()
            }),
            "{rule_id}: {results:#?}"
        );
    }
}

#[test]
fn pipeline_reports_dispatcher_findings_for_real_pre_commit_script() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    fs::create_dir_all(root.join(".githooks/pre-commit.d")).expect("create modular dir");
    write(root.join(".githooks/pre-commit"), "run-parts .githooks/pre-commit.d\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let dispatcher_results = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-SOURCE-01")
        .collect::<Vec<_>>();
    let syntax_results = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-SOURCE-02")
        .collect::<Vec<_>>();

    assert_eq!(dispatcher_results.len(), 1, "{results:#?}");
    assert_eq!(syntax_results.len(), 1, "{results:#?}");
    assert!(
        dispatcher_results.iter().any(|result| {
            result.file() == Some(".githooks/pre-commit")
                && result.title() == "dispatcher pattern present"
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        syntax_results.iter().any(|result| {
            result.file() == Some(".githooks/pre-commit")
                && result.title() == "dispatcher uses real executable syntax"
                && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_source_reports_missing_dispatcher_pattern_when_modular_dir_is_only_echoed() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "echo run-parts .githooks/pre-commit.d\n");
    fs::create_dir_all(root.join(".githooks/pre-commit.d")).expect("create modular dir");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-01"
                && result.file() == Some(".githooks/pre-commit")
                && result.title() == "dispatcher pattern missing"
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-02"
                && result.file() == Some(".githooks/pre-commit")
                && result.title() == "dispatcher syntax missing"
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_config_reports_missing_g3rs_binary_when_required() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    let bin_dir = root.join("bin");

    write(root.join(".githooks/pre-commit"), "g3rs validate --path .\n");
    write(bin_dir.join("gitleaks"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("cargo-deny"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("cargo-machete"), "#!/usr/bin/env bash\n");
    #[cfg(unix)]
    {
        make_executable(&bin_dir.join("gitleaks"));
        make_executable(&bin_dir.join("cargo-deny"));
        make_executable(&bin_dir.join("cargo-machete"));
    }

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let path_env = std::env::join_paths([bin_dir.as_path()]).expect("join path");
    let input =
        crate::run::ingest_for_config_checks_with_path(&crawl, Some(path_env.as_os_str()))
            .expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-CONFIG-02")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1, "{results:#?}");
    assert!(
        rule_results.iter().any(|result| {
            result.file() == Some(".githooks/pre-commit")
                && result.title() == "g3rs binary missing"
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_config_reports_tool_inventory_and_missing_cargo_dupes() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    let bin_dir = root.join("bin");

    write(
        root.join(".githooks/pre-commit"),
        "g3rs validate --path .\ncargo dupes check --exclude-tests\n",
    );
    write(bin_dir.join("gitleaks"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("cargo-deny"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("cargo-machete"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("g3rs"), "#!/usr/bin/env bash\n");
    #[cfg(unix)]
    {
        make_executable(&bin_dir.join("gitleaks"));
        make_executable(&bin_dir.join("cargo-deny"));
        make_executable(&bin_dir.join("cargo-machete"));
        make_executable(&bin_dir.join("g3rs"));
    }

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let path_env = std::env::join_paths([bin_dir.as_path()]).expect("join path");
    let input =
        crate::run::ingest_for_config_checks_with_path(&crawl, Some(path_env.as_os_str()))
            .expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    let rs06 = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-CONFIG-01")
        .collect::<Vec<_>>();
    let rs15 = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-CONFIG-03")
        .collect::<Vec<_>>();

    assert_eq!(rs06.len(), 3, "{results:#?}");
    assert_eq!(rs15.len(), 1, "{results:#?}");
    assert!(
        rs06.iter().any(|result| result.title() == "gitleaks installed" && result.inventory()),
        "{results:#?}"
    );
    assert!(
        rs06
            .iter()
            .any(|result| result.title() == "cargo-deny installed" && result.inventory()),
        "{results:#?}"
    );
    assert!(
        rs06
            .iter()
            .any(|result| result.title() == "cargo-machete installed" && result.inventory()),
        "{results:#?}"
    );
    assert!(
        rs15.iter().any(|result| {
            result.title() == "cargo-dupes missing"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_config_reports_present_g3rs_and_cargo_dupes_binaries() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    let bin_dir = root.join("bin");

    write(
        root.join(".githooks/pre-commit"),
        "g3rs validate --path .\ncargo dupes check --exclude-tests\n",
    );
    write(bin_dir.join("g3rs"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("cargo-dupes"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("gitleaks"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("cargo-deny"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("cargo-machete"), "#!/usr/bin/env bash\n");
    #[cfg(unix)]
    {
        make_executable(&bin_dir.join("g3rs"));
        make_executable(&bin_dir.join("cargo-dupes"));
        make_executable(&bin_dir.join("gitleaks"));
        make_executable(&bin_dir.join("cargo-deny"));
        make_executable(&bin_dir.join("cargo-machete"));
    }

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let path_env = std::env::join_paths([bin_dir.as_path()]).expect("join path");
    let input =
        crate::run::ingest_for_config_checks_with_path(&crawl, Some(path_env.as_os_str()))
            .expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    for (rule_id, title) in [
        ("RS-HOOKS-CONFIG-02", "g3rs binary available"),
        ("RS-HOOKS-CONFIG-03", "cargo-dupes installed"),
    ] {
        let matching = results
            .iter()
            .filter(|result| result.id() == rule_id)
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 1, "{rule_id}: {results:#?}");
        assert!(
            matching.iter().any(|result| {
                result.file() == Some(".githooks/pre-commit")
                    && result.title() == title
                    && result.inventory()
            }),
            "{rule_id}: {results:#?}"
        );
    }
}

#[test]
fn pipeline_config_honors_hooks_path_selected_hook_for_binary_requirements() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    let bin_dir = root.join("bin");
    git_config_hooks_path(root, "hooks");

    write(root.join(".githooks/pre-commit"), "gitleaks protect --staged --no-banner\n");
    write(
        root.join("hooks/pre-commit"),
        "g3rs validate --path .\ncargo dupes check --exclude-tests\ngitleaks protect --staged --no-banner\ncargo-deny check\ncargo-machete\n",
    );
    write(bin_dir.join("g3rs"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("cargo-dupes"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("gitleaks"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("cargo-deny"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("cargo-machete"), "#!/usr/bin/env bash\n");
    #[cfg(unix)]
    {
        make_executable(&bin_dir.join("g3rs"));
        make_executable(&bin_dir.join("cargo-dupes"));
        make_executable(&bin_dir.join("gitleaks"));
        make_executable(&bin_dir.join("cargo-deny"));
        make_executable(&bin_dir.join("cargo-machete"));
    }

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let path_env = std::env::join_paths([bin_dir.as_path()]).expect("join path");
    let input =
        crate::run::ingest_for_config_checks_with_path(&crawl, Some(path_env.as_os_str()))
            .expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    for (rule_id, title) in [
        ("RS-HOOKS-CONFIG-01", "gitleaks installed"),
        ("RS-HOOKS-CONFIG-01", "cargo-deny installed"),
        ("RS-HOOKS-CONFIG-01", "cargo-machete installed"),
        ("RS-HOOKS-CONFIG-02", "g3rs binary available"),
        ("RS-HOOKS-CONFIG-03", "cargo-dupes installed"),
    ] {
        let matching = results
            .iter()
            .filter(|result| result.id() == rule_id)
            .collect::<Vec<_>>();
        assert!(
            matching.iter().any(|result| {
                result.file() == Some("hooks/pre-commit")
                    && result.title() == title
                    && result.inventory()
            }),
            "{rule_id}: {results:#?}"
        );
    }
}

#[test]
fn pipeline_config_treats_path_qualified_tools_as_installed() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(
        root.join(".githooks/pre-commit"),
        "/opt/bin/gitleaks protect --staged --no-banner\n/opt/bin/cargo-deny check\n/opt/bin/cargo-machete\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        crate::run::ingest_for_config_checks_with_path(&crawl, None).expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    let rs06 = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-CONFIG-01")
        .collect::<Vec<_>>();

    assert_eq!(rs06.len(), 3, "{results:#?}");
    assert!(rs06.iter().all(|result| result.inventory()), "{results:#?}");
}

#[test]
fn pipeline_file_tree_reports_existing_pre_commit_hook() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "#!/usr/bin/env bash\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    let matching = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-FILETREE-01")
        .collect::<Vec<_>>();

    assert_eq!(matching.len(), 1, "{results:#?}");
    assert!(
        matching.iter().any(|result| {
            result.file() == Some(".githooks/pre-commit")
                && result.title() == "pre-commit hook exists"
                && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_file_tree_treats_non_compat_hooks_path_as_out_of_contract() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, "custom-hooks");

    write(root.join(".githooks/pre-commit"), "#!/usr/bin/env bash\ncargo fmt --check\n");
    write(root.join("hooks/pre-commit"), "#!/usr/bin/env bash\ncargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-01"
                && result.title() == "pre-commit hook missing"
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-02"
                && result.title() == "core.hooksPath has wrong value"
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_file_tree_reports_modular_inventory_for_hooks_compat_path() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, "hooks");

    write(root.join("hooks/pre-commit"), "#!/usr/bin/env bash\ncargo fmt --check\n");
    write(
        root.join(".githooks/pre-commit.d/10-rust.sh"),
        "#!/usr/bin/env bash\ncargo test --workspace\n",
    );
    #[cfg(unix)]
    {
        make_executable(&root.join(".githooks/pre-commit.d/10-rust.sh"));
    }

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-03"
                && result.title() == "pre-commit.d directory exists"
                && result.file() == Some(".githooks/pre-commit.d")
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-04"
                && result.title() == "modular hook scripts inventory"
                && result.message().contains(".githooks/pre-commit.d/10-rust.sh")
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-06"
                && result.file() == Some(".githooks/pre-commit.d/10-rust.sh")
                && result.title() == "modular hook script is executable"
                && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_file_tree_reports_missing_pre_commit_hook() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-FILETREE-01")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1, "{results:#?}");
    assert!(
        rule_results.iter().any(|result| {
            result.file() == Some(".githooks/pre-commit")
                && result.title() == "pre-commit hook missing"
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn nested_package_workspace_is_out_of_scope_for_repo_global_hooks() {
    let temp_dir = tempdir().expect("create temp dir");
    let repo_root = temp_dir.path();
    let nested = repo_root.join("packages/example");

    git_init(repo_root);
    write(
        nested.join("Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    write(
        repo_root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\ng3rs validate --path .\n",
    );
    git_config_hooks_path(repo_root, ".githooks");

    let crawl = g3rs_workspace_crawl::crawl(&nested).expect("crawl should succeed");

    let file_tree_input =
        crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingestion should succeed");
    let file_tree_results = g3rs_hooks_file_tree_checks::check(&file_tree_input);
    assert!(
        !file_tree_results
            .iter()
            .any(|result| matches!(result.id(), "RS-HOOKS-FILETREE-01" | "RS-HOOKS-FILETREE-02")),
        "{file_tree_results:#?}"
    );

    let config_input = crate::run::ingest_for_config_checks_with_path(&crawl, None)
        .expect("config ingestion should succeed");
    let config_results = g3rs_hooks_config_checks::check(&config_input);
    assert!(config_results.is_empty(), "{config_results:#?}");

    let source_inputs =
        crate::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    assert!(source_inputs.is_empty(), "{source_inputs:#?}");
}

#[test]
fn pipeline_file_tree_reports_trust_risk() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "#!/usr/bin/env bash\nexit 0\n");
    write(root.join(".git/hooks/pre-commit"), "#!/usr/bin/env bash\nexit 0\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-FILETREE-07")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1, "{results:#?}");
    assert!(
        rule_results.iter().any(|result| {
            result.title() == "competing hook system detected"
                && result.message().contains(".git/hooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_file_tree_keeps_hooks_path_compat_mode_free_of_git_hook_shadow_risk() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, "hooks");

    write(root.join("hooks/pre-commit"), "#!/usr/bin/env bash\ncargo fmt --check\n");
    write(root.join(".git/hooks/pre-commit"), "#!/usr/bin/env bash\nexit 0\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-07"
                && result.title() == "no competing hook systems detected"
                && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_file_tree_reports_layout_stats_permissions_and_overrides() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, ".githooks");

    write(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nrun-parts .githooks/pre-commit.d\n",
    );
    write(
        root.join(".githooks/pre-commit.d/10-rust.sh"),
        "#!/usr/bin/env bash\ncargo fmt --check\n",
    );
    write(
        root.join(".githooks/pre-commit.d/20-rust.sh"),
        "#!/usr/bin/env bash\ncargo test --workspace\n",
    );
    write(
        root.join(".guardrail3/overrides/pre-commit.d/90-local.sh"),
        "#!/usr/bin/env bash\necho override\n",
    );
    #[cfg(unix)]
    {
        make_executable(&root.join(".githooks/pre-commit.d/10-rust.sh"));
    }

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    for rule_id in [
        "RS-HOOKS-FILETREE-02",
        "RS-HOOKS-FILETREE-03",
        "RS-HOOKS-FILETREE-08",
        "RS-HOOKS-FILETREE-09",
        "RS-HOOKS-FILETREE-04",
        "RS-HOOKS-FILETREE-10",
        "RS-HOOKS-FILETREE-05",
        "RS-HOOKS-FILETREE-06",
    ] {
        assert!(
            results.iter().any(|result| result.id() == rule_id),
            "missing {rule_id}: {results:#?}"
        );
    }

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-02"
                && result.title() == "core.hooksPath configured"
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-03"
                && result.title() == "pre-commit.d directory exists"
                && result.file() == Some(".githooks/pre-commit.d")
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-08"
                && result.title() == "pre-commit hook is not executable"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-09"
                && result.title() == "pre-commit script stats"
                && result.file() == Some(".githooks/pre-commit")
                && result.message() == "2 lines, 53 bytes"
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-04"
                && result.message().contains(".githooks/pre-commit.d/10-rust.sh")
                && result.message().contains(".githooks/pre-commit.d/20-rust.sh")
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-10"
                && result.title() == "pre-commit file size"
                && result.file() == Some(".githooks/pre-commit")
                && result.message() == "53 bytes"
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-05"
                && result.message().contains("90-local.sh")
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-06"
                && result.file() == Some(".githooks/pre-commit.d/10-rust.sh")
                && result.title() == "modular hook script is executable"
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-FILETREE-06"
                && result.file() == Some(".githooks/pre-commit.d/20-rust.sh")
                && result.title() == "modular hook script is not executable"
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_runs_shared_source_checks_on_modular_scripts() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "run-parts .githooks/pre-commit.d\n");
    write(root.join(".githooks/pre-commit.d/10-rust.sh"), "echo cargo fmt --check\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let shebang_results = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-SOURCE-17")
        .collect::<Vec<_>>();

    assert!(
        shebang_results.iter().any(|result| {
            result.file() == Some(".githooks/pre-commit.d/10-rust.sh")
                && result.title() == "hook shebang missing"
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_source_reports_shell_safety_inventory_for_valid_hook() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\ncargo fmt --check\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        ("RS-HOOKS-SOURCE-16", "shell error handling present"),
        ("RS-HOOKS-SOURCE-17", "valid hook shebang present"),
    ] {
        let matching = results
            .iter()
            .filter(|result| result.id() == rule_id)
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 1, "{rule_id}: {results:#?}");
        assert!(
            matching.iter().any(|result| {
                result.file() == Some(".githooks/pre-commit")
                    && result.title() == title
                    && result.inventory()
            }),
            "{rule_id}: {results:#?}"
        );
    }
}

#[test]
fn pipeline_source_reports_inventory_for_normalized_wrapped_commands() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\nif true; then /opt/bin/g3rs validate --path .; fi\ncargo +nightly clippy -- -D warnings\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        ("RS-HOOKS-SOURCE-09", "Rust guardrail validate step present"),
        ("RS-HOOKS-SOURCE-10", "cargo clippy denies warnings"),
    ] {
        let matching = results
            .iter()
            .filter(|result| result.id() == rule_id)
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 1, "{rule_id}: {results:#?}");
        assert!(
            matching.iter().any(|result| {
                result.file() == Some(".githooks/pre-commit")
                    && result.title() == title
                    && result.inventory()
            }),
            "{rule_id}: {results:#?}"
        );
    }
}

#[test]
fn pipeline_source_reports_missing_rust_and_shell_steps() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(root.join(".githooks/pre-commit"), "echo nothing useful\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-04"
                && result.title() == "cargo clippy step missing"
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-05"
                && result.title() == "cargo deny check step missing"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-06"
                && result.title() == "cargo test step missing"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-07"
                && result.title() == "cargo machete step missing"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-08"
                && result.title() == "Rust duplication tool missing"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-10"
                && result.title() == "cargo clippy deny-warnings step missing"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-12"
                && result.title() == "gitleaks step missing"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-13"
                && result.title() == "cargo dupes step missing"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-14"
                && result.title() == "cargo dupes step does not exclude tests"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-15"
                && result.title() == "Rust config-change trigger coverage incomplete"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-16"
                && result.title() == "shell error handling missing"
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-18"
                && result.title() == "no unconditional exit 0 bypass"
                && result.file() == Some(".githooks/pre-commit")
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-19"
                && result.title() == "no hook bypass instructions"
                && result.file() == Some(".githooks/pre-commit")
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-20"
                && result.title() == "merge-conflict check step missing"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-21"
                && result.title() == "file-size check step missing"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-23"
                && result.title() == "concrete lockfile integrity command missing"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_source_reports_inert_text_false_pass_risk() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(
        root.join(".githooks/pre-commit"),
        "STEP='g3rs validate --path .'\nprintf '%s\n' 'cargo fmt --check'\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-22"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_source_reports_missing_workspace_scope_for_workspace_project() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(root.join(".githooks/pre-commit"), "cargo test\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-11"
                && result.title() == "cargo test missing --workspace"
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_source_reports_invalid_dispatcher_syntax() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "echo run-parts .githooks/pre-commit.d\n");
    fs::create_dir_all(root.join(".githooks/pre-commit.d")).expect("create modular dir");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-02"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_keeps_inert_g3rs_text_quiet_when_wrapped_command_executes() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(
        root.join(".githooks/pre-commit"),
        "# g3rs validate --path .\nenv -i g3rs validate --path .\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let inert_text_results = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-SOURCE-22")
        .collect::<Vec<_>>();

    assert!(inert_text_results.is_empty(), "{results:#?}");
}

#[test]
fn pipeline_reports_fail_open_wrapper_on_called_function() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(
        root.join(".githooks/pre-commit"),
        "run_tests() {\n    cargo test --workspace\n}\nrun_tests || true\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let fail_open_results = results
        .iter()
        .filter(|result| result.id() == "RS-HOOKS-SOURCE-24")
        .collect::<Vec<_>>();

    assert_eq!(fail_open_results.len(), 1, "{results:#?}");
    assert!(
        fail_open_results.iter().any(|result| {
            result.file() == Some(".githooks/pre-commit")
                && result.title() == "critical hook command is fail-open"
                && result.line() == Some(4)
                && !result.inventory()
        }),
        "{results:#?}"
    );
}
