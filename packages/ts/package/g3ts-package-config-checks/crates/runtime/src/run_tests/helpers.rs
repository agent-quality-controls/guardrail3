use g3ts_package_types::{
    G3TsPackageChecksInput, G3TsPackageLocalSnapshot, G3TsPackageLocalState,
    G3TsPackageRootSnapshot, G3TsPackageRootState,
};

pub(super) fn missing_root() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::Missing,
        locals: Vec::new(),
    }
}

pub(super) fn local_root_only() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::NotPackageManagerRoot,
        locals: vec![G3TsPackageLocalState::Parsed {
            snapshot: G3TsPackageLocalSnapshot {
                rel_path: "package.json".to_owned(),
                dependencies: vec!["react".to_owned()],
                dev_dependencies: vec!["typescript".to_owned()],
            },
        }],
    }
}

pub(super) fn root_parse_error() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::ParseError {
            rel_path: "package.json".to_owned(),
            reason: "synthetic parse failure".to_owned(),
        },
        locals: Vec::new(),
    }
}

pub(super) fn golden_root() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::Parsed {
            snapshot: G3TsPackageRootSnapshot {
                rel_path: "package.json".to_owned(),
                private_field: Some(true),
                package_manager: Some("pnpm@10.32.0".to_owned()),
                engines_node: Some(">=24".to_owned()),
                engines_pnpm: Some("10".to_owned()),
                preinstall_script: Some("npx only-allow pnpm".to_owned()),
                prepare_script: Some("echo prepare".to_owned()),
                lint_script: Some("eslint .".to_owned()),
                typecheck_script: Some("tsc --noEmit".to_owned()),
                pnpm_override_keys: vec!["@eslint/js".to_owned(), "zod".to_owned()],
                pnpm_only_built_dependencies: vec!["esbuild".to_owned()],
            },
        },
        locals: vec![G3TsPackageLocalState::Parsed {
            snapshot: G3TsPackageLocalSnapshot {
                rel_path: "apps/web/package.json".to_owned(),
                dependencies: vec!["react".to_owned()],
                dev_dependencies: vec!["typescript".to_owned()],
            },
        }],
    }
}

pub(super) fn weak_root() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::Parsed {
            snapshot: G3TsPackageRootSnapshot {
                rel_path: "package.json".to_owned(),
                private_field: Some(false),
                package_manager: Some("pnpm@latest".to_owned()),
                engines_node: Some(">=24".to_owned()),
                engines_pnpm: None,
                preinstall_script: Some("npm install".to_owned()),
                prepare_script: None,
                lint_script: None,
                typecheck_script: Some("tsc --noEmit".to_owned()),
                pnpm_override_keys: Vec::new(),
                pnpm_only_built_dependencies: Vec::new(),
            },
        },
        locals: Vec::new(),
    }
}

pub(super) fn local_banned_and_parse_error() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: golden_root().root,
        locals: vec![
            G3TsPackageLocalState::Parsed {
                snapshot: G3TsPackageLocalSnapshot {
                    rel_path: "apps/web/package.json".to_owned(),
                    dependencies: vec!["axios".to_owned(), "react".to_owned()],
                    dev_dependencies: Vec::new(),
                },
            },
            G3TsPackageLocalState::ParseError {
                rel_path: "apps/landing/package.json".to_owned(),
                reason: "synthetic parse failure".to_owned(),
            },
        ],
    }
}

pub(super) fn local_pg_dependency_allowed() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: golden_root().root,
        locals: vec![G3TsPackageLocalState::Parsed {
            snapshot: G3TsPackageLocalSnapshot {
                rel_path: "apps/web/package.json".to_owned(),
                dependencies: vec!["pg".to_owned(), "react".to_owned()],
                dev_dependencies: vec!["@types/pg".to_owned()],
            },
        }],
    }
}
