/// Assemble the checks input from selected and parsed data.
use deny_toml_parser::DenyToml;
use g3rs_deny_types::{G3RsDenyConfigChecksInput, G3RsDenyFileTreeChecksInput, G3RsDenyInputFailure};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct GuardrailState {
    pub(crate) profile_name: Option<String>,
    pub(crate) parse_error: bool,
}

/// Build the checks input from the parsed deny config and its relative path.
pub(crate) fn assemble(
    deny_rel_path: String,
    deny: DenyToml,
    guardrail: &GuardrailState,
) -> G3RsDenyConfigChecksInput {
    G3RsDenyConfigChecksInput {
        deny_rel_path,
        deny,
        profile_name: guardrail.profile_name.clone(),
        policy_context_valid: !guardrail.parse_error,
    }
}

pub(crate) fn input_failure(
    title: impl Into<String>,
    rel_path: impl Into<String>,
    message: impl Into<String>,
) -> G3RsDenyInputFailure {
    G3RsDenyInputFailure {
        title: title.into(),
        rel_path: rel_path.into(),
        message: message.into(),
    }
}

pub(crate) fn filetree_input(
    selected_deny_rel_path: Option<String>,
    candidate_deny_rel_paths: Vec<String>,
    input_failures: Vec<G3RsDenyInputFailure>,
) -> G3RsDenyFileTreeChecksInput {
    G3RsDenyFileTreeChecksInput {
        selected_deny_rel_path,
        candidate_deny_rel_paths,
        input_failures,
    }
}

pub(crate) fn profile_name_from_guardrail(raw: &toml::Value) -> Result<Option<String>, ()> {
    validate_guardrail_policy_shape(raw)?;

    if let Some(packages) = raw.get("rust").and_then(|value| value.get("packages")) {
        return Ok(
            packages
                .get("type")
                .or_else(|| packages.get("profile"))
                .and_then(toml::Value::as_str)
                .map(str::to_owned)
                .or_else(|| Some("library".to_owned())),
        );
    }

    Ok(
        raw.get("profile")
            .and_then(|value| value.get("name"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned),
    )
}

fn validate_guardrail_policy_shape(parsed: &toml::Value) -> Result<(), ()> {
    if let Some(profile) = parsed.get("profile") {
        let table = profile.as_table().ok_or(())?;
        if let Some(name) = table.get("name") {
            let Some(name) = name.as_str() else {
                return Err(());
            };
            validate_known_profile_name(name)?;
        }
    }

    if let Some(packages) = parsed.get("rust").and_then(|value| value.get("packages")) {
        validate_profile_block(packages)?;
    }

    Ok(())
}

fn validate_profile_block(value: &toml::Value) -> Result<(), ()> {
    let table = value.as_table().ok_or(())?;
    let type_name = table.get("type").map_or(Ok(None), |value| value.as_str().map(Some).ok_or(()))?;
    let profile_name = table
        .get("profile")
        .map_or(Ok(None), |value| value.as_str().map(Some).ok_or(()))?;

    if let Some(name) = type_name {
        validate_known_profile_name(name)?;
    }
    if let Some(name) = profile_name {
        validate_known_profile_name(name)?;
    }
    if let (Some(type_name), Some(profile_name)) = (type_name, profile_name)
        && type_name != profile_name
    {
        return Err(());
    }

    Ok(())
}

fn validate_known_profile_name(profile_name: &str) -> Result<(), ()> {
    if matches!(profile_name, "service" | "library") {
        Ok(())
    } else {
        Err(())
    }
}
