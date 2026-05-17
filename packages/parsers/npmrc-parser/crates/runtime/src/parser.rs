use std::collections::BTreeMap;

use npmrc_parser_types::document::{NpmrcDocument, NpmrcParseState, NpmrcSetting, NpmrcSnapshot};

/// Parses `.npmrc` content into a typed snapshot.
///
/// # Errors
/// Returns `Error::Json` when a `.npmrc` line is malformed (missing `=` or empty key).
#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized .npmrc parser"
)]
pub fn parse(input: &str) -> Result<NpmrcSnapshot, crate::error::Error> {
    normalize_snapshot(input).map_err(crate::error::Error::Json)
}

/// Parses `.npmrc` content into a typed document, capturing schema mismatches as `Invalid`.
#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized .npmrc parser"
)]
#[must_use]
pub fn parse_document(input: &str) -> NpmrcDocument {
    let raw = input.to_owned();
    let typed = match normalize_snapshot(input) {
        Ok(snapshot) => NpmrcParseState::Parsed(snapshot),
        Err(reason) => NpmrcParseState::Invalid(reason),
    };
    NpmrcDocument { raw, typed }
}

/// Reads a `.npmrc` file from `path` and parses it into a typed snapshot.
///
/// # Errors
/// Returns `Error::Io` if the file cannot be read, or `Error::Json` if a line is malformed.
pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<NpmrcSnapshot, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content)
}

/// Reads a `.npmrc` file from `path` and parses it into a typed document.
///
/// # Errors
/// Returns `Error::Io` if the file cannot be read.
pub fn from_path_document(
    path: impl AsRef<std::path::Path>,
) -> Result<NpmrcDocument, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    Ok(parse_document(&content))
}

/// Walks `.npmrc` lines, building the typed snapshot or describing the first malformed line.
fn normalize_snapshot(raw_input: &str) -> Result<NpmrcSnapshot, String> {
    let input = raw_input.strip_prefix('\u{FEFF}').unwrap_or(raw_input);
    let mut settings = Vec::new();

    for (line_idx, raw_line) in input.lines().enumerate() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }

        let display_line = line_idx.saturating_add(1);
        let Some(eq_idx) = trimmed.find('=') else {
            return Err(format!(
                ".npmrc line {display_line} must use key=value syntax"
            ));
        };

        let (key_part, value_part) = trimmed.split_at(eq_idx);
        let key = key_part.trim();
        if key.is_empty() {
            return Err(format!(".npmrc line {display_line} has an empty key"));
        }

        // Strip the leading `=` byte that `split_at` left in `value_part`.
        let value_after_eq = value_part.get(1..).unwrap_or("");
        let value = normalize_value(value_after_eq);
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

/// Trims and unwraps a single `key=value` right-hand side into the canonical stored value.
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

/// Trims an inline `#` or `;` comment from a value, respecting quoted regions.
fn strip_inline_comment(value: &str) -> &str {
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    for (idx, ch) in value.char_indices() {
        match ch {
            '\'' if !in_double_quote => in_single_quote = !in_single_quote,
            '"' if !in_single_quote => in_double_quote = !in_double_quote,
            '#' | ';' if !in_single_quote && !in_double_quote => {
                return value.get(..idx).unwrap_or("");
            }
            _ => {}
        }
    }

    value
}

/// Returns the sorted list of keys that appear more than once across `settings`.
fn duplicate_keys(settings: &[NpmrcSetting]) -> Vec<String> {
    let mut counts = BTreeMap::<&str, usize>::new();
    for setting in settings {
        let entry = counts.entry(setting.key.as_str()).or_insert(0_usize);
        *entry = entry.saturating_add(1);
    }

    counts
        .into_iter()
        .filter_map(|(key, count)| (count > 1).then_some(key.to_owned()))
        .collect()
}
