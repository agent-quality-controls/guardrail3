use g3ts_jscpd_types::{G3TsJscpdChecksInput, G3TsJscpdRootSnapshot, G3TsJscpdRootState};

pub(super) fn missing_root() -> G3TsJscpdChecksInput {
    G3TsJscpdChecksInput {
        root: G3TsJscpdRootState::Missing,
    }
}

pub(super) fn root_parse_error() -> G3TsJscpdChecksInput {
    G3TsJscpdChecksInput {
        root: G3TsJscpdRootState::ParseError {
            rel_path: ".jscpd.json".to_owned(),
            reason: "synthetic parse failure".to_owned(),
        },
    }
}

pub(super) fn golden_root() -> G3TsJscpdChecksInput {
    G3TsJscpdChecksInput {
        root: G3TsJscpdRootState::Parsed {
            snapshot: G3TsJscpdRootSnapshot {
                rel_path: ".jscpd.json".to_owned(),
                threshold: Some(0),
                min_tokens: Some(50),
                absolute: Some(true),
                format: vec!["typescript".to_owned(), "rust".to_owned()],
                ignore: vec![
                    "**/node_modules/**".to_owned(),
                    "**/.next/**".to_owned(),
                    "**/dist/**".to_owned(),
                    "**/target/**".to_owned(),
                    "**/components/ui/**".to_owned(),
                ],
                extra_keys: Vec::new(),
            },
        },
    }
}

pub(super) fn weak_threshold() -> G3TsJscpdChecksInput {
    let mut snapshot = baseline_snapshot();
    snapshot.threshold = Some(1);
    parsed_snapshot(snapshot)
}

pub(super) fn missing_absolute() -> G3TsJscpdChecksInput {
    parsed_snapshot(G3TsJscpdRootSnapshot {
        rel_path: ".jscpd.json".to_owned(),
        threshold: Some(0),
        min_tokens: Some(50),
        absolute: None,
        format: vec!["typescript".to_owned()],
        ignore: required_ignores(),
        extra_keys: Vec::new(),
    })
}

pub(super) fn missing_ignores() -> G3TsJscpdChecksInput {
    parsed_snapshot(G3TsJscpdRootSnapshot {
        rel_path: ".jscpd.json".to_owned(),
        threshold: Some(0),
        min_tokens: Some(50),
        absolute: Some(true),
        format: vec!["typescript".to_owned()],
        ignore: vec!["**/node_modules/**".to_owned()],
        extra_keys: Vec::new(),
    })
}

pub(super) fn missing_typescript_format() -> G3TsJscpdChecksInput {
    let mut snapshot = baseline_snapshot();
    snapshot.format = vec!["rust".to_owned()];
    parsed_snapshot(snapshot)
}

fn baseline_snapshot() -> G3TsJscpdRootSnapshot {
    G3TsJscpdRootSnapshot {
        rel_path: ".jscpd.json".to_owned(),
        threshold: Some(0),
        min_tokens: Some(50),
        absolute: Some(true),
        format: vec!["typescript".to_owned()],
        ignore: required_ignores(),
        extra_keys: Vec::new(),
    }
}

pub(super) fn extra_inventory_key() -> G3TsJscpdChecksInput {
    parsed_snapshot(G3TsJscpdRootSnapshot {
        rel_path: ".jscpd.json".to_owned(),
        threshold: Some(0),
        min_tokens: Some(50),
        absolute: Some(true),
        format: vec!["typescript".to_owned()],
        ignore: required_ignores(),
        extra_keys: vec!["gitignore".to_owned()],
    })
}

fn parsed_snapshot(snapshot: G3TsJscpdRootSnapshot) -> G3TsJscpdChecksInput {
    G3TsJscpdChecksInput {
        root: G3TsJscpdRootState::Parsed { snapshot },
    }
}

fn required_ignores() -> Vec<String> {
    vec![
        "**/node_modules/**".to_owned(),
        "**/.next/**".to_owned(),
        "**/dist/**".to_owned(),
        "**/target/**".to_owned(),
        "**/components/ui/**".to_owned(),
    ]
}
