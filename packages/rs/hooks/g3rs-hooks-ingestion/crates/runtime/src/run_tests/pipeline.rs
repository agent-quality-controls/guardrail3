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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(assertions::count(&results, "RS-HOOKS-SOURCE-03"), 1, "{results:#?}");
    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-03",
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(assertions::count(&results, "RS-HOOKS-SOURCE-11"), 1, "{results:#?}");
    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-11",
        "cargo test workspace scope not required",
        Some(".githooks/pre-commit"),
        true,
    );
}

#[test]
fn pipeline_keeps_echoed_fmt_text_as_missing_step() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join(".githooks/pre-commit"), "echo \"cargo fmt --check\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(assertions::count(&results, "RS-HOOKS-SOURCE-03"), 1, "{results:#?}");
    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-03",
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(assertions::count(&results, "RS-HOOKS-SOURCE-03"), 1, "{results:#?}");
    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-03",
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
    write_fixture(root.join(".githooks/pre-commit.d/10-rust.sh"), "echo cargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-03",
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(assertions::count(&results, "RS-HOOKS-SOURCE-15"), 1, "{results:#?}");
    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-15",
        "Rust config changes trigger hook validation",
        Some(".githooks/pre-commit"),
        true,
    );
}

#[test]
fn pipeline_reports_top_level_g3rs_validate_step_inventory() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join(".githooks/pre-commit"), "g3rs validate --path .\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(assertions::count(&results, "RS-HOOKS-SOURCE-09"), 1, "{results:#?}");
    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-09",
        "Rust guardrail validate step present",
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
        "#!/usr/bin/env bash\nset -e\nrun-parts .githooks/pre-commit.d\ncargo fmt --check\ncargo clippy -- -D warnings\ncargo deny check\ncargo test --workspace\ncargo machete\ngitleaks protect --staged --no-banner\ncargo dupes check --exclude-tests\ng3rs validate --path .\npnpm install --frozen-lockfile\nrg '^(<<<<<<<|=======|>>>>>>>)' .\nstat -c%s Cargo.toml >/dev/null\nif echo \"$STAGED_FILES\" | grep -qE '(guardrail3-rs\\.toml|clippy\\.toml|\\.clippy\\.toml|deny\\.toml|\\.deny\\.toml|rustfmt\\.toml|\\.rustfmt\\.toml|rust-toolchain\\.toml)$'; then\n    g3rs validate --path .\nfi\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert!(assertions::non_inventory(&results).is_empty(), "{results:#?}");
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
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
        assert_eq!(assertions::count(&results, rule_id), 1, "{rule_id}: {results:#?}");
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        ("RS-HOOKS-SOURCE-05", "cargo deny check step present"),
        ("RS-HOOKS-SOURCE-09", "Rust guardrail validate step present"),
    ] {
        assert_eq!(assertions::count(&results, rule_id), 1, "{rule_id}: {results:#?}");
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(assertions::count(&results, "RS-HOOKS-SOURCE-09"), 1, "{results:#?}");
    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-09",
        "Rust guardrail validate step present",
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        ("RS-HOOKS-SOURCE-05", "cargo deny check step present"),
        ("RS-HOOKS-SOURCE-09", "Rust guardrail validate step present"),
    ] {
        assert_eq!(assertions::count(&results, rule_id), 1, "{rule_id}: {results:#?}");
        assert_result_present(&results, rule_id, title, Some(".githooks/pre-commit"), true);
    }
}

#[test]
fn pipeline_reports_inventory_for_valid_shell_hook_steps() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\nrg '^(<<<<<<<|=======|>>>>>>>)' .\nstat -c%s Cargo.toml >/dev/null\npnpm install --frozen-lockfile\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
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
        assert_eq!(assertions::count(&results, rule_id), 1, "{rule_id}: {results:#?}");
        assert_result_present(&results, rule_id, title, Some(".githooks/pre-commit"), true);
    }
}

#[test]
fn pipeline_reports_dispatcher_findings_for_real_pre_commit_script() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    fs::create_dir_all(root.join(".githooks/pre-commit.d")).expect("create modular dir");
    write_fixture(root.join(".githooks/pre-commit"), "run-parts .githooks/pre-commit.d\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(assertions::count(&results, "RS-HOOKS-SOURCE-01"), 1, "{results:#?}");
    assert_eq!(assertions::count(&results, "RS-HOOKS-SOURCE-02"), 1, "{results:#?}");
    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-01",
        "dispatcher pattern present",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-02",
        "dispatcher uses real executable syntax",
        Some(".githooks/pre-commit"),
        true,
    );
}

#[test]
fn pipeline_source_reports_missing_dispatcher_pattern_when_modular_dir_is_only_echoed() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join(".githooks/pre-commit"), "echo run-parts .githooks/pre-commit.d\n");
    fs::create_dir_all(root.join(".githooks/pre-commit.d")).expect("create modular dir");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-01",
        "dispatcher pattern missing",
        Some(".githooks/pre-commit"),
        false,
    );
    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-02",
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

    write_fixture(root.join(".githooks/pre-commit"), "g3rs validate --path .\n");
    write_fixture(bin_dir.join("gitleaks"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("cargo-deny"), "#!/usr/bin/env bash\n");
    write_fixture(bin_dir.join("cargo-machete"), "#!/usr/bin/env bash\n");
    #[cfg(unix)]
    {
        make_executable(&bin_dir.join("gitleaks"));
        make_executable(&bin_dir.join("cargo-deny"));
        make_executable(&bin_dir.join("cargo-machete"));
    }

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let path_env =
        std::env::join_paths([bin_dir.as_path()]).expect("build PATH override for hook config check");
    let input =
        super::super::ingest_for_config_checks_with_path(&crawl, Some(path_env.as_os_str()))
            .expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    assert_eq!(assertions::count(&results, "RS-HOOKS-CONFIG-02"), 1, "{results:#?}");
    assert_result_present(
        &results,
        "RS-HOOKS-CONFIG-02",
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let path_env =
        std::env::join_paths([bin_dir.as_path()]).expect("build PATH override for hook config check");
    let input =
        super::super::ingest_for_config_checks_with_path(&crawl, Some(path_env.as_os_str()))
            .expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    assert_eq!(assertions::count(&results, "RS-HOOKS-CONFIG-01"), 3, "{results:#?}");
    assert_eq!(assertions::count(&results, "RS-HOOKS-CONFIG-03"), 1, "{results:#?}");
    assert_result_present(
        &results,
        "RS-HOOKS-CONFIG-01",
        "gitleaks installed",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "RS-HOOKS-CONFIG-01",
        "cargo-deny installed",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "RS-HOOKS-CONFIG-01",
        "cargo-machete installed",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "RS-HOOKS-CONFIG-03",
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let path_env =
        std::env::join_paths([bin_dir.as_path()]).expect("build PATH override for hook config check");
    let input =
        super::super::ingest_for_config_checks_with_path(&crawl, Some(path_env.as_os_str()))
            .expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    for (rule_id, title) in [
        ("RS-HOOKS-CONFIG-02", "g3rs binary available"),
        ("RS-HOOKS-CONFIG-03", "cargo-dupes installed"),
    ] {
        assert_eq!(assertions::count(&results, rule_id), 1, "{rule_id}: {results:#?}");
        assert_result_present(&results, rule_id, title, Some(".githooks/pre-commit"), true);
    }
}

#[test]
fn pipeline_config_honors_hooks_path_selected_hook_for_binary_requirements() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    let bin_dir = root.join("bin");
    git_config_hooks_path(root, "hooks");

    write_fixture(root.join(".githooks/pre-commit"), "gitleaks protect --staged --no-banner\n");
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let path_env =
        std::env::join_paths([bin_dir.as_path()]).expect("build PATH override for hook config check");
    let input =
        super::super::ingest_for_config_checks_with_path(&crawl, Some(path_env.as_os_str()))
            .expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    for (rule_id, title) in [
        ("RS-HOOKS-CONFIG-01", "gitleaks installed"),
        ("RS-HOOKS-CONFIG-01", "cargo-deny installed"),
        ("RS-HOOKS-CONFIG-01", "cargo-machete installed"),
        ("RS-HOOKS-CONFIG-02", "g3rs binary available"),
        ("RS-HOOKS-CONFIG-03", "cargo-dupes installed"),
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        super::super::ingest_for_config_checks_with_path(&crawl, None).expect("ingestion should succeed");
    let results = g3rs_hooks_config_checks::check(&input);

    assert_eq!(assertions::count(&results, "RS-HOOKS-CONFIG-01"), 3, "{results:#?}");
    assert_result_present(
        &results,
        "RS-HOOKS-CONFIG-01",
        "gitleaks installed",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "RS-HOOKS-CONFIG-01",
        "cargo-deny installed",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "RS-HOOKS-CONFIG-01",
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert_eq!(assertions::count(&results, "RS-HOOKS-FILETREE-01"), 1, "{results:#?}");
    assert_result_present(
        &results,
        "RS-HOOKS-FILETREE-01",
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

    write_fixture(root.join(".githooks/pre-commit"), "#!/usr/bin/env bash\ncargo fmt --check\n");
    write_fixture(root.join("hooks/pre-commit"), "#!/usr/bin/env bash\ncargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert_result_present(
        &results,
        "RS-HOOKS-FILETREE-01",
        "pre-commit hook missing",
        Some(".githooks/pre-commit"),
        false,
    );
    assert_result_present(
        &results,
        "RS-HOOKS-FILETREE-02",
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

    write_fixture(root.join("hooks/pre-commit"), "#!/usr/bin/env bash\ncargo fmt --check\n");
    write_fixture(
        root.join(".githooks/pre-commit.d/10-rust.sh"),
        "#!/usr/bin/env bash\ncargo test --workspace\n",
    );
    #[cfg(unix)]
    {
        make_executable(&root.join(".githooks/pre-commit.d/10-rust.sh"));
    }

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert_result_present(
        &results,
        "RS-HOOKS-FILETREE-03",
        "pre-commit.d directory exists",
        Some(".githooks/pre-commit.d"),
        true,
    );
    assert_result_message_contains(
        &results,
        "RS-HOOKS-FILETREE-04",
        "modular hook scripts inventory",
        Some(".githooks/pre-commit.d"),
        true,
        ".githooks/pre-commit.d/10-rust.sh",
    );
    assert_result_present(
        &results,
        "RS-HOOKS-FILETREE-06",
        "modular hook script is executable",
        Some(".githooks/pre-commit.d/10-rust.sh"),
        true,
    );
}

#[test]
fn pipeline_file_tree_reports_missing_pre_commit_hook() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert_eq!(assertions::count(&results, "RS-HOOKS-FILETREE-01"), 1, "{results:#?}");
    assert_result_present(
        &results,
        "RS-HOOKS-FILETREE-01",
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

    let crawl = g3rs_workspace_crawl::crawl(&nested).expect("crawl should succeed");

    let file_tree_input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("file-tree ingestion should succeed");
    let file_tree_results = g3rs_hooks_file_tree_checks::check(&file_tree_input);
    assert_eq!(assertions::count(&file_tree_results, "RS-HOOKS-FILETREE-01"), 0, "{file_tree_results:#?}");
    assert_eq!(assertions::count(&file_tree_results, "RS-HOOKS-FILETREE-02"), 0, "{file_tree_results:#?}");

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

    write_fixture(root.join(".githooks/pre-commit"), "#!/usr/bin/env bash\nexit 0\n");
    write_fixture(root.join(".git/hooks/pre-commit"), "#!/usr/bin/env bash\nexit 0\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert_eq!(assertions::count(&results, "RS-HOOKS-FILETREE-07"), 1, "{results:#?}");
    assert_result_message_contains(
        &results,
        "RS-HOOKS-FILETREE-07",
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

    write_fixture(root.join("hooks/pre-commit"), "#!/usr/bin/env bash\ncargo fmt --check\n");
    write_fixture(root.join(".git/hooks/pre-commit"), "#!/usr/bin/env bash\nexit 0\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_hooks_file_tree_checks::check(&input);

    assert_result_present(
        &results,
        "RS-HOOKS-FILETREE-07",
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = super::super::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");
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
        assert!(assertions::count(&results, rule_id) > 0, "missing {rule_id}: {results:#?}");
    }

    assert_result_present(&results, "RS-HOOKS-FILETREE-02", "core.hooksPath configured", None, true);
    assert_result_present(
        &results,
        "RS-HOOKS-FILETREE-03",
        "pre-commit.d directory exists",
        Some(".githooks/pre-commit.d"),
        true,
    );
    assert_result_present(
        &results,
        "RS-HOOKS-FILETREE-08",
        "pre-commit hook is not executable",
        Some(".githooks/pre-commit"),
        false,
    );
    assert_result_message_equals(
        &results,
        "RS-HOOKS-FILETREE-09",
        "pre-commit script stats",
        Some(".githooks/pre-commit"),
        true,
        "2 lines, 53 bytes",
    );
    assert_result_message_contains(
        &results,
        "RS-HOOKS-FILETREE-04",
        "modular hook scripts inventory",
        Some(".githooks/pre-commit.d"),
        true,
        ".githooks/pre-commit.d/10-rust.sh",
    );
    assert_result_message_contains(
        &results,
        "RS-HOOKS-FILETREE-04",
        "modular hook scripts inventory",
        Some(".githooks/pre-commit.d"),
        true,
        ".githooks/pre-commit.d/20-rust.sh",
    );
    assert_result_message_equals(
        &results,
        "RS-HOOKS-FILETREE-10",
        "pre-commit file size",
        Some(".githooks/pre-commit"),
        true,
        "53 bytes",
    );
    assert_result_message_contains(
        &results,
        "RS-HOOKS-FILETREE-05",
        "local hook overrides inventory",
        Some(".guardrail3/overrides/pre-commit.d"),
        true,
        "90-local.sh",
    );
    assert_result_present(
        &results,
        "RS-HOOKS-FILETREE-06",
        "modular hook script is executable",
        Some(".githooks/pre-commit.d/10-rust.sh"),
        true,
    );
    assert_result_present(
        &results,
        "RS-HOOKS-FILETREE-06",
        "modular hook script is not executable",
        Some(".githooks/pre-commit.d/20-rust.sh"),
        false,
    );
}

#[test]
fn pipeline_runs_shared_source_checks_on_modular_scripts() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join(".githooks/pre-commit"), "run-parts .githooks/pre-commit.d\n");
    write_fixture(root.join(".githooks/pre-commit.d/10-rust.sh"), "echo cargo fmt --check\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-17",
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        ("RS-HOOKS-SOURCE-16", "shell error handling present"),
        ("RS-HOOKS-SOURCE-17", "valid hook shebang present"),
    ] {
        assert_eq!(assertions::count(&results, rule_id), 1, "{rule_id}: {results:#?}");
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        ("RS-HOOKS-SOURCE-09", "Rust guardrail validate step present"),
        ("RS-HOOKS-SOURCE-10", "cargo clippy denies warnings"),
    ] {
        assert_eq!(assertions::count(&results, rule_id), 1, "{rule_id}: {results:#?}");
        assert_result_present(&results, rule_id, title, Some(".githooks/pre-commit"), true);
    }
}

#[test]
fn pipeline_source_reports_missing_rust_and_shell_steps() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write_fixture(root.join(".githooks/pre-commit"), "echo nothing useful\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    for (rule_id, title) in [
        ("RS-HOOKS-SOURCE-04", "cargo clippy step missing"),
        ("RS-HOOKS-SOURCE-05", "cargo deny check step missing"),
        ("RS-HOOKS-SOURCE-06", "cargo test step missing"),
        ("RS-HOOKS-SOURCE-07", "cargo machete step missing"),
        ("RS-HOOKS-SOURCE-08", "Rust duplication tool missing"),
        ("RS-HOOKS-SOURCE-10", "cargo clippy deny-warnings step missing"),
        ("RS-HOOKS-SOURCE-12", "gitleaks step missing"),
        ("RS-HOOKS-SOURCE-13", "cargo dupes step missing"),
        ("RS-HOOKS-SOURCE-14", "cargo dupes step does not exclude tests"),
        (
            "RS-HOOKS-SOURCE-15",
            "Rust config-change trigger coverage incomplete",
        ),
        ("RS-HOOKS-SOURCE-16", "shell error handling missing"),
        ("RS-HOOKS-SOURCE-20", "merge-conflict check step missing"),
        ("RS-HOOKS-SOURCE-21", "file-size check step missing"),
        (
            "RS-HOOKS-SOURCE-23",
            "concrete lockfile integrity command missing",
        ),
    ] {
        assert_result_present(&results, rule_id, title, Some(".githooks/pre-commit"), false);
    }
    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-18",
        "no unconditional exit 0 bypass",
        Some(".githooks/pre-commit"),
        true,
    );
    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-19",
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-22",
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-11",
        "cargo test missing --workspace",
        Some(".githooks/pre-commit"),
        false,
    );
}

#[test]
fn pipeline_source_reports_invalid_dispatcher_syntax() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join(".githooks/pre-commit"), "echo run-parts .githooks/pre-commit.d\n");
    fs::create_dir_all(root.join(".githooks/pre-commit.d")).expect("create modular dir");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_result_present(
        &results,
        "RS-HOOKS-SOURCE-02",
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

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(assertions::count(&results, "RS-HOOKS-SOURCE-22"), 0, "{results:#?}");
}

#[test]
fn pipeline_reports_fail_open_wrapper_on_called_function() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "run_tests() {\n    cargo test --workspace\n}\nrun_tests || true\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    assert_eq!(assertions::count(&results, "RS-HOOKS-SOURCE-24"), 1, "{results:#?}");
    assert_result_present_on_line(
        &results,
        "RS-HOOKS-SOURCE-24",
        "critical hook command is fail-open",
        Some(".githooks/pre-commit"),
        false,
        4,
    );
}
