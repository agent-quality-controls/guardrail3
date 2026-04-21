use g3ts_npmrc_types::{
    G3TsNpmrcChecksInput, G3TsNpmrcRootSnapshot, G3TsNpmrcRootState, G3TsNpmrcSetting,
};

pub(super) fn missing_root() -> G3TsNpmrcChecksInput {
    G3TsNpmrcChecksInput {
        root: G3TsNpmrcRootState::Missing,
    }
}

pub(super) fn not_package_manager_root() -> G3TsNpmrcChecksInput {
    G3TsNpmrcChecksInput {
        root: G3TsNpmrcRootState::NotPackageManagerRoot,
    }
}

pub(super) fn root_parse_error() -> G3TsNpmrcChecksInput {
    G3TsNpmrcChecksInput {
        root: G3TsNpmrcRootState::ParseError {
            rel_path: ".npmrc".to_owned(),
            reason: "synthetic parse failure".to_owned(),
        },
    }
}

pub(super) fn golden_root() -> G3TsNpmrcChecksInput {
    G3TsNpmrcChecksInput {
        root: G3TsNpmrcRootState::Parsed {
            snapshot: G3TsNpmrcRootSnapshot {
                rel_path: ".npmrc".to_owned(),
                settings: vec![
                    setting("strict-peer-dependencies", "true"),
                    setting("disallow-workspace-cycles", "true"),
                    setting("engine-strict", "true"),
                    setting("minimum-release-age", "1440"),
                    setting("block-exotic-subdeps", "true"),
                    setting("trust-policy", "warn"),
                    setting("minimum-release-age-exclude", "@base-ui/react"),
                ],
                duplicate_keys: Vec::new(),
            },
        },
    }
}

pub(super) fn root_with_duplicate_keys() -> G3TsNpmrcChecksInput {
    G3TsNpmrcChecksInput {
        root: G3TsNpmrcRootState::Parsed {
            snapshot: G3TsNpmrcRootSnapshot {
                rel_path: ".npmrc".to_owned(),
                settings: vec![
                    setting("strict-peer-dependencies", "true"),
                    setting("strict-peer-dependencies", "false"),
                    setting("disallow-workspace-cycles", "true"),
                    setting("engine-strict", "true"),
                    setting("minimum-release-age", "1440"),
                    setting("block-exotic-subdeps", "true"),
                    setting("trust-policy", "warn"),
                ],
                duplicate_keys: vec!["strict-peer-dependencies".to_owned()],
            },
        },
    }
}

pub(super) fn root_missing_required_settings() -> G3TsNpmrcChecksInput {
    G3TsNpmrcChecksInput {
        root: G3TsNpmrcRootState::Parsed {
            snapshot: G3TsNpmrcRootSnapshot {
                rel_path: ".npmrc".to_owned(),
                settings: vec![
                    setting("strict-peer-dependencies", "true"),
                    setting("engine-strict", "true"),
                ],
                duplicate_keys: Vec::new(),
            },
        },
    }
}

pub(super) fn root_with_weakened_values() -> G3TsNpmrcChecksInput {
    G3TsNpmrcChecksInput {
        root: G3TsNpmrcRootState::Parsed {
            snapshot: G3TsNpmrcRootSnapshot {
                rel_path: ".npmrc".to_owned(),
                settings: vec![
                    setting("strict-peer-dependencies", "false"),
                    setting("disallow-workspace-cycles", "true"),
                    setting("engine-strict", "false"),
                    setting("minimum-release-age", "60"),
                    setting("block-exotic-subdeps", "false"),
                    setting("trust-policy", "warn"),
                ],
                duplicate_keys: Vec::new(),
            },
        },
    }
}

fn setting(key: &str, value: &str) -> G3TsNpmrcSetting {
    G3TsNpmrcSetting {
        key: key.to_owned(),
        value: value.to_owned(),
    }
}
