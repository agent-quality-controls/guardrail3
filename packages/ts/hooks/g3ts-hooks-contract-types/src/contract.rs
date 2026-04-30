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
    id: String,
    owner_family: String,
    trigger_patterns: Vec<G3TsHookTriggerPattern>,
    required_commands: Vec<G3TsHookCommandRequirement>,
    critical_commands: Vec<G3TsHookCriticalCommand>,
}

impl G3TsHookRequirement {
    #[must_use]
    pub fn new(
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

    #[must_use]
    pub fn trigger_patterns(&self) -> &[G3TsHookTriggerPattern] {
        &self.trigger_patterns
    }

    #[must_use]
    pub fn required_commands(&self) -> &[G3TsHookCommandRequirement] {
        &self.required_commands
    }

    #[must_use]
    pub fn critical_commands(&self) -> &[G3TsHookCriticalCommand] {
        &self.critical_commands
    }
}
