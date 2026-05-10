use std::collections::BTreeMap;

use eslint_config_parser::types::{
    EslintConfigSnapshot, EslintEffectiveConfigProbe, EslintRuleSetting,
};

use crate::types::{
    G3TsEslintConfigSnapshot, G3TsEslintEffectiveConfigProbe, G3TsEslintRuleSetting,
    G3TsEslintSelectedConfig,
};

/// Converts a parser `ESLint` snapshot into the g3ts contract snapshot.
#[must_use]
pub fn snapshot(snapshot: &EslintConfigSnapshot) -> G3TsEslintConfigSnapshot {
    G3TsEslintConfigSnapshot {
        selected_config: G3TsEslintSelectedConfig {
            rel_path: snapshot.selected_config.rel_path.clone(),
            kind: snapshot.selected_config.kind,
        },
        probes: snapshot.probes.iter().map(probe).collect(),
    }
}

/// Converts a parser `ESLint` probe into the g3ts contract probe.
fn probe(probe: &EslintEffectiveConfigProbe) -> G3TsEslintEffectiveConfigProbe {
    G3TsEslintEffectiveConfigProbe {
        probe: probe.probe,
        rel_path: probe.rel_path.clone(),
        ignored: probe.ignored,
        plugins: probe.plugins.clone(),
        rules: probe
            .rules
            .iter()
            .map(|(name, setting)| (name.clone(), rule_setting(setting)))
            .collect::<BTreeMap<_, _>>(),
        project_service: probe.project_service,
    }
}

/// Converts a parser `ESLint` rule setting into the g3ts contract rule setting.
fn rule_setting(setting: &EslintRuleSetting) -> G3TsEslintRuleSetting {
    G3TsEslintRuleSetting {
        severity: setting.severity,
        options: setting.options.clone(),
    }
}
