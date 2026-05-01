use g3ts_spelling_types::{
    G3TsSpellingConfigChecksInput, G3TsSpellingConfigSurfaceState, G3TsSpellingContractInput,
    G3TsSpellingPackageSurfaceSnapshot, G3TsSpellingPackageSurfaceState,
    G3TsSpellingSyncpackSurfaceState,
};

#[test]
fn parse_error_config_reports_config_rule() {
    let input = input(G3TsSpellingConfigSurfaceState::ParseError {
        rel_path: "cspell.json".to_owned(),
        reason: "invalid JSON".to_owned(),
    });

    g3ts_spelling_config_checks_assertions::cspell_config_present::assert_error(
        &input,
        Some("cspell.json"),
    );
}

fn input(cspell_config: G3TsSpellingConfigSurfaceState) -> G3TsSpellingConfigChecksInput {
    G3TsSpellingConfigChecksInput {
        contracts: vec![G3TsSpellingContractInput {
            app_root_rel_path: ".".to_owned(),
            package: G3TsSpellingPackageSurfaceState::Parsed {
                snapshot: G3TsSpellingPackageSurfaceSnapshot {
                    rel_path: "package.json".to_owned(),
                    dependencies: Vec::new(),
                    dev_dependencies: vec!["cspell".to_owned()],
                    script_names: vec!["spellcheck".to_owned()],
                    script_tool_invocations: Vec::new(),
                    script_parse_blockers: Vec::new(),
                },
            },
            cspell_config,
            syncpack_config: G3TsSpellingSyncpackSurfaceState::Missing {
                rel_path: ".syncpackrc".to_owned(),
            },
        }],
    }
}
