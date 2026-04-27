use super::prelude::*;
use super::constants::*;
use g3ts_astro_types::{
    G3TsAstroPackageScriptCommand, G3TsAstroPackageScriptCommandSeparator,
    G3TsAstroPackageScriptParseBlocker, G3TsAstroPackageScriptToolInvocation,
};

pub fn ingest_package_surface(
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


pub(super) fn package_surface_has_astro_dependency(package: &G3TsAstroPackageSurfaceState) -> bool {
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
