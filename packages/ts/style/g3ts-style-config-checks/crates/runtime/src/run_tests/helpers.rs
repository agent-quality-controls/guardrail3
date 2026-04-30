use std::collections::BTreeMap;

use g3ts_style_types::{
    G3TsStyleConfigChecksInput, G3TsStyleContractInput, G3TsStyleEslintSurfaceSnapshot,
    G3TsStyleEslintSurfaceState, G3TsStylePackageScriptCommandSeparator,
    G3TsStylePackageScriptToolInvocation, G3TsStylePackageSurfaceSnapshot,
    G3TsStylePackageSurfaceState, G3TsStylePolicySnapshot, G3TsStylePolicySurfaceState,
    G3TsStylelintConfigSnapshot, G3TsStylelintConfigSurfaceState,
};

pub(super) fn golden() -> G3TsStyleConfigChecksInput {
    G3TsStyleConfigChecksInput {
        contracts: vec![G3TsStyleContractInput {
            app_root_rel_path: ".".to_owned(),
            policy: policy(),
            package: package(),
            stylelint_config: stylelint_config(),
            eslint_config: eslint_config(),
        }],
    }
}

pub(super) fn parsed_package_mut(
    input: &mut G3TsStyleConfigChecksInput,
) -> &mut G3TsStylePackageSurfaceSnapshot {
    let G3TsStylePackageSurfaceState::Parsed { snapshot } = &mut input.contracts[0].package else {
        panic!("golden package should be parsed");
    };
    snapshot
}

pub(super) fn parsed_policy_mut(
    input: &mut G3TsStyleConfigChecksInput,
) -> &mut G3TsStylePolicySnapshot {
    let G3TsStylePolicySurfaceState::Parsed { snapshot } = &mut input.contracts[0].policy else {
        panic!("golden policy should be parsed");
    };
    snapshot
}

pub(super) fn parsed_stylelint_mut(
    input: &mut G3TsStyleConfigChecksInput,
) -> &mut G3TsStylelintConfigSnapshot {
    let G3TsStylelintConfigSurfaceState::Parsed { snapshot } =
        &mut input.contracts[0].stylelint_config
    else {
        panic!("golden Stylelint config should be parsed");
    };
    snapshot
}

pub(super) fn parsed_eslint_mut(
    input: &mut G3TsStyleConfigChecksInput,
) -> &mut G3TsStyleEslintSurfaceSnapshot {
    let G3TsStyleEslintSurfaceState::Parsed { snapshot } = &mut input.contracts[0].eslint_config
    else {
        panic!("golden ESLint config should be parsed");
    };
    snapshot
}

fn policy() -> G3TsStylePolicySurfaceState {
    G3TsStylePolicySurfaceState::Parsed {
        snapshot: G3TsStylePolicySnapshot {
            rel_path: "guardrail3-ts.toml".to_owned(),
            source_globs: vec!["src/**/*.tsx".to_owned()],
            stylelint_css_globs: vec!["src/**/*.css".to_owned()],
            extra_fields: Vec::new(),
        },
    }
}

fn package() -> G3TsStylePackageSurfaceState {
    G3TsStylePackageSurfaceState::Parsed {
        snapshot: G3TsStylePackageSurfaceSnapshot {
            rel_path: "package.json".to_owned(),
            dependencies: Vec::new(),
            dev_dependencies: vec![
                "stylelint".to_owned(),
                "stylelint-config-standard".to_owned(),
                "stylelint-config-tailwindcss".to_owned(),
                "@double-great/stylelint-a11y".to_owned(),
                "g3ts-eslint-plugin-style-policy".to_owned(),
            ],
            script_names: vec!["lint:css".to_owned()],
            script_tool_invocations: vec![G3TsStylePackageScriptToolInvocation {
                script_name: "lint:css".to_owned(),
                executable: "stylelint".to_owned(),
                args: vec![
                    "--max-warnings".to_owned(),
                    "0".to_owned(),
                    "src/**/*.css".to_owned(),
                ],
                preceded_by: None,
                followed_by: None,
            }],
            script_parse_blockers: Vec::new(),
        },
    }
}

fn stylelint_config() -> G3TsStylelintConfigSurfaceState {
    G3TsStylelintConfigSurfaceState::Parsed {
        snapshot: G3TsStylelintConfigSnapshot {
            rel_path: "stylelint.config.mjs".to_owned(),
            raw_extends: vec![
                "stylelint-config-standard".to_owned(),
                "stylelint-config-tailwindcss".to_owned(),
            ],
            raw_plugins: vec!["@double-great/stylelint-a11y".to_owned()],
            resolved_extends: Vec::new(),
            resolved_plugins: Vec::new(),
            resolved_rule_names: vec![
                "a11y/content-property-no-static-value".to_owned(),
                "a11y/font-size-is-readable".to_owned(),
                "a11y/line-height-is-vertical-rhythmed".to_owned(),
                "a11y/media-prefers-reduced-motion".to_owned(),
                "a11y/no-display-none".to_owned(),
                "a11y/no-obsolete-attribute".to_owned(),
                "a11y/no-obsolete-element".to_owned(),
                "a11y/no-outline-none".to_owned(),
                "a11y/no-spread-text".to_owned(),
                "a11y/no-text-align-justify".to_owned(),
                "a11y/selector-pseudo-class-focus".to_owned(),
            ],
            probe_present: true,
            probe_ignored: false,
        },
    }
}

fn eslint_config() -> G3TsStyleEslintSurfaceState {
    G3TsStyleEslintSurfaceState::Parsed {
        snapshot: G3TsStyleEslintSurfaceSnapshot {
            rel_path: "eslint.config.mjs".to_owned(),
            source_probe_present: true,
            source_probe_ignored: false,
            source_plugins: vec!["style-policy".to_owned()],
            source_plugin_package_names: BTreeMap::from([(
                "style-policy".to_owned(),
                vec!["g3ts-eslint-plugin-style-policy".to_owned()],
            )]),
            style_policy_rule_effective: true,
        },
    }
}

#[allow(
    dead_code,
    reason = "test case helper is kept beside the golden script model"
)]
pub(super) fn fail_open_separator() -> G3TsStylePackageScriptCommandSeparator {
    G3TsStylePackageScriptCommandSeparator::Or
}
