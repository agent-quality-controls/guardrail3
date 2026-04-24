use g3ts_astro_types::{
    G3TsAstroEslintPluginContractInput, G3TsAstroEslintSurfaceSnapshot,
    G3TsAstroEslintSurfaceState, G3TsAstroIntegrationContractInput,
    G3TsAstroPackageSurfaceSnapshot, G3TsAstroPackageSurfaceState,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

#[must_use]
pub(crate) fn parsed_package(
    contract: &G3TsAstroIntegrationContractInput,
) -> Option<&G3TsAstroPackageSurfaceSnapshot> {
    match &contract.package {
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroPackageSurfaceState::Missing { .. }
        | G3TsAstroPackageSurfaceState::Unreadable { .. }
        | G3TsAstroPackageSurfaceState::ParseError { .. } => None,
    }
}

#[must_use]
pub(crate) fn package_rel_path(contract: &G3TsAstroIntegrationContractInput) -> Option<&str> {
    match &contract.package {
        G3TsAstroPackageSurfaceState::Missing { rel_path }
        | G3TsAstroPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroPackageSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

#[must_use]
pub(crate) fn package_has_dependency(
    contract: &G3TsAstroIntegrationContractInput,
    dependency_name: &str,
) -> bool {
    parsed_package(contract).is_some_and(|snapshot| {
        snapshot
            .dependencies
            .iter()
            .chain(snapshot.dev_dependencies.iter())
            .any(|dependency| dependency == dependency_name)
    })
}

#[must_use]
pub(crate) fn package_has_script_fragment(
    contract: &G3TsAstroIntegrationContractInput,
    fragment: &str,
) -> bool {
    parsed_package(contract)
        .is_some_and(|snapshot| snapshot.script_bodies.iter().any(|(_, body)| has_command(body, fragment)))
}

#[must_use]
pub(crate) fn parsed_eslint_surface(
    contract: &G3TsAstroEslintPluginContractInput,
) -> Option<&G3TsAstroEslintSurfaceSnapshot> {
    match &contract.config {
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroEslintSurfaceState::Missing { .. }
        | G3TsAstroEslintSurfaceState::Unreadable { .. }
        | G3TsAstroEslintSurfaceState::ParseError { .. } => None,
    }
}

#[must_use]
pub(crate) fn eslint_rel_path(contract: &G3TsAstroEslintPluginContractInput) -> Option<&str> {
    match &contract.config {
        G3TsAstroEslintSurfaceState::Missing { rel_path }
        | G3TsAstroEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

#[must_use]
pub(crate) fn eslint_required_lanes_have_effective_pipeline_rules(
    contract: &G3TsAstroEslintPluginContractInput,
    plugin_name: &str,
    required_rules: &[&str],
    route_scoped_rules: &[&str],
) -> bool {
    parsed_eslint_surface(contract).is_some_and(|snapshot| {
        lane_has_plugin_and_rules(
            snapshot.astro_source_probe_present,
            &snapshot.astro_source_plugins,
            &snapshot.astro_source_error_rules,
            Some(&snapshot.astro_source_effective_route_scoped_pipeline_rules),
            plugin_name,
            required_rules,
            route_scoped_rules,
        ) &&
        lane_has_plugin_and_rules(
            snapshot.ts_source_probe_present,
            &snapshot.ts_source_plugins,
            &snapshot.ts_source_error_rules,
            Some(&snapshot.ts_source_effective_route_scoped_pipeline_rules),
            plugin_name,
            required_rules,
            route_scoped_rules,
        ) && lane_has_plugin_and_rules(
            snapshot.tsx_source_probe_present,
            &snapshot.tsx_source_plugins,
            &snapshot.tsx_source_error_rules,
            Some(&snapshot.tsx_source_effective_route_scoped_pipeline_rules),
            plugin_name,
            required_rules,
            route_scoped_rules,
        )
    })
}

#[must_use]
pub(crate) fn eslint_required_lanes_have_plugin(
    contract: &G3TsAstroEslintPluginContractInput,
    plugin_name: &str,
) -> bool {
    parsed_eslint_surface(contract).is_some_and(|snapshot| {
        lane_has_plugin_and_rules(
            snapshot.astro_source_probe_present,
            &snapshot.astro_source_plugins,
            &snapshot.astro_source_error_rules,
            None,
            plugin_name,
            &[],
            &[],
        ) &&
        lane_has_plugin_and_rules(
            snapshot.ts_source_probe_present,
            &snapshot.ts_source_plugins,
            &snapshot.ts_source_error_rules,
            None,
            plugin_name,
            &[],
            &[],
        ) && lane_has_plugin_and_rules(
            snapshot.tsx_source_probe_present,
            &snapshot.tsx_source_plugins,
            &snapshot.tsx_source_error_rules,
            None,
            plugin_name,
            &[],
            &[],
        )
    })
}

#[must_use]
pub(crate) fn info(id: &str, title: &str, message: String, file: &str) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Info,
        title.to_owned(),
        message,
        Some(file.to_owned()),
        None,
    )
    .into_inventory()
}

#[must_use]
pub(crate) fn error(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Error,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
}

fn has_command(script_body: &str, command_fragment: &str) -> bool {
    let wanted = command_fragment.split_whitespace().collect::<Vec<_>>();
    let commands = shell_like_commands(script_body);

    commands
        .iter()
        .any(|command| command_invokes(command, &wanted))
}

fn lane_has_plugin_and_rules(
    lane_present: bool,
    plugins: &[String],
    error_rules: &[String],
    effective_route_scoped_rules: Option<&[String]>,
    plugin_name: &str,
    required_rules: &[&str],
    route_scoped_rules: &[&str],
) -> bool {
    if !lane_present {
        return true;
    }

    if !plugins.iter().any(|plugin| plugin == plugin_name) {
        return false;
    }

    let enabled_rules = error_rules
        .iter()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();

    let effective_route_scope = effective_route_scoped_rules
        .map(|rules| rules.iter().map(String::as_str).collect::<std::collections::BTreeSet<_>>());

    required_rules
        .iter()
        .all(|required_rule| enabled_rules.contains(*required_rule))
        && route_scoped_rules.iter().all(|required_rule| {
            effective_route_scope
                .as_ref()
                .is_none_or(|effective_rules| effective_rules.contains(*required_rule))
        })
}

fn command_invokes(command: &[String], wanted: &[&str]) -> bool {
    let mut start = 0;
    while command
        .get(start)
        .is_some_and(|token| token.contains('=') && !token.starts_with('='))
    {
        start += 1;
    }

    let remainder = &command[skip_command_wrapper(command, start)..];
    remainder.len() >= wanted.len()
        && remainder
            .iter()
            .take(wanted.len())
            .zip(wanted.iter())
            .all(|(left, right)| left == right)
}

fn skip_command_wrapper(command: &[String], start: usize) -> usize {
    let Some(first) = command.get(start).map(String::as_str) else {
        return start;
    };

    if matches!(first, "npx" | "bunx" | "pnpm" | "yarn") {
        return skip_wrapper_options(command, start + 1);
    }

    let Some(second) = command.get(start + 1).map(String::as_str) else {
        return start;
    };

    if matches!(
        (first, second),
        ("npm", "exec")
            | ("pnpm", "exec")
            | ("yarn", "exec")
            | ("pnpm", "dlx")
            | ("yarn", "dlx")
    ) {
        return skip_wrapper_options(command, start + 2);
    }

    start
}

fn skip_wrapper_options(command: &[String], mut index: usize) -> usize {
    while command
        .get(index)
        .is_some_and(|token| token == "--" || token.starts_with('-'))
    {
        index += 1;
    }

    index
}

fn shell_like_commands(script_body: &str) -> Vec<Vec<String>> {
    let mut commands = Vec::new();
    let mut current_command = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;

    for ch in script_body.chars() {
        if let Some(active_quote) = quote {
            if ch == active_quote {
                quote = None;
            }
            current.push(ch);
            continue;
        }

        match ch {
            '"' | '\'' => {
                quote = Some(ch);
            }
            ' ' | '\t' | '\n' | '\r' => {
                if !current.is_empty() {
                    current_command.push(std::mem::take(&mut current));
                }
            }
            ';' | '&' | '|' | '(' | ')' => {
                if !current.is_empty() {
                    current_command.push(std::mem::take(&mut current));
                }
                if !current_command.is_empty() {
                    commands.push(std::mem::take(&mut current_command));
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        current_command.push(current);
    }

    if !current_command.is_empty() {
        commands.push(current_command);
    }

    commands
}
