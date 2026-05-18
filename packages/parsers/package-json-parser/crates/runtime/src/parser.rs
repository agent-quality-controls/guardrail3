use std::collections::BTreeMap;

use package_json_parser_types::document::{
    PackageJsonDependencyDeclarationSnapshot, PackageJsonDocument, PackageJsonParseState,
    PackageJsonSnapshot,
};
use serde_json::{Map, Value};

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized package.json parser"
)]
pub fn parse(input: &str) -> Result<PackageJsonSnapshot, crate::error::Error> {
    let raw: Value =
        serde_json::from_str(input).map_err(|err| crate::error::Error::Json(err.to_string()))?;
    normalize_snapshot(&raw).map_err(crate::error::Error::Json)
}

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized package.json parser"
)]
pub fn parse_document(input: &str) -> Result<PackageJsonDocument, crate::error::Error> {
    let raw: Value =
        serde_json::from_str(input).map_err(|err| crate::error::Error::Json(err.to_string()))?;
    let typed = match normalize_snapshot(&raw) {
        Ok(snapshot) => PackageJsonParseState::Parsed(Box::new(snapshot)),
        Err(reason) => PackageJsonParseState::Invalid(reason),
    };
    Ok(PackageJsonDocument { raw, typed })
}

pub fn from_path(
    path: impl AsRef<std::path::Path>,
) -> Result<PackageJsonSnapshot, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

pub fn from_path_document(
    path: impl AsRef<std::path::Path>,
) -> Result<PackageJsonDocument, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse_document(&content)
}

fn normalize_snapshot(raw: &Value) -> Result<PackageJsonSnapshot, String> {
    let root = raw
        .as_object()
        .ok_or_else(|| "package.json root must be a JSON object".to_owned())?;

    Ok(PackageJsonSnapshot {
        name: normalize_optional_string(root.get("name"), "name")?,
        private_field: normalize_optional_bool(root.get("private"), "private")?,
        package_manager: normalize_optional_string(root.get("packageManager"), "packageManager")?,
        engines_node: normalize_optional_nested_string(root.get("engines"), "engines", "node")?,
        engines_pnpm: normalize_optional_nested_string(root.get("engines"), "engines", "pnpm")?,
        scripts: normalize_string_map(root.get("scripts"), "scripts")?,
        pnpm_override_keys: normalize_pnpm_override_keys(root.get("pnpm"))?,
        pnpm_only_built_dependencies: normalize_pnpm_only_built_dependencies(root.get("pnpm"))?,
        dependencies: normalize_dependency_names(root.get("dependencies"), "dependencies")?,
        dev_dependencies: normalize_dependency_names(
            root.get("devDependencies"),
            "devDependencies",
        )?,
        optional_dependencies: normalize_dependency_names(
            root.get("optionalDependencies"),
            "optionalDependencies",
        )?,
        peer_dependencies: normalize_dependency_names(
            root.get("peerDependencies"),
            "peerDependencies",
        )?,
        dependency_declarations: dependency_declarations(raw),
    })
}

pub fn dependency_declarations(raw: &Value) -> Vec<PackageJsonDependencyDeclarationSnapshot> {
    let mut declarations = Vec::new();
    collect_dependency_declarations(raw, "dependencies", "prod", &mut declarations);
    collect_dependency_declarations(raw, "devDependencies", "dev", &mut declarations);
    declarations
}

fn collect_dependency_declarations(
    raw: &Value,
    field: &str,
    lane: &str,
    declarations: &mut Vec<PackageJsonDependencyDeclarationSnapshot>,
) {
    let Some(object) = raw.get(field).and_then(Value::as_object) else {
        return;
    };
    declarations.extend(object.iter().filter_map(|(name, value)| {
        value
            .as_str()
            .map(|specifier| PackageJsonDependencyDeclarationSnapshot {
                name: name.clone(),
                lane: lane.to_owned(),
                specifier_type: specifier_type(specifier).to_owned(),
            })
    }));
}

pub fn specifier_type(specifier: &str) -> &'static str {
    let specifier = specifier.trim();
    if specifier.is_empty() {
        return "unsupported";
    }
    if specifier.starts_with("npm:") {
        return "alias";
    }
    if specifier.starts_with("catalog:") {
        return "catalog";
    }
    if specifier.starts_with("file:") {
        return "file";
    }
    if specifier.starts_with("link:") {
        return "link";
    }
    if specifier.starts_with("workspace:") {
        return "workspace-protocol";
    }
    if specifier.starts_with("git+")
        || specifier.starts_with("git:")
        || specifier.starts_with("github:")
        || specifier.starts_with("gitlab:")
        || specifier.starts_with("bitbucket:")
    {
        return "git";
    }
    if specifier.starts_with("http://") || specifier.starts_with("https://") {
        return "url";
    }
    if matches!(specifier, "*" | "latest") {
        return "latest";
    }
    if specifier.contains("||") || specifier.contains(" - ") {
        return "range-complex";
    }
    if let Some(range_tail) = specifier.strip_prefix(['^', '~', '>', '<', '=']) {
        return range_specifier_type(range_tail.trim_start_matches(['=', ' ']));
    }
    if exact_semver_like(specifier) {
        return "exact";
    }
    match numeric_part_count(specifier) {
        Some(1) => "major",
        Some(2) => "minor",
        _ if tag_like(specifier) => "tag",
        _ => "unsupported",
    }
}

fn range_specifier_type(range_tail: &str) -> &'static str {
    if exact_semver_like(range_tail) {
        return "range";
    }
    match numeric_part_count(range_tail) {
        Some(1) => "range-major",
        Some(2) => "range-minor",
        _ => "range-complex",
    }
}

fn numeric_part_count(specifier: &str) -> Option<usize> {
    let parts = specifier.split('.').collect::<Vec<_>>();
    if parts.is_empty() || parts.iter().any(|part| part.is_empty()) {
        return None;
    }
    if parts
        .iter()
        .all(|part| part.chars().all(|ch| ch.is_ascii_digit()))
    {
        return Some(parts.len());
    }
    None
}

fn tag_like(specifier: &str) -> bool {
    specifier
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
}

fn exact_semver_like(specifier: &str) -> bool {
    let parts = specifier.split('.').collect::<Vec<_>>();
    parts.len() == 3
        && parts
            .iter()
            .all(|part| !part.is_empty() && part.chars().all(|ch| ch.is_ascii_digit()))
}

fn normalize_optional_bool(
    value: Option<&Value>,
    field_name: &str,
) -> Result<Option<bool>, String> {
    let Some(value) = value else {
        return Ok(None);
    };
    value
        .as_bool()
        .map(Some)
        .ok_or_else(|| format!("package.json field `{field_name}` must be a boolean"))
}

fn normalize_optional_string(
    value: Option<&Value>,
    field_name: &str,
) -> Result<Option<String>, String> {
    let Some(value) = value else {
        return Ok(None);
    };
    value
        .as_str()
        .map(|item| Some(item.to_owned()))
        .ok_or_else(|| format!("package.json field `{field_name}` must be a string"))
}

fn normalize_optional_nested_string(
    parent: Option<&Value>,
    parent_name: &str,
    field_name: &str,
) -> Result<Option<String>, String> {
    let Some(parent) = parent else {
        return Ok(None);
    };
    let object = parent
        .as_object()
        .ok_or_else(|| format!("package.json field `{parent_name}` must be an object"))?;
    normalize_optional_string(
        object.get(field_name),
        &format!("{parent_name}.{field_name}"),
    )
}

fn normalize_string_map(
    value: Option<&Value>,
    field_name: &str,
) -> Result<BTreeMap<String, String>, String> {
    let Some(value) = value else {
        return Ok(BTreeMap::new());
    };
    let object = value
        .as_object()
        .ok_or_else(|| format!("package.json field `{field_name}` must be an object"))?;

    object
        .iter()
        .map(|(key, value)| {
            value
                .as_str()
                .map(|item| (key.clone(), item.to_owned()))
                .ok_or_else(|| format!("package.json field `{field_name}.{key}` must be a string"))
        })
        .collect()
}

fn normalize_dependency_names(
    value: Option<&Value>,
    field_name: &str,
) -> Result<Vec<String>, String> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    let object = value
        .as_object()
        .ok_or_else(|| format!("package.json field `{field_name}` must be an object"))?;

    object
        .iter()
        .map(|(key, value)| {
            if value.is_string() {
                Ok(key.clone())
            } else {
                Err(format!(
                    "package.json field `{field_name}.{key}` must be a string"
                ))
            }
        })
        .collect()
}

fn normalize_pnpm_override_keys(pnpm: Option<&Value>) -> Result<Vec<String>, String> {
    let Some(pnpm) = pnpm else {
        return Ok(Vec::new());
    };
    let object = pnpm
        .as_object()
        .ok_or_else(|| "package.json field `pnpm` must be an object".to_owned())?;
    let Some(overrides) = object.get("overrides") else {
        return Ok(Vec::new());
    };
    let overrides = overrides
        .as_object()
        .ok_or_else(|| "package.json field `pnpm.overrides` must be an object".to_owned())?;
    Ok(sorted_keys(overrides))
}

fn normalize_pnpm_only_built_dependencies(pnpm: Option<&Value>) -> Result<Vec<String>, String> {
    let Some(pnpm) = pnpm else {
        return Ok(Vec::new());
    };
    let object = pnpm
        .as_object()
        .ok_or_else(|| "package.json field `pnpm` must be an object".to_owned())?;
    let Some(items) = object.get("onlyBuiltDependencies") else {
        return Ok(Vec::new());
    };
    let items = items.as_array().ok_or_else(|| {
        "package.json field `pnpm.onlyBuiltDependencies` must be a string array".to_owned()
    })?;

    items
        .iter()
        .map(|item| {
            item.as_str().map(str::to_owned).ok_or_else(|| {
                "package.json field `pnpm.onlyBuiltDependencies` must be a string array".to_owned()
            })
        })
        .collect()
}

fn sorted_keys(object: &Map<String, Value>) -> Vec<String> {
    let mut keys = object.keys().cloned().collect::<Vec<_>>();
    keys.sort_unstable();
    keys
}
