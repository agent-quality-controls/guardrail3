use std::fs;

use g3rs_hooks_ingestion_assertions::run::{
    self as assertions, assert_message_contains as assert_result_message_contains,
    assert_message_equals as assert_result_message_equals, assert_present as assert_result_present,
    assert_present_on_line as assert_result_present_on_line,
};
use tempfile::tempdir;

use super::helpers::{git_config_hooks_path, git_init, make_executable, repo_root, write_fixture};

#[test]
fn pipeline_reports_fmt_step_when_real_command_exists() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join(".githooks/pre-commit"), "cargo fmt --check\n");

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/fmt-step-present"),
        1,
        "{results:#?}"
    );
    assert_result_present(
        &results,
        "g3rs-hooks/fmt-step-present",
        "cargo fmt --check step present",
        Some(".githooks/pre-commit"),
        true,
    );
}

#[test]
fn pipeline_keeps_hook_rs_10_quiet_for_single_crate_repo() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_fixture(root.join(".githooks/pre-commit"), "cargo test\n");

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/test-uses-workspace"),
        1,
        "{results:#?}"
    );
    assert_result_present(
        &results,
        "g3rs-hooks/test-uses-workspace",
        "cargo test workspace scope not required",
        Some(".githooks/pre-commit"),
        true,
    );
}

#[test]
fn pipeline_keeps_echoed_fmt_text_as_missing_step() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "echo \"cargo fmt --check\"\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/fmt-step-present"),
        1,
        "{results:#?}"
    );
    assert_result_present(
        &results,
        "g3rs-hooks/fmt-step-present",
        "cargo fmt --check step missing",
        Some(".githooks/pre-commit"),
        false,
    );
}

#[test]
fn pipeline_works_through_hooks_pre_commit_fallback() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join("hooks/pre-commit"), "cargo fmt --check\n");

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/fmt-step-present"),
        1,
        "{results:#?}"
    );
    assert_result_present(
        &results,
        "g3rs-hooks/fmt-step-present",
        "cargo fmt --check step present",
        Some("hooks/pre-commit"),
        true,
    );
}

#[test]
fn pipeline_fallback_hook_stays_quiet_about_inactive_modular_layout() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join("hooks/pre-commit"), "cargo fmt --check\n");
    write_fixture(
        root.join(".githooks/pre-commit.d/10-rust.sh"),
        "echo cargo test --workspace\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_result_present(
        &results,
        "g3rs-hooks/fmt-step-present",
        "cargo fmt --check step present",
        Some("hooks/pre-commit"),
        true,
    );
    assertions::assert_no_results_for_file(&results, ".githooks/pre-commit.d/10-rust.sh");
}

#[test]
fn pipeline_reports_rs_config_trigger_for_guardrail3_rs_toml() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write_fixture(
        root.join(".githooks/pre-commit"),
        "if echo \"$STAGED_FILES\" | grep -qE '(guardrail3-rs\\.toml|clippy\\.toml|\\.clippy\\.toml|deny\\.toml|\\.deny\\.toml|rustfmt\\.toml|\\.rustfmt\\.toml|rust-toolchain\\.toml)$'; then\n    g3rs validate --path .\nfi\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/config-changes-trigger-validation"),
        1,
        "{results:#?}"
    );
    assert_result_present(
        &results,
        "g3rs-hooks/config-changes-trigger-validation",
        "`.githooks/pre-commit` triggers Rust validation on guardrail config changes",
        Some(".githooks/pre-commit"),
        true,
    );
}

#[test]
fn pipeline_reports_top_level_g3rs_validate_step_inventory() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "g3rs validate --path .\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/guardrail-validate-staged-present"),
        1,
        "{results:#?}"
    );
    assert_result_present(
        &results,
        "g3rs-hooks/guardrail-validate-staged-present",
        "`.githooks/pre-commit` runs `g3rs validate --path ...`",
        Some(".githooks/pre-commit"),
        true,
    );
}

#[test]
fn pipeline_stays_clean_for_valid_githooks_setup() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    fs::create_dir_all(root.join(".githooks/pre-commit.d")).expect("create modular dir");
    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\nREPO_ROOT=$(git rev-parse --show-toplevel)\nexport CARGO_TARGET_DIR=\"$REPO_ROOT/.cargo-target\"\nrun-parts .githooks/pre-commit.d\ncargo metadata --locked\ncargo fmt --check\ncargo clippy -- -D warnings\ncargo deny check\ncargo test --workspace\ncargo machete\ngitleaks protect --staged --no-banner\ncargo dupes check --exclude-tests\ng3rs validate --path .\npnpm install --frozen-lockfile\nrg '^(<<<<<<<|=======|>>>>>>>)' .\nstat -c%s Cargo.toml >/dev/null\nif echo \"$STAGED_FILES\" | grep -qE '(guardrail3-rs\\.toml|clippy\\.toml|\\.clippy\\.toml|deny\\.toml|\\.deny\\.toml|rustfmt\\.toml|\\.rustfmt\\.toml|rust-toolchain\\.toml)$'; then\n    g3rs validate --path .\nfi\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert!(
        assertions::non_inventory(&results).is_empty(),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_inventory_for_valid_rust_hook_steps() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\ncargo fmt --check\ncargo clippy -- -D warnings\ncargo deny check\ncargo test --workspace\ncargo machete\ncargo dupes check --exclude-tests\ngitleaks protect --staged --no-banner\ng3rs validate --path .\nrg '^(<<<<<<<|=======|>>>>>>>)' .\nstat -c%s Cargo.toml >/dev/null\npnpm install --frozen-lockfile\nif echo \"$STAGED_FILES\" | grep -qE '(guardrail3-rs\\.toml|clippy\\.toml|\\.clippy\\.toml|deny\\.toml|\\.deny\\.toml|rustfmt\\.toml|\\.rustfmt\\.toml|rust-toolchain\\.toml)$'; then\n    g3rs validate --path .\nfi\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        (
            "g3rs-hooks/clippy-step-present",
            "cargo clippy step present",
        ),
        (
            "g3rs-hooks/cargo-deny-step-present",
            "cargo deny check step present",
        ),
        ("g3rs-hooks/test-step-present", "cargo test step present"),
        (
            "g3rs-hooks/cargo-machete-step-present",
            "cargo machete step present",
        ),
        (
            "g3rs-hooks/duplication-tool-is-cargo-dupes",
            "`.githooks/pre-commit` uses `cargo dupes` for Rust dependency duplication",
        ),
        (
            "g3rs-hooks/clippy-denies-warnings",
            "`.githooks/pre-commit` runs clippy in deny-warnings mode",
        ),
        ("g3rs-hooks/gitleaks-step-present", "gitleaks step present"),
        (
            "g3rs-hooks/cargo-dupes-step-present",
            "`.githooks/pre-commit` runs `cargo dupes`",
        ),
        (
            "g3rs-hooks/cargo-dupes-excludes",
            "`.githooks/pre-commit` runs `cargo dupes --exclude-tests`",
        ),
    ] {
        assert_eq!(
            assertions::count(&results, rule_id),
            1,
            "{rule_id}: {results:#?}"
        );
        assert_result_present(&results, rule_id, title, Some(".githooks/pre-commit"), true);
    }
}

#[test]
fn pipeline_reports_inventory_for_command_substitution_and_binary_aliases() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\nOUT=\"$(g3rs validate --path .)\"\ncargo-deny check\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        (
            "g3rs-hooks/cargo-deny-step-present",
            "cargo deny check step present",
        ),
        (
            "g3rs-hooks/guardrail-validate-staged-present",
            "`.githooks/pre-commit` runs `g3rs validate --path ...`",
        ),
    ] {
        assert_eq!(
            assertions::count(&results, rule_id),
            1,
            "{rule_id}: {results:#?}"
        );
        assert_result_present(&results, rule_id, title, Some(".githooks/pre-commit"), true);
    }
}

#[test]
fn pipeline_reports_inventory_for_called_function_commands() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nrun_guardrails() {\n    g3rs validate --path .\n}\nrun_guardrails\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/guardrail-validate-staged-present"),
        1,
        "{results:#?}"
    );
    assert_result_present(
        &results,
        "g3rs-hooks/guardrail-validate-staged-present",
        "`.githooks/pre-commit` runs `g3rs validate --path ...`",
        Some(".githooks/pre-commit"),
        true,
    );
}

#[test]
fn pipeline_reports_inventory_for_executed_subshell_commands() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\n(g3rs validate --path .)\n(cargo-deny check)\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        (
            "g3rs-hooks/cargo-deny-step-present",
            "cargo deny check step present",
        ),
        (
            "g3rs-hooks/guardrail-validate-staged-present",
            "`.githooks/pre-commit` runs `g3rs validate --path ...`",
        ),
    ] {
        assert_eq!(
            assertions::count(&results, rule_id),
            1,
            "{rule_id}: {results:#?}"
        );
        assert_result_present(&results, rule_id, title, Some(".githooks/pre-commit"), true);
    }
}

#[test]
fn pipeline_reports_inventory_for_valid_shell_hook_steps() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\nrg '^(<<<<<<<|=======|>>>>>>>)' .\nstat -c%s Cargo.toml >/dev/null\ncargo metadata --locked\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        (
            "g3rs-hooks/merge-conflict-step-present",
            "`.githooks/pre-commit` scans for merge-conflict markers",
        ),
        (
            "g3rs-hooks/file-size-step-present",
            "file-size check step present",
        ),
        (
            "g3rs-hooks/concrete-lockfile-command",
            "`.githooks/pre-commit` runs a concrete lockfile integrity command",
        ),
    ] {
        assert_eq!(
            assertions::count(&results, rule_id),
            1,
            "{rule_id}: {results:#?}"
        );
        assert_result_present(&results, rule_id, title, Some(".githooks/pre-commit"), true);
    }
}

#[test]
fn pipeline_reports_dispatcher_findings_for_real_pre_commit_script() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    fs::create_dir_all(root.join(".githooks/pre-commit.d")).expect("create modular dir");
    write_fixture(
        root.join(".githooks/pre-commit"),
        "run-parts .githooks/pre-commit.d\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/dispatcher-pattern"),
        1,
        "{results:#?}"
    );
    assert_eq!(
        assertions::count(&results, "g3rs-hooks/real-dispatcher-syntax-only"),
        1,
        "{results:#?}"
    );
    assert_result_present(
        &results,
        "g3rs-hooks/dispatcher-pattern",
        "dispatcher pattern present",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "g3rs-hooks/real-dispatcher-syntax-only",
        "dispatcher uses real executable syntax",
        Some(".githooks/pre-commit"),
        true,
    );
}

#[test]
fn pipeline_source_reports_missing_dispatcher_pattern_when_modular_dir_is_only_echoed() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "echo run-parts .githooks/pre-commit.d\n",
    );
    fs::create_dir_all(root.join(".githooks/pre-commit.d")).expect("create modular dir");

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_result_present(
        &results,
        "g3rs-hooks/dispatcher-pattern",
        "dispatcher pattern missing",
        Some(".githooks/pre-commit"),
        false,
    );
    assert_result_present(
        &results,
        "g3rs-hooks/real-dispatcher-syntax-only",
        "dispatcher syntax missing",
        Some(".githooks/pre-commit"),
        false,
    );
}

#[test]
fn pipeline_config_reports_missing_g3rs_binary_when_required() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    let bin_dir = root.join("bin");

    write_fixture(
        root.join(".githooks/pre-commit"),
        "g3rs validate --path .\n",
    );
    write_fixture(bin_dir.join("gitleaks"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("cargo-deny"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("cargo-machete"), "#!/usr/bin/env bash\n");
    #[cfg(unix)]
    {
        make_executable(&bin_dir.join("gitleaks"));
        make_executable(&bin_dir.join("cargo-deny"));
        make_executable(&bin_dir.join("cargo-machete"));
    }

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let path_env = std::env::join_paths([bin_dir.as_path()])
        .expect("build PATH override for hook config check");
    let input =
        super::super::ingest_for_config_checks_with_path(&crawl, Some(path_env.as_os_str()))
            .expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/guardrail-binary-available"),
        1,
        "{results:#?}"
    );
    assert_result_present(
        &results,
        "g3rs-hooks/guardrail-binary-available",
        "g3rs binary missing",
        Some(".githooks/pre-commit"),
        false,
    );
}

#[test]
fn pipeline_config_reports_tool_inventory_and_missing_cargo_dupes() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    let bin_dir = root.join("bin");

    write_fixture(
        root.join(".githooks/pre-commit"),
        "g3rs validate --path .\ncargo dupes check --exclude-tests\n",
    );
    write_fixture(bin_dir.join("gitleaks"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("cargo-deny"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("cargo-machete"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("g3rs"), "#!/usr/bin/env bash\n");
    #[cfg(unix)]
    {
        make_executable(&bin_dir.join("gitleaks"));
        make_executable(&bin_dir.join("cargo-deny"));
        make_executable(&bin_dir.join("cargo-machete"));
        make_executable(&bin_dir.join("g3rs"));
    }

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let path_env = std::env::join_paths([bin_dir.as_path()])
        .expect("build PATH override for hook config check");
    let input =
        super::super::ingest_for_config_checks_with_path(&crawl, Some(path_env.as_os_str()))
            .expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/required-tools-installed"),
        3,
        "{results:#?}"
    );
    assert_eq!(
        assertions::count(&results, "g3rs-hooks/cargo-dupes-installed"),
        1,
        "{results:#?}"
    );
    assert_result_present(
        &results,
        "g3rs-hooks/required-tools-installed",
        "gitleaks installed",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "g3rs-hooks/required-tools-installed",
        "cargo-deny installed",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "g3rs-hooks/required-tools-installed",
        "cargo-machete installed",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "g3rs-hooks/cargo-dupes-installed",
        "cargo-dupes missing",
        Some(".githooks/pre-commit"),
        false,
    );
}

#[test]
fn pipeline_config_reports_present_g3rs_and_cargo_dupes_binaries() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    let bin_dir = root.join("bin");

    write_fixture(
        root.join(".githooks/pre-commit"),
        "g3rs validate --path .\ncargo dupes check --exclude-tests\n",
    );
    write_fixture(bin_dir.join("g3rs"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("cargo-dupes"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("gitleaks"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("cargo-deny"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("cargo-machete"), "#!/usr/bin/env bash\n");
    #[cfg(unix)]
    {
        make_executable(&bin_dir.join("g3rs"));
        make_executable(&bin_dir.join("cargo-dupes"));
        make_executable(&bin_dir.join("gitleaks"));
        make_executable(&bin_dir.join("cargo-deny"));
        make_executable(&bin_dir.join("cargo-machete"));
    }

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let path_env = std::env::join_paths([bin_dir.as_path()])
        .expect("build PATH override for hook config check");
    let input =
        super::super::ingest_for_config_checks_with_path(&crawl, Some(path_env.as_os_str()))
            .expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    for (rule_id, title) in [
        (
            "g3rs-hooks/guardrail-binary-available",
            "g3rs binary available",
        ),
        ("g3rs-hooks/cargo-dupes-installed", "cargo-dupes installed"),
    ] {
        assert_eq!(
            assertions::count(&results, rule_id),
            1,
            "{rule_id}: {results:#?}"
        );
        assert_result_present(&results, rule_id, title, Some(".githooks/pre-commit"), true);
    }
}

#[test]
fn pipeline_config_honors_hooks_path_selected_hook_for_binary_requirements() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    let bin_dir = root.join("bin");
    git_config_hooks_path(root, "hooks");

    write_fixture(
        root.join(".githooks/pre-commit"),
        "gitleaks protect --staged --no-banner\n",
    );
    write_fixture(
        root.join("hooks/pre-commit"),
        "g3rs validate --path .\ncargo dupes check --exclude-tests\ngitleaks protect --staged --no-banner\ncargo-deny check\ncargo-machete\n",
    );
    write_fixture(bin_dir.join("g3rs"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("cargo-dupes"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("gitleaks"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("cargo-deny"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("cargo-machete"), "#!/usr/bin/env bash\n");
    #[cfg(unix)]
    {
        make_executable(&bin_dir.join("g3rs"));
        make_executable(&bin_dir.join("cargo-dupes"));
        make_executable(&bin_dir.join("gitleaks"));
        make_executable(&bin_dir.join("cargo-deny"));
        make_executable(&bin_dir.join("cargo-machete"));
    }

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let path_env = std::env::join_paths([bin_dir.as_path()])
        .expect("build PATH override for hook config check");
    let input =
        super::super::ingest_for_config_checks_with_path(&crawl, Some(path_env.as_os_str()))
            .expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    for (rule_id, title) in [
        ("g3rs-hooks/required-tools-installed", "gitleaks installed"),
        (
            "g3rs-hooks/required-tools-installed",
            "cargo-deny installed",
        ),
        (
            "g3rs-hooks/required-tools-installed",
            "cargo-machete installed",
        ),
        (
            "g3rs-hooks/guardrail-binary-available",
            "g3rs binary available",
        ),
        ("g3rs-hooks/cargo-dupes-installed", "cargo-dupes installed"),
    ] {
        assert_result_present(&results, rule_id, title, Some("hooks/pre-commit"), true);
    }
}

#[test]
fn pipeline_config_treats_path_qualified_tools_as_installed() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "/opt/bin/gitleaks protect --staged --no-banner\n/opt/bin/cargo-deny check\n/opt/bin/cargo-machete\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_config_checks_with_path(&crawl, None)
        .expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/required-tools-installed"),
        3,
        "{results:#?}"
    );
    assert_result_present(
        &results,
        "g3rs-hooks/required-tools-installed",
        "gitleaks installed",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "g3rs-hooks/required-tools-installed",
        "cargo-deny installed",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "g3rs-hooks/required-tools-installed",
        "cargo-machete installed",
        Some(".githooks/pre-commit"),
        true,
    );
}

#[test]
fn pipeline_file_tree_reports_existing_pre_commit_hook() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join(".githooks/pre-commit"), "#!/usr/bin/env bash\n");

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/pre-commit-exists"),
        1,
        "{results:#?}"
    );
    assert_result_present(
        &results,
        "g3rs-hooks/pre-commit-exists",
        "pre-commit hook exists",
        Some(".githooks/pre-commit"),
        true,
    );
}

#[test]
fn pipeline_file_tree_treats_non_compat_hooks_path_as_out_of_contract() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, "custom-hooks");

    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\ncargo fmt --check\n",
    );
    write_fixture(
        root.join("hooks/pre-commit"),
        "#!/usr/bin/env bash\ncargo test --workspace\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert_result_present(
        &results,
        "g3rs-hooks/pre-commit-exists",
        "pre-commit hook missing",
        Some(".githooks/pre-commit"),
        false,
    );
    assert_result_present(
        &results,
        "g3rs-hooks/hooks-path-configured",
        "core.hooksPath has wrong value",
        None,
        false,
    );
}

#[test]
fn pipeline_file_tree_reports_modular_inventory_for_hooks_compat_path() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, "hooks");

    write_fixture(
        root.join("hooks/pre-commit"),
        "#!/usr/bin/env bash\ncargo fmt --check\n",
    );
    write_fixture(
        root.join(".githooks/pre-commit.d/10-rust.sh"),
        "#!/usr/bin/env bash\ncargo test --workspace\n",
    );
    #[cfg(unix)]
    {
        make_executable(&root.join(".githooks/pre-commit.d/10-rust.sh"));
    }

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert_result_present(
        &results,
        "g3rs-hooks/modular-directory-inventory",
        "pre-commit.d directory exists",
        Some(".githooks/pre-commit.d"),
        true,
    );
    assert_result_message_contains(
        &results,
        "g3rs-hooks/modular-scripts-inventory",
        "modular hook scripts inventory",
        Some(".githooks/pre-commit.d"),
        true,
        ".githooks/pre-commit.d/10-rust.sh",
    );
    assert_result_present(
        &results,
        "g3rs-hooks/modular-scripts-executable",
        "modular hook script is executable",
        Some(".githooks/pre-commit.d/10-rust.sh"),
        true,
    );
}

#[test]
fn pipeline_file_tree_reports_missing_pre_commit_hook() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/pre-commit-exists"),
        1,
        "{results:#?}"
    );
    assert_result_present(
        &results,
        "g3rs-hooks/pre-commit-exists",
        "pre-commit hook missing",
        Some(".githooks/pre-commit"),
        false,
    );
}

#[test]
fn nested_package_workspace_is_out_of_scope_for_repo_global_hooks() {
    let temp_dir = tempdir().expect("create temp dir");
    let repo_root = temp_dir.path();
    let nested = repo_root.join("packages/example");

    git_init(repo_root);
    write_fixture(
        nested.join("Cargo.toml"),
        "[workspace]\nmembers = []\nresolver = \"2\"\n",
    );
    write_fixture(
        repo_root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\ng3rs validate --path .\n",
    );
    git_config_hooks_path(repo_root, ".githooks");

    let crawl = g3_workspace_crawl::crawl(&nested).expect("crawl should succeed");

    let file_tree_input = super::super::ingest_for_file_tree_checks(&crawl)
        .expect("file-tree ingestion should succeed");
    let file_tree_results = g3rs_hooks_file_tree_checks::check(&file_tree_input);
    assert_eq!(
        assertions::count(&file_tree_results, "g3rs-hooks/pre-commit-exists"),
        0,
        "{file_tree_results:#?}"
    );
    assert_eq!(
        assertions::count(&file_tree_results, "g3rs-hooks/hooks-path-configured"),
        0,
        "{file_tree_results:#?}"
    );

    let config_input = super::super::ingest_for_config_checks_with_path(&crawl, None)
        .expect("config ingestion should succeed");
    let config_results = g3rs_hooks_config_checks::check(&config_input);
    assert!(config_results.is_empty(), "{config_results:#?}");

    let source_inputs =
        super::super::ingest_for_source_checks(&crawl).expect("source ingestion should succeed");
    assert!(source_inputs.is_empty(), "{source_inputs:#?}");
}

#[test]
fn pipeline_file_tree_reports_trust_risk() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nexit 0\n",
    );
    write_fixture(
        root.join(".git/hooks/pre-commit"),
        "#!/usr/bin/env bash\nexit 0\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/execution-trust"),
        1,
        "{results:#?}"
    );
    assert_result_message_contains(
        &results,
        "g3rs-hooks/execution-trust",
        "competing hook system detected",
        None,
        false,
        ".git/hooks/pre-commit",
    );
}

#[test]
fn pipeline_file_tree_keeps_hooks_path_compat_mode_free_of_git_hook_shadow_risk() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, "hooks");

    write_fixture(
        root.join("hooks/pre-commit"),
        "#!/usr/bin/env bash\ncargo fmt --check\n",
    );
    write_fixture(
        root.join(".git/hooks/pre-commit"),
        "#!/usr/bin/env bash\nexit 0\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert_result_present(
        &results,
        "g3rs-hooks/execution-trust",
        "no competing hook systems detected",
        None,
        true,
    );
}

#[test]
fn pipeline_file_tree_reports_layout_stats_permissions_and_overrides() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, ".githooks");

    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nrun-parts .githooks/pre-commit.d\n",
    );
    write_fixture(
        root.join(".githooks/pre-commit.d/10-rust.sh"),
        "#!/usr/bin/env bash\ncargo fmt --check\n",
    );
    write_fixture(
        root.join(".githooks/pre-commit.d/20-rust.sh"),
        "#!/usr/bin/env bash\ncargo test --workspace\n",
    );
    write_fixture(
        root.join(".guardrail3/overrides/pre-commit.d/90-local.sh"),
        "#!/usr/bin/env bash\necho override\n",
    );
    #[cfg(unix)]
    {
        make_executable(&root.join(".githooks/pre-commit.d/10-rust.sh"));
    }

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    for rule_id in [
        "g3rs-hooks/hooks-path-configured",
        "g3rs-hooks/modular-directory-inventory",
        "g3rs-hooks/pre-commit-executable",
        "g3rs-hooks/script-stats-inventory",
        "g3rs-hooks/modular-scripts-inventory",
        "g3rs-hooks/pre-commit-file-size-inventory",
        "g3rs-hooks/local-override-inventory",
        "g3rs-hooks/modular-scripts-executable",
    ] {
        assert!(
            assertions::count(&results, rule_id) > 0,
            "missing {rule_id}: {results:#?}"
        );
    }

    assert_result_present(
        &results,
        "g3rs-hooks/hooks-path-configured",
        "core.hooksPath configured",
        None,
        true,
    );
    assert_result_present(
        &results,
        "g3rs-hooks/modular-directory-inventory",
        "pre-commit.d directory exists",
        Some(".githooks/pre-commit.d"),
        true,
    );
    assert_result_present(
        &results,
        "g3rs-hooks/pre-commit-executable",
        "pre-commit hook is not executable",
        Some(".githooks/pre-commit"),
        false,
    );
    assert_result_message_equals(
        &results,
        "g3rs-hooks/script-stats-inventory",
        "pre-commit script stats",
        Some(".githooks/pre-commit"),
        true,
        "2 lines, 53 bytes",
    );
    assert_result_message_contains(
        &results,
        "g3rs-hooks/modular-scripts-inventory",
        "modular hook scripts inventory",
        Some(".githooks/pre-commit.d"),
        true,
        ".githooks/pre-commit.d/10-rust.sh",
    );
    assert_result_message_contains(
        &results,
        "g3rs-hooks/modular-scripts-inventory",
        "modular hook scripts inventory",
        Some(".githooks/pre-commit.d"),
        true,
        ".githooks/pre-commit.d/20-rust.sh",
    );
    assert_result_message_equals(
        &results,
        "g3rs-hooks/pre-commit-file-size-inventory",
        "pre-commit file size",
        Some(".githooks/pre-commit"),
        true,
        "53 bytes",
    );
    assert_result_message_contains(
        &results,
        "g3rs-hooks/local-override-inventory",
        "local hook overrides inventory",
        Some(".guardrail3/overrides/pre-commit.d"),
        true,
        "90-local.sh",
    );
    assert_result_present(
        &results,
        "g3rs-hooks/modular-scripts-executable",
        "modular hook script is executable",
        Some(".githooks/pre-commit.d/10-rust.sh"),
        true,
    );
    assert_result_present(
        &results,
        "g3rs-hooks/modular-scripts-executable",
        "modular hook script is not executable",
        Some(".githooks/pre-commit.d/20-rust.sh"),
        false,
    );
}

#[test]
fn pipeline_runs_shared_source_checks_on_modular_scripts() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "run-parts .githooks/pre-commit.d\n",
    );
    write_fixture(
        root.join(".githooks/pre-commit.d/10-rust.sh"),
        "echo cargo fmt --check\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_result_present(
        &results,
        "g3rs-hooks/valid-shebang",
        "hook shebang missing",
        Some(".githooks/pre-commit.d/10-rust.sh"),
        false,
    );
}

#[test]
fn pipeline_source_reports_shell_safety_inventory_for_valid_hook() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\ncargo fmt --check\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        (
            "g3rs-hooks/shell-error-handling",
            "`.githooks/pre-commit` enables fail-closed shell options",
        ),
        ("g3rs-hooks/valid-shebang", "valid hook shebang present"),
    ] {
        assert_eq!(
            assertions::count(&results, rule_id),
            1,
            "{rule_id}: {results:#?}"
        );
        assert_result_present(&results, rule_id, title, Some(".githooks/pre-commit"), true);
    }
}

#[test]
fn pipeline_source_reports_inventory_for_normalized_wrapped_commands() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\nif true; then /opt/bin/g3rs validate --path .; fi\ncargo +nightly clippy -- -D warnings\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        (
            "g3rs-hooks/guardrail-validate-staged-present",
            "`.githooks/pre-commit` runs `g3rs validate --path ...`",
        ),
        (
            "g3rs-hooks/clippy-denies-warnings",
            "`.githooks/pre-commit` runs clippy in deny-warnings mode",
        ),
    ] {
        assert_eq!(
            assertions::count(&results, rule_id),
            1,
            "{rule_id}: {results:#?}"
        );
        assert_result_present(&results, rule_id, title, Some(".githooks/pre-commit"), true);
    }
}

#[test]
fn pipeline_source_reports_missing_rust_and_shell_steps() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write_fixture(root.join(".githooks/pre-commit"), "echo nothing useful\n");

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        (
            "g3rs-hooks/clippy-step-present",
            "cargo clippy step missing",
        ),
        (
            "g3rs-hooks/cargo-deny-step-present",
            "cargo deny check step missing",
        ),
        ("g3rs-hooks/test-step-present", "cargo test step missing"),
        (
            "g3rs-hooks/cargo-machete-step-present",
            "cargo machete step missing",
        ),
        (
            "g3rs-hooks/duplication-tool-is-cargo-dupes",
            "missing `cargo dupes --exclude-tests` command in `.githooks/pre-commit`",
        ),
        (
            "g3rs-hooks/clippy-denies-warnings",
            "missing deny-warnings `cargo clippy` command in `.githooks/pre-commit`",
        ),
        ("g3rs-hooks/gitleaks-step-present", "gitleaks step missing"),
        (
            "g3rs-hooks/cargo-dupes-step-present",
            "missing executable `cargo dupes` command in `.githooks/pre-commit`",
        ),
        (
            "g3rs-hooks/cargo-dupes-excludes",
            "missing `--exclude-tests` on `cargo dupes` in `.githooks/pre-commit`",
        ),
        (
            "g3rs-hooks/config-changes-trigger-validation",
            "incomplete Rust guardrail config trigger coverage in `.githooks/pre-commit`",
        ),
        (
            "g3rs-hooks/shell-error-handling",
            "missing fail-closed shell options in `.githooks/pre-commit`",
        ),
        (
            "g3rs-hooks/merge-conflict-step-present",
            "missing merge-conflict marker scan in `.githooks/pre-commit`",
        ),
        (
            "g3rs-hooks/file-size-step-present",
            "file-size check step missing",
        ),
        (
            "g3rs-hooks/concrete-lockfile-command",
            "missing concrete lockfile integrity command in `.githooks/pre-commit`",
        ),
    ] {
        assert_result_present(
            &results,
            rule_id,
            title,
            Some(".githooks/pre-commit"),
            false,
        );
    }
    assert_result_present(
        &results,
        "g3rs-hooks/no-unconditional-exit-zero",
        "no unconditional `exit 0` bypass in `.githooks/pre-commit`",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "g3rs-hooks/no-bypass-instructions",
        "no hook bypass instructions",
        Some(".githooks/pre-commit"),
        true,
    );
}

#[test]
fn pipeline_source_reports_inert_text_false_pass_risk() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "STEP='g3rs validate --path .'\nprintf '%s\n' 'cargo fmt --check'\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_result_present(
        &results,
        "g3rs-hooks/executable-command-context-only",
        "required hook step appears only in inert text",
        Some(".githooks/pre-commit"),
        false,
    );
}

#[test]
fn pipeline_source_reports_missing_workspace_scope_for_workspace_project() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write_fixture(root.join(".githooks/pre-commit"), "cargo test\n");

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_result_present(
        &results,
        "g3rs-hooks/test-uses-workspace",
        "cargo test missing --workspace",
        Some(".githooks/pre-commit"),
        false,
    );
}

#[test]
fn pipeline_source_reports_invalid_dispatcher_syntax() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "echo run-parts .githooks/pre-commit.d\n",
    );
    fs::create_dir_all(root.join(".githooks/pre-commit.d")).expect("create modular dir");

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_result_present(
        &results,
        "g3rs-hooks/real-dispatcher-syntax-only",
        "dispatcher syntax missing",
        Some(".githooks/pre-commit"),
        false,
    );
}

#[test]
fn pipeline_keeps_inert_g3rs_text_quiet_when_wrapped_command_executes() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "# g3rs validate --path .\nenv -i g3rs validate --path .\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/executable-command-context-only"),
        0,
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_fail_open_wrapper_on_called_function() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "run_tests() {\n    cargo test --workspace\n}\nrun_tests || true\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(
        assertions::count(&results, "g3rs-hooks/no-fail-open-wrappers"),
        1,
        "{results:#?}"
    );
    assert_result_present_on_line(
        &results,
        "g3rs-hooks/no-fail-open-wrappers",
        "critical hook command is fail-open",
        Some(".githooks/pre-commit"),
        false,
        4,
    );
}
