use g3_workspace_crawl::G3RsWorkspaceCrawl as G3WorkspaceCrawl;
use g3ts_spelling_types::{
    G3TsSpellingPackageScriptCommandSeparator, G3TsSpellingPackageScriptParseBlocker,
    G3TsSpellingPackageScriptToolInvocation, G3TsSpellingPackageSurfaceSnapshot,
    G3TsSpellingPackageSurfaceState,
};
use package_script_command_parser::types::{
    PackageScriptCommandSeparator, PackageScriptParseFact, PackageScriptParseState,
    PackageScriptToolInvocation,
};

/// Read and parse the `package.json` at `app_root_rel_path` from `crawl`,
/// returning a surface-state describing what was found.
pub(crate) fn ingest_package_surface(
    crawl: &G3WorkspaceCrawl,
    app_root_rel_path: &str,
) -> G3TsSpellingPackageSurfaceState {
    let rel_path = crate::roots::scoped_rel_path(app_root_rel_path, "package.json");
    let Some(entry) = crawl
        .entries
        .iter()
        .find(|entry| entry.path.rel_path == rel_path)
    else {
        return G3TsSpellingPackageSurfaceState::Missing { rel_path };
    };

    if !entry.readable {
        return G3TsSpellingPackageSurfaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the package manifest unreadable".to_owned(),
        };
    }

    let document = match package_json_parser::from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsSpellingPackageSurfaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };
    if let Some(reason) = package_json_parser::parse_error_reason(&document) {
        return G3TsSpellingPackageSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }
    let Some(typed) = package_json_parser::typed(&document) else {
        return G3TsSpellingPackageSurfaceState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: "package.json parsed without typed package data".to_owned(),
        };
    };
    let script_facts = typed
        .scripts
        .iter()
        .map(|(name, body)| parse_package_script(name, body))
        .collect::<Vec<_>>();

    G3TsSpellingPackageSurfaceState::Parsed {
        snapshot: G3TsSpellingPackageSurfaceSnapshot {
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

/// Parse a single `name`/`body` script entry into a parse-fact, mapping any
/// parser error into a `ParseError` state for downstream checks.
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

/// Project all tool invocations recorded for `fact` into the spelling
/// snapshot variant.
fn script_tool_invocations(
    fact: &PackageScriptParseFact,
) -> Vec<G3TsSpellingPackageScriptToolInvocation> {
    fact.tool_invocations
        .iter()
        .map(script_tool_invocation)
        .collect()
}

/// Project a single parsed tool invocation into the spelling snapshot variant.
fn script_tool_invocation(
    invocation: &PackageScriptToolInvocation,
) -> G3TsSpellingPackageScriptToolInvocation {
    G3TsSpellingPackageScriptToolInvocation {
        script_name: invocation.script_name.clone(),
        executable: invocation.executable.clone(),
        args: invocation.args.clone(),
        preceded_by: invocation.preceded_by.map(script_command_separator),
        followed_by: invocation.followed_by.map(script_command_separator),
    }
}

/// Map a parser-side command separator to the spelling snapshot enum.
const fn script_command_separator(
    separator: PackageScriptCommandSeparator,
) -> G3TsSpellingPackageScriptCommandSeparator {
    match separator {
        PackageScriptCommandSeparator::And => G3TsSpellingPackageScriptCommandSeparator::And,
        PackageScriptCommandSeparator::Or => G3TsSpellingPackageScriptCommandSeparator::Or,
    }
}

/// Return a parse-blocker descriptor for `fact` when its state represents
/// an unsupported or parse-failed script body.
fn script_parse_blocker(
    fact: &PackageScriptParseFact,
) -> Option<G3TsSpellingPackageScriptParseBlocker> {
    match &fact.state {
        PackageScriptParseState::Unsupported { reason }
        | PackageScriptParseState::ParseError { reason } => {
            Some(G3TsSpellingPackageScriptParseBlocker {
                script_name: fact.script_name.clone(),
                reason: reason.clone(),
            })
        }
        PackageScriptParseState::Parsed { .. } | PackageScriptParseState::NoEslintInvocation => {
            None
        }
    }
}
