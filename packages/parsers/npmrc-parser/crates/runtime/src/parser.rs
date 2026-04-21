use std::collections::BTreeMap;

use npmrc_parser_types::document::{NpmrcDocument, NpmrcParseState, NpmrcSetting, NpmrcSnapshot};

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized .npmrc parser"
)]
pub fn parse(input: &str) -> Result<NpmrcSnapshot, crate::error::Error> {
    normalize_snapshot(input).map_err(crate::error::Error::Json)
}

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized .npmrc parser"
)]
pub fn parse_document(input: &str) -> Result<NpmrcDocument, crate::error::Error> {
    let raw = input.to_owned();
    let typed = match normalize_snapshot(input) {
        Ok(snapshot) => NpmrcParseState::Parsed(snapshot),
        Err(reason) => NpmrcParseState::Invalid(reason),
    };
    Ok(NpmrcDocument { raw, typed })
}

pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<NpmrcSnapshot, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

pub fn from_path_document(
    path: impl AsRef<std::path::Path>,
) -> Result<NpmrcDocument, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse_document(&content)
}

fn normalize_snapshot(raw_input: &str) -> Result<NpmrcSnapshot, String> {
    let input = raw_input.strip_prefix('\u{FEFF}').unwrap_or(raw_input);
    let mut settings = Vec::new();

    for (line_idx, raw_line) in input.lines().enumerate() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }

        let Some(eq_idx) = trimmed.find('=') else {
            return Err(format!(
                ".npmrc line {} must use key=value syntax",
                line_idx + 1
            ));
        };

        let key = trimmed[..eq_idx].trim();
        if key.is_empty() {
            return Err(format!(".npmrc line {} has an empty key", line_idx + 1));
        }

        let value = normalize_value(&trimmed[eq_idx + 1..]);
        settings.push(NpmrcSetting {
            key: key.to_owned(),
            value,
        });
    }

    let duplicate_keys = duplicate_keys(&settings);
    Ok(NpmrcSnapshot {
        settings,
        duplicate_keys,
    })
}

fn normalize_value(raw_value: &str) -> String {
    let without_inline_comment = strip_inline_comment(raw_value.trim());
    let trimmed = without_inline_comment.trim();

    if let Some(stripped) = trimmed
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
    {
        return stripped.to_owned();
    }

    if let Some(stripped) = trimmed
        .strip_prefix('\'')
        .and_then(|value| value.strip_suffix('\''))
    {
        return stripped.to_owned();
    }

    trimmed.to_owned()
}

fn strip_inline_comment(value: &str) -> &str {
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    for (idx, ch) in value.char_indices() {
        match ch {
            '\'' if !in_double_quote => in_single_quote = !in_single_quote,
            '"' if !in_single_quote => in_double_quote = !in_double_quote,
            '#' | ';' if !in_single_quote && !in_double_quote => return &value[..idx],
            _ => {}
        }
    }

    value
}

fn duplicate_keys(settings: &[NpmrcSetting]) -> Vec<String> {
    let mut counts = BTreeMap::<&str, usize>::new();
    for setting in settings {
        let entry = counts.entry(setting.key.as_str()).or_insert(0);
        *entry += 1;
    }

    counts
        .into_iter()
        .filter_map(|(key, count)| (count > 1).then_some(key.to_owned()))
        .collect()
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"]
mod parser_tests;
