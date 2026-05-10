use g3ts_astro_seo_types::{
    G3TsAstroConfigSurfaceSnapshot, G3TsAstroConfigSurfaceState, G3TsAstroOutputMode,
    G3TsAstroPackageScriptCommandSeparator, G3TsAstroPackageScriptToolInvocation,
    G3TsAstroPackageSurfaceSnapshot, G3TsAstroPackageSurfaceState, G3TsAstroSeoPolicySnapshot,
    G3TsAstroSeoPolicySurfaceState, G3TsAstroStaticObjectProperty, G3TsAstroStaticValue,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Internal const helper exported within the runtime crate.
pub(crate) const fn parsed_package(
    package: &G3TsAstroPackageSurfaceState,
) -> Option<&G3TsAstroPackageSurfaceSnapshot> {
    match package {
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroPackageSurfaceState::Missing { .. }
        | G3TsAstroPackageSurfaceState::Unreadable { .. }
        | G3TsAstroPackageSurfaceState::ParseError { .. } => None,
    }
}

/// Internal helper exported within the runtime crate.
pub(crate) fn package_rel_path(package: &G3TsAstroPackageSurfaceState) -> &str {
    match package {
        G3TsAstroPackageSurfaceState::Missing { rel_path }
        | G3TsAstroPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroPackageSurfaceState::ParseError { rel_path, .. } => rel_path,
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}

/// Internal helper exported within the runtime crate.
pub(crate) fn package_has_dependency(
    package: &G3TsAstroPackageSurfaceState,
    dependency_name: &str,
) -> bool {
    parsed_package(package).is_some_and(|snapshot| {
        snapshot
            .dependencies
            .iter()
            .chain(snapshot.dev_dependencies.iter())
            .any(|dependency| dependency == dependency_name)
    })
}

/// Internal helper exported within the runtime crate.
pub(crate) fn package_mentions_dependency(
    package: &G3TsAstroPackageSurfaceState,
    dependency_name: &str,
) -> bool {
    parsed_package(package).is_some_and(|snapshot| {
        snapshot
            .dependencies
            .iter()
            .chain(snapshot.dev_dependencies.iter())
            .chain(snapshot.optional_dependencies.iter())
            .chain(snapshot.peer_dependencies.iter())
            .any(|dependency| dependency == dependency_name)
    })
}

/// Internal helper exported within the runtime crate.
pub(crate) fn package_safely_runs_astro_build(package: &G3TsAstroPackageSurfaceState) -> bool {
    package_safely_runs_tool(package, Some("build"), "astro", "build")
}

/// Internal helper used by the rule.
fn package_safely_runs_tool(
    package: &G3TsAstroPackageSurfaceState,
    script_name: Option<&str>,
    executable: &str,
    first_arg: &str,
) -> bool {
    parsed_package(package).is_some_and(|snapshot| {
        snapshot.script_tool_invocations.iter().any(|invocation| {
            invocation_targets_tool(invocation, executable, first_arg)
                && script_name.is_none_or(|expected| invocation.script_name == expected)
                && script_has_no_parse_blocker(snapshot, &invocation.script_name)
                && script_commands_are_fail_closed(snapshot, &invocation.script_name)
                && invocation.preceded_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
                && invocation.followed_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
        })
    })
}

/// Internal helper used by the rule.
fn script_has_no_parse_blocker(
    snapshot: &G3TsAstroPackageSurfaceSnapshot,
    script_name: &str,
) -> bool {
    snapshot
        .script_parse_blockers
        .iter()
        .all(|blocker| blocker.script_name != script_name)
}

/// Internal helper used by the rule.
fn script_commands_are_fail_closed(
    snapshot: &G3TsAstroPackageSurfaceSnapshot,
    script_name: &str,
) -> bool {
    snapshot.script_commands.iter().all(|command| {
        command.script_name != script_name
            || command.preceded_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
    })
}

/// Internal helper used by the rule.
fn invocation_targets_tool(
    invocation: &G3TsAstroPackageScriptToolInvocation,
    executable: &str,
    first_arg: &str,
) -> bool {
    invocation.executable == executable
        && invocation.args.first().is_some_and(|arg| arg == first_arg)
}

/// Internal const helper exported within the runtime crate.
pub(crate) const fn parsed_astro_config(
    config: &G3TsAstroConfigSurfaceState,
) -> Option<&G3TsAstroConfigSurfaceSnapshot> {
    match config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => None,
    }
}

/// Internal helper exported within the runtime crate.
pub(crate) fn astro_config_rel_path(config: &G3TsAstroConfigSurfaceState) -> &str {
    match config {
        G3TsAstroConfigSurfaceState::Missing { rel_path }
        | G3TsAstroConfigSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroConfigSurfaceState::ParseError { rel_path, .. } => rel_path,
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}

/// Internal helper exported within the runtime crate.
pub(crate) fn astro_config_is_static(config: &G3TsAstroConfigSurfaceState) -> bool {
    parsed_astro_config(config)
        .is_some_and(|snapshot| snapshot.output == Some(G3TsAstroOutputMode::Static))
}

/// Internal const helper exported within the runtime crate.
pub(crate) const fn parsed_seo_policy(
    policy: &G3TsAstroSeoPolicySurfaceState,
) -> Option<&G3TsAstroSeoPolicySnapshot> {
    match policy {
        G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroSeoPolicySurfaceState::Missing { .. }
        | G3TsAstroSeoPolicySurfaceState::Unreadable { .. }
        | G3TsAstroSeoPolicySurfaceState::ParseError { .. }
        | G3TsAstroSeoPolicySurfaceState::MissingAstroPolicy { .. } => None,
    }
}

/// Internal helper exported within the runtime crate.
pub(crate) fn seo_policy_rel_path(policy: &G3TsAstroSeoPolicySurfaceState) -> &str {
    match policy {
        G3TsAstroSeoPolicySurfaceState::Missing { rel_path }
        | G3TsAstroSeoPolicySurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroSeoPolicySurfaceState::ParseError { rel_path, .. }
        | G3TsAstroSeoPolicySurfaceState::MissingAstroPolicy { rel_path } => rel_path,
        G3TsAstroSeoPolicySurfaceState::Parsed { snapshot } => &snapshot.rel_path,
    }
}

/// Internal helper exported within the runtime crate.
pub(crate) fn astro_config_site_is_https(snapshot: &G3TsAstroConfigSurfaceSnapshot) -> bool {
    snapshot.site.as_deref().is_some_and(|site| {
        url::Url::parse(site).is_ok_and(|url| url.scheme() == "https" && url.host_str().is_some())
    })
}

/// Internal helper exported within the runtime crate.
pub(crate) fn astro_config_has_integration(
    snapshot: &G3TsAstroConfigSurfaceSnapshot,
    module: &str,
) -> bool {
    snapshot
        .integrations
        .iter()
        .any(|integration| integration.source_module.as_deref() == Some(module))
}

/// Internal generic helper exported within the runtime crate.
pub(crate) fn astro_config_integration_first_arg<'a>(
    snapshot: &'a G3TsAstroConfigSurfaceSnapshot,
    module: &str,
) -> Option<&'a G3TsAstroStaticValue> {
    snapshot
        .integrations
        .iter()
        .find(|integration| integration.source_module.as_deref() == Some(module))
        .and_then(|integration| integration.call.as_ref())
        .and_then(|call| call.first_arg.as_ref())
}

/// Internal helper exported within the runtime crate.
pub(crate) fn object_properties(
    value: &G3TsAstroStaticValue,
) -> Option<&[G3TsAstroStaticObjectProperty]> {
    match value {
        G3TsAstroStaticValue::Object(properties) => Some(properties),
        G3TsAstroStaticValue::Bool(_)
        | G3TsAstroStaticValue::Number(_)
        | G3TsAstroStaticValue::String(_)
        | G3TsAstroStaticValue::Null
        | G3TsAstroStaticValue::Array(_)
        | G3TsAstroStaticValue::ImportedIdentifier { .. }
        | G3TsAstroStaticValue::UnsupportedExpression { .. } => None,
    }
}

/// Internal helper exported within the runtime crate.
pub(crate) fn object_has_duplicate_keys(properties: &[G3TsAstroStaticObjectProperty]) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    properties
        .iter()
        .any(|property| !seen.insert(property.key.as_str()))
}

/// Internal helper exported within the runtime crate.
pub(crate) fn object_has_only_allowed_keys(
    properties: &[G3TsAstroStaticObjectProperty],
    allowed: &[&str],
) -> bool {
    properties
        .iter()
        .all(|property| allowed.contains(&property.key.as_str()))
}

/// Internal generic helper exported within the runtime crate.
pub(crate) fn property_value<'a>(
    properties: &'a [G3TsAstroStaticObjectProperty],
    key: &str,
) -> Option<&'a G3TsAstroStaticValue> {
    properties
        .iter()
        .find(|property| property.key == key)
        .map(|property| &property.value)
}

/// Internal generic helper exported within the runtime crate.
pub(crate) fn property_string<'a>(
    properties: &'a [G3TsAstroStaticObjectProperty],
    key: &str,
) -> Option<&'a str> {
    match property_value(properties, key) {
        Some(G3TsAstroStaticValue::String(value)) => Some(value),
        _ => None,
    }
}

/// Internal generic helper exported within the runtime crate.
pub(crate) fn property_array<'a>(
    properties: &'a [G3TsAstroStaticObjectProperty],
    key: &str,
) -> Option<&'a [G3TsAstroStaticValue]> {
    match property_value(properties, key) {
        Some(G3TsAstroStaticValue::Array(values)) => Some(values),
        _ => None,
    }
}

/// Internal helper exported within the runtime crate.
pub(crate) fn strict_ai_readable_enabled(policy: &G3TsAstroSeoPolicySurfaceState) -> bool {
    parsed_seo_policy(policy).is_some_and(|snapshot| snapshot.strict_ai_readable)
}

/// Internal helper exported within the runtime crate.
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

/// Internal helper exported within the runtime crate.
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

/// Internal helper exported within the runtime crate.
pub(crate) fn warning(id: &str, title: &str, message: String, file: Option<&str>) -> G3CheckResult {
    G3CheckResult::new(
        id.to_owned(),
        G3Severity::Warn,
        title.to_owned(),
        message,
        file.map(str::to_owned),
        None,
    )
    .into_inventory()
}
