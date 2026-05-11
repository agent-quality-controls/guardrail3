use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_typecov_types::{
    G3TsTypecovPackageScriptCommandSeparator, G3TsTypecovPackageScriptParseBlocker,
    G3TsTypecovPackageScriptToolInvocation, G3TsTypecovPackageSurfaceSnapshot,
    G3TsTypecovPackageSurfaceState,
};
use package_script_command_parser::types::{
    PackageScriptCommandSeparator, PackageScriptParseFact, PackageScriptParseState,
    PackageScriptToolInvocation,
};

/// `ingest_package_surface`: ingest package surface.
pub(crate) fn ingest_package_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsTypecovPackageSurfaceState {
    let rel_path = crate::roots::scoped_rel_path(app_root_rel_path, "package.json");
    let Some(entry) = crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
    else {
        return G3TsTypecovPackageSurfaceState::Missing { rel_path };
    };

    if !entry.readable {
        return G3TsTypecovPackageSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the package manifest unreadable".to_owned(),
        };
    }

    let document = match package_json_parser::from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsTypecovPackageSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };
    if let Some(reason) = package_json_parser::parse_error_reason(&document) {
        return G3TsTypecovPackageSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }
    let Some(typed) = package_json_parser::typed(&document) else {
        return G3TsTypecovPackageSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: "package.json parsed without typed package data".to_owned(),
        };
    };
    let script_facts = typed
        .scripts
        .iter()
        .map(|(name, body)| parse_package_script(name, body))
        .collect::<Vec<_>>();

    G3TsTypecovPackageSurfaceState::Parsed {
        snapshot: G3TsTypecovPackageSurfaceSnapshot {
            rel_path: entry.path.rel_path.clone(),
            dependencies: typed.dependencies.clone(),
            dev_dependencies: typed.dev_dependencies.clone(),
            script_names: typed.scripts.keys().cloned().collect(),
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

/// `parse_package_script`: parse package script.
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

/// `script_tool_invocations`: script tool invocations.
fn script_tool_invocations(
    fact: &PackageScriptParseFact,
) -> Vec<G3TsTypecovPackageScriptToolInvocation> {
    fact.tool_invocations
        .iter()
        .map(script_tool_invocation)
        .collect()
}

/// `script_tool_invocation`: script tool invocation.
fn script_tool_invocation(
    invocation: &PackageScriptToolInvocation,
) -> G3TsTypecovPackageScriptToolInvocation {
    G3TsTypecovPackageScriptToolInvocation {
        script_name: invocation.script_name.clone(),
        executable: invocation.executable.clone(),
        args: invocation.args.clone(),
        preceded_by: invocation.preceded_by.map(script_command_separator),
        followed_by: invocation.followed_by.map(script_command_separator),
    }
}

/// `script_command_separator`: script command separator.
const fn script_command_separator(
    separator: PackageScriptCommandSeparator,
) -> G3TsTypecovPackageScriptCommandSeparator {
    match separator {
        PackageScriptCommandSeparator::And => G3TsTypecovPackageScriptCommandSeparator::And,
        PackageScriptCommandSeparator::Or => G3TsTypecovPackageScriptCommandSeparator::Or,
    }
}

/// `script_parse_blocker`: script parse blocker.
fn script_parse_blocker(
    fact: &PackageScriptParseFact,
) -> Option<G3TsTypecovPackageScriptParseBlocker> {
    match &fact.state {
        PackageScriptParseState::Unsupported { reason }
        | PackageScriptParseState::ParseError { reason } => {
            Some(G3TsTypecovPackageScriptParseBlocker {
                script_name: fact.script_name.clone(),
                reason: reason.clone(),
            })
        }
        PackageScriptParseState::Parsed { .. } | PackageScriptParseState::NoEslintInvocation => {
            None
        }
    }
}
