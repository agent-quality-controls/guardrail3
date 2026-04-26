use astro_config_parser::{
    parse_document as parse_astro_config_document,
    parse_error_reason as astro_config_parse_error_reason,
};
use eslint_config_parser::{
    parse_document as parse_eslint_document, parse_error_reason as eslint_parse_error_reason,
};
use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_types::{
    G3TsAstroAppRootInput, G3TsAstroCallSnapshot, G3TsAstroConfigChecksInput,
    G3TsAstroConfigSurfaceSnapshot, G3TsAstroConfigSurfaceState, G3TsAstroContentMode,
    G3TsAstroEslintPluginContractInput, G3TsAstroEslintSurfaceSnapshot,
    G3TsAstroEslintSurfaceState, G3TsAstroFileTreeChecksInput, G3TsAstroIntegrationContractInput,
    G3TsAstroIntegrationSnapshot, G3TsAstroOutputMode, G3TsAstroPackageScriptCommand,
    G3TsAstroPackageScriptCommandSeparator, G3TsAstroPackageScriptParseBlocker,
    G3TsAstroPackageScriptToolInvocation, G3TsAstroPackageSurfaceSnapshot,
    G3TsAstroPackageSurfaceState, G3TsAstroRouteMarkdownPageInput, G3TsAstroStaticObjectProperty,
    G3TsAstroStaticValue, G3TsAstroSyncpackConfigSnapshot, G3TsAstroSyncpackConfigState,
    G3TsAstroSyncpackRequiredPin,
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
const ROUTE_SCOPED_PIPELINE_RULES: [&str; 8] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
    "astro-pipeline/no-content-data-modules-in-routes",
    "astro-pipeline/no-direct-astro-content-in-routes",
    "astro-pipeline/require-approved-content-adapter-in-routes",
    "astro-pipeline/no-side-loader-imports",
    "astro-pipeline/no-velite-imports",
];
const CONTENT_DATA_PIPELINE_RULES: [&str; 1] = ["astro-pipeline/no-content-data-modules-in-routes"];
const CONTENT_SOURCE_PIPELINE_RULES: [&str; 3] = [
    "astro-pipeline/no-authored-content-fs-read",
    "astro-pipeline/no-authored-content-glob",
    "astro-pipeline/no-authored-content-imports",
];
const INLINE_PUBLIC_CONTENT_RULE: &str = "i18next/no-literal-string";
const INLINE_PUBLIC_CONTENT_MESSAGE: &str = "Inline public copy must live in Astro content entries. Move this text into the content collection, validate it through the collection schema, and pass the typed value into source.";
const CONTENT_ADAPTER_PIPELINE_RULE: &str =
    "astro-pipeline/require-approved-content-adapter-in-routes";
const REQUIRED_SYNCPACK_PINS: [(&str, &str); 18] = [
    ("astro", "6.1.9"),
    ("@astrojs/react", "5.0.4"),
    ("@astrojs/mdx", "5.0.4"),
    ("@astrojs/check", "0.9.8"),
    ("@astrojs/sitemap", "3.7.2"),
    ("astro-robots", "2.3.1"),
    ("@nuasite/checks", "0.18.0"),
    ("g3ts-astro-nuasite-checks", "0.1.0"),
    ("schema-dts", "2.0.0"),
    ("react", "19.2.5"),
    ("react-dom", "19.2.5"),
    ("@types/react", "19.2.14"),
    ("@types/react-dom", "19.2.3"),
    ("typescript", "5.9.3"),
    ("eslint-plugin-astro", "1.7.0"),
    ("g3ts-eslint-plugin-astro-pipeline", "0.1.5"),
    ("eslint-plugin-i18next", "6.1.4"),
    ("eslint-plugin-mdx", "3.7.0"),
];
const FORBIDDEN_SYNCPACK_DEPS: [&str; 7] = [
    "next",
    "velite",
    "@astrojs/node",
    "eslint-plugin-astro-pipeline",
    "@codemint/astro-meta",
    "astro-seo-meta",
    "astro-seo-schema",
];
const PIN_DEPENDENCY_TYPES: [&str; 2] = ["prod", "dev"];
const BAN_DEPENDENCY_TYPES: [&str; 4] = ["prod", "dev", "optional", "peer"];
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
                let astro_config = ingest_astro_config_surface(crawl, app_root_rel_path);
                G3TsAstroIntegrationContractInput {
                    app_root_rel_path: app_root_rel_path.clone(),
                    content_mode: classify_content_mode(crawl, app_root_rel_path),
                    package,
                    syncpack_config,
                    astro_config,
                    llms_txt_rel_path: select_llms_txt(crawl, app_root_rel_path),
                    required_syncpack_pins: required_syncpack_pins(),
                    forbidden_syncpack_deps: FORBIDDEN_SYNCPACK_DEPS
                        .into_iter()
                        .map(str::to_owned)
                        .collect(),
                }
            })
            .collect(),
        eslint_contracts: app_roots
            .iter()
            .map(|app_root_rel_path| G3TsAstroEslintPluginContractInput {
                app_root_rel_path: app_root_rel_path.clone(),
                config: ingest_eslint_surface(crawl, app_root_rel_path),
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
            safely_runs_astro_build: has_safe_tool_invocation_in_script(
                &script_facts,
                "build",
                "astro",
                "build",
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

fn has_safe_tool_invocation_in_script(
    facts: &[PackageScriptParseFact],
    script_name: &str,
    executable: &str,
    first_arg: &str,
) -> bool {
    let scoped_facts = facts
        .iter()
        .filter(|fact| fact.script_name == script_name)
        .cloned()
        .collect::<Vec<_>>();

    !scoped_facts.is_empty()
        && package_script_command_parser::has_safe_tool_invocation(
            &scoped_facts,
            executable,
            first_arg,
        )
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

fn ingest_astro_config_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroConfigSurfaceState {
    let Some(entry) = crate::select::select_astro_config(crawl, app_root_rel_path) else {
        return G3TsAstroConfigSurfaceState::Missing {
            rel_path: if app_root_rel_path == "." {
                "astro.config.*".to_owned()
            } else {
                format!("{app_root_rel_path}/astro.config.*")
            },
        };
    };

    if !entry.readable {
        return G3TsAstroConfigSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the selected Astro config unreadable".to_owned(),
        };
    }

    let document = match parse_astro_config_document(&crawl.root_abs_path, &entry.path.rel_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsAstroConfigSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    if let Some(reason) = astro_config_parse_error_reason(&document) {
        return G3TsAstroConfigSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let typed = astro_config_parser::typed(&document)
        .expect("parsed Astro config document should stay typed");
    G3TsAstroConfigSurfaceState::Parsed {
        snapshot: G3TsAstroConfigSurfaceSnapshot {
            rel_path: typed.selected_config.rel_path.clone(),
            site: typed.site.clone(),
            output: typed.output.map(astro_output_mode),
            integrations: typed.integrations.iter().map(astro_integration).collect(),
            adapter: typed.adapter.as_ref().map(astro_adapter_as_integration),
        },
    }
}

fn astro_output_mode(value: astro_config_parser::types::AstroOutputMode) -> G3TsAstroOutputMode {
    match value {
        astro_config_parser::types::AstroOutputMode::Static => G3TsAstroOutputMode::Static,
        astro_config_parser::types::AstroOutputMode::Server => G3TsAstroOutputMode::Server,
    }
}

fn astro_integration(
    value: &astro_config_parser::types::AstroIntegrationSnapshot,
) -> G3TsAstroIntegrationSnapshot {
    G3TsAstroIntegrationSnapshot {
        source_module: value.source_module.clone(),
        name: value.name.clone(),
        imported_name: value.imported_name.clone(),
        call: value.call.as_ref().map(astro_call),
    }
}

fn astro_adapter_as_integration(
    value: &astro_config_parser::types::AstroAdapterSnapshot,
) -> G3TsAstroIntegrationSnapshot {
    G3TsAstroIntegrationSnapshot {
        source_module: value.source_module.clone(),
        name: value.name.clone(),
        imported_name: value.imported_name.clone(),
        call: value.call.as_ref().map(astro_call),
    }
}

fn astro_call(value: &astro_config_parser::types::AstroCallSnapshot) -> G3TsAstroCallSnapshot {
    G3TsAstroCallSnapshot {
        first_arg: value.first_arg.as_ref().map(astro_static_value),
    }
}

fn astro_static_value(
    value: &astro_config_parser::types::AstroStaticValue,
) -> G3TsAstroStaticValue {
    match value {
        astro_config_parser::types::AstroStaticValue::Bool(value) => {
            G3TsAstroStaticValue::Bool(*value)
        }
        astro_config_parser::types::AstroStaticValue::Number(value) => {
            G3TsAstroStaticValue::Number(*value)
        }
        astro_config_parser::types::AstroStaticValue::String(value) => {
            G3TsAstroStaticValue::String(value.clone())
        }
        astro_config_parser::types::AstroStaticValue::Null => G3TsAstroStaticValue::Null,
        astro_config_parser::types::AstroStaticValue::Array(values) => {
            G3TsAstroStaticValue::Array(values.iter().map(astro_static_value).collect())
        }
        astro_config_parser::types::AstroStaticValue::Object(properties) => {
            G3TsAstroStaticValue::Object(
                properties
                    .iter()
                    .map(|property| G3TsAstroStaticObjectProperty {
                        key: property.key.clone(),
                        value: astro_static_value(&property.value),
                    })
                    .collect(),
            )
        }
        astro_config_parser::types::AstroStaticValue::ImportedIdentifier {
            local_name,
            source_module,
            imported_name,
        } => G3TsAstroStaticValue::ImportedIdentifier {
            local_name: local_name.clone(),
            source_module: source_module.clone(),
            imported_name: imported_name.clone(),
        },
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
            !has_one_canonical_pin_group(
                &typed.version_groups,
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
            !has_one_canonical_ban_group(&typed.version_groups, dependency, &BAN_DEPENDENCY_TYPES)
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
    source.len() == 1
        && source.first().is_some_and(|entry| entry == "package.json")
        && syncpack_config_is_app_local(syncpack_rel_path, package_rel_path)
}

fn has_one_canonical_pin_group(
    version_groups: &[syncpack_config_parser::types::SyncpackVersionGroup],
    dependency: &str,
    version: &str,
    dependency_types: &[&str],
) -> bool {
    let mut matching_groups = version_groups
        .iter()
        .filter(|group| group_targets_dependency(group, dependency));

    let Some(group) = matching_groups.next() else {
        return false;
    };

    matching_groups.next().is_none() && canonical_pin_group(group, version, dependency_types)
}

fn has_one_canonical_ban_group(
    version_groups: &[syncpack_config_parser::types::SyncpackVersionGroup],
    dependency: &str,
    dependency_types: &[&str],
) -> bool {
    let mut matching_groups = version_groups
        .iter()
        .filter(|group| group_targets_dependency(group, dependency));

    let Some(group) = matching_groups.next() else {
        return false;
    };

    matching_groups.next().is_none() && canonical_ban_group(group, dependency_types)
}

fn group_targets_dependency(
    group: &syncpack_config_parser::types::SyncpackVersionGroup,
    dependency: &str,
) -> bool {
    string_sets_match_exactly(&group.dependencies, &[dependency])
}

fn canonical_pin_group(
    group: &syncpack_config_parser::types::SyncpackVersionGroup,
    version: &str,
    dependency_types: &[&str],
) -> bool {
    group.packages.is_none()
        && group.specifier_types.is_none()
        && string_sets_match_exactly(&group.dependency_types, dependency_types)
        && group.is_ignored.is_none()
        && group.is_banned.is_none()
        && group.pin_version.as_deref() == Some(version)
}

fn canonical_ban_group(
    group: &syncpack_config_parser::types::SyncpackVersionGroup,
    dependency_types: &[&str],
) -> bool {
    group.packages.is_none()
        && group.specifier_types.is_none()
        && string_sets_match_exactly(&group.dependency_types, dependency_types)
        && group.is_ignored.is_none()
        && group.is_banned == Some(true)
        && group.pin_version.is_none()
}

fn string_sets_match_exactly(left: &[String], right: &[&str]) -> bool {
    left.len() == right.len()
        && BTreeSet::from_iter(left.iter().map(String::as_str))
        == BTreeSet::from_iter(right.iter().copied())
}

fn syncpack_config_is_app_local(syncpack_rel_path: &str, package_rel_path: &str) -> bool {
    let expected_rel_path = package_rel_path.strip_suffix("/package.json").map_or_else(
        || ".syncpackrc".to_owned(),
        |app_root| format!("{app_root}/.syncpackrc"),
    );

    syncpack_rel_path == expected_rel_path
}

fn select_syncpack_config<'crawl>(
    crawl: &'crawl G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> Option<&'crawl g3_workspace_crawl::G3RsWorkspaceEntry> {
    let app_config = scoped_rel_path(app_root_rel_path, SYNCPACK_CONFIG_REL_PATH);
    exact_included_file(crawl, &app_config)
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

fn select_llms_txt(crawl: &G3WorkspaceCrawl, app_root_rel_path: &str) -> Option<String> {
    let rel_path = scoped_rel_path(app_root_rel_path, "public/llms.txt");
    exact_included_file(crawl, &rel_path).map(|entry| entry.path.rel_path.clone())
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

    let probes = crate::select::probe_targets(crawl, app_root_rel_path, &entry.path.rel_path);
    let document = match parse_eslint_document(&crawl.root_abs_path, &entry.path.rel_path, &probes)
    {
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
    let mdx_content_probe = active_probe(
        &typed,
        eslint_config_parser::types::EslintProbeKind::MdxContent,
    );

    G3TsAstroEslintSurfaceState::Parsed {
        snapshot: G3TsAstroEslintSurfaceSnapshot {
            rel_path: entry.path.rel_path.clone(),
            astro_source_probe_present: astro_source_probe.is_some(),
            ts_source_probe_present: ts_source_probe.is_some(),
            tsx_source_probe_present: tsx_source_probe.is_some(),
            mdx_content_probe_present: mdx_content_probe.is_some(),
            astro_source_plugins: astro_source_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            ts_source_plugins: ts_source_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            tsx_source_plugins: tsx_source_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            mdx_content_plugins: mdx_content_probe
                .map(|probe| probe.plugins.clone())
                .unwrap_or_default(),
            astro_source_plugin_meta_names: astro_source_probe
                .map(|probe| probe.plugin_meta_names.clone())
                .unwrap_or_default(),
            ts_source_plugin_meta_names: ts_source_probe
                .map(|probe| probe.plugin_meta_names.clone())
                .unwrap_or_default(),
            tsx_source_plugin_meta_names: tsx_source_probe
                .map(|probe| probe.plugin_meta_names.clone())
                .unwrap_or_default(),
            mdx_content_plugin_meta_names: mdx_content_probe
                .map(|probe| probe.plugin_meta_names.clone())
                .unwrap_or_default(),
            astro_source_plugin_package_names: astro_source_probe
                .map(|probe| probe.plugin_package_names.clone())
                .unwrap_or_default(),
            ts_source_plugin_package_names: ts_source_probe
                .map(|probe| probe.plugin_package_names.clone())
                .unwrap_or_default(),
            tsx_source_plugin_package_names: tsx_source_probe
                .map(|probe| probe.plugin_package_names.clone())
                .unwrap_or_default(),
            mdx_content_plugin_package_names: mdx_content_probe
                .map(|probe| probe.plugin_package_names.clone())
                .unwrap_or_default(),
            astro_source_error_rules: active_error_rules(astro_source_probe),
            ts_source_error_rules: active_error_rules(ts_source_probe),
            tsx_source_error_rules: active_error_rules(tsx_source_probe),
            mdx_content_error_rules: active_error_rules(mdx_content_probe),
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
            astro_source_effective_inline_public_content_rules:
                effective_inline_public_content_rules(astro_source_probe),
            ts_source_effective_inline_public_content_rules: effective_inline_public_content_rules(
                ts_source_probe,
            ),
            tsx_source_effective_inline_public_content_rules: effective_inline_public_content_rules(
                tsx_source_probe,
            ),
            astro_source_probe_ignored: probe_ignored(
                &typed,
                eslint_config_parser::types::EslintProbeKind::AstroSource,
            ),
            ts_source_probe_ignored: probe_ignored(
                &typed,
                eslint_config_parser::types::EslintProbeKind::TsSource,
            ),
            tsx_source_probe_ignored: probe_ignored(
                &typed,
                eslint_config_parser::types::EslintProbeKind::TsxSource,
            ),
            mdx_content_probe_ignored: probe_ignored(
                &typed,
                eslint_config_parser::types::EslintProbeKind::MdxContent,
            ),
        },
    }
}

fn active_probe<'a>(
    typed: &'a eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&'a eslint_config_parser::types::EslintEffectiveConfigProbe> {
    probe_by_kind(typed, kind).filter(|probe| !probe.ignored)
}

fn probe_by_kind(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> Option<&eslint_config_parser::types::EslintEffectiveConfigProbe> {
    typed.probes.iter().find(|probe| probe.probe == kind)
}

fn probe_ignored(
    typed: &eslint_config_parser::types::EslintConfigSnapshot,
    kind: eslint_config_parser::types::EslintProbeKind,
) -> bool {
    probe_by_kind(typed, kind).is_none_or(|probe| probe.ignored)
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
                rule_setting_is_error(setting)
                    && rule_setting_has_route_and_endpoint_coverage(
                        setting,
                        route_page_paths,
                        endpoint_paths,
                    )
                    && (**rule_name != CONTENT_ADAPTER_PIPELINE_RULE
                        || rule_setting_has_approved_content_adapter_scope(setting))
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
                rule_setting_is_error(setting)
                    && rule_setting_has_route_and_endpoint_coverage(
                        setting,
                        route_page_paths,
                        endpoint_paths,
                    )
                    && rule_setting_has_content_data_scope(setting)
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
                rule_setting_is_error(setting)
                    && rule_setting_has_route_and_endpoint_coverage(
                        setting,
                        route_page_paths,
                        endpoint_paths,
                    )
                    && rule_setting_has_content_source_scope(setting)
            })
        })
        .map(|rule_name| (*rule_name).to_owned())
        .collect()
}

fn effective_inline_public_content_rules(
    probe: Option<&eslint_config_parser::types::EslintEffectiveConfigProbe>,
) -> Vec<String> {
    let Some(probe) = probe else {
        return Vec::new();
    };

    probe
        .rules
        .get(INLINE_PUBLIC_CONTENT_RULE)
        .map_or_else(Vec::new, |setting| {
            if rule_setting_is_error(setting)
                && rule_setting_has_inline_public_content_policy(setting)
            {
                vec![INLINE_PUBLIC_CONTENT_RULE.to_owned()]
            } else {
                Vec::new()
            }
        })
}

fn rule_setting_has_route_and_endpoint_coverage(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    route_page_paths: &[String],
    endpoint_paths: &[String],
) -> bool {
    let route_coverage = !route_page_paths.is_empty()
        && rule_setting_option_globs_match_any_path(setting, "routeGlobs", route_page_paths);
    let endpoint_coverage = if endpoint_paths.is_empty() {
        rule_setting_option_globs_are_valid(setting, "endpointGlobs")
    } else {
        rule_setting_option_globs_match_any_path(setting, "endpointGlobs", endpoint_paths)
    };

    route_coverage && endpoint_coverage
}

fn rule_setting_has_content_data_scope(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    first_option_object(setting).is_some_and(|object| {
        has_non_empty_string_array_option(object.get("contentDataModuleGlobs"))
    })
}

fn rule_setting_has_content_source_scope(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    first_option_object(setting).is_some_and(|object| {
        has_non_empty_string_array_option(object.get("authoredContentGlobs"))
            || has_non_empty_string_array_option(object.get("specContentGlobs"))
    })
}

fn rule_setting_has_approved_content_adapter_scope(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    first_option_object(setting).is_some_and(|object| {
        has_non_empty_string_array_option(object.get("approvedContentAdapterModules"))
    })
}

fn rule_setting_has_inline_public_content_policy(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> bool {
    let Some(object) = setting
        .options
        .first()
        .and_then(serde_json::Value::as_object)
    else {
        return false;
    };

    object.len() == 10
        && object_string_value(object.get("framework")) == Some("react")
        && object_string_value(object.get("mode")) == Some("all")
        && object_string_value(object.get("message")) == Some(INLINE_PUBLIC_CONTENT_MESSAGE)
        && object_bool_value(object.get("should-validate-template")) == Some(true)
        && object_has_exact_string_arrays(
            object.get("words"),
            "include",
            &[],
            "exclude",
            &["[0-9!-/:-@[-`{-~]+", "[A-Z_-]+"],
        )
        && object_has_exact_string_arrays(
            object.get("jsx-components"),
            "include",
            &[],
            "exclude",
            &[],
        )
        && object_has_exact_string_arrays(
            object.get("jsx-attributes"),
            "include",
            &[],
            "exclude",
            &[
                "as",
                "class",
                "className",
                "color",
                "data-.+",
                "height",
                "href",
                "id",
                "intent",
                "key",
                "name",
                "rel",
                "role",
                "size",
                "slot",
                "src",
                "style",
                "styleName",
                "target",
                "tone",
                "type",
                "variant",
                "width",
                "aria-hidden",
            ],
        )
        && object_has_exact_string_arrays(
            object.get("callees"),
            "include",
            &[],
            "exclude",
            &[
                "require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL",
            ],
        )
        && object_has_exact_string_arrays(
            object.get("object-properties"),
            "include",
            &[],
            "exclude",
            &["[A-Z_-]+"],
        )
        && object_has_exact_string_arrays(
            object.get("class-properties"),
            "include",
            &[],
            "exclude",
            &["displayName"],
        )
}

fn object_has_exact_string_arrays(
    value: Option<&serde_json::Value>,
    first_key: &str,
    first_expected: &[&str],
    second_key: &str,
    second_expected: &[&str],
) -> bool {
    let Some(object) = value.and_then(serde_json::Value::as_object) else {
        return false;
    };

    object.len() == 2
        && string_array_exactly(object.get(first_key), first_expected)
        && string_array_exactly(object.get(second_key), second_expected)
}

fn string_array_exactly(value: Option<&serde_json::Value>, expected: &[&str]) -> bool {
    let Some(values) = value.and_then(serde_json::Value::as_array) else {
        return false;
    };

    values.len() == expected.len()
        && values
            .iter()
            .zip(expected.iter().copied())
            .all(|(value, expected)| value.as_str() == Some(expected))
}

fn object_string_value(option: Option<&serde_json::Value>) -> Option<&str> {
    option.and_then(serde_json::Value::as_str)
}

fn object_bool_value(option: Option<&serde_json::Value>) -> Option<bool> {
    option.and_then(serde_json::Value::as_bool)
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
    first_option_object(setting).is_some_and(|object| {
        non_empty_string_array_option(object.get(option_name))
            .is_some_and(|patterns| all_paths_match_globs(&patterns, candidate_paths))
    })
}

fn rule_setting_option_globs_are_valid(
    setting: &eslint_config_parser::types::EslintRuleSetting,
    option_name: &str,
) -> bool {
    first_option_object(setting).is_some_and(|object| {
        non_empty_string_array_option(object.get(option_name))
            .is_some_and(|patterns| globs_are_valid(&patterns))
    })
}

fn first_option_object(
    setting: &eslint_config_parser::types::EslintRuleSetting,
) -> Option<&serde_json::Map<String, serde_json::Value>> {
    setting
        .options
        .first()
        .and_then(serde_json::Value::as_object)
}

fn rule_setting_is_error(setting: &eslint_config_parser::types::EslintRuleSetting) -> bool {
    setting.severity == eslint_config_parser::types::EslintRuleSeverity::Error
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

fn globs_are_valid(patterns: &[&str]) -> bool {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        let Ok(glob) = Glob::new(&normalize_glob(pattern)) else {
            return false;
        };
        let _ = builder.add(glob);
    }

    builder.build().is_ok()
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
