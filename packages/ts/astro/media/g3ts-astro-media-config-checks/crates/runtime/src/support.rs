use g3ts_astro_media_types::{
    G3TsAstroConfigSurfaceSnapshot, G3TsAstroConfigSurfaceState, G3TsAstroMediaEslintSurfaceState,
    G3TsAstroMediaPolicySnapshot, G3TsAstroMediaPolicySurfaceState,
    G3TsAstroPackageScriptCommandSeparator, G3TsAstroPackageScriptToolInvocation,
    G3TsAstroPackageSurfaceSnapshot, G3TsAstroPackageSurfaceState, G3TsAstroStaticObjectProperty,
    G3TsAstroStaticValue,
};
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub(crate) fn parsed_package(
    package: &G3TsAstroPackageSurfaceState,
) -> Option<&G3TsAstroPackageSurfaceSnapshot> {
    match package {
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroPackageSurfaceState::Missing { .. }
        | G3TsAstroPackageSurfaceState::Unreadable { .. }
        | G3TsAstroPackageSurfaceState::ParseError { .. } => None,
    }
}

pub(crate) fn package_rel_path(package: &G3TsAstroPackageSurfaceState) -> Option<&str> {
    match package {
        G3TsAstroPackageSurfaceState::Missing { rel_path }
        | G3TsAstroPackageSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroPackageSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroPackageSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

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

pub(crate) fn package_safely_runs_astro_build(package: &G3TsAstroPackageSurfaceState) -> bool {
    parsed_package(package).is_some_and(|snapshot| {
        snapshot.script_tool_invocations.iter().any(|invocation| {
            invocation_targets_tool(invocation, "astro", "build")
                && invocation.script_name == "validate"
                && script_has_no_parse_blocker(snapshot, &invocation.script_name)
                && script_commands_are_fail_closed(snapshot, &invocation.script_name)
                && invocation.preceded_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
                && invocation.followed_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
        })
    })
}

fn script_has_no_parse_blocker(
    snapshot: &G3TsAstroPackageSurfaceSnapshot,
    script_name: &str,
) -> bool {
    snapshot
        .script_parse_blockers
        .iter()
        .all(|blocker| blocker.script_name != script_name)
}

fn script_commands_are_fail_closed(
    snapshot: &G3TsAstroPackageSurfaceSnapshot,
    script_name: &str,
) -> bool {
    snapshot.script_commands.iter().all(|command| {
        command.script_name != script_name
            || command.preceded_by != Some(G3TsAstroPackageScriptCommandSeparator::Or)
    })
}

fn invocation_targets_tool(
    invocation: &G3TsAstroPackageScriptToolInvocation,
    executable: &str,
    first_arg: &str,
) -> bool {
    invocation.executable == executable
        && invocation.args.first().is_some_and(|arg| arg == first_arg)
}

pub(crate) fn astro_config_rel_path(config: &G3TsAstroConfigSurfaceState) -> Option<&str> {
    match config {
        G3TsAstroConfigSurfaceState::Missing { rel_path }
        | G3TsAstroConfigSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroConfigSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

pub(crate) fn parsed_astro_config(
    config: &G3TsAstroConfigSurfaceState,
) -> Option<&G3TsAstroConfigSurfaceSnapshot> {
    match config {
        G3TsAstroConfigSurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroConfigSurfaceState::Missing { .. }
        | G3TsAstroConfigSurfaceState::Unreadable { .. }
        | G3TsAstroConfigSurfaceState::ParseError { .. } => None,
    }
}

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
        | G3TsAstroStaticValue::ImportedIdentifier { .. } => None,
    }
}

pub(crate) fn object_has_only_allowed_keys(
    properties: &[G3TsAstroStaticObjectProperty],
    allowed: &[&str],
) -> bool {
    properties
        .iter()
        .all(|property| allowed.contains(&property.key.as_str()))
}

pub(crate) fn property_value<'a>(
    properties: &'a [G3TsAstroStaticObjectProperty],
    key: &str,
) -> Option<&'a G3TsAstroStaticValue> {
    properties
        .iter()
        .find(|property| property.key == key)
        .map(|property| &property.value)
}

pub(crate) fn property_string<'a>(
    properties: &'a [G3TsAstroStaticObjectProperty],
    key: &str,
) -> Option<&'a str> {
    match property_value(properties, key) {
        Some(G3TsAstroStaticValue::String(value)) => Some(value),
        _ => None,
    }
}

pub(crate) fn property_bool(
    properties: &[G3TsAstroStaticObjectProperty],
    key: &str,
) -> Option<bool> {
    match property_value(properties, key) {
        Some(G3TsAstroStaticValue::Bool(value)) => Some(*value),
        _ => None,
    }
}

pub(crate) fn property_string_array(
    properties: &[G3TsAstroStaticObjectProperty],
    key: &str,
) -> Option<Vec<String>> {
    let Some(G3TsAstroStaticValue::Array(values)) = property_value(properties, key) else {
        return None;
    };
    values
        .iter()
        .map(|value| match value {
            G3TsAstroStaticValue::String(item) => Some(item.clone()),
            G3TsAstroStaticValue::Bool(_)
            | G3TsAstroStaticValue::Number(_)
            | G3TsAstroStaticValue::Null
            | G3TsAstroStaticValue::Array(_)
            | G3TsAstroStaticValue::Object(_)
            | G3TsAstroStaticValue::ImportedIdentifier { .. } => None,
        })
        .collect()
}

pub(crate) fn parsed_media_policy(
    policy: &G3TsAstroMediaPolicySurfaceState,
) -> Option<&G3TsAstroMediaPolicySnapshot> {
    match policy {
        G3TsAstroMediaPolicySurfaceState::Parsed { snapshot } => Some(snapshot),
        G3TsAstroMediaPolicySurfaceState::Missing { .. }
        | G3TsAstroMediaPolicySurfaceState::Unreadable { .. }
        | G3TsAstroMediaPolicySurfaceState::ParseError { .. }
        | G3TsAstroMediaPolicySurfaceState::MissingAstroPolicy { .. }
        | G3TsAstroMediaPolicySurfaceState::MissingMediaPolicy { .. } => None,
    }
}

pub(crate) fn media_policy_rel_path(policy: &G3TsAstroMediaPolicySurfaceState) -> Option<&str> {
    match policy {
        G3TsAstroMediaPolicySurfaceState::Missing { rel_path }
        | G3TsAstroMediaPolicySurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroMediaPolicySurfaceState::ParseError { rel_path, .. }
        | G3TsAstroMediaPolicySurfaceState::MissingAstroPolicy { rel_path }
        | G3TsAstroMediaPolicySurfaceState::MissingMediaPolicy { rel_path } => Some(rel_path),
        G3TsAstroMediaPolicySurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

pub(crate) fn eslint_rel_path(config: &G3TsAstroMediaEslintSurfaceState) -> Option<&str> {
    match config {
        G3TsAstroMediaEslintSurfaceState::Missing { rel_path }
        | G3TsAstroMediaEslintSurfaceState::Unreadable { rel_path, .. }
        | G3TsAstroMediaEslintSurfaceState::ParseError { rel_path, .. } => Some(rel_path),
        G3TsAstroMediaEslintSurfaceState::Parsed { snapshot } => Some(&snapshot.rel_path),
    }
}

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
