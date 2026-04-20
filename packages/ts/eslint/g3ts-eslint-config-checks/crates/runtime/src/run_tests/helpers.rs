use std::collections::BTreeMap;

use eslint_config_parser::types::{
    EslintConfigDocument, EslintConfigFileKind, EslintConfigParseState, EslintConfigSnapshot,
    EslintEffectiveConfigProbe, EslintProbeKind, EslintRuleSetting, EslintRuleSeverity,
    EslintSelectedConfigFile,
};
use g3ts_eslint_types::{G3TsEslintConfigChecksInput, G3TsEslintConfigState};
use serde_json::json;

pub(super) fn missing() -> G3TsEslintConfigChecksInput {
    G3TsEslintConfigChecksInput {
        config: G3TsEslintConfigState::Missing,
    }
}

pub(super) fn parse_error() -> G3TsEslintConfigChecksInput {
    G3TsEslintConfigChecksInput {
        config: G3TsEslintConfigState::ParseError {
            rel_path: "eslint.config.mjs".to_owned(),
            reason: "synthetic parse failure".to_owned(),
        },
    }
}

pub(super) fn parsed(
    plugins: &[&str],
    project_service: Option<bool>,
    no_explicit_any: EslintRuleSeverity,
    no_console: EslintRuleSeverity,
) -> G3TsEslintConfigChecksInput {
    let mut rules = BTreeMap::new();
    assert!(
        rules
            .insert(
                "@typescript-eslint/no-explicit-any".to_owned(),
                EslintRuleSetting {
                    severity: no_explicit_any,
                    options: vec![],
                },
            )
            .is_none(),
        "no-explicit-any seed should not overwrite an existing rule"
    );
    assert!(
        rules
            .insert(
                "no-console".to_owned(),
                EslintRuleSetting {
                    severity: no_console,
                    options: vec![],
                },
            )
            .is_none(),
        "no-console seed should not overwrite an existing rule"
    );

    let snapshot = EslintConfigSnapshot {
        selected_config: EslintSelectedConfigFile {
            rel_path: "eslint.config.mjs".to_owned(),
            kind: EslintConfigFileKind::Mjs,
        },
        probes: vec![EslintEffectiveConfigProbe {
            probe: EslintProbeKind::TsSource,
            rel_path: "src/index.ts".to_owned(),
            ignored: false,
            plugins: plugins.iter().map(|plugin| (*plugin).to_owned()).collect(),
            rules,
            project_service,
        }],
    };

    G3TsEslintConfigChecksInput {
        config: G3TsEslintConfigState::Parsed {
            rel_path: "eslint.config.mjs".to_owned(),
            document: EslintConfigDocument {
                raw: json!({ "selected_config": { "rel_path": "eslint.config.mjs" } }),
                typed: EslintConfigParseState::Parsed(snapshot),
            },
        },
    }
}
