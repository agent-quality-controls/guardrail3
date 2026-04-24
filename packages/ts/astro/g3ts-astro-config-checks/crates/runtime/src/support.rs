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
pub(crate) fn package_safely_runs_astro_check(
    contract: &G3TsAstroIntegrationContractInput,
) -> bool {
    parsed_package(contract).is_some_and(|snapshot| snapshot.safely_runs_astro_check)
}

#[must_use]
pub(crate) fn package_safely_runs_syncpack_lint(
    contract: &G3TsAstroIntegrationContractInput,
) -> bool {
    parsed_package(contract).is_some_and(|snapshot| snapshot.safely_runs_syncpack_lint)
}

#[must_use]
pub(crate) fn package_invokes_tool(
    contract: &G3TsAstroIntegrationContractInput,
    executable: &str,
    first_arg: &str,
) -> bool {
    parsed_package(contract).is_some_and(|snapshot| {
        snapshot.script_tool_invocations.iter().any(|invocation| {
            invocation.executable == executable
                && invocation.args.first().is_some_and(|arg| arg == first_arg)
        })
    })
}

#[must_use]
pub(crate) fn expected_syncpack_source_entry(
    syncpack_rel_path: &str,
    package_rel_path: &str,
) -> Option<String> {
    let config_dir = syncpack_rel_path
        .rsplit_once('/')
        .map_or("", |(parent, _)| parent);
    if config_dir.is_empty() {
        return Some(package_rel_path.to_owned());
    }

    package_rel_path
        .strip_prefix(&format!("{config_dir}/"))
        .map(str::to_owned)
}

#[must_use]
pub(crate) fn required_syncpack_pins_message(
    contract: &G3TsAstroIntegrationContractInput,
) -> String {
    contract
        .required_syncpack_pins
        .iter()
        .map(|pin| format!("`{}` -> `{}`", pin.dependency, pin.version))
        .collect::<Vec<_>>()
        .join(", ")
}

#[must_use]
pub(crate) fn forbidden_syncpack_deps_message(
    contract: &G3TsAstroIntegrationContractInput,
) -> String {
    contract
        .forbidden_syncpack_deps
        .iter()
        .map(|dependency| format!("`{dependency}`"))
        .collect::<Vec<_>>()
        .join(", ")
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
    content_data_rules: &[&str],
    content_source_rules: &[&str],
) -> bool {
    parsed_eslint_surface(contract).is_some_and(|snapshot| {
        lane_has_plugin_and_rules(
            snapshot.astro_source_probe_present,
            &snapshot.astro_source_plugins,
            &snapshot.astro_source_error_rules,
            Some(&snapshot.astro_source_effective_route_scoped_pipeline_rules),
            Some(&snapshot.astro_source_effective_content_data_pipeline_rules),
            Some(&snapshot.astro_source_effective_content_source_pipeline_rules),
            plugin_name,
            required_rules,
            route_scoped_rules,
            content_data_rules,
            content_source_rules,
        ) && lane_has_plugin_and_rules(
            snapshot.ts_source_probe_present,
            &snapshot.ts_source_plugins,
            &snapshot.ts_source_error_rules,
            Some(&snapshot.ts_source_effective_route_scoped_pipeline_rules),
            Some(&snapshot.ts_source_effective_content_data_pipeline_rules),
            Some(&snapshot.ts_source_effective_content_source_pipeline_rules),
            plugin_name,
            required_rules,
            route_scoped_rules,
            content_data_rules,
            content_source_rules,
        ) && lane_has_plugin_and_rules(
            snapshot.tsx_source_probe_present,
            &snapshot.tsx_source_plugins,
            &snapshot.tsx_source_error_rules,
            Some(&snapshot.tsx_source_effective_route_scoped_pipeline_rules),
            Some(&snapshot.tsx_source_effective_content_data_pipeline_rules),
            Some(&snapshot.tsx_source_effective_content_source_pipeline_rules),
            plugin_name,
            required_rules,
            route_scoped_rules,
            content_data_rules,
            content_source_rules,
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
            None,
            None,
            plugin_name,
            &[],
            &[],
            &[],
            &[],
        ) && lane_has_plugin_and_rules(
            snapshot.ts_source_probe_present,
            &snapshot.ts_source_plugins,
            &snapshot.ts_source_error_rules,
            None,
            None,
            None,
            plugin_name,
            &[],
            &[],
            &[],
            &[],
        ) && lane_has_plugin_and_rules(
            snapshot.tsx_source_probe_present,
            &snapshot.tsx_source_plugins,
            &snapshot.tsx_source_error_rules,
            None,
            None,
            None,
            plugin_name,
            &[],
            &[],
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

fn lane_has_plugin_and_rules(
    lane_present: bool,
    plugins: &[String],
    error_rules: &[String],
    effective_route_scoped_rules: Option<&[String]>,
    effective_content_data_rules: Option<&[String]>,
    effective_content_source_rules: Option<&[String]>,
    plugin_name: &str,
    required_rules: &[&str],
    route_scoped_rules: &[&str],
    content_data_rules: &[&str],
    content_source_rules: &[&str],
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

    let effective_route_scope = effective_route_scoped_rules.map(|rules| {
        rules
            .iter()
            .map(String::as_str)
            .collect::<std::collections::BTreeSet<_>>()
    });
    let effective_content_data_scope = effective_content_data_rules.map(|rules| {
        rules
            .iter()
            .map(String::as_str)
            .collect::<std::collections::BTreeSet<_>>()
    });
    let effective_content_source_scope = effective_content_source_rules.map(|rules| {
        rules
            .iter()
            .map(String::as_str)
            .collect::<std::collections::BTreeSet<_>>()
    });

    required_rules
        .iter()
        .all(|required_rule| enabled_rules.contains(*required_rule))
        && route_scoped_rules.iter().all(|required_rule| {
            effective_route_scope
                .as_ref()
                .is_none_or(|effective_rules| effective_rules.contains(*required_rule))
        })
        && content_data_rules.iter().all(|required_rule| {
            effective_content_data_scope
                .as_ref()
                .is_none_or(|effective_rules| effective_rules.contains(*required_rule))
        })
        && content_source_rules.iter().all(|required_rule| {
            effective_content_source_scope
                .as_ref()
                .is_none_or(|effective_rules| effective_rules.contains(*required_rule))
        })
}
