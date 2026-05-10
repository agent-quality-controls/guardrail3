use g3ts_hooks_types::G3TsHooksFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Runs the g3ts hooks file-tree checks against `input`.
#[must_use]
pub fn check(input: &G3TsHooksFileTreeChecksInput) -> Vec<G3CheckResult> {
    if !input.active() {
        return Vec::new();
    }

    let mut results = Vec::new();
    pre_commit_exists(input, &mut results);
    hooks_path_configured(input, &mut results);
    pre_commit_executable(input, &mut results);
    modular_directory_inventory(input, &mut results);
    modular_scripts_inventory(input, &mut results);
    local_override_inventory(input, &mut results);
    script_stats_inventory(input, &mut results);
    file_size_inventory(input, &mut results);
    trust_risk_inventory(input, &mut results);
    results
}

/// Errors when no pre-commit hook is configured.
fn pre_commit_exists(input: &G3TsHooksFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.pre_commit().is_none() {
        results.push(error(
            "g3ts-hooks/pre-commit-exists",
            "pre-commit hook is missing",
            "TypeScript projects must have a selected pre-commit hook. Configure `git config core.hooksPath .githooks` and create `.githooks/pre-commit`.",
            None,
            None,
        ));
    }
}

/// Errors when `core.hooksPath` is not the repo-owned `.githooks` directory.
fn hooks_path_configured(input: &G3TsHooksFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    if input.hooks_path() != Some(".githooks") {
        results.push(error(
            "g3ts-hooks/hooks-path-configured",
            "git hooks path is not .githooks",
            "Git must use the repo-owned hook directory: run `git config core.hooksPath .githooks`. Other hook locations can bypass G3TS without changing repo files.",
            None,
            None,
        ));
    }
}

/// Errors when the pre-commit hook script is not executable.
fn pre_commit_executable(input: &G3TsHooksFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    let Some(script) = input.pre_commit() else {
        return;
    };
    if script.executable() == Some(false) {
        results.push(error(
            "g3ts-hooks/pre-commit-executable",
            "pre-commit hook is not executable",
            "Make `.githooks/pre-commit` executable so Git can run the G3TS contract before commits.",
            Some(script.rel_path()),
            None,
        ));
    }
}

/// Reports inventory for the optional `.githooks/pre-commit.d` modular directory.
fn modular_directory_inventory(
    input: &G3TsHooksFileTreeChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    let message = if input.has_modular_dir() {
        ".githooks/pre-commit.d exists and can hold focused hook steps.".to_owned()
    } else {
        ".githooks/pre-commit.d is absent; all hook logic is in the selected pre-commit script."
            .to_owned()
    };
    results.push(info(
        "g3ts-hooks/modular-directory-inventory",
        "modular hook directory inventory",
        message,
        None,
        None,
    ));
}

/// Reports inventory for each modular hook script discovered in the modular dir.
fn modular_scripts_inventory(
    input: &G3TsHooksFileTreeChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    for script in input.modular_scripts() {
        results.push(info(
            "g3ts-hooks/modular-scripts-inventory",
            "modular hook script inventory",
            format!(
                "`{}` has {} lines and {} bytes.",
                script.rel_path(),
                script.line_count(),
                script.byte_count()
            ),
            Some(script.rel_path()),
            None,
        ));
    }
}

/// Reports inventory for any local hook override scripts.
fn local_override_inventory(
    input: &G3TsHooksFileTreeChecksInput,
    results: &mut Vec<G3CheckResult>,
) {
    for script in input.local_override_scripts() {
        results.push(info(
            "g3ts-hooks/local-override-inventory",
            "local hook override inventory",
            format!("Local hook override `{script}` exists and is intentionally inventory-only."),
            None,
            None,
        ));
    }
}

/// Reports inventory for the pre-commit script's line and byte counts.
fn script_stats_inventory(input: &G3TsHooksFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    if let Some(script) = input.pre_commit() {
        results.push(info(
            "g3ts-hooks/script-stats-inventory",
            "pre-commit script inventory",
            format!(
                "`{}` has {} lines and {} bytes.",
                script.rel_path(),
                script.line_count(),
                script.byte_count()
            ),
            Some(script.rel_path()),
            None,
        ));
    }
}

/// Reports inventory for the pre-commit script's file size.
fn file_size_inventory(input: &G3TsHooksFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    if let Some(script) = input.pre_commit() {
        results.push(info(
            "g3ts-hooks/pre-commit-file-size-inventory",
            "pre-commit file size inventory",
            format!("`{}` is {} bytes.", script.rel_path(), script.byte_count()),
            Some(script.rel_path()),
            None,
        ));
    }
}

/// Reports inventory for any alternate hook surfaces that bypass the contract.
fn trust_risk_inventory(input: &G3TsHooksFileTreeChecksInput, results: &mut Vec<G3CheckResult>) {
    for path in input.trust_risks() {
        results.push(info(
            "g3ts-hooks/alternate-hook-surface-inventory",
            "alternate hook surface inventory",
            format!(
                "`{path}` exists. G3TS only trusts the configured `.githooks/pre-commit` surface."
            ),
            Some(path.as_str()),
            None,
        ));
    }
}

/// Builds an error-severity check result.
fn error(
    id: &str,
    title: &str,
    message: &str,
    file: Option<&str>,
    line: Option<usize>,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message.to_owned(),
        file.map(ToOwned::to_owned),
        line,
    )
}

/// Builds an info-severity inventory check result.
fn info(
    id: &str,
    title: &str,
    message: String,
    file: Option<&str>,
    line: Option<usize>,
) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        file.map(ToOwned::to_owned),
        line,
    )
    .into_inventory()
}
