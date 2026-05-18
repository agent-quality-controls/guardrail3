use g3_workspace_crawl::G3WorkspaceCrawl;
use g3ts_fmt_types::{
    G3TsFmtDependencyDeclarationSnapshot, G3TsFmtPackageScriptCommandSeparator,
    G3TsFmtPackageScriptParseBlocker, G3TsFmtPackageScriptToolInvocation,
    G3TsFmtPackageSurfaceSnapshot, G3TsFmtPackageSurfaceState,
};
use package_script_command_parser::types::{
    PackageScriptCommandSeparator, PackageScriptParseFact, PackageScriptParseState,
    PackageScriptToolInvocation,
};

/// Ingests `package.json` under `app_root_rel_path` into a surface state.
pub(crate) fn ingest_package_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsFmtPackageSurfaceState {
    let rel_path = crate::roots::scoped_rel_path(app_root_rel_path, "package.json");
    let Some(entry) = crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
    else {
        return G3TsFmtPackageSurfaceState::Missing { rel_path };
    };

    if !entry.readable {
        return G3TsFmtPackageSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the package manifest unreadable".to_owned(),
        };
    }

    let document = match package_json_parser::from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsFmtPackageSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };
    if let Some(reason) = package_json_parser::parse_error_reason(&document) {
        return G3TsFmtPackageSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }
    let Some(typed) = package_json_parser::typed(&document) else {
        return G3TsFmtPackageSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: "package.json parsed without typed package data".to_owned(),
        };
    };
    let script_facts = typed
        .scripts
        .iter()
        .map(|(name, body)| parse_package_script(name, body))
        .collect::<Vec<_>>();

    G3TsFmtPackageSurfaceState::Parsed {
        snapshot: G3TsFmtPackageSurfaceSnapshot {
            rel_path: entry.path.rel_path.clone(),
            name: typed.name.clone(),
            dependencies: typed.dependencies.clone(),
            dev_dependencies: typed.dev_dependencies.clone(),
            dependency_declarations: package_json_parser::dependency_declarations(&document.raw)
                .into_iter()
                .map(|declaration| G3TsFmtDependencyDeclarationSnapshot {
                    name: declaration.name,
                    lane: declaration.lane,
                    specifier_type: declaration.specifier_type,
                })
                .collect(),
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

/// Parses a single `package.json` script body into a structured fact.
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

/// Maps a parse fact's tool invocations into the contract type.
fn script_tool_invocations(
    fact: &PackageScriptParseFact,
) -> Vec<G3TsFmtPackageScriptToolInvocation> {
    fact.tool_invocations
        .iter()
        .map(script_tool_invocation)
        .collect()
}

/// Converts a single tool invocation into the contract type.
fn script_tool_invocation(
    invocation: &PackageScriptToolInvocation,
) -> G3TsFmtPackageScriptToolInvocation {
    G3TsFmtPackageScriptToolInvocation {
        script_name: invocation.script_name.clone(),
        invocation: invocation.invocation.clone(),
        executable: invocation.executable.clone(),
        args: invocation.args.clone(),
        preceded_by: invocation.preceded_by.map(script_command_separator),
        followed_by: invocation.followed_by.map(script_command_separator),
    }
}

/// Maps a parser command separator into the contract enum.
const fn script_command_separator(
    separator: PackageScriptCommandSeparator,
) -> G3TsFmtPackageScriptCommandSeparator {
    match separator {
        PackageScriptCommandSeparator::And => G3TsFmtPackageScriptCommandSeparator::And,
        PackageScriptCommandSeparator::Or => G3TsFmtPackageScriptCommandSeparator::Or,
    }
}

/// Returns a parse blocker for `fact` if its parse state is unsuccessful.
fn script_parse_blocker(fact: &PackageScriptParseFact) -> Option<G3TsFmtPackageScriptParseBlocker> {
    match &fact.state {
        PackageScriptParseState::Unsupported { reason }
        | PackageScriptParseState::ParseError { reason } => {
            Some(G3TsFmtPackageScriptParseBlocker {
                script_name: fact.script_name.clone(),
                reason: reason.clone(),
            })
        }
        PackageScriptParseState::Parsed { .. } | PackageScriptParseState::NoEslintInvocation => {
            None
        }
    }
}
