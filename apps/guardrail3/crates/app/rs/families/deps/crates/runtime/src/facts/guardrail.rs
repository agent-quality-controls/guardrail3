use std::collections::BTreeMap;

use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;

use super::ParsedGuardrail;

pub(super) fn parse_guardrail(tree: &ProjectTree) -> Option<ParsedGuardrail> {
    let Some(content) = tree.file_content("guardrail3.toml") else {
        return tree
            .file_exists("guardrail3.toml")
            .then(|| ParsedGuardrail {
                root_profile_name: None,
                apps: BTreeMap::new(),
                packages: None,
                parse_error: Some(
                    "Failed to read guardrail3.toml for dependency policy resolution.".to_owned(),
                ),
            });
    };
    match toml::from_str::<toml::Value>(content) {
        Ok(parsed) => match validate_deps_guardrail_shape(&parsed) {
            Ok(()) => match parsed.clone().try_into::<GuardrailConfig>() {
                Ok(config) => Some(ParsedGuardrail {
                    root_profile_name: config.profile().map(|profile| profile.name().to_owned()),
                    apps: config
                        .rust()
                        .and_then(|rust| rust.apps().cloned())
                        .unwrap_or_default(),
                    packages: config.rust().and_then(|rust| rust.packages().cloned()),
                    parse_error: None,
                }),
                Err(parse_error) => Some(parse_error_snapshot(parse_error.to_string())),
            },
            Err(parse_error) => Some(parse_error_snapshot(parse_error)),
        },
        Err(parse_error) => Some(parse_error_snapshot(parse_error.to_string())),
    }
}

fn parse_error_snapshot(parse_error: String) -> ParsedGuardrail {
    ParsedGuardrail {
        root_profile_name: None,
        apps: BTreeMap::new(),
        packages: None,
        parse_error: Some(format!(
            "Failed to parse guardrail3.toml for dependency policy resolution: {parse_error}"
        )),
    }
}

fn validate_deps_guardrail_shape(parsed: &toml::Value) -> Result<(), String> {
    let Some(root) = parsed.as_table() else {
        return Err("guardrail3.toml root must be a table.".to_owned());
    };

    if let Some(profile) = root.get("profile") {
        let Some(profile) = profile.as_table() else {
            return Err("`profile` must be a table.".to_owned());
        };
        if let Some(name) = profile.get("name") {
            let Some(name) = name.as_str() else {
                return Err("`profile.name` must be a string.".to_owned());
            };
            if name.is_empty() {
                return Err("`profile.name` must be non-empty.".to_owned());
            }
        }
    }

    let Some(rust) = root.get("rust") else {
        return Ok(());
    };
    let Some(rust) = rust.as_table() else {
        return Err("`rust` must be a table.".to_owned());
    };

    for key in rust.keys() {
        if !matches!(
            key.as_str(),
            "workspace_root" | "workspaces" | "apps" | "packages" | "checks"
        ) {
            return Err(format!("`rust` contains unsupported key `{key}`."));
        }
    }

    if let Some(apps) = rust.get("apps") {
        let Some(apps) = apps.as_table() else {
            return Err("`rust.apps` must be a table.".to_owned());
        };
        for (app_name, config) in apps {
            validate_crate_policy_table(config, &format!("rust.apps.{app_name}"))?;
        }
    }

    if let Some(packages) = rust.get("packages") {
        validate_crate_policy_table(packages, "rust.packages")?;
    }

    Ok(())
}

fn validate_crate_policy_table(value: &toml::Value, context: &str) -> Result<(), String> {
    let Some(table) = value.as_table() else {
        return Err(format!("`{context}` must be a table."));
    };

    for key in table.keys() {
        if !matches!(
            key.as_str(),
            "layer" | "profile" | "type" | "allowed_deps" | "checks"
        ) {
            return Err(format!("`{context}` contains unsupported key `{key}`."));
        }
    }

    for key in ["layer", "profile", "type"] {
        if let Some(value) = table.get(key) {
            let Some(value) = value.as_str() else {
                return Err(format!("`{context}.{key}` must be a string."));
            };
            if value.is_empty() {
                return Err(format!("`{context}.{key}` must be non-empty."));
            }
        }
    }

    if let Some(allowed_deps) = table.get("allowed_deps") {
        let Some(allowed_deps) = allowed_deps.as_array() else {
            return Err(format!(
                "`{context}.allowed_deps` must be an array of strings."
            ));
        };
        for dep in allowed_deps {
            let Some(dep) = dep.as_str() else {
                return Err(format!(
                    "`{context}.allowed_deps` must contain only strings."
                ));
            };
            if dep.is_empty() {
                return Err(format!(
                    "`{context}.allowed_deps` must not contain empty dependency names."
                ));
            }
        }
    }

    Ok(())
}

pub(super) fn validate_workspace_manifest_shape(parsed: &toml::Value) -> Result<(), String> {
    let Some(workspace) = parsed.get("workspace") else {
        return Ok(());
    };
    let Some(workspace) = workspace.as_table() else {
        return Err("`[workspace]` must be a table.".to_owned());
    };

    if let Some(members) = workspace.get("members") {
        let Some(members) = members.as_array() else {
            return Err("`[workspace].members` must be an array of strings.".to_owned());
        };
        for member in members {
            let Some(member) = member.as_str() else {
                return Err("`[workspace].members` must contain only strings.".to_owned());
            };
            if member.is_empty() {
                return Err("`[workspace].members` must not contain empty patterns.".to_owned());
            }
        }
    }

    if let Some(dependencies) = workspace.get("dependencies") {
        let Some(dependencies) = dependencies.as_table() else {
            return Err("`[workspace.dependencies]` must be a table.".to_owned());
        };
        for (alias, value) in dependencies {
            validate_workspace_dependency_shape(alias, value)?;
        }
    }

    Ok(())
}

fn validate_workspace_dependency_shape(alias: &str, value: &toml::Value) -> Result<(), String> {
    if value.is_str() {
        return Ok(());
    }

    let Some(table) = value.as_table() else {
        return Err(format!(
            "`[workspace.dependencies].{alias}` must be a string or table."
        ));
    };

    if let Some(package) = table.get("package") {
        let Some(package) = package.as_str() else {
            return Err(format!(
                "`[workspace.dependencies].{alias}.package` must be a string."
            ));
        };
        if package.is_empty() {
            return Err(format!(
                "`[workspace.dependencies].{alias}.package` must be non-empty."
            ));
        }
    }

    if let Some(path) = table.get("path") {
        let Some(path) = path.as_str() else {
            return Err(format!(
                "`[workspace.dependencies].{alias}.path` must be a string."
            ));
        };
        if path.is_empty() {
            return Err(format!(
                "`[workspace.dependencies].{alias}.path` must be non-empty."
            ));
        }
    }

    Ok(())
}

pub(super) fn validate_dependency_manifest_shape(parsed: &toml::Value) -> Result<(), String> {
    for section_key in ["dependencies", "build-dependencies", "dev-dependencies"] {
        let Some(section) = parsed.get(section_key) else {
            continue;
        };
        let Some(section) = section.as_table() else {
            return Err(format!("`[{section_key}]` must be a table."));
        };
        for (alias, value) in section {
            validate_dependency_spec_shape(section_key, alias, value)?;
        }
    }

    if let Some(target) = parsed.get("target") {
        let Some(target) = target.as_table() else {
            return Err("`[target]` must be a table.".to_owned());
        };
        for (target_name, target_value) in target {
            let Some(target_table) = target_value.as_table() else {
                return Err(format!("`[target.{target_name}]` must be a table."));
            };
            for section_key in ["dependencies", "build-dependencies", "dev-dependencies"] {
                let Some(section) = target_table.get(section_key) else {
                    continue;
                };
                let Some(section) = section.as_table() else {
                    return Err(format!(
                        "`[target.{target_name}.{section_key}]` must be a table."
                    ));
                };
                for (alias, value) in section {
                    validate_dependency_spec_shape(
                        &format!("target.{target_name}.{section_key}"),
                        alias,
                        value,
                    )?;
                }
            }
        }
    }

    Ok(())
}

fn validate_dependency_spec_shape(
    section_key: &str,
    alias: &str,
    value: &toml::Value,
) -> Result<(), String> {
    if value.is_str() {
        return Ok(());
    }

    let Some(table) = value.as_table() else {
        return Err(format!(
            "`[{section_key}].{alias}` must be a string or table."
        ));
    };

    if let Some(package) = table.get("package") {
        let Some(package) = package.as_str() else {
            return Err(format!(
                "`[{section_key}].{alias}.package` must be a string."
            ));
        };
        if package.is_empty() {
            return Err(format!(
                "`[{section_key}].{alias}.package` must be non-empty."
            ));
        }
    }

    if let Some(path) = table.get("path") {
        let Some(path) = path.as_str() else {
            return Err(format!("`[{section_key}].{alias}.path` must be a string."));
        };
        if path.is_empty() {
            return Err(format!("`[{section_key}].{alias}.path` must be non-empty."));
        }
    }

    if let Some(workspace) = table.get("workspace") {
        if workspace.as_bool().is_none() {
            return Err(format!(
                "`[{section_key}].{alias}.workspace` must be a boolean."
            ));
        }
    }

    Ok(())
}
