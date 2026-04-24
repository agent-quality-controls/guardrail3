use serde_json::{Map, Value};
use syncpack_config_parser_types::document::{
    SyncpackConfigDocument, SyncpackConfigParseState, SyncpackConfigSnapshot, SyncpackVersionGroup,
};

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized Syncpack config parser"
)]
pub fn parse(input: &str) -> Result<SyncpackConfigSnapshot, crate::error::Error> {
    let raw: Value =
        serde_json::from_str(input).map_err(|err| crate::error::Error::Json(err.to_string()))?;
    normalize_snapshot(&raw).map_err(crate::error::Error::Json)
}

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized Syncpack config parser"
)]
pub fn parse_document(input: &str) -> Result<SyncpackConfigDocument, crate::error::Error> {
    let raw: Value =
        serde_json::from_str(input).map_err(|err| crate::error::Error::Json(err.to_string()))?;
    let typed = match normalize_snapshot(&raw) {
        Ok(snapshot) => SyncpackConfigParseState::Parsed(Box::new(snapshot)),
        Err(reason) => SyncpackConfigParseState::Invalid(reason),
    };
    Ok(SyncpackConfigDocument { raw, typed })
}

pub fn from_path(
    path: impl AsRef<std::path::Path>,
) -> Result<SyncpackConfigSnapshot, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

pub fn from_path_document(
    path: impl AsRef<std::path::Path>,
) -> Result<SyncpackConfigDocument, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse_document(&content)
}

#[must_use]
pub fn typed(document: &SyncpackConfigDocument) -> Option<&SyncpackConfigSnapshot> {
    match &document.typed {
        SyncpackConfigParseState::Parsed(snapshot) => Some(snapshot),
        SyncpackConfigParseState::Invalid(_) => None,
    }
}

#[must_use]
pub fn parse_error_reason(document: &SyncpackConfigDocument) -> Option<&str> {
    match &document.typed {
        SyncpackConfigParseState::Parsed(_) => None,
        SyncpackConfigParseState::Invalid(reason) => Some(reason),
    }
}

fn normalize_snapshot(raw: &Value) -> Result<SyncpackConfigSnapshot, String> {
    let root = raw
        .as_object()
        .ok_or_else(|| "Syncpack config root must be a JSON object".to_owned())?;

    Ok(SyncpackConfigSnapshot {
        source: normalize_string_array(root.get("source"), "source")?,
        version_groups: normalize_version_groups(root.get("versionGroups"))?,
    })
}

fn normalize_version_groups(value: Option<&Value>) -> Result<Vec<SyncpackVersionGroup>, String> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    let groups = value
        .as_array()
        .ok_or_else(|| "Syncpack config field `versionGroups` must be an array".to_owned())?;

    groups
        .iter()
        .enumerate()
        .map(|(index, value)| normalize_version_group(value, index))
        .collect()
}

fn normalize_version_group(value: &Value, index: usize) -> Result<SyncpackVersionGroup, String> {
    let object = value.as_object().ok_or_else(|| {
        format!("Syncpack config field `versionGroups[{index}]` must be an object")
    })?;

    Ok(SyncpackVersionGroup {
        label: normalize_optional_string(object, "label", index)?,
        dependencies: normalize_string_array(object.get("dependencies"), "dependencies")?,
        dependency_types: normalize_string_array(object.get("dependencyTypes"), "dependencyTypes")?,
        packages: normalize_string_array(object.get("packages"), "packages")?,
        specifier_types: normalize_string_array(object.get("specifierTypes"), "specifierTypes")?,
        pin_version: normalize_optional_string(object, "pinVersion", index)?,
        is_banned: normalize_optional_bool(object, "isBanned", index)?.unwrap_or(false),
        is_ignored: normalize_optional_bool(object, "isIgnored", index)?.unwrap_or(false),
    })
}

fn normalize_optional_string(
    object: &Map<String, Value>,
    field_name: &str,
    index: usize,
) -> Result<Option<String>, String> {
    let Some(value) = object.get(field_name) else {
        return Ok(None);
    };
    value
        .as_str()
        .map(|item| Some(item.to_owned()))
        .ok_or_else(|| {
            format!("Syncpack config field `versionGroups[{index}].{field_name}` must be a string")
        })
}

fn normalize_optional_bool(
    object: &Map<String, Value>,
    field_name: &str,
    index: usize,
) -> Result<Option<bool>, String> {
    let Some(value) = object.get(field_name) else {
        return Ok(None);
    };
    value.as_bool().map(Some).ok_or_else(|| {
        format!("Syncpack config field `versionGroups[{index}].{field_name}` must be a boolean")
    })
}

fn normalize_string_array(value: Option<&Value>, field_name: &str) -> Result<Vec<String>, String> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    let array = value
        .as_array()
        .ok_or_else(|| format!("Syncpack config field `{field_name}` must be an array"))?;

    array
        .iter()
        .enumerate()
        .map(|(index, item)| {
            item.as_str().map(str::to_owned).ok_or_else(|| {
                format!("Syncpack config field `{field_name}[{index}]` must be a string")
            })
        })
        .collect()
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"]
mod parser_tests;
