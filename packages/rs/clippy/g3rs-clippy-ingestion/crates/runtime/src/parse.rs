use std::path::Path;

use g3rs_clippy_types::{
    G3RsClippyCargoConfigState, G3RsClippyCargoMemberState, G3RsClippyCargoRootState,
    G3RsClippyConfigState, G3RsClippyRustPolicyState, G3RsClippyWaiver,
};
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

    match clippy_toml_parser::parse_document(&content) {
        Ok(document) => G3RsClippyConfigState::Parsed(document),
        Err(err) => G3RsClippyConfigState::ParseError {
            reason: err.to_string(),
        },
    }
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
    let parsed =
        guardrail3_rs_toml_parser::parse(&content).map_err(|err| IngestionError::ParseFailed {
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

pub(crate) fn parse_cargo_config_state(
    rel_path: &str,
    abs_path: &Path,
) -> G3RsClippyCargoConfigState {
    let content = match read_to_string(abs_path) {
        Ok(content) => content,
        Err(reason) => {
            return G3RsClippyCargoConfigState::Unreadable {
                rel_path: rel_path.to_owned(),
                reason,
            };
        }
    };

    let cargo_config = match cargo_config_toml_parser::parse(&content) {
        Ok(parsed) => parsed,
        Err(err) => {
            return G3RsClippyCargoConfigState::ParseError {
                rel_path: rel_path.to_owned(),
                reason: err.to_string(),
            };
        }
    };

    G3RsClippyCargoConfigState::Parsed {
        rel_path: rel_path.to_owned(),
        cargo_config,
    }
}

pub(crate) fn parse_cargo_root_state(rel_path: &str, abs_path: &Path) -> G3RsClippyCargoRootState {
    let content = match read_to_string(abs_path) {
        Ok(content) => content,
        Err(reason) => {
            return G3RsClippyCargoRootState::Unreadable {
                rel_path: rel_path.to_owned(),
                reason,
            };
        }
    };

    match cargo_toml_parser::parse_document(&content) {
        Ok(cargo) => G3RsClippyCargoRootState::Parsed {
            rel_path: rel_path.to_owned(),
            cargo,
        },
        Err(err) => G3RsClippyCargoRootState::ParseError {
            rel_path: rel_path.to_owned(),
            reason: err.to_string(),
        },
    }
}

pub(crate) fn parse_cargo_member_state(
    member_rel: &str,
    rel_path: &str,
    abs_path: &Path,
) -> G3RsClippyCargoMemberState {
    let content = match read_to_string(abs_path) {
        Ok(content) => content,
        Err(reason) => {
            return G3RsClippyCargoMemberState::Unreadable {
                member_rel: member_rel.to_owned(),
                rel_path: rel_path.to_owned(),
                reason,
            };
        }
    };

    match cargo_toml_parser::parse_document(&content) {
        Ok(cargo) => G3RsClippyCargoMemberState::Parsed {
            member_rel: member_rel.to_owned(),
            rel_path: rel_path.to_owned(),
            cargo,
        },
        Err(err) => G3RsClippyCargoMemberState::ParseError {
            member_rel: member_rel.to_owned(),
            rel_path: rel_path.to_owned(),
            reason: err.to_string(),
        },
    }
}

pub(crate) fn collect_declared_member_rels(
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
    let stripped = trimmed
        .strip_prefix("./")
        .unwrap_or(trimmed)
        .trim_matches('/');
    if stripped == "." {
        String::new()
    } else {
        stripped.to_owned()
    }
}
