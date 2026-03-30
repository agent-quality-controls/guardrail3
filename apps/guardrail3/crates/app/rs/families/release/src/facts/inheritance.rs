use crate::release_support::binaries::{is_publishable, string_field_present, valid_semver};

pub(super) fn inherited_string_field_present(
    package: Option<&toml::Value>,
    workspace_package: Option<&toml::Value>,
    field: &str,
) -> bool {
    string_field_present(package, field)
        || package
            .and_then(|package| package.get(field))
            .and_then(toml::Value::as_table)
            .and_then(|table| table.get("workspace"))
            .and_then(toml::Value::as_bool)
            .is_some_and(|workspace| workspace)
            && string_field_present(workspace_package, field)
}

pub(super) fn inherited_license_present(
    package: Option<&toml::Value>,
    workspace_package: Option<&toml::Value>,
) -> bool {
    inherited_string_field_present(package, workspace_package, "license")
        || inherited_string_field_present(package, workspace_package, "license-file")
}

pub(super) fn docs_rs_table<'a>(
    metadata: &'a toml::Value,
) -> Option<&'a toml::map::Map<String, toml::Value>> {
    metadata
        .get("docs.rs")
        .and_then(toml::Value::as_table)
        .or_else(|| {
            metadata
                .get("docs")
                .and_then(toml::Value::as_table)
                .and_then(|docs| docs.get("rs"))
                .and_then(toml::Value::as_table)
        })
}

pub(super) fn has_supported_docs_rs_settings(table: &toml::map::Map<String, toml::Value>) -> bool {
    const SUPPORTED_KEYS: &[&str] = &[
        "all-features",
        "features",
        "no-default-features",
        "default-target",
        "targets",
        "rustdoc-args",
        "cargo-args",
    ];

    SUPPORTED_KEYS.iter().any(|key| table.contains_key(*key))
}

pub(super) fn inherited_array_count(
    package: Option<&toml::Value>,
    workspace_package: Option<&toml::Value>,
    field: &str,
) -> Option<usize> {
    package
        .and_then(|package| package.get(field))
        .and_then(toml::Value::as_array)
        .map(Vec::len)
        .or_else(|| {
            package
                .and_then(|package| package.get(field))
                .and_then(toml::Value::as_table)
                .and_then(|table| table.get("workspace"))
                .and_then(toml::Value::as_bool)
                .is_some_and(|workspace| workspace)
                .then(|| {
                    workspace_package
                        .and_then(|workspace_package| workspace_package.get(field))
                        .and_then(toml::Value::as_array)
                        .map(Vec::len)
                })
                .flatten()
        })
}

pub(super) fn inherited_publishable(
    package: Option<&toml::Value>,
    workspace_package: Option<&toml::Value>,
) -> bool {
    if !is_publishable(package) {
        return false;
    }
    let inherits = package
        .and_then(|package| package.get("publish"))
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    if !inherits {
        return true;
    }
    is_publishable(workspace_package)
}

pub(super) fn inherited_readme_declared_false(
    package: Option<&toml::Value>,
    workspace_package: Option<&toml::Value>,
) -> bool {
    package
        .and_then(|package| package.get("readme"))
        .and_then(toml::Value::as_bool)
        .is_some_and(|value| !value)
        || package
            .and_then(|package| package.get("readme"))
            .and_then(toml::Value::as_table)
            .and_then(|table| table.get("workspace"))
            .and_then(toml::Value::as_bool)
            .is_some_and(|workspace| workspace)
            && workspace_package
                .and_then(|workspace_package| workspace_package.get("readme"))
                .and_then(toml::Value::as_bool)
                .is_some_and(|value| !value)
}

pub(super) fn inherited_readme_path<'a>(
    package: Option<&'a toml::Value>,
    workspace_package: Option<&'a toml::Value>,
) -> (Option<&'a str>, bool) {
    if let Some(local) = package
        .and_then(|package| package.get("readme"))
        .and_then(toml::Value::as_str)
    {
        return (Some(local), false);
    }
    let inherited = package
        .and_then(|package| package.get("readme"))
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    if !inherited {
        return (None, false);
    }
    (
        workspace_package
            .and_then(|workspace_package| workspace_package.get("readme"))
            .and_then(toml::Value::as_str),
        true,
    )
}

pub(super) fn inherited_version_string(
    package: Option<&toml::Value>,
    workspace_package: Option<&toml::Value>,
) -> Option<String> {
    let version_value = package.and_then(|package| package.get("version"));
    if let Some(version) = version_value.and_then(toml::Value::as_str) {
        return Some(version.to_owned());
    }
    let inherits = version_value
        .and_then(toml::Value::as_table)
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(false);
    if !inherits {
        return None;
    }
    workspace_package
        .and_then(|workspace_package| workspace_package.get("version"))
        .and_then(toml::Value::as_str)
        .map(str::to_owned)
}

pub(super) fn has_include_or_exclude_patterns(package: &toml::Value) -> bool {
    has_pattern_entries(package.get("include")) || has_pattern_entries(package.get("exclude"))
}

fn has_pattern_entries(value: Option<&toml::Value>) -> bool {
    value
        .and_then(toml::Value::as_array)
        .is_some_and(|entries| !entries.is_empty())
}

pub(super) fn version_is_valid(version: Option<&str>) -> bool {
    version.is_some_and(valid_semver)
}
