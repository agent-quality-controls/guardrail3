use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_astro_setup_types::{
    G3TsAstroPackageScriptCommand, G3TsAstroPackageScriptCommandSeparator,
    G3TsAstroPackageScriptParseBlocker, G3TsAstroPackageScriptToolInvocation,
    G3TsAstroPackageSurfaceSnapshot, G3TsAstroPackageSurfaceState,
};
use package_script_command_parser::types::{
    PackageScriptCommand, PackageScriptCommandSeparator, PackageScriptParseFact,
    PackageScriptParseState, PackageScriptToolInvocation,
};

pub(crate) fn ingest_package_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsAstroPackageSurfaceState {
    let rel_path =
        g3ts_astro_check_support::surfaces::scoped_rel_path(app_root_rel_path, "package.json");
    let Some(entry) = crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
    else {
        return G3TsAstroPackageSurfaceState::Missing { rel_path };
    };

    if !entry.readable {
        return G3TsAstroPackageSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the package manifest unreadable".to_owned(),
        };
    }

    let document = match package_json_parser::from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsAstroPackageSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    if let Some(reason) = package_json_parser::parse_error_reason(&document) {
        return G3TsAstroPackageSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let Some(typed) = package_json_parser::typed(&document) else {
        return G3TsAstroPackageSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: "package.json parsed without typed package data".to_owned(),
        };
    };
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
        },
    }
}

fn parse_package_script(name: &str, body: &str) -> PackageScriptParseFact {
    match package_script_command_parser::parse(name, body) {
        Ok(fact) => fact,
        Err(error) => PackageScriptParseFact {
            script_name: name.to_owned(),
            state: PackageScriptParseState::ParseError {
                reason: error.to_string(),
            },
            commands: Vec::new(),
            tool_invocations: Vec::new(),
            all_tool_invocations: Vec::new(),
        },
    }
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
