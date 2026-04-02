#[cfg(feature = "api")]
pub mod canonical;
#[cfg(feature = "api")]
pub mod clippy;
#[cfg(feature = "api")]
pub mod cspell;
#[cfg(feature = "api")]
pub mod deny;
#[cfg(feature = "api")]
pub mod eslint;
#[cfg(feature = "api")]
pub mod guide;
#[cfg(feature = "api")]
pub mod pre_commit;
#[cfg(feature = "api")]
pub mod release;
#[cfg(feature = "api")]
pub mod stylelint;

#[derive(Debug)]
pub struct Module {
    name: &'static str,
    description: &'static str,
    content: &'static str,
}

impl Module {
    #[must_use]
    pub const fn new(name: &'static str, description: &'static str, content: &'static str) -> Self {
        Self {
            name,
            description,
            content,
        }
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    #[must_use]
    pub const fn content(&self) -> &'static str {
        self.content
    }
}

/// Returns all registered modules across all categories.
pub fn all_modules() -> Vec<&'static Module> {
    vec![
        // Clippy method modules
        &clippy::METHOD_ENV_VARS,
        &clippy::METHOD_ENV_MUTATION,
        &clippy::METHOD_PROCESS_CONTROL,
        &clippy::METHOD_BLOCKING_SLEEP,
        &clippy::METHOD_FILESYSTEM,
        &clippy::METHOD_HTTP_CLIENT,
        &clippy::METHOD_GARDE_DESERIALIZATION,
        // Clippy type modules
        &clippy::TYPE_COLLECTIONS,
        &clippy::TYPE_SYNC,
        &clippy::TYPE_FILESYSTEM,
        &clippy::TYPE_DYNAMIC,
        &clippy::TYPE_GLOBAL_STATE,
        &clippy::TYPE_GARDE_EXTRACTORS,
        &clippy::MACRO_DEBUGGING,
        // Deny modules
        &deny::DENY_GRAPH,
        &deny::DENY_BANS_BASE,
        &deny::DENY_BANS_JSON,
        &deny::DENY_BANS_TLS,
        &deny::DENY_BANS_HTTP,
        &deny::DENY_BANS_LOGGING,
        &deny::DENY_BANS_ASYNC,
        &deny::DENY_BANS_ERROR,
        &deny::DENY_BANS_WEB,
        &deny::DENY_BANS_DATETIME,
        &deny::DENY_BANS_ORM,
        &deny::DENY_BANS_SERIALIZATION,
        &deny::DENY_BANS_REGEX,
        &deny::DENY_FEATURE_BANS_TOKIO,
        &deny::DENY_LICENSES,
        &deny::DENY_ADVISORIES,
        &deny::DENY_SOURCES,
        // Canonical modules
        &canonical::RUSTFMT,
        &canonical::RUST_TOOLCHAIN,
        &canonical::CARGO_LINTS,
        // Pre-commit modules
        &pre_commit::PRE_COMMIT_SCRIPT,
        // TS canonical modules
        &canonical::NPMRC,
        &canonical::TSCONFIG_BASE,
        &canonical::JSCPD,
        &canonical::ESLINT_STARTER,
        // Release modules
        &release::RELEASE_PLZ_TOML,
        &release::CLIFF_TOML,
        // Cspell module
        &cspell::CSPELL_CONFIG,
    ]
}

/// Find a module by name.
pub fn find_module(name: &str) -> Option<&'static Module> {
    all_modules().into_iter().find(|m| m.name() == name)
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
