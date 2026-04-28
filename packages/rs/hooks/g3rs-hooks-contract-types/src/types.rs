#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3HookRequirement {
    pub id: String,
    pub owner_family: String,
    pub trigger_patterns: Vec<G3HookTriggerPattern>,
    pub required_commands: Vec<G3HookCommandRequirement>,
    pub critical_commands: Vec<G3HookCriticalCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3HookTriggerPattern {
    ExactPath(String),
    Glob(String),
    Extension(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3HookCommandRequirement {
    CargoFmtCheck,
    CargoClippyDenyWarnings,
    CargoDenyCheck,
    ConcreteLockfileCommand,
    CargoTest,
    CargoMachete,
    CargoDupes,
    CargoDupesExcludeTests,
    Gitleaks,
    G3RsValidatePath,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3HookCriticalCommand {
    Binary(String),
    CargoSubcommand(String),
}
