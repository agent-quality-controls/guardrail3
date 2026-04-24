use std::collections::BTreeMap;

use package_json_parser_types::document::{
    PackageDependencySection, PackageDependencySpec, PackageDependencySpecParseState,
    PackageJsonDocument, PackageJsonParseState, PackageJsonSnapshot, SemverVersion,
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
        dependency_specs: normalize_dependency_specs(root)?,
    })
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

fn normalize_dependency_specs(
    root: &Map<String, Value>,
) -> Result<Vec<PackageDependencySpec>, String> {
    let mut specs = Vec::new();
    specs.extend(normalize_dependency_section_specs(
        root.get("dependencies"),
        "dependencies",
        PackageDependencySection::Dependencies,
    )?);
    specs.extend(normalize_dependency_section_specs(
        root.get("devDependencies"),
        "devDependencies",
        PackageDependencySection::DevDependencies,
    )?);
    specs.extend(normalize_dependency_section_specs(
        root.get("optionalDependencies"),
        "optionalDependencies",
        PackageDependencySection::OptionalDependencies,
    )?);
    specs.extend(normalize_dependency_section_specs(
        root.get("peerDependencies"),
        "peerDependencies",
        PackageDependencySection::PeerDependencies,
    )?);
    Ok(specs)
}

fn normalize_dependency_section_specs(
    value: Option<&Value>,
    field_name: &str,
    section: PackageDependencySection,
) -> Result<Vec<PackageDependencySpec>, String> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    let object = value
        .as_object()
        .ok_or_else(|| format!("package.json field `{field_name}` must be an object"))?;

    object
        .iter()
        .map(|(name, value)| {
            let raw_spec = value.as_str().ok_or_else(|| {
                format!("package.json field `{field_name}.{name}` must be a string")
            })?;
            Ok(PackageDependencySpec {
                name: name.clone(),
                raw_spec: raw_spec.to_owned(),
                section,
                parsed: parse_dependency_spec(raw_spec),
            })
        })
        .collect()
}

fn parse_dependency_spec(raw_spec: &str) -> PackageDependencySpecParseState {
    let trimmed = raw_spec.trim();

    if trimmed.starts_with("workspace:") {
        return PackageDependencySpecParseState::Workspace {
            raw: trimmed.to_owned(),
        };
    }
    if trimmed.starts_with("file:") {
        return PackageDependencySpecParseState::File {
            raw: trimmed.to_owned(),
        };
    }
    if trimmed.starts_with("link:") {
        return PackageDependencySpecParseState::Link {
            raw: trimmed.to_owned(),
        };
    }
    if trimmed.starts_with("catalog:") {
        return PackageDependencySpecParseState::Catalog {
            raw: trimmed.to_owned(),
        };
    }

    if let Some(version) = parse_exact_semver(trimmed) {
        return PackageDependencySpecParseState::Exact { version };
    }

    if is_range_spec(trimmed) {
        let range = range_bounds(trimmed);
        return PackageDependencySpecParseState::Range {
            minimum: range.minimum,
            allows_below_minimum_unknown: range.allows_below_minimum_unknown,
        };
    }

    PackageDependencySpecParseState::Unsupported {
        raw: trimmed.to_owned(),
        reason: "dependency version spec is not a supported exact, range, workspace, file, link, or catalog spec".to_owned(),
    }
}

fn is_range_spec(spec: &str) -> bool {
    spec.contains('^')
        || spec.contains('~')
        || spec.contains('>')
        || spec.contains('<')
        || spec.contains('=')
        || spec.contains('*')
        || spec.contains('x')
        || spec.contains('X')
        || spec.contains('|')
        || spec.contains(' ')
}

fn parse_exact_semver(spec: &str) -> Option<SemverVersion> {
    parse_semver(spec).filter(|_| !is_range_spec(spec))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RangeBounds {
    minimum: Option<SemverVersion>,
    allows_below_minimum_unknown: bool,
}

fn range_bounds(spec: &str) -> RangeBounds {
    let mut disjunct_minimums = Vec::new();
    let mut unknown = false;

    for disjunct in spec.split("||") {
        let minimum = disjunct
            .split_whitespace()
            .filter_map(lower_bound_semver_in_comparator)
            .min();
        if let Some(minimum) = minimum {
            disjunct_minimums.push(minimum);
        } else {
            unknown = true;
        }
    }

    RangeBounds {
        minimum: (!unknown).then(|| disjunct_minimums.into_iter().min()).flatten(),
        allows_below_minimum_unknown: unknown,
    }
}

fn lower_bound_semver_in_comparator(comparator: &str) -> Option<SemverVersion> {
    let comparator = comparator.trim_matches(|character| matches!(character, ',' | '(' | ')'));
    if let Some(candidate) = comparator.strip_prefix(">=") {
        return parse_semver(candidate);
    }
    if let Some(candidate) = comparator.strip_prefix('>') {
        return parse_semver(candidate).map(increment_patch_for_exclusive_lower_bound);
    }
    let candidate = comparator
        .strip_prefix('^')
        .or_else(|| comparator.strip_prefix('~'))
        .or_else(|| comparator.strip_prefix('='))
        .or_else(|| (!comparator.starts_with('<')).then_some(comparator));
    candidate.and_then(parse_semver)
}

fn increment_patch_for_exclusive_lower_bound(version: SemverVersion) -> SemverVersion {
    if version.pre.is_some() {
        return version;
    }
    SemverVersion {
        major: version.major,
        minor: version.minor,
        patch: version.patch.saturating_add(1),
        pre: None,
    }
}

fn parse_semver(candidate: &str) -> Option<SemverVersion> {
    let candidate = candidate.trim();
    let (without_build, _build) = candidate.split_once('+').unwrap_or((candidate, ""));
    let (core, pre) = match without_build.split_once('-') {
        Some((_core, "")) => return None,
        Some((core, pre)) => (core, Some(pre.to_owned())),
        None => (without_build, None),
    };
    let mut parts = core.split('.');
    let major = parts.next()?.parse::<u64>().ok()?;
    let minor = parts.next()?.parse::<u64>().ok()?;
    let patch = parts.next()?.parse::<u64>().ok()?;

    if parts.next().is_some() {
        return None;
    }

    Some(SemverVersion {
        major,
        minor,
        patch,
        pre,
    })
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

#[cfg(test)]
#[path = "parser_tests/mod.rs"]
mod parser_tests;
