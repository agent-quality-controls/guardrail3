use g3_workspace_crawl::{G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind, root_file};
use g3ts_package_types as package_types;
use package_script_command_parser::types as parser_types;
use serde::Deserialize;
use std::collections::BTreeSet;
use syncpack_config_parser::types::SyncpackVersionGroup;

/// Workspace-relative path of the workspace-root `package.json` manifest.
const PACKAGE_JSON_REL_PATH: &str = "package.json";
/// Workspace-relative path of the pnpm workspace policy file.
const PNPM_WORKSPACE_REL_PATH: &str = "pnpm-workspace.yaml";
/// Workspace-relative path of the Syncpack config file.
const SYNCPACK_CONFIG_REL_PATH: &str = ".syncpackrc";
/// Dependency names whose use must be forbidden via Syncpack version groups.
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
/// Dependency types that must be covered by every forbidden-dependency ban.
const BAN_DEPENDENCY_TYPES: [&str; 4] = ["prod", "dev", "optional", "peer"];
/// Number of leading Syncpack version-groups inspected when verifying that
/// every forbidden dependency is banned.
const SYNCPACK_PACKAGE_POLICY_PREFIX_LEN: usize = FORBIDDEN_SYNCPACK_DEPS.len();

/// Typed projection of the root pnpm workspace policy fields used by G3TS.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PnpmWorkspacePolicy {
    /// Exact Node runtime version enforced by pnpm during install.
    node_version: Option<String>,
    /// Whether pnpm fails installs when dependency `engines` are incompatible.
    engine_strict: Option<bool>,
}

/// Ingest the workspace crawl into a `package_types::G3TsPackageChecksInput` describing
/// the package family inputs (root manifest, syncpack config, locals).
#[must_use]
pub fn ingest_for_config_checks(crawl: &G3WorkspaceCrawl) -> package_types::G3TsPackageChecksInput {
    let root_policy_applies = root_policy_applies(crawl);
    let locals = crawl
        .entries
        .iter()
        .filter(|entry| is_local_package_json(entry, root_policy_applies))
        .map(ingest_local)
        .collect::<Vec<_>>();

    package_types::G3TsPackageChecksInput {
        root: if root_policy_applies {
            ingest_root(crawl)
        } else {
            package_types::G3TsPackageRootState::NotPackageManagerRoot
        },
        pnpm_workspace: if root_policy_applies {
            ingest_pnpm_workspace(crawl)
        } else {
            package_types::G3TsPackagePnpmWorkspaceState::NotRequired
        },
        syncpack_config: if root_policy_applies {
            ingest_syncpack_config(crawl, &locals)
        } else {
            package_types::G3TsPackageSyncpackConfigState::NotRequired
        },
        forbidden_syncpack_deps: FORBIDDEN_SYNCPACK_DEPS
            .into_iter()
            .map(str::to_owned)
            .collect(),
        locals,
    }
}

/// Ingest the workspace-root `package.json` into the corresponding
/// `package_types::G3TsPackageRootState` variant, populating script-derived facts when the
/// manifest parses successfully.
fn ingest_root(crawl: &G3WorkspaceCrawl) -> package_types::G3TsPackageRootState {
    let Some(entry) = root_file(crawl, "package.json") else {
        return package_types::G3TsPackageRootState::Missing;
    };

    if !entry.readable {
        return package_types::G3TsPackageRootState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the selected root manifest unreadable".to_owned(),
        };
    }

    let document = match package_json_parser::from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(err) => {
            return package_types::G3TsPackageRootState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: err.to_string(),
            };
        }
    };

    if let Some(reason) = package_json_parser::parse_error_reason(&document) {
        return package_types::G3TsPackageRootState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let Some(snapshot) = package_json_parser::typed(&document) else {
        return package_types::G3TsPackageRootState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: "parsed package.json document did not yield a typed snapshot".to_owned(),
        };
    };
    let script_facts = snapshot
        .scripts
        .iter()
        .map(|(name, body)| parse_package_script(name, body))
        .collect::<Vec<_>>();
    let mut root = package_types::root_snapshot(&entry.path.rel_path, snapshot);
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

    package_types::G3TsPackageRootState::Parsed { snapshot: root }
}

/// Parse a single `package.json` script, returning the parser's
/// `parser_types::PackageScriptParseFact`. The parser API never fails for string inputs,
/// so a parse error is surfaced as an unsupported state inside the fact.
fn parse_package_script(name: &str, body: &str) -> parser_types::PackageScriptParseFact {
    package_script_command_parser::parse(name, body).unwrap_or_else(|err| {
        parser_types::PackageScriptParseFact {
            script_name: name.to_owned(),
            commands: Vec::new(),
            tool_invocations: Vec::new(),
            all_tool_invocations: Vec::new(),
            state: parser_types::PackageScriptParseState::ParseError {
                reason: err.to_string(),
            },
        }
    })
}

/// Convert the commands carried by `fact` into the public types-crate
/// representation.
fn script_commands(
    fact: &parser_types::PackageScriptParseFact,
) -> Vec<package_types::G3TsPackageScriptCommand> {
    fact.commands
        .iter()
        .map(|command| script_command(&fact.script_name, command))
        .collect()
}

/// Convert a single parser command into the public types-crate
/// representation.
fn script_command(
    script_name: &str,
    command: &parser_types::PackageScriptCommand,
) -> package_types::G3TsPackageScriptCommand {
    package_types::G3TsPackageScriptCommand {
        script_name: script_name.to_owned(),
        invocation: command.invocation.clone(),
        executable: command.executable.clone(),
        args: command.args.clone(),
        preceded_by: command.preceded_by.map(script_command_separator),
    }
}

/// Convert the parser separator enum into the public types-crate enum.
const fn script_command_separator(
    separator: parser_types::PackageScriptCommandSeparator,
) -> package_types::G3TsPackageScriptCommandSeparator {
    match separator {
        parser_types::PackageScriptCommandSeparator::And => {
            package_types::G3TsPackageScriptCommandSeparator::And
        }
        parser_types::PackageScriptCommandSeparator::Or => {
            package_types::G3TsPackageScriptCommandSeparator::Or
        }
    }
}

/// Convert the safe tool-invocation list carried by `fact` into the public
/// types-crate representation.
fn script_tool_invocations(
    fact: &parser_types::PackageScriptParseFact,
) -> Vec<package_types::G3TsPackageScriptToolInvocation> {
    fact.tool_invocations
        .iter()
        .map(script_tool_invocation)
        .collect()
}

/// Convert a single parser tool-invocation into the public types-crate
/// representation.
fn script_tool_invocation(
    invocation: &parser_types::PackageScriptToolInvocation,
) -> package_types::G3TsPackageScriptToolInvocation {
    package_types::G3TsPackageScriptToolInvocation {
        script_name: invocation.script_name.clone(),
        command_index: invocation.command_index,
        invocation: invocation.invocation.clone(),
        executable: invocation.executable.clone(),
        args: invocation.args.clone(),
        preceded_by: invocation.preceded_by.map(script_command_separator),
        followed_by: invocation.followed_by.map(script_command_separator),
    }
}

/// Project a script parse fact into a `package_types::G3TsPackageScriptParseBlocker` when
/// the parser reports an unsupported or error state, returning `None` when
/// the script parsed successfully.
fn script_parse_blocker(
    fact: &parser_types::PackageScriptParseFact,
) -> Option<package_types::G3TsPackageScriptParseBlocker> {
    match &fact.state {
        parser_types::PackageScriptParseState::Unsupported { reason }
        | parser_types::PackageScriptParseState::ParseError { reason } => {
            Some(package_types::G3TsPackageScriptParseBlocker {
                script_name: fact.script_name.clone(),
                reason: reason.clone(),
            })
        }
        parser_types::PackageScriptParseState::Parsed { .. }
        | parser_types::PackageScriptParseState::NoEslintInvocation => None,
    }
}

/// Ingest a single non-root `package.json` manifest into the corresponding
/// `package_types::G3TsPackageLocalState` variant.
fn ingest_local(entry: &G3WorkspaceEntry) -> package_types::G3TsPackageLocalState {
    if !entry.readable {
        return package_types::G3TsPackageLocalState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the local manifest unreadable".to_owned(),
        };
    }

    let document = match package_json_parser::from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(err) => {
            return package_types::G3TsPackageLocalState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: err.to_string(),
            };
        }
    };

    if let Some(reason) = package_json_parser::parse_error_reason(&document) {
        return package_types::G3TsPackageLocalState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let Some(snapshot) = package_json_parser::typed(&document) else {
        return package_types::G3TsPackageLocalState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: "parsed package.json document did not yield a typed snapshot".to_owned(),
        };
    };
    package_types::G3TsPackageLocalState::Parsed {
        snapshot: package_types::local_snapshot(&entry.path.rel_path, snapshot),
    }
}

/// Returns `true` when `entry` is a `package.json` file that should be
/// ingested as a local manifest under the active root policy.
fn is_local_package_json(entry: &G3WorkspaceEntry, root_policy_applies: bool) -> bool {
    entry.kind == G3WorkspaceEntryKind::File
        && if root_policy_applies {
            entry.path.rel_path.ends_with("/package.json")
        } else {
            entry.path.rel_path == "package.json" || entry.path.rel_path.ends_with("/package.json")
        }
}

/// Returns `true` when the workspace is a pnpm package-manager root that
/// the package family policy applies to.
fn root_policy_applies(crawl: &G3WorkspaceCrawl) -> bool {
    root_file(crawl, PNPM_WORKSPACE_REL_PATH).is_some()
        || root_file(crawl, "pnpm-lock.yaml").is_some()
}

/// Ingest the workspace-root `pnpm-workspace.yaml` policy file.
fn ingest_pnpm_workspace(crawl: &G3WorkspaceCrawl) -> package_types::G3TsPackagePnpmWorkspaceState {
    let Some(entry) = root_file(crawl, PNPM_WORKSPACE_REL_PATH) else {
        return package_types::G3TsPackagePnpmWorkspaceState::Missing {
            rel_path: PNPM_WORKSPACE_REL_PATH.to_owned(),
        };
    };

    if !entry.readable {
        return package_types::G3TsPackagePnpmWorkspaceState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the pnpm workspace file unreadable".to_owned(),
        };
    }

    let content = match crate::fs::read_to_string(&entry.path.abs_path) {
        Ok(content) => content,
        Err(error) => {
            return package_types::G3TsPackagePnpmWorkspaceState::Unreadable {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };
    let policy = match serde_norway::from_str::<PnpmWorkspacePolicy>(&content) {
        Ok(policy) => policy,
        Err(error) => {
            return package_types::G3TsPackagePnpmWorkspaceState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    package_types::G3TsPackagePnpmWorkspaceState::Parsed {
        snapshot: package_types::G3TsPackagePnpmWorkspaceSnapshot {
            rel_path: entry.path.rel_path.clone(),
            node_version: policy.node_version,
            engine_strict: policy.engine_strict,
        },
    }
}

/// Ingest the workspace-root `.syncpackrc` config file, capturing the
/// missing required source entries and the missing forbidden-dependency bans
/// for downstream checks.
fn ingest_syncpack_config(
    crawl: &G3WorkspaceCrawl,
    locals: &[package_types::G3TsPackageLocalState],
) -> package_types::G3TsPackageSyncpackConfigState {
    let Some(entry) = root_file(crawl, SYNCPACK_CONFIG_REL_PATH) else {
        return package_types::G3TsPackageSyncpackConfigState::Missing {
            rel_path: SYNCPACK_CONFIG_REL_PATH.to_owned(),
        };
    };

    if !entry.readable {
        return package_types::G3TsPackageSyncpackConfigState::Unreadable {
            rel_path: entry.path.rel_path.clone(),
            reason: "workspace crawl marked the Syncpack config unreadable".to_owned(),
        };
    }

    let document = match syncpack_config_parser::from_path_document(&entry.path.abs_path) {
        Ok(document) => document,
        Err(error) => {
            return package_types::G3TsPackageSyncpackConfigState::ParseError {
                rel_path: entry.path.rel_path.clone(),
                reason: error.to_string(),
            };
        }
    };

    if let Some(reason) = syncpack_config_parser::parse_error_reason(&document) {
        return package_types::G3TsPackageSyncpackConfigState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: reason.to_owned(),
        };
    }

    let Some(typed) = syncpack_config_parser::typed(&document) else {
        return package_types::G3TsPackageSyncpackConfigState::ParseError {
            rel_path: entry.path.rel_path.clone(),
            reason: "parsed Syncpack config document did not yield a typed snapshot".to_owned(),
        };
    };
    let required_sources = required_syncpack_source_entries(locals);
    let missing_source_entries = required_sources
        .iter()
        .filter(|source| !typed.source.iter().any(|declared| declared == *source))
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

    package_types::G3TsPackageSyncpackConfigState::Parsed {
        snapshot: package_types::G3TsPackageSyncpackConfigSnapshot {
            rel_path: entry.path.rel_path.clone(),
            missing_source_entries,
            missing_forbidden_bans,
        },
    }
}

/// Collect the set of `source` entries the Syncpack config must declare:
/// the workspace root manifest plus every local manifest path.
fn required_syncpack_source_entries(
    locals: &[package_types::G3TsPackageLocalState],
) -> Vec<String> {
    let mut sources = BTreeSet::from([PACKAGE_JSON_REL_PATH.to_owned()]);
    for local in locals {
        let path = match local {
            package_types::G3TsPackageLocalState::Unreadable { rel_path, .. }
            | package_types::G3TsPackageLocalState::ParseError { rel_path, .. } => rel_path.clone(),
            package_types::G3TsPackageLocalState::Parsed { snapshot } => snapshot.rel_path.clone(),
        };
        let _ = sources.insert(path);
    }
    sources.into_iter().collect()
}

/// Returns `true` when one of the first `prefix_len` version groups bans
/// `dependency` across exactly `dependency_types`.
fn has_canonical_ban_in_prefix(
    version_groups: &[SyncpackVersionGroup],
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

/// Returns `true` when `group` targets exactly `dependency` across
/// exactly `dependency_types`.
fn group_targets_dependency(
    group: &SyncpackVersionGroup,
    dependency: &str,
    dependency_types: &[&str],
) -> bool {
    strings_match_exactly(&group.dependencies, &[dependency])
        && strings_match_exactly(&group.dependency_types, dependency_types)
}

/// Returns `true` when `group` is shaped exactly like the canonical
/// forbidden-dependency ban (no extra fields, `is_banned: true`, etc.).
fn canonical_ban_group(group: &SyncpackVersionGroup) -> bool {
    group.packages.is_none()
        && group.specifier_types.is_none()
        && group.is_ignored.is_none()
        && group.is_banned == Some(true)
        && group.pin_version.is_none()
}

/// Returns `true` when `left` and `right` contain the same strings in the
/// same order.
fn strings_match_exactly(left: &[String], right: &[&str]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .map(String::as_str)
            .zip(right.iter().copied())
            .all(|(left, right)| left == right)
}
