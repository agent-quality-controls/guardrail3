use g3ts_astro_types::{G3TsAstroStaticObjectProperty, G3TsAstroStaticValue};

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

pub(crate) fn checks_options_are_fail_closed(value: &G3TsAstroStaticValue) -> bool {
    let Some(properties) = object_properties(value) else {
        return false;
    };
    if object_has_duplicate_keys(properties) {
        return false;
    }

    object_has_only_allowed_keys(properties, &ALLOWED_OPTION_KEYS)
        && property_string(properties, "mode") == Some("full")
        && property_bool(properties, "failOnError") == Some(true)
        && property_bool(properties, "failOnWarning") == Some(true)
        && property_bool(properties, "reportJson") == Some(true)
        && property_bool(properties, "ai") == Some(false)
        && overrides_absent_or_empty(properties)
        && validator_lane_not_disabled(properties, "seo")
        && validator_lane_not_disabled(properties, "geo")
        && validator_lane_not_disabled(properties, "performance")
        && validator_lane_not_disabled(properties, "accessibility")
        && checks_options_have_structured_data_custom_check(value)
}

pub(crate) fn checks_options_have_structured_data_custom_check(
    value: &G3TsAstroStaticValue,
) -> bool {
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

    values.iter().any(|value| {
        matches!(
            value,
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

fn object_properties(value: &G3TsAstroStaticValue) -> Option<&[G3TsAstroStaticObjectProperty]> {
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

fn property_value<'a>(
    properties: &'a [G3TsAstroStaticObjectProperty],
    key: &str,
) -> Option<&'a G3TsAstroStaticValue> {
    properties
        .iter()
        .find(|property| property.key == key)
        .map(|property| &property.value)
}

fn object_has_duplicate_keys(properties: &[G3TsAstroStaticObjectProperty]) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    properties
        .iter()
        .any(|property| !seen.insert(property.key.as_str()))
}

fn object_has_only_allowed_keys(
    properties: &[G3TsAstroStaticObjectProperty],
    allowed_keys: &[&str],
) -> bool {
    properties
        .iter()
        .all(|property| allowed_keys.contains(&property.key.as_str()))
}

fn property_string<'a>(
    properties: &'a [G3TsAstroStaticObjectProperty],
    key: &str,
) -> Option<&'a str> {
    match property_value(properties, key) {
        Some(G3TsAstroStaticValue::String(value)) => Some(value),
        _ => None,
    }
}

fn property_bool(properties: &[G3TsAstroStaticObjectProperty], key: &str) -> Option<bool> {
    match property_value(properties, key) {
        Some(G3TsAstroStaticValue::Bool(value)) => Some(*value),
        _ => None,
    }
}

fn overrides_absent_or_empty(properties: &[G3TsAstroStaticObjectProperty]) -> bool {
    match property_value(properties, "overrides") {
        None => true,
        Some(G3TsAstroStaticValue::Object(properties)) => properties.is_empty(),
        Some(_) => false,
    }
}

fn validator_lane_not_disabled(properties: &[G3TsAstroStaticObjectProperty], key: &str) -> bool {
    match property_value(properties, key) {
        None => true,
        Some(G3TsAstroStaticValue::Bool(false)) => false,
        Some(G3TsAstroStaticValue::Bool(true) | G3TsAstroStaticValue::Object(_)) => true,
        Some(_) => false,
    }
}
