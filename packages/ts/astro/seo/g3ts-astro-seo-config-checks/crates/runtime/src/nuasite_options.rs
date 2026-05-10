use g3ts_astro_seo_types::{
    G3TsAstroConfigSurfaceSnapshot, G3TsAstroStaticObjectProperty, G3TsAstroStaticValue,
};

/// Static rule data.
const ALLOWED_OPTION_KEYS: [&str; 11] = [
    "mode",
    "failOnError",
    "failOnWarning",
    "reportJson",
    "ai",
    "customChecks",
    "overrides",
    "seo",
    "geo",
    "performance",
    "accessibility",
];

/// Internal helper exported within the runtime crate.
pub(crate) fn astro_config_has_nuasite_checks_with_required_options(
    snapshot: &G3TsAstroConfigSurfaceSnapshot,
) -> bool {
    snapshot.integrations.iter().any(|integration| {
        integration.source_module.as_deref() == Some("@nuasite/checks")
            && integration.call.is_some()
            && matches!(integration.imported_name.as_deref(), None | Some("checks"))
            && integration
                .call
                .as_ref()
                .and_then(|call| call.first_arg.as_ref())
                .is_some_and(checks_options_are_fail_closed)
    })
}

/// Internal helper exported within the runtime crate.
pub(crate) fn checks_options_include_structured_data_check(
    snapshot: &G3TsAstroConfigSurfaceSnapshot,
) -> bool {
    snapshot.integrations.iter().any(|integration| {
        integration.source_module.as_deref() == Some("@nuasite/checks")
            && integration.call.is_some()
            && matches!(integration.imported_name.as_deref(), None | Some("checks"))
            && integration
                .call
                .as_ref()
                .and_then(|call| call.first_arg.as_ref())
                .is_some_and(checks_options_have_structured_data_custom_check)
    })
}

/// Internal helper used by the rule.
fn checks_options_are_fail_closed(value: &G3TsAstroStaticValue) -> bool {
    let Some(properties) = object_properties(value) else {
        return false;
    };
    if object_has_duplicate_keys(properties) {
        return false;
    }

    object_has_only_allowed_keys(properties)
        && property_bool(properties, "failOnError") == Some(true)
        && property_bool(properties, "failOnWarning") == Some(true)
        && property_bool(properties, "reportJson") == Some(true)
        && overrides_absent_or_empty(properties)
        && validator_lane_not_disabled(properties, "seo")
        && validator_lane_not_disabled(properties, "geo")
        && validator_lane_not_disabled(properties, "performance")
        && validator_lane_not_disabled(properties, "accessibility")
        && checks_options_have_structured_data_custom_check(value)
}

/// Internal helper used by the rule.
fn checks_options_have_structured_data_custom_check(value: &G3TsAstroStaticValue) -> bool {
    let Some(properties) = object_properties(value) else {
        return false;
    };
    if object_has_duplicate_keys(properties) {
        return false;
    }
    let Some(G3TsAstroStaticValue::Array(values)) = property_value(properties, "customChecks")
    else {
        return false;
    };

    values.iter().any(|item| {
        matches!(
            item,
            G3TsAstroStaticValue::ImportedIdentifier {
                local_name,
                source_module: Some(source_module),
                imported_name: Some(imported_name),
            } if local_name == "structuredDataPresentCheck"
                && source_module == "g3ts-astro-nuasite-checks"
                && imported_name == "structuredDataPresentCheck"
        )
    })
}

/// Internal helper used by the rule.
fn object_properties(value: &G3TsAstroStaticValue) -> Option<&[G3TsAstroStaticObjectProperty]> {
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

/// Internal generic helper used by the rule.
fn property_value<'a>(
    properties: &'a [G3TsAstroStaticObjectProperty],
    key: &str,
) -> Option<&'a G3TsAstroStaticValue> {
    properties
        .iter()
        .find(|property| property.key == key)
        .map(|property| &property.value)
}

/// Internal helper used by the rule.
fn object_has_duplicate_keys(properties: &[G3TsAstroStaticObjectProperty]) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    properties
        .iter()
        .any(|property| !seen.insert(property.key.as_str()))
}

/// Internal helper used by the rule.
fn object_has_only_allowed_keys(properties: &[G3TsAstroStaticObjectProperty]) -> bool {
    properties
        .iter()
        .all(|property| ALLOWED_OPTION_KEYS.contains(&property.key.as_str()))
}

/// Internal helper used by the rule.
fn property_bool(properties: &[G3TsAstroStaticObjectProperty], key: &str) -> Option<bool> {
    match property_value(properties, key) {
        Some(G3TsAstroStaticValue::Bool(value)) => Some(*value),
        _ => None,
    }
}

/// Internal helper used by the rule.
fn overrides_absent_or_empty(properties: &[G3TsAstroStaticObjectProperty]) -> bool {
    match property_value(properties, "overrides") {
        None => true,
        Some(G3TsAstroStaticValue::Object(properties)) => properties.is_empty(),
        Some(_) => false,
    }
}

/// Internal helper used by the rule.
fn validator_lane_not_disabled(properties: &[G3TsAstroStaticObjectProperty], key: &str) -> bool {
    match property_value(properties, key) {
        None | Some(G3TsAstroStaticValue::Bool(true) | G3TsAstroStaticValue::Object(_)) => true,
        Some(G3TsAstroStaticValue::Bool(false) | _) => false,
    }
}
