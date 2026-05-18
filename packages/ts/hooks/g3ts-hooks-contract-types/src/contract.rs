#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsHookTriggerPattern {
    Glob(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsHookCommandRequirement {
    /// Meta-reference: hook delegates to the in-binary `g3ts validate workspace --path` entry.
    G3TsValidatePath,
    /// Meta-reference: hook delegates to the unit's `validate` package script.
    AppValidateScript,
    /// `tsc -p tsconfig.json --noEmit` typecheck gate.
    Tsc,
    /// `eslint --max-warnings 0` lint gate.
    Eslint,
    /// `prettier --check .` format gate.
    Prettier,
    /// `cspell . --no-progress --no-summary` spelling gate.
    Cspell,
    /// `stylelint --max-warnings 0 **/*.css` style lint gate.
    Stylelint,
    /// `syncpack lint` package policy gate.
    SyncpackLint,
    /// `type-coverage --at-least 100` type coverage gate.
    TypeCoverage,
}

/// Node package manager that determines the `exec` prefix for toolchain gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Pnpm,
    Yarn,
    Npm,
    Bun,
}

impl G3TsHookCommandRequirement {
    /// Returns the concrete argv that satisfies this requirement under the
    /// given package manager, or `None` when the requirement does not map to
    /// a runnable toolchain gate command.
    ///
    /// Variants that return `None`:
    /// - `G3TsValidatePath`: the in-binary workspace validator IS the entry
    ///   point that receives this delegation, so it does not re-invoke itself.
    /// - `AppValidateScript`: the hook executes the unit-level `validate` npm
    ///   script directly; there is no toolchain-gate equivalent.
    #[must_use]
    pub fn concrete_command(self, manager: PackageManager) -> Option<Vec<String>> {
        let tail: &[&str] = match self {
            Self::G3TsValidatePath | Self::AppValidateScript => return None,
            Self::Tsc => &["tsc", "-p", "tsconfig.json", "--noEmit"],
            Self::Eslint => &["eslint", "--max-warnings", "0"],
            Self::Prettier => &["prettier", "--check", "."],
            Self::Cspell => &["cspell", ".", "--no-progress", "--no-summary"],
            Self::Stylelint => &["stylelint", "--max-warnings", "0", "**/*.css"],
            Self::SyncpackLint => &["syncpack", "lint"],
            Self::TypeCoverage => &["type-coverage", "--at-least", "100"],
        };
        let mut argv: Vec<String> = manager
            .exec_prefix()
            .iter()
            .map(|token| (*token).to_owned())
            .collect();
        for token in tail {
            argv.push((*token).to_owned());
        }
        Some(argv)
    }
}

impl PackageManager {
    /// Returns the argv tokens that prepend a tool invocation under this
    /// package manager. For example, pnpm prepends `pnpm exec`, npm prepends
    /// `npx --no-install`, yarn prepends `yarn exec`, bun prepends `bunx`.
    #[must_use]
    pub const fn exec_prefix(self) -> &'static [&'static str] {
        match self {
            Self::Pnpm => &["pnpm", "exec"],
            Self::Yarn => &["yarn", "exec"],
            Self::Npm => &["npx", "--no-install"],
            Self::Bun => &["bunx"],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsHookCriticalCommand {
    Binary(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHookRequirement {
    /// Stable identifier for the hook contract.
    id: String,
    /// Family that owns and governs this hook contract.
    owner_family: String,
    /// File-pattern triggers under which the hook must run.
    trigger_patterns: Vec<G3TsHookTriggerPattern>,
    /// Commands the hook must execute to satisfy the contract.
    required_commands: Vec<G3TsHookCommandRequirement>,
    /// Commands considered critical for fail-closed enforcement.
    critical_commands: Vec<G3TsHookCriticalCommand>,
}

impl G3TsHookRequirement {
    #[must_use]
    pub const fn new(
        id: String,
        owner_family: String,
        trigger_patterns: Vec<G3TsHookTriggerPattern>,
        required_commands: Vec<G3TsHookCommandRequirement>,
        critical_commands: Vec<G3TsHookCriticalCommand>,
    ) -> Self {
        Self {
            id,
            owner_family,
            trigger_patterns,
            required_commands,
            critical_commands,
        }
    }

    /// File-pattern triggers under which the hook must run.
    #[must_use]
    pub fn trigger_patterns(&self) -> &[G3TsHookTriggerPattern] {
        self.trigger_patterns.as_slice()
    }

    /// Commands the hook must execute to satisfy the contract.
    #[must_use]
    pub fn required_commands(&self) -> &[G3TsHookCommandRequirement] {
        let required: &Vec<G3TsHookCommandRequirement> = &self.required_commands;
        required.as_slice()
    }

    /// Commands considered critical for fail-closed enforcement.
    #[must_use]
    pub fn critical_commands(&self) -> &[G3TsHookCriticalCommand] {
        let critical: &[G3TsHookCriticalCommand] = &self.critical_commands;
        critical
    }
}
