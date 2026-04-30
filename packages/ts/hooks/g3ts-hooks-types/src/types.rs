use g3ts_hooks_contract_types::G3TsHookRequirement;
use hook_shell_parser::types::ParsedShellScript;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksSelectedHookConfigFact {
    rel_path: String,
    parsed: ParsedShellScript,
}

impl G3TsHooksSelectedHookConfigFact {
    #[must_use]
    pub fn new(rel_path: String, parsed: ParsedShellScript) -> Self {
        Self { rel_path, parsed }
    }

    #[must_use]
    pub fn rel_path(&self) -> &str {
        &self.rel_path
    }

    #[must_use]
    pub fn parsed(&self) -> &ParsedShellScript {
        &self.parsed
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksConfigChecksInput {
    active: bool,
    selected_hook: Option<G3TsHooksSelectedHookConfigFact>,
    installed_tools: Vec<String>,
    requirements: Vec<G3TsHookRequirement>,
}

impl G3TsHooksConfigChecksInput {
    #[must_use]
    pub fn new(
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
    pub fn active(&self) -> bool {
        self.active
    }

    #[must_use]
    pub fn selected_hook(&self) -> Option<&G3TsHooksSelectedHookConfigFact> {
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
    Modular,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksSourceChecksInput {
    rel_path: String,
    kind: G3TsHookScriptKind,
    parsed: ParsedShellScript,
    has_modular_dir: bool,
    app_package_roots: Vec<String>,
    requirements: Vec<G3TsHookRequirement>,
}

impl G3TsHooksSourceChecksInput {
    #[must_use]
    pub fn new(
        rel_path: String,
        kind: G3TsHookScriptKind,
        parsed: ParsedShellScript,
        has_modular_dir: bool,
        app_package_roots: Vec<String>,
        requirements: Vec<G3TsHookRequirement>,
    ) -> Self {
        Self {
            rel_path,
            kind,
            parsed,
            has_modular_dir,
            app_package_roots,
            requirements,
        }
    }

    #[must_use]
    pub fn rel_path(&self) -> &str {
        &self.rel_path
    }

    #[must_use]
    pub fn kind(&self) -> G3TsHookScriptKind {
        self.kind
    }

    #[must_use]
    pub fn parsed(&self) -> &ParsedShellScript {
        &self.parsed
    }

    #[must_use]
    pub fn has_modular_dir(&self) -> bool {
        self.has_modular_dir
    }

    #[must_use]
    pub fn app_package_roots(&self) -> &[String] {
        &self.app_package_roots
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
    rel_path: String,
    line_count: usize,
    byte_count: usize,
    executable: Option<bool>,
}

impl G3TsHooksScriptFileFact {
    #[must_use]
    pub fn new(
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
    pub fn line_count(&self) -> usize {
        self.line_count
    }

    #[must_use]
    pub fn byte_count(&self) -> usize {
        self.byte_count
    }

    #[must_use]
    pub fn executable(&self) -> Option<bool> {
        self.executable
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHooksFileTreeChecksInput {
    active: bool,
    pre_commit: Option<G3TsHooksScriptFileFact>,
    has_modular_dir: bool,
    modular_scripts: Vec<G3TsHooksScriptFileFact>,
    local_override_scripts: Vec<String>,
    hooks_path: Option<String>,
    trust_risks: Vec<String>,
}

impl G3TsHooksFileTreeChecksInput {
    #[must_use]
    pub fn new(
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
    pub fn active(&self) -> bool {
        self.active
    }

    #[must_use]
    pub fn pre_commit(&self) -> Option<&G3TsHooksScriptFileFact> {
        self.pre_commit.as_ref()
    }

    #[must_use]
    pub fn has_modular_dir(&self) -> bool {
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
