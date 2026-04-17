use std::path::Path;

use cargo_toml_parser::{types::CargoToml, types::InheritableValue, types::PackageSection, types::VecStringOrBool};
use g3rs_clippy_types::{
    G3RsClippyCargoConfigOverride, G3RsClippyConfigState, G3RsClippyRustPolicyState,
    G3RsClippyWaiver,
};
use guardrail3_rs_toml_parser::RustProfile;
use glob::Pattern;

use crate::run::IngestionError;

pub(crate) fn read_to_string(abs_path: &Path) -> Result<String, String> {
    crate::fs::read_to_string(abs_path).map_err(|err| err.to_string())
}

pub(crate) fn parse_clippy_state(abs_path: &Path) -> G3RsClippyConfigState {
    let content = match read_to_string(abs_path) {
        Ok(content) => content,
        Err(reason) => return G3RsClippyConfigState::Unreadable { reason },
    };

    let raw = match toml::from_str::<toml::Value>(&content) {
        Ok(raw) => raw,
        Err(err) => {
            return G3RsClippyConfigState::ParseError {
                reason: err.to_string(),
            };
        }
    };

    let typed = clippy_toml_parser::parse(&content).map_err(|err| err.to_string());
    G3RsClippyConfigState::Parsed { raw, typed }
}

pub(crate) fn parse_rust_policy_state(
    rel_path: &str,
    abs_path: &Path,
) -> G3RsClippyRustPolicyState {
    let content = match read_to_string(abs_path) {
        Ok(content) => content,
        Err(reason) => {
            return G3RsClippyRustPolicyState::Unreadable {
                rel_path: rel_path.to_owned(),
                reason,
            };
        }
    };

    let parsed = match guardrail3_rs_toml_parser::parse(&content) {
        Ok(parsed) => parsed,
        Err(err) => {
            return G3RsClippyRustPolicyState::ParseError {
                rel_path: rel_path.to_owned(),
                reason: err.to_string(),
            };
        }
    };

    let garde_enabled = parsed
        .checks
        .as_ref()
        .and_then(|checks| checks.garde)
        .unwrap_or(true);

    G3RsClippyRustPolicyState::Parsed {
        rel_path: rel_path.to_owned(),
        profile: parsed.profile,
        garde_enabled,
    }
}

pub(crate) fn parse_waivers(abs_path: &Path) -> Result<Vec<G3RsClippyWaiver>, IngestionError> {
    let content = read_to_string(abs_path).map_err(|reason| IngestionError::Unreadable {
        path: abs_path.to_path_buf(),
        reason,
    })?;
    let parsed = guardrail3_rs_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
        path: abs_path.to_path_buf(),
        reason: err.to_string(),
    })?;

    Ok(parsed
        .waivers
        .into_iter()
        .map(|waiver| G3RsClippyWaiver {
            rule: waiver.rule,
            file: waiver.file,
            selector: waiver.selector,
            reason: waiver.reason,
        })
        .collect())
}

pub(crate) fn parse_cargo_override(
    rel_path: &str,
    abs_path: &Path,
) -> Option<G3RsClippyCargoConfigOverride> {
    let content = match read_to_string(abs_path) {
        Ok(content) => content,
        Err(reason) => {
            return Some(G3RsClippyCargoConfigOverride {
                rel_path: rel_path.to_owned(),
                parse_error: Some(reason),
            });
        }
    };

    let parsed = match toml::from_str::<toml::Value>(&content) {
        Ok(parsed) => parsed,
        Err(err) => {
            return Some(G3RsClippyCargoConfigOverride {
                rel_path: rel_path.to_owned(),
                parse_error: Some(err.to_string()),
            });
        }
    };

    let Some(env) = parsed.get("env") else {
        return None;
    };
    let Some(env_table) = env.as_table() else {
        return Some(G3RsClippyCargoConfigOverride {
            rel_path: rel_path.to_owned(),
            parse_error: Some(format!(
                "invalid cargo config shape: `env` must be a table, found {}",
                value_kind(env)
            )),
        });
    };

    env_table
        .get("CLIPPY_CONF_DIR")
        .map(|_| G3RsClippyCargoConfigOverride {
            rel_path: rel_path.to_owned(),
            parse_error: None,
        })
}

pub(crate) fn compute_published_library_policy(
    root_abs_path: &Path,
    root_cargo_abs_path: &Path,
    profile: Option<RustProfile>,
) -> bool {
    if profile != Some(RustProfile::Library) {
        return false;
    }

    let Ok(root_content) = read_to_string(root_cargo_abs_path) else {
        return false;
    };
    let Ok(root_cargo) = cargo_toml_parser::parse(&root_content) else {
        return false;
    };
    let Ok(root_raw) = toml::from_str::<toml::Value>(&root_content) else {
        return false;
    };
    let workspace_publish = root_cargo
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.package.as_ref())
        .and_then(|package| package.publish.as_ref());

    if manifest_publishable(&root_cargo, workspace_publish) {
        return true;
    }

    let Ok(declared_members) = collect_declared_member_rels(root_abs_path, &root_raw) else {
        return false;
    };
    for member_rel in declared_members {
        if member_rel.is_empty() {
            continue;
        }
        let member_abs_path = root_abs_path.join(format!("{member_rel}/Cargo.toml"));
        let member_content = match read_to_string(&member_abs_path) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let member_cargo = match cargo_toml_parser::parse(&member_content) {
            Ok(cargo) => cargo,
            Err(_) => continue,
        };
        if manifest_publishable(&member_cargo, workspace_publish) {
            return true;
        }
    }

    if root_cargo.workspace.is_none() && root_cargo.package.is_none() {
        return false;
    }

    false
}

fn collect_declared_member_rels(
    root_abs_path: &Path,
    root_raw: &toml::Value,
) -> Result<Vec<String>, IngestionError> {
    let member_patterns = parse_string_array(
        root_raw
            .get("workspace")
            .and_then(|value| value.get("members")),
        "[workspace].members",
        root_abs_path,
    )?;
    let exclude_patterns = parse_string_array(
        root_raw
            .get("workspace")
            .and_then(|value| value.get("exclude")),
        "[workspace].exclude",
        root_abs_path,
    )?;
    let exclude_patterns = exclude_patterns
        .iter()
        .map(|pattern| {
            Pattern::new(pattern).map_err(|err| IngestionError::ParseFailed {
                path: root_abs_path.join("Cargo.toml"),
                reason: format!("invalid workspace exclude pattern `{pattern}`: {err}"),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut members = std::collections::BTreeSet::new();
    for pattern in member_patterns {
        for member_rel in expand_member_pattern(root_abs_path, &pattern)? {
            if exclude_patterns
                .iter()
                .any(|exclude| exclude.matches(&member_rel))
            {
                continue;
            }
            let _ = members.insert(member_rel);
        }
    }

    Ok(members.into_iter().collect())
}

fn parse_string_array(
    value: Option<&toml::Value>,
    label: &str,
    root_abs_path: &Path,
) -> Result<Vec<String>, IngestionError> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    let Some(array) = value.as_array() else {
        return Err(IngestionError::ParseFailed {
            path: root_abs_path.join("Cargo.toml"),
            reason: format!("{label} must be an array of strings."),
        });
    };
    array
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(normalize_member_rel)
                .ok_or_else(|| IngestionError::ParseFailed {
                    path: root_abs_path.join("Cargo.toml"),
                    reason: format!("{label} must contain only string entries."),
                })
        })
        .collect()
}

fn expand_member_pattern(
    root_abs_path: &Path,
    pattern: &str,
) -> Result<Vec<String>, IngestionError> {
    if looks_like_glob(pattern) {
        let compiled = Pattern::new(pattern).map_err(|err| IngestionError::ParseFailed {
            path: root_abs_path.join("Cargo.toml"),
            reason: format!("invalid workspace member pattern `{pattern}`: {err}"),
        })?;

        let mut matches = Vec::new();
        let mut stack = vec![root_abs_path.to_path_buf()];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = crate::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path.clone());
                    let Ok(rel_path) = path.strip_prefix(root_abs_path) else {
                        continue;
                    };
                    let rel_path = rel_path.to_string_lossy().replace('\\', "/");
                    if !rel_path.is_empty() && compiled.matches(&rel_path) {
                        matches.push(rel_path);
                    }
                }
            }
        }
        Ok(matches)
    } else {
        Ok(vec![pattern.to_owned()])
    }
}

fn looks_like_glob(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?') || pattern.contains('[')
}

fn normalize_member_rel(pattern: &str) -> String {
    let trimmed = pattern.trim_matches('/');
    let stripped = trimmed.strip_prefix("./").unwrap_or(trimmed).trim_matches('/');
    if stripped == "." {
        String::new()
    } else {
        stripped.to_owned()
    }
}

fn manifest_publishable(
    cargo: &CargoToml,
    workspace_publish: Option<&VecStringOrBool>,
) -> bool {
    let Some(package) = cargo.package.as_ref().or(cargo.project.as_ref()) else {
        return false;
    };
    package_publishable(package, workspace_publish)
}

fn package_publishable(
    package: &PackageSection,
    workspace_publish: Option<&VecStringOrBool>,
) -> bool {
    match package.publish.as_ref() {
        None => true,
        Some(InheritableValue::Value(value)) => publish_value_allows_publish(value),
        Some(InheritableValue::Inherit(inheritance)) if inheritance.workspace => {
            workspace_publish.is_some_and(publish_value_allows_publish)
        }
        Some(InheritableValue::Inherit(_)) => false,
    }
}

fn publish_value_allows_publish(value: &VecStringOrBool) -> bool {
    match value {
        VecStringOrBool::Bool(flag) => *flag,
        VecStringOrBool::VecString(registries) => !registries.is_empty(),
    }
}

fn value_kind(value: &toml::Value) -> &'static str {
    match value {
        toml::Value::String(_) => "string",
        toml::Value::Integer(_) => "integer",
        toml::Value::Float(_) => "float",
        toml::Value::Boolean(_) => "bool",
        toml::Value::Datetime(_) => "datetime",
        toml::Value::Array(_) => "array",
        toml::Value::Table(_) => "table",
    }
}
