use g3ts_spelling_types::{
    G3TsSpellingConfigChecksInput, G3TsSpellingConfigSurfaceState, G3TsSpellingContractInput,
    G3TsSpellingPackageScriptParseBlocker, G3TsSpellingPackageSurfaceSnapshot,
    G3TsSpellingPackageSurfaceState, G3TsSpellingSyncpackSurfaceState,
};

#[test]
fn unparseable_spellcheck_reports_script_rule() {
    let input = input(vec!["spellcheck"], vec!["spellcheck"]);

    g3ts_spelling_config_checks_assertions::spellcheck_script::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn missing_spellcheck_reports_script_rule() {
    let input = input(vec!["validate"], Vec::new());

    g3ts_spelling_config_checks_assertions::spellcheck_script::assert_error(
        &input,
        Some("package.json"),
    );
}

fn input(script_names: Vec<&str>, blocked_scripts: Vec<&str>) -> G3TsSpellingConfigChecksInput {
    G3TsSpellingConfigChecksInput {
        contracts: vec![G3TsSpellingContractInput {
            app_root_rel_path: ".".to_owned(),
            package: G3TsSpellingPackageSurfaceState::Parsed {
                snapshot: G3TsSpellingPackageSurfaceSnapshot {
                    rel_path: "package.json".to_owned(),
                    dependencies: Vec::new(),
                    dev_dependencies: vec!["cspell".to_owned()],
                    script_names: script_names.into_iter().map(str::to_owned).collect(),
                    script_tool_invocations: Vec::new(),
                    script_parse_blockers: blocked_scripts
                        .into_iter()
                        .map(|script_name| G3TsSpellingPackageScriptParseBlocker {
                            script_name: script_name.to_owned(),
                            reason: "unsupported shell syntax".to_owned(),
                        })
                        .collect(),
                },
            },
            cspell_config: G3TsSpellingConfigSurfaceState::Parsed {
                rel_path: "cspell.json".to_owned(),
            },
            syncpack_config: G3TsSpellingSyncpackSurfaceState::Missing {
                rel_path: ".syncpackrc".to_owned(),
            },
        }],
    }
}
