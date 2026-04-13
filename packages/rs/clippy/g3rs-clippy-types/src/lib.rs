use clippy_toml_parser::ClippyToml;

#[derive(Debug, Clone)]
pub enum G3RsClippyConfigState {
    Unreadable { reason: String },
    ParseError { reason: String },
    Parsed {
        raw: toml::Value,
        typed: Result<ClippyToml, String>,
    },
}

#[derive(Debug, Clone)]
pub enum G3RsClippyPolicyContextState {
    Missing,
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed {
        rel_path: String,
        profile_name: Option<String>,
        garde_enabled: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsClippyCargoConfigOverride {
    pub rel_path: String,
    pub parse_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct G3RsClippyConfigChecksInput {
    pub clippy_rel_path: String,
    pub clippy: G3RsClippyConfigState,
    pub policy_context: G3RsClippyPolicyContextState,
    pub published_library_policy: bool,
    pub cargo_config_overrides: Vec<G3RsClippyCargoConfigOverride>,
}

impl G3RsClippyConfigChecksInput {
    #[must_use]
    pub fn from_typed(clippy_rel_path: impl Into<String>, clippy: ClippyToml) -> Self {
        Self {
            clippy_rel_path: clippy_rel_path.into(),
            clippy: G3RsClippyConfigState::Parsed {
                raw: toml::from_str(
                    &toml::to_string(&clippy).expect("typed clippy config should serialize"),
                )
                .expect("serialized clippy config should parse back to toml::Value"),
                typed: Ok(clippy),
            },
            policy_context: G3RsClippyPolicyContextState::Missing,
            published_library_policy: false,
            cargo_config_overrides: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsClippyShadowedConfig {
    pub rel_path: String,
    pub preferred_rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsClippyFileTreeChecksInput {
    pub preferred_root_config_rel_path: Option<String>,
    pub shadowed_same_root_configs: Vec<G3RsClippyShadowedConfig>,
}
