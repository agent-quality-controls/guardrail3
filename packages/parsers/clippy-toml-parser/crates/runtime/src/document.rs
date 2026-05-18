use clippy_toml_parser_types::clippy_toml::ClippyToml;
use clippy_toml_parser_types::document::{
    ClippyBanEntry, ClippyBanSection, ClippyBoolSetting, ClippyTomlDocument, ClippyTomlParseState,
};
use toml::Value;

#[must_use]
pub const fn typed(document: &ClippyTomlDocument) -> Option<&ClippyToml> {
    match &document.typed {
        ClippyTomlParseState::Parsed(clippy) => Some(clippy),
        ClippyTomlParseState::Invalid(_) => None,
    }
}

#[must_use]
pub fn parse_error_reason(document: &ClippyTomlDocument) -> Option<&str> {
    match &document.typed {
        ClippyTomlParseState::Parsed(_) => None,
        ClippyTomlParseState::Invalid(reason) => Some(reason),
    }
}

#[must_use]
pub fn top_level_keys(document: &ClippyTomlDocument) -> Vec<&str> {
    document
        .raw
        .as_table()
        .map(|table| table.keys().map(String::as_str).collect())
        .unwrap_or_default()
}

/// Parses the named ban section, returning entries and any malformed messages.
#[must_use]
pub fn ban_section(document: &ClippyTomlDocument, key: &str) -> ClippyBanSection {
    let Some(value) = document.raw.get(key) else {
        return ClippyBanSection {
            entries: Vec::new(),
            malformed_messages: Vec::new(),
        };
    };

    let Some(entries) = value.as_array() else {
        return ClippyBanSection {
            entries: Vec::new(),
            malformed_messages: vec![format!(
                "`{key}` must be an array, found {}.",
                value_kind(value)
            )],
        };
    };

    let mut parsed_entries = Vec::new();
    let mut malformed_messages = Vec::new();

    for (index, entry) in entries.iter().enumerate() {
        match entry {
            Value::String(path) => parsed_entries.push(ClippyBanEntry {
                path: path.clone(),
                reason: None,
                is_plain_string: true,
            }),
            Value::Table(table) => parse_ban_table_entry(
                table,
                key,
                index,
                &mut parsed_entries,
                &mut malformed_messages,
            ),
            Value::Integer(_)
            | Value::Float(_)
            | Value::Boolean(_)
            | Value::Datetime(_)
            | Value::Array(_) => malformed_messages.push(format!(
                "`{key}[{index}]` must be a string or table, found {}.",
                value_kind(entry)
            )),
        }
    }

    ClippyBanSection {
        entries: parsed_entries,
        malformed_messages,
    }
}

/// Parses a single ban entry expressed as a TOML table.
fn parse_ban_table_entry(
    table: &toml::map::Map<String, Value>,
    key: &str,
    index: usize,
    parsed_entries: &mut Vec<ClippyBanEntry>,
    malformed_messages: &mut Vec<String>,
) {
    match table.get("path") {
        Some(Value::String(path)) => {
            if let Some(reason) = table.get("reason") {
                if !reason.is_str() {
                    malformed_messages.push(format!(
                        "`{key}[{index}].reason` must be a string when present, found {}.",
                        value_kind(reason)
                    ));
                    return;
                }
            }
            parsed_entries.push(ClippyBanEntry {
                path: path.to_owned(),
                reason: table
                    .get("reason")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
                is_plain_string: false,
            });
        }
        Some(path) => malformed_messages.push(format!(
            "`{key}[{index}].path` must be a string, found {}.",
            value_kind(path)
        )),
        None => {
            malformed_messages.push(format!(
                "`{key}[{index}]` must contain a string `path` field."
            ));
        }
    }
}

#[must_use]
pub fn bool_setting<'a>(document: &'a ClippyTomlDocument, key: &str) -> ClippyBoolSetting<'a> {
    document
        .raw
        .get(key)
        .map_or(ClippyBoolSetting::Missing, |value| {
            value.as_bool().map_or(
                ClippyBoolSetting::WrongType(value),
                ClippyBoolSetting::Value,
            )
        })
}

/// Returns a static label describing a TOML value's kind for diagnostic messages.
const fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::String(_) => "string",
        Value::Integer(_) => "integer",
        Value::Float(_) => "float",
        Value::Boolean(_) => "bool",
        Value::Datetime(_) => "datetime",
        Value::Array(_) => "array",
        Value::Table(_) => "table",
    }
}
