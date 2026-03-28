pub mod canonical;
pub mod clippy;
pub mod cspell;
pub mod deny;
pub mod eslint;
pub mod guide;
pub mod pre_commit;
pub mod release;
pub mod stylelint;

#[derive(Debug)]
pub struct Module {
    pub name: &'static str,
    pub description: &'static str,
    pub content: &'static str,
}

/// Returns all registered modules across all categories.
#[allow(clippy::vec_init_then_push)] // reason: push-based construction is clearer for module registry
pub fn all_modules() -> Vec<&'static Module> {
    let mut modules: Vec<&'static Module> = Vec::new();

    // Clippy method modules
    modules.push(&clippy::METHOD_ENV_VARS);
    modules.push(&clippy::METHOD_ENV_MUTATION);
    modules.push(&clippy::METHOD_PROCESS_CONTROL);
    modules.push(&clippy::METHOD_BLOCKING_SLEEP);
    modules.push(&clippy::METHOD_FILESYSTEM);
    modules.push(&clippy::METHOD_HTTP_CLIENT);
    modules.push(&clippy::METHOD_GARDE_DESERIALIZATION);

    // Clippy type modules
    modules.push(&clippy::TYPE_COLLECTIONS);
    modules.push(&clippy::TYPE_SYNC);
    modules.push(&clippy::TYPE_FILESYSTEM);
    modules.push(&clippy::TYPE_DYNAMIC);
    modules.push(&clippy::TYPE_GLOBAL_STATE);
    modules.push(&clippy::TYPE_GARDE_EXTRACTORS);
    modules.push(&clippy::MACRO_DEBUGGING);

    // Deny modules
    modules.push(&deny::DENY_GRAPH);
    modules.push(&deny::DENY_BANS_BASE);
    modules.push(&deny::DENY_BANS_JSON);
    modules.push(&deny::DENY_BANS_TLS);
    modules.push(&deny::DENY_BANS_HTTP);
    modules.push(&deny::DENY_BANS_LOGGING);
    modules.push(&deny::DENY_BANS_ASYNC);
    modules.push(&deny::DENY_BANS_ERROR);
    modules.push(&deny::DENY_BANS_WEB);
    modules.push(&deny::DENY_BANS_DATETIME);
    modules.push(&deny::DENY_BANS_ORM);
    modules.push(&deny::DENY_BANS_SERIALIZATION);
    modules.push(&deny::DENY_BANS_REGEX);
    modules.push(&deny::DENY_FEATURE_BANS_TOKIO);
    modules.push(&deny::DENY_LICENSES);
    modules.push(&deny::DENY_ADVISORIES);
    modules.push(&deny::DENY_SOURCES);

    // Canonical modules
    modules.push(&canonical::RUSTFMT);
    modules.push(&canonical::RUST_TOOLCHAIN);
    modules.push(&canonical::CARGO_LINTS);

    // Pre-commit modules
    modules.push(&pre_commit::PRE_COMMIT_SCRIPT);

    // TS canonical modules
    modules.push(&canonical::NPMRC);
    modules.push(&canonical::TSCONFIG_BASE);
    modules.push(&canonical::JSCPD);
    modules.push(&canonical::ESLINT_STARTER);

    // Release modules
    modules.push(&release::RELEASE_PLZ_TOML);
    modules.push(&release::CLIFF_TOML);

    // Cspell module
    modules.push(&cspell::CSPELL_CONFIG);

    modules
}

/// Find a module by name.
pub fn find_module(name: &str) -> Option<&'static Module> {
    all_modules().into_iter().find(|m| m.name == name)
}

/// Remove override entries that already exist in the base content.
pub fn deduplicated_override(base: &str, override_content: &str) -> String {
    if override_content.trim().is_empty() {
        return String::new();
    }

    let mut result = String::new();
    for line in override_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let key = trimmed.trim_end_matches(',');
        if base.contains(key) {
            continue;
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}
