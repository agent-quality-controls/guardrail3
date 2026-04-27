use clippy_toml_parser_types::clippy_toml::ClippyToml;
use clippy_toml_parser_types::document::{
    ClippyBanEntry, ClippyBanSection, ClippyBoolSetting, ClippyTomlDocument, ClippyTomlParseState,
};
use toml::Value;

pub fn typed(document: &ClippyTomlDocument) -> Option<&ClippyToml> {
    match &document.typed {
        ClippyTomlParseState::Parsed(clippy) => Some(clippy),
        ClippyTomlParseState::Invalid(_) => None,
    }
}

pub fn parse_error_reason(document: &ClippyTomlDocument) -> Option<&str> {
    match &document.typed {
        ClippyTomlParseState::Parsed(_) => None,
        ClippyTomlParseState::Invalid(reason) => Some(reason),
    }
}

pub fn top_level_keys(document: &ClippyTomlDocument) -> Vec<&str> {
    document
        .raw
        .as_table()
        .map(|table| table.keys().map(String::as_str).collect())
        .unwrap_or_default()
}

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
            Value::Table(table) => match table.get("path") {
                Some(Value::String(path)) => {
                    if let Some(reason) = table.get("reason") {
                        if !reason.is_str() {
                            malformed_messages.push(format!(
                                "`{key}[{index}].reason` must be a string when present, found {}.",
                                value_kind(reason)
                            ));
                            continue;
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
                None => malformed_messages.push(format!(
                    "`{key}[{index}]` must contain a string `path` field."
                )),
            },
            other => malformed_messages.push(format!(
                "`{key}[{index}]` must be a string or table, found {}.",
                value_kind(other)
            )),
        }
    }

    ClippyBanSection {
        entries: parsed_entries,
        malformed_messages,
    }
}

pub fn bool_setting<'a>(document: &'a ClippyTomlDocument, key: &str) -> ClippyBoolSetting<'a> {
    match document.raw.get(key) {
        None => ClippyBoolSetting::Missing,
        Some(value) => match value.as_bool() {
            Some(actual) => ClippyBoolSetting::Value(actual),
            None => ClippyBoolSetting::WrongType(value),
        },
    }
}

fn value_kind(value: &Value) -> &'static str {
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
