use g3ts_spelling_types::{
    G3TsSpellingConfigChecksInput, G3TsSpellingConfigSurfaceState, G3TsSpellingContractInput,
    G3TsSpellingPackageSurfaceSnapshot, G3TsSpellingPackageSurfaceState,
    G3TsSpellingSyncpackSnapshot, G3TsSpellingSyncpackSurfaceState,
    G3TsSpellingSyncpackVersionGroupSnapshot,
};

#[test]
fn ignored_cspell_group_reports_syncpack_rule() {
    let input = input(vec![group(Some(true), Some("8.20.0"))]);

    g3ts_spelling_config_checks_assertions::syncpack_cspell_pin::assert_error(
        &input,
        Some(".syncpackrc"),
    );
}

#[test]
fn missing_pin_version_reports_syncpack_rule() {
    let input = input(vec![group(None, None)]);

    g3ts_spelling_config_checks_assertions::syncpack_cspell_pin::assert_error(
        &input,
        Some(".syncpackrc"),
    );
}

#[test]
fn non_ignored_cspell_pin_is_accepted() {
    let input = input(vec![group(None, Some("8.20.0"))]);

    g3ts_spelling_config_checks_assertions::syncpack_cspell_pin::assert_info(
        &input,
        Some(".syncpackrc"),
    );
}

fn input(
    version_groups: Vec<G3TsSpellingSyncpackVersionGroupSnapshot>,
) -> G3TsSpellingConfigChecksInput {
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
            cspell_config: G3TsSpellingConfigSurfaceState::Parsed {
                rel_path: "cspell.json".to_owned(),
            },
            syncpack_config: G3TsSpellingSyncpackSurfaceState::Parsed {
                snapshot: G3TsSpellingSyncpackSnapshot {
                    rel_path: ".syncpackrc".to_owned(),
                    source: Vec::new(),
                    version_groups,
                },
            },
        }],
    }
}

fn group(
    is_ignored: Option<bool>,
    pin_version: Option<&str>,
) -> G3TsSpellingSyncpackVersionGroupSnapshot {
    G3TsSpellingSyncpackVersionGroupSnapshot {
        dependencies: vec!["cspell".to_owned()],
        dependency_types: Vec::new(),
        packages: None,
        specifier_types: None,
        is_ignored,
        is_banned: None,
        pin_version: pin_version.map(str::to_owned),
    }
}
