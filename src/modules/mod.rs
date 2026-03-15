pub mod canonical;
pub mod clippy;
pub mod deny;
pub mod pre_commit;

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

    // Clippy type modules
    modules.push(&clippy::TYPE_COLLECTIONS);
    modules.push(&clippy::TYPE_SYNC);
    modules.push(&clippy::TYPE_FILESYSTEM);
    modules.push(&clippy::TYPE_GLOBAL_STATE);

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

    modules
}

/// Find a module by name.
pub fn find_module(name: &str) -> Option<&'static Module> {
    all_modules().into_iter().find(|m| m.name == name)
}
