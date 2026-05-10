use std::collections::BTreeMap;

use eslint_config_parser::types::{EslintConfigFileKind, EslintProbeKind, EslintRuleSeverity};
use g3ts_eslint_types::{
    G3TsEslintConfigChecksInput, G3TsEslintConfigSnapshot, G3TsEslintConfigState,
    G3TsEslintEffectiveConfigProbe, G3TsEslintRuleSetting, G3TsEslintSelectedConfig,
};
use serde_json::{Value, json};

const TS_PLUGINS: &[&str] = &["@typescript-eslint", "unicorn", "regexp", "sonarjs"];
const TEST_RULE_RELAXATIONS: &[&str] = &["@typescript-eslint/no-explicit-any"];
const CORE_BASELINE_RULES: &[&str] = &[
    "@typescript-eslint/no-floating-promises",
    "eqeqeq",
    "no-restricted-globals",
    "import-x/no-cycle",
    "import-x/max-dependencies",
    "@typescript-eslint/explicit-function-return-type",
    "@typescript-eslint/strict-boolean-expressions",
];
const TYPE_SAFETY_RULES: &[&str] = &[
    "@typescript-eslint/no-misused-promises",
    "@typescript-eslint/await-thenable",
    "@typescript-eslint/consistent-type-imports",
    "@typescript-eslint/no-non-null-assertion",
    "@typescript-eslint/switch-exhaustiveness-check",
    "@typescript-eslint/no-unused-vars",
    "@typescript-eslint/require-await",
    "no-param-reassign",
    "@typescript-eslint/no-unsafe-assignment",
    "@typescript-eslint/no-unsafe-member-access",
    "@typescript-eslint/no-unsafe-call",
    "@typescript-eslint/no-unsafe-return",
    "@typescript-eslint/no-unsafe-argument",
];
const HYGIENE_RULES: &[&str] = &[
    "@typescript-eslint/explicit-module-boundary-types",
    "@typescript-eslint/promise-function-async",
    "@typescript-eslint/consistent-type-exports",
    "@typescript-eslint/consistent-type-definitions",
    "@typescript-eslint/no-unnecessary-condition",
    "@typescript-eslint/prefer-nullish-coalescing",
    "@typescript-eslint/prefer-optional-chain",
    "@typescript-eslint/no-deprecated",
    "@typescript-eslint/restrict-template-expressions",
    "no-throw-literal",
    "no-empty",
];
const UNICORN_RULES: &[&str] = &[
    "unicorn/no-keyword-prefix",
    "unicorn/no-unused-properties",
    "unicorn/require-post-message-target-origin",
    "unicorn/no-anonymous-default-export",
];
const REGEXP_RULES: &[&str] = &[
    "regexp/require-unicode-regexp",
    "regexp/require-unicode-sets-regexp",
    "regexp/prefer-named-capture-group",
    "regexp/prefer-named-backreference",
    "regexp/prefer-result-array-groups",
    "regexp/no-misleading-capturing-group",
];
const SONARJS_RULES: &[&str] = &[
    "sonarjs/cognitive-complexity",
    "sonarjs/no-identical-functions",
    "sonarjs/no-all-duplicated-branches",
    "sonarjs/no-duplicated-branches",
    "sonarjs/no-collapsible-if",
    "sonarjs/no-identical-conditions",
    "sonarjs/no-identical-expressions",
    "sonarjs/no-inverted-boolean-check",
    "sonarjs/no-redundant-boolean",
    "sonarjs/prefer-single-boolean-return",
    "sonarjs/no-gratuitous-expressions",
    "sonarjs/no-invariant-returns",
    "sonarjs/no-collection-size-mischeck",
    "sonarjs/no-empty-collection",
    "sonarjs/no-element-overwrite",
    "sonarjs/no-unused-collection",
    "sonarjs/no-use-of-empty-return-value",
    "sonarjs/no-nested-switch",
    "sonarjs/no-nested-template-literals",
    "sonarjs/no-redundant-jump",
    "sonarjs/expression-complexity",
    "sonarjs/no-async-constructor",
    "sonarjs/no-hook-setter-in-body",
    "sonarjs/no-useless-react-setstate",
];

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

pub(super) fn golden() -> G3TsEslintConfigChecksInput {
    parsed_fixture(Fixture::golden())
}

pub(super) fn wrong_thresholds() -> G3TsEslintConfigChecksInput {
    parsed_fixture(Fixture::wrong_thresholds())
}

pub(super) fn missing_rule_groups() -> G3TsEslintConfigChecksInput {
    parsed_fixture(Fixture::missing_rule_groups())
}

pub(super) fn broken_carveouts() -> G3TsEslintConfigChecksInput {
    parsed_fixture(Fixture::broken_carveouts())
}

pub(super) fn missing_plugin_stack() -> G3TsEslintConfigChecksInput {
    parsed_fixture(Fixture::missing_plugin_stack())
}

struct Fixture {
    ts_plugins: Vec<String>,
    ts_rules: BTreeMap<String, G3TsEslintRuleSetting>,
    ts_project_service: Option<bool>,
    tsx_plugins: Vec<String>,
    tsx_rules: BTreeMap<String, G3TsEslintRuleSetting>,
    tsx_project_service: Option<bool>,
    ts_test_rules: BTreeMap<String, G3TsEslintRuleSetting>,
    ts_test_project_service: Option<bool>,
    js_rules: BTreeMap<String, G3TsEslintRuleSetting>,
    js_project_service: Option<bool>,
}

impl Fixture {
    fn golden() -> Self {
        let mut ts_rules = BTreeMap::new();
        seed_error_rule(&mut ts_rules, "@typescript-eslint/no-explicit-any");
        seed_error_rule(&mut ts_rules, "no-console");
        seed_threshold_rule(&mut ts_rules, "max-lines", 400, "max");
        seed_threshold_rule(&mut ts_rules, "max-lines-per-function", 100, "max");
        seed_threshold_rule(&mut ts_rules, "complexity", 25, "max");
        seed_error_rule(&mut ts_rules, "no-restricted-imports");
        seed_error_rules(&mut ts_rules, CORE_BASELINE_RULES);
        seed_error_rules(&mut ts_rules, TYPE_SAFETY_RULES);
        seed_error_rules(&mut ts_rules, HYGIENE_RULES);
        seed_error_rules(&mut ts_rules, UNICORN_RULES);
        seed_error_rules(&mut ts_rules, REGEXP_RULES);
        seed_error_rules(&mut ts_rules, SONARJS_RULES);

        let mut ts_test_rules = ts_rules.clone();
        seed_off_rules(&mut ts_test_rules, TEST_RULE_RELAXATIONS);

        let js_rules = BTreeMap::new();
        let tsx_source_rules = ts_rules.clone();

        Self {
            ts_plugins: TS_PLUGINS
                .iter()
                .map(|plugin| (*plugin).to_owned())
                .collect(),
            ts_rules,
            ts_project_service: Some(true),
            tsx_plugins: TS_PLUGINS
                .iter()
                .map(|plugin| (*plugin).to_owned())
                .collect(),
            tsx_rules: tsx_source_rules,
            tsx_project_service: Some(true),
            ts_test_rules,
            ts_test_project_service: Some(true),
            js_rules,
            js_project_service: Some(false),
        }
    }

    fn wrong_thresholds() -> Self {
        let mut fixture = Self::golden();
        set_threshold_rule(&mut fixture.ts_rules, "max-lines", 500, "max");
        fixture
    }

    fn missing_rule_groups() -> Self {
        let mut fixture = Self::golden();
        let _ = fixture.ts_rules.remove("unicorn/no-keyword-prefix");
        let _ = fixture.tsx_rules.remove("unicorn/no-keyword-prefix");
        let _ = fixture.ts_rules.remove("regexp/require-unicode-regexp");
        let _ = fixture.tsx_rules.remove("regexp/require-unicode-regexp");
        let _ = fixture.ts_rules.remove("sonarjs/cognitive-complexity");
        let _ = fixture.tsx_rules.remove("sonarjs/cognitive-complexity");
        let _ = fixture
            .ts_rules
            .remove("@typescript-eslint/no-floating-promises");
        let _ = fixture
            .tsx_rules
            .remove("@typescript-eslint/no-floating-promises");
        let _ = fixture
            .ts_rules
            .remove("@typescript-eslint/no-unsafe-assignment");
        let _ = fixture
            .tsx_rules
            .remove("@typescript-eslint/no-unsafe-assignment");
        let _ = fixture
            .ts_rules
            .remove("@typescript-eslint/no-unnecessary-condition");
        let _ = fixture
            .tsx_rules
            .remove("@typescript-eslint/no-unnecessary-condition");
        fixture
            .ts_plugins
            .retain(|plugin| plugin != "@typescript-eslint");
        fixture
            .tsx_plugins
            .retain(|plugin| plugin != "@typescript-eslint");
        fixture
    }

    fn broken_carveouts() -> Self {
        let mut fixture = Self::golden();
        set_rule_severity(
            &mut fixture.ts_test_rules,
            "@typescript-eslint/no-explicit-any",
            EslintRuleSeverity::Error,
        );
        fixture.js_project_service = Some(true);
        seed_error_rule(
            &mut fixture.js_rules,
            "@typescript-eslint/no-unsafe-assignment",
        );
        fixture
    }

    fn missing_plugin_stack() -> Self {
        let mut fixture = Self::golden();
        fixture
            .ts_plugins
            .retain(|plugin| plugin == "@typescript-eslint");
        fixture
            .tsx_plugins
            .retain(|plugin| plugin == "@typescript-eslint");
        fixture
    }
}

fn parsed_fixture(fixture: Fixture) -> G3TsEslintConfigChecksInput {
    let snapshot = G3TsEslintConfigSnapshot {
        selected_config: G3TsEslintSelectedConfig {
            rel_path: "eslint.config.mjs".to_owned(),
            kind: EslintConfigFileKind::Mjs,
        },
        probes: vec![
            G3TsEslintEffectiveConfigProbe {
                probe: EslintProbeKind::TsSource,
                rel_path: "src/index.ts".to_owned(),
                ignored: false,
                plugins: fixture.ts_plugins,
                rules: fixture.ts_rules,
                project_service: fixture.ts_project_service,
            },
            G3TsEslintEffectiveConfigProbe {
                probe: EslintProbeKind::TsxSource,
                rel_path: "src/app/page.tsx".to_owned(),
                ignored: false,
                plugins: fixture.tsx_plugins,
                rules: fixture.tsx_rules,
                project_service: fixture.tsx_project_service,
            },
            G3TsEslintEffectiveConfigProbe {
                probe: EslintProbeKind::TsTest,
                rel_path: "src/index.test.ts".to_owned(),
                ignored: false,
                plugins: vec!["@typescript-eslint".to_owned()],
                rules: fixture.ts_test_rules,
                project_service: fixture.ts_test_project_service,
            },
            G3TsEslintEffectiveConfigProbe {
                probe: EslintProbeKind::JsSource,
                rel_path: "scripts/build.js".to_owned(),
                ignored: false,
                plugins: vec![],
                rules: fixture.js_rules,
                project_service: fixture.js_project_service,
            },
        ],
    };

    G3TsEslintConfigChecksInput {
        config: G3TsEslintConfigState::Parsed { snapshot },
    }
}

fn seed_error_rules(rules: &mut BTreeMap<String, G3TsEslintRuleSetting>, rule_names: &[&str]) {
    for rule_name in rule_names {
        seed_error_rule(rules, rule_name);
    }
}

fn seed_error_rule(rules: &mut BTreeMap<String, G3TsEslintRuleSetting>, rule_name: &str) {
    assert!(
        rules
            .insert(rule_name.to_owned(), error_rule_setting(vec![]))
            .is_none(),
        "{rule_name} seed should not overwrite an existing rule",
    );
}

fn seed_off_rules(rules: &mut BTreeMap<String, G3TsEslintRuleSetting>, rule_names: &[&str]) {
    for rule_name in rule_names {
        set_rule_severity(rules, rule_name, EslintRuleSeverity::Off);
    }
}

fn set_rule_severity(
    rules: &mut BTreeMap<String, G3TsEslintRuleSetting>,
    rule_name: &str,
    severity: EslintRuleSeverity,
) {
    assert!(
        rules
            .insert(
                rule_name.to_owned(),
                G3TsEslintRuleSetting {
                    severity,
                    options: vec![],
                },
            )
            .is_some(),
        "{rule_name} should already exist before changing severity",
    );
}

fn seed_threshold_rule(
    rules: &mut BTreeMap<String, G3TsEslintRuleSetting>,
    rule_name: &str,
    max: i64,
    key: &str,
) {
    assert!(
        rules
            .insert(
                rule_name.to_owned(),
                error_rule_setting(vec![json!({ key: max })])
            )
            .is_none(),
        "{rule_name} threshold seed should not overwrite an existing rule",
    );
}

fn set_threshold_rule(
    rules: &mut BTreeMap<String, G3TsEslintRuleSetting>,
    rule_name: &str,
    max: i64,
    key: &str,
) {
    assert!(
        rules
            .insert(
                rule_name.to_owned(),
                error_rule_setting(vec![json!({ key: max })])
            )
            .is_some(),
        "{rule_name} threshold should already exist before changing value",
    );
}

fn error_rule_setting(options: Vec<Value>) -> G3TsEslintRuleSetting {
    G3TsEslintRuleSetting {
        severity: EslintRuleSeverity::Error,
        options,
    }
}
