use g3ts_package_types::{
    G3TsPackageChecksInput, G3TsPackageLocalSnapshot, G3TsPackageLocalState,
    G3TsPackageRootSnapshot, G3TsPackageRootState, G3TsPackageScriptCommandSeparator,
    G3TsPackageScriptToolInvocation, G3TsPackageSyncpackConfigSnapshot,
    G3TsPackageSyncpackConfigState,
};

pub(super) fn missing_root() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::Missing,
        locals: Vec::new(),
        syncpack_config: G3TsPackageSyncpackConfigState::Missing {
            rel_path: ".syncpackrc".to_owned(),
        },
        forbidden_syncpack_deps: forbidden_syncpack_deps(),
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
        syncpack_config: G3TsPackageSyncpackConfigState::NotRequired,
        forbidden_syncpack_deps: forbidden_syncpack_deps(),
    }
}

pub(super) fn root_parse_error() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::ParseError {
            rel_path: "package.json".to_owned(),
            reason: "synthetic parse failure".to_owned(),
        },
        locals: Vec::new(),
        syncpack_config: G3TsPackageSyncpackConfigState::Missing {
            rel_path: ".syncpackrc".to_owned(),
        },
        forbidden_syncpack_deps: forbidden_syncpack_deps(),
    }
}

pub(super) fn golden_root() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::Parsed {
            snapshot: root_snapshot(true, true, true, true),
        },
        locals: vec![web_local(&["react"], &["typescript"])],
        syncpack_config: syncpack_config(Vec::new(), Vec::new()),
        forbidden_syncpack_deps: forbidden_syncpack_deps(),
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
                validate_script: None,
                dependencies: Vec::new(),
                dev_dependencies: Vec::new(),
                pnpm_override_keys: Vec::new(),
                pnpm_only_built_dependencies: Vec::new(),
                script_commands: Vec::new(),
                script_tool_invocations: Vec::new(),
                script_parse_blockers: Vec::new(),
                safely_runs_only_allow_pnpm: false,
                safely_runs_syncpack_lint: false,
            },
        },
        locals: Vec::new(),
        syncpack_config: G3TsPackageSyncpackConfigState::Missing {
            rel_path: ".syncpackrc".to_owned(),
        },
        forbidden_syncpack_deps: forbidden_syncpack_deps(),
    }
}

pub(super) fn fail_open_syncpack_script() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::Parsed {
            snapshot: root_snapshot(true, true, true, false),
        },
        locals: vec![web_local(&["axios", "react"], &[])],
        syncpack_config: syncpack_config(Vec::new(), Vec::new()),
        forbidden_syncpack_deps: forbidden_syncpack_deps(),
    }
}

pub(super) fn missing_syncpack_config() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::Parsed {
            snapshot: root_snapshot(true, true, true, true),
        },
        locals: vec![web_local(&["axios", "react"], &[])],
        syncpack_config: G3TsPackageSyncpackConfigState::Missing {
            rel_path: ".syncpackrc".to_owned(),
        },
        forbidden_syncpack_deps: forbidden_syncpack_deps(),
    }
}

pub(super) fn missing_syncpack_source_and_bans() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::Parsed {
            snapshot: root_snapshot(true, true, true, true),
        },
        locals: vec![web_local(&["axios", "react"], &[])],
        syncpack_config: syncpack_config(
            vec!["apps/web/package.json".to_owned()],
            vec!["axios".to_owned()],
        ),
        forbidden_syncpack_deps: forbidden_syncpack_deps(),
    }
}

pub(super) fn fake_only_allow_preinstall() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::Parsed {
            snapshot: root_snapshot(true, false, true, true),
        },
        locals: vec![web_local(&["react"], &["typescript"])],
        syncpack_config: syncpack_config(Vec::new(), Vec::new()),
        forbidden_syncpack_deps: forbidden_syncpack_deps(),
    }
}

pub(super) fn local_pg_dependency_allowed() -> G3TsPackageChecksInput {
    G3TsPackageChecksInput {
        root: G3TsPackageRootState::Parsed {
            snapshot: root_snapshot(true, true, true, true),
        },
        locals: vec![web_local(&["pg", "react"], &["@types/pg"])],
        syncpack_config: syncpack_config(Vec::new(), Vec::new()),
        forbidden_syncpack_deps: forbidden_syncpack_deps(),
    }
}

#[allow(
    clippy::fn_params_excessive_bools,
    reason = "test helper takes four orthogonal toggles to construct fixture variants; collapsing into a struct only displaces the bools"
)]
fn root_snapshot(
    has_syncpack_dependency: bool,
    safely_runs_only_allow_pnpm: bool,
    invokes_syncpack_lint: bool,
    safely_runs_syncpack_lint: bool,
) -> G3TsPackageRootSnapshot {
    let mut dev_dependencies = vec!["typescript".to_owned()];
    if has_syncpack_dependency {
        dev_dependencies.push("syncpack".to_owned());
    }

    G3TsPackageRootSnapshot {
        rel_path: "package.json".to_owned(),
        private_field: Some(true),
        package_manager: Some("pnpm@10.32.0".to_owned()),
        engines_node: Some(">=24".to_owned()),
        engines_pnpm: Some("10".to_owned()),
        preinstall_script: Some(if safely_runs_only_allow_pnpm {
            "npx only-allow pnpm".to_owned()
        } else {
            "echo only-allow pnpm".to_owned()
        }),
        prepare_script: Some("echo prepare".to_owned()),
        lint_script: Some("eslint .".to_owned()),
        typecheck_script: Some("tsc --noEmit".to_owned()),
        validate_script: Some("pnpm lint && pnpm typecheck".to_owned()),
        dependencies: Vec::new(),
        dev_dependencies,
        pnpm_override_keys: vec!["@eslint/js".to_owned(), "zod".to_owned()],
        pnpm_only_built_dependencies: vec!["esbuild".to_owned()],
        script_commands: Vec::new(),
        script_tool_invocations: {
            let mut invocations = vec![
                G3TsPackageScriptToolInvocation {
                    script_name: "validate".to_owned(),
                    command_index: 0,
                    invocation: "pnpm lint".to_owned(),
                    executable: "package-script".to_owned(),
                    args: vec!["lint".to_owned()],
                    preceded_by: None,
                    followed_by: Some(G3TsPackageScriptCommandSeparator::And),
                },
                G3TsPackageScriptToolInvocation {
                    script_name: "validate".to_owned(),
                    command_index: 1,
                    invocation: "pnpm typecheck".to_owned(),
                    executable: "package-script".to_owned(),
                    args: vec!["typecheck".to_owned()],
                    preceded_by: Some(G3TsPackageScriptCommandSeparator::And),
                    followed_by: None,
                },
            ];
            if invokes_syncpack_lint {
                invocations.push(G3TsPackageScriptToolInvocation {
                    script_name: "check".to_owned(),
                    command_index: 0,
                    invocation: if safely_runs_syncpack_lint {
                        "syncpack lint".to_owned()
                    } else {
                        "syncpack lint || true".to_owned()
                    },
                    executable: "syncpack".to_owned(),
                    args: vec!["lint".to_owned()],
                    preceded_by: None,
                    followed_by: if safely_runs_syncpack_lint {
                        None
                    } else {
                        Some(G3TsPackageScriptCommandSeparator::Or)
                    },
                });
            }
            invocations
        },
        script_parse_blockers: Vec::new(),
        safely_runs_only_allow_pnpm,
        safely_runs_syncpack_lint,
    }
}

fn web_local(dependencies: &[&str], dev_dependencies: &[&str]) -> G3TsPackageLocalState {
    G3TsPackageLocalState::Parsed {
        snapshot: G3TsPackageLocalSnapshot {
            rel_path: "apps/web/package.json".to_owned(),
            dependencies: dependencies
                .iter()
                .map(|value| (*value).to_owned())
                .collect(),
            dev_dependencies: dev_dependencies
                .iter()
                .map(|value| (*value).to_owned())
                .collect(),
        },
    }
}

fn syncpack_config(
    missing_source_entries: Vec<String>,
    missing_forbidden_bans: Vec<String>,
) -> G3TsPackageSyncpackConfigState {
    G3TsPackageSyncpackConfigState::Parsed {
        snapshot: G3TsPackageSyncpackConfigSnapshot {
            rel_path: ".syncpackrc".to_owned(),
            missing_source_entries,
            missing_forbidden_bans,
        },
    }
}

fn forbidden_syncpack_deps() -> Vec<String> {
    vec!["axios".to_owned()]
}
