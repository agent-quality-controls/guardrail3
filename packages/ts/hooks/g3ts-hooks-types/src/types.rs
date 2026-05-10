use g3ts_hooks_contract_types::G3TsHookRequirement;
use hook_shell_parser::types::ParsedShellScript;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksSelectedHookConfigFact {
    /// Workspace-relative path of the source.
    rel_path: String,
    /// Parsed shell script representation.
    parsed: ParsedShellScript,
}

impl G3TsHooksSelectedHookConfigFact {
    #[must_use]
    pub const fn new(rel_path: String, parsed: ParsedShellScript) -> Self {
        Self { rel_path, parsed }
    }

    #[must_use]
    pub fn rel_path(&self) -> &str {
        &self.rel_path
    }

    #[must_use]
    pub const fn parsed(&self) -> &ParsedShellScript {
        &self.parsed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksConfigChecksInput {
    /// Whether the hooks family is active for this run.
    active: bool,
    /// Selected hook config fact, if any.
    selected_hook: Option<G3TsHooksSelectedHookConfigFact>,
    /// Tools installed and on PATH for this run.
    installed_tools: Vec<String>,
    /// Hook requirements aggregated from contributing families.
    requirements: Vec<G3TsHookRequirement>,
}

impl G3TsHooksConfigChecksInput {
    #[must_use]
    pub const fn new(
        active: bool,
        selected_hook: Option<G3TsHooksSelectedHookConfigFact>,
        installed_tools: Vec<String>,
        requirements: Vec<G3TsHookRequirement>,
    ) -> Self {
        Self {
            active,
            selected_hook,
            installed_tools,
            requirements,
        }
    }

    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }

    #[must_use]
    pub const fn selected_hook(&self) -> Option<&G3TsHooksSelectedHookConfigFact> {
        self.selected_hook.as_ref()
    }

    #[must_use]
    pub fn installed_tools(&self) -> &[String] {
        &self.installed_tools
    }

    pub fn replace_requirements(&mut self, requirements: Vec<G3TsHookRequirement>) {
        self.requirements = requirements;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum G3TsHookScriptKind {
    PreCommit,
    Verifier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct G3TsHooksEnabledCategories {
    /// Whether stylelint enforcement is enabled.
    stylelint: bool,
    /// Whether package policy enforcement is enabled.
    package_policy: bool,
    /// Whether typecov enforcement is enabled.
    typecov: bool,
}

impl G3TsHooksEnabledCategories {
    #[must_use]
    pub const fn new(stylelint: bool, package_policy: bool, typecov: bool) -> Self {
        Self {
            stylelint,
            package_policy,
            typecov,
        }
    }

    #[must_use]
    pub const fn none() -> Self {
        Self::new(false, false, false)
    }

    #[must_use]
    pub const fn all() -> Self {
        Self::new(true, true, true)
    }

    /// Returns whether stylelint enforcement is enabled.
    #[must_use]
    pub const fn stylelint(self) -> bool {
        self.stylelint
    }

    /// Returns whether package policy enforcement is enabled.
    #[must_use]
    pub const fn package_policy(self) -> bool {
        self.package_policy
    }

    /// Returns whether typecov enforcement is enabled.
    #[must_use]
    pub const fn typecov(self) -> bool {
        self.typecov
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksSourceChecksInput {
    /// Workspace-relative path of the source.
    rel_path: String,
    /// Whether the script is the pre-commit or verifier entry.
    kind: G3TsHookScriptKind,
    /// Parsed shell script representation.
    parsed: ParsedShellScript,
    /// Whether the modular hooks directory is present.
    has_modular_dir: bool,
    /// Detected app/package roots.
    app_package_roots: Vec<String>,
    /// Enabled hook categories.
    enabled_categories: G3TsHooksEnabledCategories,
    /// Hook requirements aggregated from contributing families.
    requirements: Vec<G3TsHookRequirement>,
}

impl G3TsHooksSourceChecksInput {
    #[must_use]
    pub const fn new(
        rel_path: String,
        kind: G3TsHookScriptKind,
        parsed: ParsedShellScript,
        has_modular_dir: bool,
        app_package_roots: Vec<String>,
        enabled_categories: G3TsHooksEnabledCategories,
        requirements: Vec<G3TsHookRequirement>,
    ) -> Self {
        Self {
            rel_path,
            kind,
            parsed,
            has_modular_dir,
            app_package_roots,
            enabled_categories,
            requirements,
        }
    }

    #[must_use]
    pub fn rel_path(&self) -> &str {
        &self.rel_path
    }

    #[must_use]
    pub const fn kind(&self) -> G3TsHookScriptKind {
        self.kind
    }

    #[must_use]
    pub const fn parsed(&self) -> &ParsedShellScript {
        &self.parsed
    }

    #[must_use]
    pub const fn has_modular_dir(&self) -> bool {
        self.has_modular_dir
    }

    #[must_use]
    pub fn app_package_roots(&self) -> &[String] {
        &self.app_package_roots
    }

    #[must_use]
    pub const fn enabled_categories(&self) -> G3TsHooksEnabledCategories {
        self.enabled_categories
    }

    #[must_use]
    pub fn requirements(&self) -> &[G3TsHookRequirement] {
        &self.requirements
    }

    pub fn replace_requirements(&mut self, requirements: Vec<G3TsHookRequirement>) {
        self.requirements = requirements;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksScriptFileFact {
    /// Workspace-relative path of the source.
    rel_path: String,
    /// Number of lines in the script file.
    line_count: usize,
    /// Number of bytes in the script file.
    byte_count: usize,
    /// Whether the file is executable, if known.
    executable: Option<bool>,
}

impl G3TsHooksScriptFileFact {
    #[must_use]
    pub const fn new(
        rel_path: String,
        line_count: usize,
        byte_count: usize,
        executable: Option<bool>,
    ) -> Self {
        Self {
            rel_path,
            line_count,
            byte_count,
            executable,
        }
    }

    #[must_use]
    pub fn rel_path(&self) -> &str {
        &self.rel_path
    }

    #[must_use]
    pub const fn line_count(&self) -> usize {
        self.line_count
    }

    #[must_use]
    pub const fn byte_count(&self) -> usize {
        self.byte_count
    }

    #[must_use]
    pub const fn executable(&self) -> Option<bool> {
        self.executable
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksFileTreeChecksInput {
    /// Whether the hooks family is active for this run.
    active: bool,
    /// Pre-commit script file fact, when present.
    pre_commit: Option<G3TsHooksScriptFileFact>,
    /// Whether the modular hooks directory is present.
    has_modular_dir: bool,
    /// Modular script file facts under the hooks directory.
    modular_scripts: Vec<G3TsHooksScriptFileFact>,
    /// Local override script paths.
    local_override_scripts: Vec<String>,
    /// Configured `core.hooksPath` value, when set.
    hooks_path: Option<String>,
    /// Detected trust risks for hook execution.
    trust_risks: Vec<String>,
}

impl G3TsHooksFileTreeChecksInput {
    #[must_use]
    pub const fn new(
        active: bool,
        pre_commit: Option<G3TsHooksScriptFileFact>,
        has_modular_dir: bool,
        modular_scripts: Vec<G3TsHooksScriptFileFact>,
        local_override_scripts: Vec<String>,
        hooks_path: Option<String>,
        trust_risks: Vec<String>,
    ) -> Self {
        Self {
            active,
            pre_commit,
            has_modular_dir,
            modular_scripts,
            local_override_scripts,
            hooks_path,
            trust_risks,
        }
    }

    #[must_use]
    pub const fn active(&self) -> bool {
        self.active
    }

    #[must_use]
    pub const fn pre_commit(&self) -> Option<&G3TsHooksScriptFileFact> {
        self.pre_commit.as_ref()
    }

    #[must_use]
    pub const fn has_modular_dir(&self) -> bool {
        self.has_modular_dir
    }

    #[must_use]
    pub fn modular_scripts(&self) -> &[G3TsHooksScriptFileFact] {
        &self.modular_scripts
    }

    #[must_use]
    pub fn local_override_scripts(&self) -> &[String] {
        &self.local_override_scripts
    }

    #[must_use]
    pub fn hooks_path(&self) -> Option<&str> {
        self.hooks_path.as_deref()
    }

    #[must_use]
    pub fn trust_risks(&self) -> &[String] {
        &self.trust_risks
    }
}
