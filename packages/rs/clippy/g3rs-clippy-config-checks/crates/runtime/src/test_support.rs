use clippy_toml_parser::parse as parse_clippy_toml;
use g3rs_clippy_config_checks_types::{
    G3RsClippyCargoConfigOverride, G3RsClippyConfigChecksInput, G3RsClippyConfigState,
    G3RsClippyPolicyContextState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_domain_modules::clippy::build_clippy_toml;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Finding {
    pub(crate) id: String,
    pub(crate) severity: G3Severity,
    pub(crate) title: String,
    pub(crate) message: String,
    pub(crate) file: Option<String>,
    pub(crate) inventory: bool,
}

pub(crate) fn findings(results: &[G3CheckResult]) -> Vec<Finding> {
    results
        .iter()
        .map(|result| Finding {
            id: result.id().to_owned(),
            severity: result.severity(),
            title: result.title().to_owned(),
            message: result.message().to_owned(),
            file: result.file().map(str::to_owned),
            inventory: result.inventory(),
        })
        .collect()
}

pub(crate) fn baseline_toml(profile_name: &str, garde_enabled: bool) -> String {
    build_clippy_toml(profile_name, false, garde_enabled, "", "")
}

pub(crate) fn parsed_policy(
    rel_path: &str,
    profile_name: Option<&str>,
    garde_enabled: bool,
) -> G3RsClippyPolicyContextState {
    G3RsClippyPolicyContextState::Parsed {
        rel_path: rel_path.to_owned(),
        profile_name: profile_name.map(str::to_owned),
        garde_enabled,
    }
}

pub(crate) fn parse_error_policy(rel_path: &str, reason: &str) -> G3RsClippyPolicyContextState {
    G3RsClippyPolicyContextState::ParseError {
        rel_path: rel_path.to_owned(),
        reason: reason.to_owned(),
    }
}

pub(crate) fn override_facts(
    rel_path: &str,
    parse_error: Option<&str>,
) -> G3RsClippyCargoConfigOverride {
    G3RsClippyCargoConfigOverride {
        rel_path: rel_path.to_owned(),
        parse_error: parse_error.map(str::to_owned),
    }
}

pub(crate) fn input_from_raw(rel_path: &str, raw: &str) -> G3RsClippyConfigChecksInput {
    input_with_raw(
        rel_path,
        raw,
        G3RsClippyPolicyContextState::Missing,
        false,
        Vec::new(),
    )
}

pub(crate) fn input_with_raw(
    rel_path: &str,
    raw: &str,
    policy_context: G3RsClippyPolicyContextState,
    published_library_policy: bool,
    cargo_config_overrides: Vec<G3RsClippyCargoConfigOverride>,
) -> G3RsClippyConfigChecksInput {
    let clippy = match toml::from_str::<toml::Value>(raw) {
        Ok(raw_value) => G3RsClippyConfigState::Parsed {
            raw: raw_value,
            typed: parse_clippy_toml(raw).map_err(|err| err.to_string()),
        },
        Err(err) => G3RsClippyConfigState::ParseError {
            reason: err.to_string(),
        },
    };

    G3RsClippyConfigChecksInput {
        clippy_rel_path: rel_path.to_owned(),
        clippy,
        policy_context,
        published_library_policy,
        cargo_config_overrides,
    }
}
