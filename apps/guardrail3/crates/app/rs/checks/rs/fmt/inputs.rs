use super::facts::{RustfmtConfigKind, RustfmtFacts};

pub struct RustfmtRootInput<'a> {
    pub config_rel: Option<&'a str>,
    pub parsed: Option<&'a toml::Value>,
    pub workspace_edition: Option<&'a str>,
}

pub struct RustfmtExtraConfigInput<'a> {
    pub config_rel: &'a str,
    pub config_kind: RustfmtConfigKind,
}

pub struct RustfmtDualConflictInput<'a> {
    pub dir_rel: &'a str,
}

impl<'a> RustfmtRootInput<'a> {
    pub fn from_facts(facts: &'a RustfmtFacts) -> Self {
        Self {
            config_rel: facts.root_config_rel.as_deref(),
            parsed: facts.root_parsed.as_ref(),
            workspace_edition: facts.workspace_edition.as_deref(),
        }
    }
}
