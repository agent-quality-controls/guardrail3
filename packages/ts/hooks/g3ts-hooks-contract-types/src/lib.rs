#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsHookTriggerPattern {
    Glob(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsHookCommandRequirement {
    G3TsValidatePath,
    AppValidateScript,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsHookCriticalCommand {
    Binary(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsHookRequirement {
    pub id: String,
    pub owner_family: String,
    pub trigger_patterns: Vec<G3TsHookTriggerPattern>,
    pub required_commands: Vec<G3TsHookCommandRequirement>,
    pub critical_commands: Vec<G3TsHookCriticalCommand>,
}
