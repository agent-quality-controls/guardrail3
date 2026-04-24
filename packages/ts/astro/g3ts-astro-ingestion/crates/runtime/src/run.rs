use eslint_config_parser::{parse_document, parse_error_reason as eslint_parse_error_reason};
use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_types::{
    G3TsAstroAppRootInput, G3TsAstroConfigChecksInput, G3TsAstroContentMode,
    G3TsAstroEslintPluginContractInput, G3TsAstroEslintSurfaceSnapshot,
    G3TsAstroEslintSurfaceState, G3TsAstroFileTreeChecksInput, G3TsAstroIntegrationContractInput,
    G3TsAstroPackageScriptCommand, G3TsAstroPackageScriptCommandSeparator,
    G3TsAstroPackageScriptParseBlocker, G3TsAstroPackageScriptToolInvocation,
    G3TsAstroPackageSurfaceSnapshot, G3TsAstroPackageSurfaceState, G3TsAstroRouteMarkdownPageInput,
    G3TsAstroSyncpackConfigSnapshot, G3TsAstroSyncpackConfigState, G3TsAstroSyncpackRequiredPin,
};
use globset::{Glob, GlobSetBuilder};
use package_json_parser::{from_path_document, parse_error_reason as package_parse_error_reason};
use package_script_command_parser::types::{
    PackageScriptCommand, PackageScriptCommandSeparator, PackageScriptParseFact,
    PackageScriptParseState, PackageScriptToolInvocation,
};
use std::collections::BTreeSet;
use syncpack_config_parser::{
    from_path_document as syncpack_from_path_document,
    parse_error_reason as syncpack_parse_error_reason,
};

const ESLINT_CONFIG_PATTERN: &str = "eslint.config.*";
const PACKAGE_JSON_REL_PATH: &str = "package.json";
const SYNCPACK_CONFIG_REL_PATH: &str = ".syncpackrc";
const ROUTE_SCOPED_PIPELINE_RULES: [&str; 7] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
    "astro-pipeline/no-content-data-modules-in-routes",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/no-side-loader-imports",
    "astro-pipeline/no-velite-imports",
];
const CONTENT_DATA_PIPELINE_RULES: [&str; 1] = ["astro-pipeline/no-content-data-modules-in-routes"];
const CONTENT_SOURCE_PIPELINE_RULES: [&str; 3] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
];
const REQUIRED_SYNCPACK_PINS: [(&str, &str); 20] = [
    ("astro", "6.1.9"),
    ("@astrojs/node", "10.0.6"),
    ("@astrojs/react", "5.0.4"),
    ("@astrojs/mdx", "5.0.4"),
    ("@astrojs/check", "0.9.8"),
    ("react", "19.2.5"),
    ("react-dom", "19.2.5"),
    ("@types/react", "19.2.14"),
    ("@types/react-dom", "19.2.3"),
    ("typescript", "5.9.3"),
    ("eslint-plugin-astro", "1.7.0"),
    ("eslint-plugin-astro-pipeline", "0.1.2"),
    ("tailwindcss", "4.2.4"),
    ("@tailwindcss/postcss", "4.2.4"),
    ("class-variance-authority", "0.7.1"),
    ("clsx", "2.1.1"),
    ("tailwind-merge", "3.5.0"),
    ("lucide-react", "0.577.0"),
    ("zod", "4.3.6"),
    ("@types/node", "25.6.0"),
];
const FORBIDDEN_SYNCPACK_DEPS: [&str; 3] = ["next", "velite", "eslint-mdx"];
const PIN_DEPENDENCY_TYPES: [&str; 2] = ["prod", "dev"];
const BAN_DEPENDENCY_TYPES: [&str; 4] = ["prod", "dev", "optional", "peer"];
const SYNCPACK_ASTRO_POLICY_PREFIX_LEN: usize =
    REQUIRED_SYNCPACK_PINS.len() + FORBIDDEN_SYNCPACK_DEPS.len();

pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroConfigChecksInput {
    let app_roots = astro_app_roots(crawl);

    if app_roots.is_empty() {
        return G3TsAstroConfigChecksInput {
            integration_contracts: Vec::new(),
            eslint_contracts: Vec::new(),
        };
    }

    G3TsAstroConfigChecksInput {
        integration_contracts: app_roots
            .iter()
            .map(|app_root_rel_path| {
                let package = ingest_package_surface(crawl, app_root_rel_path);
                let syncpack_config =
                    ingest_syncpack_config_surface(crawl, app_root_rel_path, &package);
                G3TsAstroIntegrationContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    content_mode: classify_content_mode(crawl, app_root_rel_path),
                    package,
                    syncpack_config,
                    required_syncpack_pins: required_syncpack_pins(),
                    forbidden_syncpack_deps: FORBIDDEN_SYNCPACK_DEPS
                        .into_iter()
                        .map(str::to_owned)
                        .collect(),
                    requires_source_pipeline_linting: true,
                }
            })
            .collect(),
        eslint_contracts: app_roots
            .iter()
            .map(|app_root_rel_path| G3TsAstroEslintPluginContractInput {
                app_root_rel_path: app_root_rel_path.clone(),
                config: ingest_eslint_surface(crawl, app_root_rel_path),
                requires_source_pipeline_linting: true,
            })
            .collect(),
    }
}

pub fn ingest_for_file_tree_checks(crawl: &G3WorkspaceCrawl) -> G3TsAstroFileTreeChecksInput {
    let app_roots = astro_app_roots(crawl);

    if app_roots.is_empty() {
        return G3TsAstroFileTreeChecksInput {
            app_roots: Vec::new(),
            build_collection_roots: Vec::new(),
            live_collection_roots: Vec::new(),
            route_markdown_pages: Vec::new(),
        };
    }

    let roots: Vec<_> = app_roots
        .iter()
        .map(|app_root_rel_path| G3TsAstroAppRootInput {
            app_root_rel_path: app_root_rel_path.clone(),
            astro_config_rel_path: crate::select::select_astro_config(crawl, app_root_rel_path)
                .map(|entry| entry.path.rel_path.clone()),
            content_config_rel_path: crate::select::select_content_config(crawl, app_root_rel_path)
                .map(|entry| entry.path.rel_path.clone()),
            live_config_rel_path: crate::select::select_live_config(crawl, app_root_rel_path)
                .map(|entry| entry.path.rel_path.clone()),
            velite_config_rel_path: crate::select::select_velite_config(crawl, app_root_rel_path)
                .map(|entry| entry.path.rel_path.clone()),
            velite_output_rel_paths: crate::select::velite_output_paths(crawl, app_root_rel_path),
        })
        .collect();

    G3TsAstroFileTreeChecksInput {
        app_roots: roots.clone(),
        build_collection_roots: roots
            .iter()
            .filter(|root| {
                classify_content_mode(crawl, &root.app_root_rel_path)
                    == G3TsAstroContentMode::BuildCollections
            })
            .cloned()
            .collect(),
        live_collection_roots: roots
            .iter()
            .filter(|root| {
                classify_content_mode(crawl, &root.app_root_rel_path)
                    == G3TsAstroContentMode::LiveCollections
            })
            .cloned()
            .collect(),
        route_markdown_pages: app_roots
            .iter()
            .flat_map(|app_root_rel_path| {
                crate::select::route_markdown_pages(crawl, app_root_rel_path)
            })
            .into_iter()
            .map(|rel_path| G3TsAstroRouteMarkdownPageInput { rel_path })
            .collect(),
    }
}

fn astro_app_roots(crawl: &G3WorkspaceCrawl) -> Vec<String> {
    let mut roots: BTreeSet<String> = crate::select::select_astro_app_roots(crawl)
        .into_iter()
        .collect();

    for entry in crawl.entries.iter().filter(|entry| {
        entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
            && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
            && (entry.path.rel_path.ends_with("/package.json")
                || entry.path.rel_path == PACKAGE_JSON_REL_PATH)
    }) {
        let app_root_rel_path = if entry.path.rel_path == PACKAGE_JSON_REL_PATH {
            ".".to_owned()
        } else {
            std::path::Path::new(&entry.path.rel_path)
                .parent()
                .and_then(|parent| parent.to_str())
                .unwrap_or(".")
                .to_owned()
        };

        if package_surface_has_astro_dependency(&ingest_package_surface(crawl, &app_root_rel_path))
        {
            let _ = roots.insert(app_root_rel_path);
        }
    }

    roots.into_iter().collect()
}

fn classify_content_mode(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroContentMode {
    if crate::select::select_live_config(crawl, app_root_rel_path).is_some() {
        G3TsAstroContentMode::LiveCollections
    } else if crate::select::select_content_config(crawl, app_root_rel_path).is_some()
        || crate::select::has_content_files(crawl, app_root_rel_path)
    {
        G3TsAstroContentMode::BuildCollections
    } else {
        G3TsAstroContentMode::None
    }
}

fn ingest_package_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroPackageSurfaceState {
    let Some(entry) = crate::select::select_package_json(crawl, app_root_rel_path) else {
        return G3TsAstroPackageSurfaceState::Missing {
            rel_path: if app_root_rel_path == "." {
                PACKAGE_JSON_REL_PATH.to_owned()
            } else {
                format!("{app_root_rel_path}/{PACKAGE_JSON_REL_PATH}")
            },
        };
    };

    if !entry.readable {
        return G3TsAstroPackageSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the package manifest unreadable".to_owned(),
        };
    }

    let document = match from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsAstroPackageSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    if let Some(reason) = package_parse_error_reason(&document) {
        return G3TsAstroPackageSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let typed = package_json_parser::typed(&document)
        .expect("parsed package.json document should stay typed");
    let script_facts = typed
        .scripts
        .iter()
        .map(|(name, body)| parse_package_script(name, body))
        .collect::<Vec<_>>();
    G3TsAstroPackageSurfaceState::Parsed {
        snapshot: G3TsAstroPackageSurfaceSnapshot {
            rel_path: entry.path.rel_path.clone(),
            package_name: typed.name.clone(),
            dependencies: typed.dependencies.clone(),
            dev_dependencies: typed.dev_dependencies.clone(),
            script_names: typed.scripts.keys().cloned().collect(),
            script_bodies: typed
                .scripts
                .iter()
                .map(|(name, body)| (name.clone(), body.clone()))
                .collect(),
            script_commands: script_facts.iter().flat_map(script_commands).collect(),
            script_tool_invocations: script_facts
                .iter()
                .flat_map(script_tool_invocations)
                .collect(),
            script_parse_blockers: script_facts
                .iter()
                .filter_map(script_parse_blocker)
                .collect(),
            safely_runs_astro_check: package_script_command_parser::has_safe_tool_invocation(
                &script_facts,
                "astro",
                "check",
            ),
            safely_runs_syncpack_lint: package_script_command_parser::has_safe_tool_invocation(
                &script_facts,
                "syncpack",
                "lint",
            ),
        },
    }
}

fn parse_package_script(name: &str, body: &str) -> PackageScriptParseFact {
    package_script_command_parser::parse(name, body)
        .expect("package script command parser should not fail on string input")
}

fn script_commands(fact: &PackageScriptParseFact) -> Vec<G3TsAstroPackageScriptCommand> {
    fact.commands
        .iter()
        .map(|command| script_command(&fact.script_name, command))
        .collect()
}

fn script_command(
    script_name: &str,
    command: &PackageScriptCommand,
) -> G3TsAstroPackageScriptCommand {
    G3TsAstroPackageScriptCommand {
        script_name: script_name.to_owned(),
        invocation: command.invocation.clone(),
        executable: command.executable.clone(),
        args: command.args.clone(),
        preceded_by: command.preceded_by.map(script_command_separator),
    }
}

fn script_command_separator(
    separator: PackageScriptCommandSeparator,
) -> G3TsAstroPackageScriptCommandSeparator {
    match separator {
        PackageScriptCommandSeparator::And => G3TsAstroPackageScriptCommandSeparator::And,
        PackageScriptCommandSeparator::Or => G3TsAstroPackageScriptCommandSeparator::Or,
    }
}

fn script_tool_invocations(
    fact: &PackageScriptParseFact,
) -> Vec<G3TsAstroPackageScriptToolInvocation> {
    fact.tool_invocations
        .iter()
        .map(script_tool_invocation)
        .collect()
}

fn script_tool_invocation(
    invocation: &PackageScriptToolInvocation,
) -> G3TsAstroPackageScriptToolInvocation {
    G3TsAstroPackageScriptToolInvocation {
        script_name: invocation.script_name.clone(),
        command_index: invocation.command_index,
        invocation: invocation.invocation.clone(),
        executable: invocation.executable.clone(),
        args: invocation.args.clone(),
        preceded_by: invocation.preceded_by.map(script_command_separator),
        followed_by: invocation.followed_by.map(script_command_separator),
    }
}

fn script_parse_blocker(
    fact: &PackageScriptParseFact,
) -> Option<G3TsAstroPackageScriptParseBlocker> {
    match &fact.state {
        PackageScriptParseState::Unsupported { reason }
        | PackageScriptParseState::ParseError { reason } => {
            Some(G3TsAstroPackageScriptParseBlocker {
                script_name: fact.script_name.clone(),
                reason: reason.clone(),
            })
        }
        PackageScriptParseState::Parsed { .. } | PackageScriptParseState::NoEslintInvocation => {
            None
        }
    }
}

fn ingest_syncpack_config_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
    package: &G3TsAstroPackageSurfaceState,
) -> G3TsAstroSyncpackConfigState {
    let Some(entry) = select_syncpack_config(crawl, app_root_rel_path) else {
        return G3TsAstroSyncpackConfigState::Missing {
            rel_path: missing_syncpack_config_rel_path(app_root_rel_path),
        };
    };

    if !entry.readable {
        return G3TsAstroSyncpackConfigState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the Syncpack config unreadable".to_owned(),
        };
    }

    let document = match syncpack_from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsAstroSyncpackConfigState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    if let Some(reason) = syncpack_parse_error_reason(&document) {
        return G3TsAstroSyncpackConfigState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let typed = syncpack_config_parser::typed(&document)
        .expect("parsed Syncpack config document should stay typed");
    let package_rel_path = package_rel_path_for_app(app_root_rel_path, package);
    let source_covers_package_manifest =
        syncpack_source_covers_package(&typed.source, &entry.path.rel_path, &package_rel_path);
    let missing_required_stack_pins = REQUIRED_SYNCPACK_PINS
        .iter()
        .filter(|(dependency, version)| {
            !has_canonical_pin_in_prefix(
                &typed.version_groups,
                SYNCPACK_ASTRO_POLICY_PREFIX_LEN,
                dependency,
                version,
                &PIN_DEPENDENCY_TYPES,
            )
        })
        .map(|(dependency, version)| G3TsAstroSyncpackRequiredPin {
            dependency: (*dependency).to_owned(),
            version: (*version).to_owned(),
        })
        .collect();
    let missing_forbidden_bans = FORBIDDEN_SYNCPACK_DEPS
        .iter()
        .filter(|dependency| {
            !has_canonical_ban_in_prefix(
                &typed.version_groups,
                SYNCPACK_ASTRO_POLICY_PREFIX_LEN,
                dependency,
                &BAN_DEPENDENCY_TYPES,
            )
        })
        .map(|dependency| (*dependency).to_owned())
        .collect();
    G3TsAstroSyncpackConfigState::Parsed {
        snapshot: G3TsAstroSyncpackConfigSnapshot {
            rel_path: entry.path.rel_path.clone(),
            source_covers_package_manifest,
            missing_required_stack_pins,
            missing_forbidden_bans,
        },
    }
}

fn package_rel_path_for_app(
    app_root_rel_path: &str,
    package: &G3TsAstroPackageSurfaceState,
) -> String {
    let rel_path = match package {
        G3TsAstroPackageSurfaceState::Missing { rel_path }
        | G3TsAstroPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroPackageSurfaceState::ParseError { rel_path, .. } => rel_path.clone(),
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => snapshot.rel_path.clone(),
    };

    if rel_path.is_empty() {
        scoped_rel_path(app_root_rel_path, PACKAGE_JSON_REL_PATH)
    } else {
        rel_path
    }
}

fn required_syncpack_pins() -> Vec<G3TsAstroSyncpackRequiredPin> {
    REQUIRED_SYNCPACK_PINS
        .into_iter()
        .map(|(dependency, version)| G3TsAstroSyncpackRequiredPin {
            dependency: dependency.to_owned(),
            version: version.to_owned(),
        })
        .collect()
}

fn syncpack_source_covers_package(
    source: &[String],
    syncpack_rel_path: &str,
    package_rel_path: &str,
) -> bool {
    !source.is_empty()
        && source.iter().any(|entry| {
            exact_source_entry_matches_package(entry, syncpack_rel_path, package_rel_path)
        })
}

fn has_canonical_pin_in_prefix(
    version_groups: &[syncpack_config_parser::types::SyncpackVersionGroup],
    prefix_len: usize,
    dependency: &str,
    version: &str,
    dependency_types: &[&str],
) -> bool {
    version_groups
        .iter()
        .take(prefix_len)
        .find(|group| group_targets_dependency(group, dependency, dependency_types))
        .is_some_and(|group| canonical_pin_group(group, version))
}

fn has_canonical_ban_in_prefix(
    version_groups: &[syncpack_config_parser::types::SyncpackVersionGroup],
    prefix_len: usize,
    dependency: &str,
    dependency_types: &[&str],
) -> bool {
    version_groups
        .iter()
        .take(prefix_len)
        .find(|group| group_targets_dependency(group, dependency, dependency_types))
        .is_some_and(canonical_ban_group)
}

fn group_targets_dependency(
    group: &syncpack_config_parser::types::SyncpackVersionGroup,
    dependency: &str,
    dependency_types: &[&str],
) -> bool {
    strings_match_exactly(&group.dependencies, &[dependency])
        && strings_match_exactly(&group.dependency_types, dependency_types)
}

fn canonical_pin_group(
    group: &syncpack_config_parser::types::SyncpackVersionGroup,
    version: &str,
) -> bool {
    group.packages.is_empty()
        && group.specifier_types.is_empty()
        && !group.is_ignored
        && !group.is_banned
        && group.pin_version.as_deref() == Some(version)
}

fn canonical_ban_group(group: &syncpack_config_parser::types::SyncpackVersionGroup) -> bool {
    group.packages.is_empty()
        && group.specifier_types.is_empty()
        && !group.is_ignored
        && group.is_banned
        && group.pin_version.is_none()
}

fn strings_match_exactly(left: &[String], right: &[&str]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .map(String::as_str)
            .zip(right.iter().copied())
            .all(|(left, right)| left == right)
}

fn exact_source_entry_matches_package(
    source_entry: &str,
    syncpack_rel_path: &str,
    package_rel_path: &str,
) -> bool {
    let config_dir = rel_parent(syncpack_rel_path);
    let expected_source_entry = if config_dir.is_empty() {
        package_rel_path
    } else {
        let config_prefix = format!("{config_dir}/");
        let Some(app_local_package_rel_path) = package_rel_path.strip_prefix(&config_prefix) else {
            return false;
        };
        app_local_package_rel_path
    };

    source_entry == expected_source_entry
}

fn rel_parent(rel_path: &str) -> String {
    rel_path
        .rsplit_once('/')
        .map_or_else(String::new, |(parent, _)| parent.to_owned())
}

fn select_syncpack_config<'crawl>(
    crawl: &'crawl G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'crawl g3_workspace_crawl::G3RsWorkspaceEntry> {
    let app_config = scoped_rel_path(app_root_rel_path, SYNCPACK_CONFIG_REL_PATH);
    exact_included_file(crawl, &app_config)
        .or_else(|| exact_included_file(crawl, SYNCPACK_CONFIG_REL_PATH))
}

fn exact_included_file<'crawl>(
    crawl: &'crawl G3WorkspaceCrawl,
    rel_path: &str,
) -> Option<&'crawl g3_workspace_crawl::G3RsWorkspaceEntry> {
    crawl.entries.iter().find(|entry| {
        entry.kind == g3_workspace_crawl::G3RsWorkspaceEntryKind::File
            && entry.ignore_state == g3_workspace_crawl::G3RsWorkspaceIgnoreState::Included
            && entry.path.rel_path == rel_path
    })
}

fn missing_syncpack_config_rel_path(app_root_rel_path: &str) -> String {
    if app_root_rel_path == "." {
        SYNCPACK_CONFIG_REL_PATH.to_owned()
    } else {
        format!("{app_root_rel_path}/{SYNCPACK_CONFIG_REL_PATH}")
    }
}

fn scoped_rel_path(app_root_rel_path: &str, rel_path: &str) -> String {
    if app_root_rel_path == "." {
        rel_path.to_owned()
    } else {
        format!("{app_root_rel_path}/{rel_path}")
    }
}

fn ingest_eslint_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroEslintSurfaceState {
    let Some(entry) = crate::select::select_active_eslint_config(crawl, app_root_rel_path) else {
        return G3TsAstroEslintSurfaceState::Missing {
            rel_path: if app_root_rel_path == "." {
                ESLINT_CONFIG_PATTERN.to_owned()
            } else {
                format!("{app_root_rel_path}/{ESLINT_CONFIG_PATTERN}")
            },
        };
    };

    if !entry.readable {
        return G3TsAstroEslintSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the selected eslint config unreadable".to_owned(),
        };
    }

    let probes = crate::select::probe_targets(crawl, &entry.path.rel_path);
    let document = match parse_document(&crawl.root_abs_path, &entry.path.rel_path, &probes) {
        Ok(document) => document,
        Err(error) => {
            return G3TsAstroEslintSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    if let Some(reason) = eslint_parse_error_reason(&document) {
        return G3TsAstroEslintSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let typed = eslint_config_parser::typed(&document)
        .expect("parsed eslint config document should stay typed");
    let route_page_paths = crate::select::route_page_paths(crawl, app_root_rel_path);
    let endpoint_paths = crate::select::endpoint_paths(crawl, app_root_rel_path);
    let astro_source_probe = active_probe(
        &typed,
        eslint_config_parser::types::EslintProbeKind::AstroSource,
    );
    let ts_source_probe = active_probe(
        &typed,
        eslint_config_parser::types::EslintProbeKind::TsSource,
    );
    let tsx_source_probe = active_probe(
        &typed,
        eslint_config_parser::types::EslintProbeKind::TsxSource,
    );

    G3TsAstroEslintSurfaceState::Parsed {
        snapshot: G3TsAstroEslintSurfaceSnapshot {
            rel_path: entry.path.rel_path.clone(),
            astro_source_probe_present: astro_source_probe.is_some(),
            ts_source_probe_present: ts_source_probe.is_some(),
            tsx_source_probe_present: tsx_source_probe.is_some(),
            astro_source_plugins: astro_source_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            ts_source_plugins: ts_source_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            tsx_source_plugins: tsx_source_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            astro_source_error_rules: active_error_rules(astro_source_probe),
            ts_source_error_rules: active_error_rules(ts_source_probe),
            tsx_source_error_rules: active_error_rules(tsx_source_probe),
            astro_source_effective_route_scoped_pipeline_rules:
                effective_route_scoped_pipeline_rules(
                    astro_source_probe,
                    &route_page_paths,
                    &endpoint_paths,
                ),
            ts_source_effective_route_scoped_pipeline_rules: effective_route_scoped_pipeline_rules(
                ts_source_probe,
                &route_page_paths,
                &endpoint_paths,
            ),
            tsx_source_effective_route_scoped_pipeline_rules: effective_route_scoped_pipeline_rules(
                tsx_source_probe,
                &route_page_paths,
                &endpoint_paths,
            ),
            astro_source_effective_content_data_pipeline_rules:
                effective_content_data_pipeline_rules(
                    astro_source_probe,
                    &route_page_paths,
                    &endpoint_paths,
                ),
            ts_source_effective_content_data_pipeline_rules: effective_content_data_pipeline_rules(
                ts_source_probe,
                &route_page_paths,
                &endpoint_paths,
            ),
            tsx_source_effective_content_data_pipeline_rules: effective_content_data_pipeline_rules(
                tsx_source_probe,
                &route_page_paths,
                &endpoint_paths,
            ),
            astro_source_effective_content_source_pipeline_rules:
                effective_content_source_pipeline_rules(
                    astro_source_probe,
                    &route_page_paths,
                    &endpoint_paths,
                ),
            ts_source_effective_content_source_pipeline_rules:
                effective_content_source_pipeline_rules(
                    ts_source_probe,
                    &route_page_paths,
                    &endpoint_paths,
                ),
            tsx_source_effective_content_source_pipeline_rules:
                effective_content_source_pipeline_rules(
                    tsx_source_probe,
                    &route_page_paths,
                    &endpoint_paths,
                ),
        },
    }
}

fn active_probe<'a>(
    typed: &'a eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&'a eslint_config_parser::types::EslintEffectiveConfigProbe> {
    typed
        .probes
        .iter()
        .find(|probe| probe.probe == kind && !probe.ignored)
}

fn active_error_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    probe
        .map(|probe| {
            probe
                .rules
                .iter()
                .filter_map(|(rule_name, setting)| {
                    (setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error)
                        .then_some(rule_name.clone())
                })
                .collect()
        })
        .unwrap_or_default()
}

fn effective_route_scoped_pipeline_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    route_page_paths: &[String],
    endpoint_paths: &[String],
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    ROUTE_SCOPED_PIPELINE_RULES
        .iter()
        .filter(|rule_name| {
            probe.rules.get(**rule_name).is_some_and(|setting| {
                rule_setting_has_route_and_endpoint_coverage(
                    setting,
                    route_page_paths,
                    endpoint_paths,
                )
            })
        })
        .map(|rule_name| (*rule_name).to_owned())
        .collect()
}

fn effective_content_data_pipeline_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    route_page_paths: &[String],
    endpoint_paths: &[String],
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    CONTENT_DATA_PIPELINE_RULES
        .iter()
        .filter(|rule_name| {
            probe.rules.get(**rule_name).is_some_and(|setting| {
                rule_setting_has_route_and_endpoint_coverage(
                    setting,
                    route_page_paths,
                    endpoint_paths,
                ) && rule_setting_has_content_data_scope(setting)
            })
        })
        .map(|rule_name| (*rule_name).to_owned())
        .collect()
}

fn effective_content_source_pipeline_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
    route_page_paths: &[String],
    endpoint_paths: &[String],
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    CONTENT_SOURCE_PIPELINE_RULES
        .iter()
        .filter(|rule_name| {
            probe.rules.get(**rule_name).is_some_and(|setting| {
                rule_setting_has_route_and_endpoint_coverage(
                    setting,
                    route_page_paths,
                    endpoint_paths,
                ) && rule_setting_has_content_source_scope(setting)
            })
        })
        .map(|rule_name| (*rule_name).to_owned())
        .collect()
}

fn rule_setting_has_route_and_endpoint_coverage(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    route_page_paths: &[String],
    endpoint_paths: &[String],
) -> bool {
    let route_coverage = route_page_paths.is_empty()
        || rule_setting_option_globs_match_any_path(setting, "routeGlobs", route_page_paths);
    let endpoint_coverage = endpoint_paths.is_empty()
        || rule_setting_option_globs_match_any_path(setting, "endpointGlobs", endpoint_paths);

    route_coverage && endpoint_coverage
}

fn rule_setting_has_content_data_scope(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    setting.options.iter().any(|option| {
        option.as_object().is_some_and(|object| {
            has_non_empty_string_array_option(object.get("contentDataModuleGlobs"))
        })
    })
}

fn rule_setting_has_content_source_scope(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    setting.options.iter().any(|option| {
        option.as_object().is_some_and(|object| {
            has_non_empty_string_array_option(object.get("authoredContentGlobs"))
                || has_non_empty_string_array_option(object.get("specContentGlobs"))
        })
    })
}

fn has_non_empty_string_array_option(option: Option<&serde_json::Value>) -> bool {
    option
        .and_then(serde_json::Value::as_array)
        .is_some_and(|values| {
            !values.is_empty()
                && values
                    .iter()
                    .all(|value| value.as_str().is_some_and(|text| !text.trim().is_empty()))
        })
}

fn rule_setting_option_globs_match_any_path(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    option_name: &str,
    candidate_paths: &[String],
) -> bool {
    setting.options.iter().any(|option| {
        option.as_object().is_some_and(|object| {
            non_empty_string_array_option(object.get(option_name))
                .is_some_and(|patterns| all_paths_match_globs(&patterns, candidate_paths))
        })
    })
}

fn non_empty_string_array_option(option: Option<&serde_json::Value>) -> Option<Vec<&str>> {
    let values = option.and_then(serde_json::Value::as_array)?;

    if values.is_empty() {
        return None;
    }

    let mut strings = Vec::with_capacity(values.len());

    for value in values {
        let text = value.as_str()?.trim();
        if text.is_empty() {
            return None;
        }
        strings.push(text);
    }

    Some(strings)
}

fn all_paths_match_globs(patterns: &[&str], candidate_paths: &[String]) -> bool {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        let Ok(glob) = Glob::new(&normalize_glob(pattern)) else {
            return false;
        };
        let _ = builder.add(glob);
    }

    let Ok(glob_set) = builder.build() else {
        return false;
    };

    candidate_paths
        .iter()
        .all(|candidate_path| glob_set.is_match(normalize_glob(candidate_path)))
}

fn normalize_glob(value: &str) -> String {
    let mut normalized = value.replace('\\', "/");
    while normalized.contains("//") {
        normalized = normalized.replace("//", "/");
    }
    normalized.trim_start_matches("./").to_owned()
}

fn package_surface_has_astro_dependency(package: &G3TsAstroPackageSurfaceState) -> bool {
    match package {
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => snapshot
            .dependencies
            .iter()
            .chain(snapshot.dev_dependencies.iter())
            .any(|dependency| dependency == "astro"),
        G3TsAstroPackageSurfaceState::Missing { .. }
        | G3TsAstroPackageSurfaceState::Unreadable { .. }
        | G3TsAstroPackageSurfaceState::ParseError { .. } => false,
    }
}

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
