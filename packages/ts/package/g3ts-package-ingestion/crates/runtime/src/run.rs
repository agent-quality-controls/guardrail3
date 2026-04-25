use g3_workspace_crawl::{
    G3RsWorkspaceCrawl as G3WorkspaceCrawl, G3RsWorkspaceEntry as G3WorkspaceEntry,
    G3RsWorkspaceEntryKind as G3WorkspaceEntryKind, root_file,
};
use g3ts_package_types::{
    G3TsPackageChecksInput, G3TsPackageLocalState, G3TsPackageRootState, G3TsPackageScriptCommand,
    G3TsPackageScriptCommandSeparator, G3TsPackageScriptParseBlocker,
    G3TsPackageScriptToolInvocation, G3TsPackageSyncpackConfigSnapshot,
    G3TsPackageSyncpackConfigState, local_snapshot, root_snapshot,
};
use package_json_parser::{from_path_document, parse_error_reason, typed};
use package_script_command_parser::types::{
    PackageScriptCommand, PackageScriptCommandSeparator, PackageScriptParseFact,
    PackageScriptParseState, PackageScriptToolInvocation,
};
use std::collections::BTreeSet;
use syncpack_config_parser::{
    from_path_document as syncpack_from_path_document,
    parse_error_reason as syncpack_parse_error_reason,
};

const PACKAGE_JSON_REL_PATH: &str = "package.json";
const SYNCPACK_CONFIG_REL_PATH: &str = ".syncpackrc";
const FORBIDDEN_SYNCPACK_DEPS: [&str; 19] = [
    "axios",
    "lodash",
    "moment",
    "uuid",
    "nanoid",
    "express",
    "classnames",
    "winston",
    "pino",
    "request",
    "got",
    "superagent",
    "node-fetch",
    "isomorphic-fetch",
    "underscore",
    "request-promise",
    "cross-fetch",
    "xregexp",
    "regexp-tree",
];
const BAN_DEPENDENCY_TYPES: [&str; 4] = ["prod", "dev", "optional", "peer"];
const SYNCPACK_PACKAGE_POLICY_PREFIX_LEN: usize = FORBIDDEN_SYNCPACK_DEPS.len();

pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> G3TsPackageChecksInput {
    let root_policy_applies = root_policy_applies(crawl);
    let locals = crawl
        .entries
        .iter()
        .filter(|entry| is_local_package_json(entry, root_policy_applies))
        .map(ingest_local)
        .collect::<Vec<_>>();

    G3TsPackageChecksInput {
        root: if root_policy_applies {
            ingest_root(crawl)
        } else {
            G3TsPackageRootState::NotPackageManagerRoot
        },
        syncpack_config: if root_policy_applies {
            ingest_syncpack_config(crawl, &locals)
        } else {
            G3TsPackageSyncpackConfigState::NotRequired
        },
        forbidden_syncpack_deps: FORBIDDEN_SYNCPACK_DEPS
            .into_iter()
            .map(str::to_owned)
            .collect(),
        locals,
    }
}

fn ingest_root(crawl: &G3WorkspaceCrawl) -> G3TsPackageRootState {
    let Some(entry) = root_file(crawl, "package.json") else {
        return G3TsPackageRootState::Missing;
    };

    if !entry.readable {
        return G3TsPackageRootState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the selected root manifest unreadable".to_owned(),
        };
    }

    let document = match from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(err) => {
            return G3TsPackageRootState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: err.to_string(),
            };
        }
    };

    if let Some(reason) = parse_error_reason(&document) {
        return G3TsPackageRootState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let snapshot = typed(&document).expect("parsed package.json document should stay typed");
    let script_facts = snapshot
        .scripts
        .iter()
        .map(|(name, body)| parse_package_script(name, body))
        .collect::<Vec<_>>();
    let mut root = root_snapshot(&entry.path.rel_path, snapshot);
    root.script_commands = script_facts.iter().flat_map(script_commands).collect();
    root.script_tool_invocations = script_facts
        .iter()
        .flat_map(script_tool_invocations)
        .collect();
    root.script_parse_blockers = script_facts
        .iter()
        .filter_map(script_parse_blocker)
        .collect();
    let preinstall_script_facts = script_facts
        .iter()
        .filter(|fact| fact.script_name == "preinstall")
        .cloned()
        .collect::<Vec<_>>();
    root.safely_runs_only_allow_pnpm = package_script_command_parser::has_safe_tool_invocation(
        &preinstall_script_facts,
        "only-allow",
        "pnpm",
    );
    root.safely_runs_syncpack_lint =
        package_script_command_parser::has_safe_tool_invocation(&script_facts, "syncpack", "lint");

    G3TsPackageRootState::Parsed { snapshot: root }
}

fn parse_package_script(name: &str, body: &str) -> PackageScriptParseFact {
    package_script_command_parser::parse(name, body)
        .expect("package script command parser should not fail on string input")
}

fn script_commands(fact: &PackageScriptParseFact) -> Vec<G3TsPackageScriptCommand> {
    fact.commands
        .iter()
        .map(|command| script_command(&fact.script_name, command))
        .collect()
}

fn script_command(script_name: &str, command: &PackageScriptCommand) -> G3TsPackageScriptCommand {
    G3TsPackageScriptCommand {
        script_name: script_name.to_owned(),
        invocation: command.invocation.clone(),
        executable: command.executable.clone(),
        args: command.args.clone(),
        preceded_by: command.preceded_by.map(script_command_separator),
    }
}

fn script_command_separator(
    separator: PackageScriptCommandSeparator,
) -> G3TsPackageScriptCommandSeparator {
    match separator {
        PackageScriptCommandSeparator::And => G3TsPackageScriptCommandSeparator::And,
        PackageScriptCommandSeparator::Or => G3TsPackageScriptCommandSeparator::Or,
    }
}

fn script_tool_invocations(fact: &PackageScriptParseFact) -> Vec<G3TsPackageScriptToolInvocation> {
    fact.tool_invocations
        .iter()
        .map(script_tool_invocation)
        .collect()
}

fn script_tool_invocation(
    invocation: &PackageScriptToolInvocation,
) -> G3TsPackageScriptToolInvocation {
    G3TsPackageScriptToolInvocation {
        script_name: invocation.script_name.clone(),
        command_index: invocation.command_index,
        invocation: invocation.invocation.clone(),
        executable: invocation.executable.clone(),
        args: invocation.args.clone(),
        preceded_by: invocation.preceded_by.map(script_command_separator),
        followed_by: invocation.followed_by.map(script_command_separator),
    }
}

fn script_parse_blocker(fact: &PackageScriptParseFact) -> Option<G3TsPackageScriptParseBlocker> {
    match &fact.state {
        PackageScriptParseState::Unsupported { reason }
        | PackageScriptParseState::ParseError { reason } => Some(G3TsPackageScriptParseBlocker {
            script_name: fact.script_name.clone(),
            reason: reason.clone(),
        }),
        PackageScriptParseState::Parsed { .. } | PackageScriptParseState::NoEslintInvocation => {
            None
        }
    }
}

fn ingest_local(entry: &G3WorkspaceEntry) -> G3TsPackageLocalState {
    if !entry.readable {
        return G3TsPackageLocalState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the local manifest unreadable".to_owned(),
        };
    }

    let document = match from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(err) => {
            return G3TsPackageLocalState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: err.to_string(),
            };
        }
    };

    if let Some(reason) = parse_error_reason(&document) {
        return G3TsPackageLocalState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let snapshot = typed(&document).expect("parsed package.json document should stay typed");
    G3TsPackageLocalState::Parsed {
        snapshot: local_snapshot(&entry.path.rel_path, snapshot),
    }
}

fn is_local_package_json(entry: &G3WorkspaceEntry, root_policy_applies: bool) -> bool {
    entry.kind == G3WorkspaceEntryKind::File
        && if root_policy_applies {
            entry.path.rel_path.ends_with("/package.json")
        } else {
            entry.path.rel_path == "package.json" || entry.path.rel_path.ends_with("/package.json")
        }
}

fn root_policy_applies(crawl: &G3WorkspaceCrawl) -> bool {
    root_file(crawl, "pnpm-workspace.yaml").is_some()
        || root_file(crawl, "pnpm-lock.yaml").is_some()
}

fn ingest_syncpack_config(
    crawl: &G3WorkspaceCrawl,
    locals: &[G3TsPackageLocalState],
) -> G3TsPackageSyncpackConfigState {
    let Some(entry) = root_file(crawl, SYNCPACK_CONFIG_REL_PATH) else {
        return G3TsPackageSyncpackConfigState::Missing {
            rel_path: SYNCPACK_CONFIG_REL_PATH.to_owned(),
        };
    };

    if !entry.readable {
        return G3TsPackageSyncpackConfigState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the Syncpack config unreadable".to_owned(),
        };
    }

    let document = match syncpack_from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return G3TsPackageSyncpackConfigState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    if let Some(reason) = syncpack_parse_error_reason(&document) {
        return G3TsPackageSyncpackConfigState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let typed = syncpack_config_parser::typed(&document)
        .expect("parsed Syncpack config document should stay typed");
    let required_sources = required_syncpack_source_entries(locals);
    let missing_source_entries = required_sources
        .iter()
        .filter(|source| !typed.source.iter().any(|entry| entry == *source))
        .cloned()
        .collect();
    let missing_forbidden_bans = FORBIDDEN_SYNCPACK_DEPS
        .iter()
        .filter(|dependency| {
            !has_canonical_ban_in_prefix(
                &typed.version_groups,
                SYNCPACK_PACKAGE_POLICY_PREFIX_LEN,
                dependency,
                &BAN_DEPENDENCY_TYPES,
            )
        })
        .map(|dependency| (*dependency).to_owned())
        .collect();

    G3TsPackageSyncpackConfigState::Parsed {
        snapshot: G3TsPackageSyncpackConfigSnapshot {
            rel_path: entry.path.rel_path.clone(),
            missing_source_entries,
            missing_forbidden_bans,
        },
    }
}

fn required_syncpack_source_entries(locals: &[G3TsPackageLocalState]) -> Vec<String> {
    let mut sources = BTreeSet::from([PACKAGE_JSON_REL_PATH.to_owned()]);
    for local in locals {
        let _ = sources.insert(local_rel_path(local).to_owned());
    }
    sources.into_iter().collect()
}

fn local_rel_path(local: &G3TsPackageLocalState) -> &str {
    match local {
        G3TsPackageLocalState::Unreadable { rel_path, .. }
        | G3TsPackageLocalState::ParseError { rel_path, .. } => rel_path,
        G3TsPackageLocalState::Parsed { snapshot } => &snapshot.rel_path,
    }
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

fn canonical_ban_group(group: &syncpack_config_parser::types::SyncpackVersionGroup) -> bool {
    group.packages.is_none()
        && group.specifier_types.is_none()
        && group.is_ignored.is_none()
        && group.is_banned == Some(true)
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

#[cfg(test)]
#[path = "run_tests/mod.rs"]
mod run_tests;
