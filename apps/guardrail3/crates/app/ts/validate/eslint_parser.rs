//! Tree-sitter-javascript based parser for `ESLint` flat config (`.mjs`) files.
//!
//! Parses the JavaScript AST to extract rules, presets, plugin imports,
//! and marker patterns from `ESLint` flat config files. This replaces
//! string-matching (`content.contains(...)`) with structural analysis.

use std::collections::BTreeMap;

use tree_sitter::{Node, Parser, Tree};

/// Parsed `ESLint` flat config (`.mjs` file).
#[allow(clippy::struct_excessive_bools)] // reason: ESLint config has multiple independent boolean markers
pub struct EslintConfig {
    /// All rules across all config blocks, keyed by rule name.
    /// Value is the LAST occurrence (later blocks override earlier).
    pub rules: BTreeMap<String, RuleConfig>,
    /// Detected preset spreads (e.g., "strictTypeChecked", "stylisticTypeChecked").
    pub presets: Vec<String>,
    /// Whether the boundaries plugin is configured.
    pub has_boundaries: bool,
    /// Whether `process.env` ban is configured (`no-restricted-syntax` with `process.env` selector).
    pub has_process_env_ban: bool,
    /// Whether route wrapper enforcement is configured.
    pub has_route_wrappers: bool,
    /// Whether `RegExp` is banned.
    pub has_regexp_ban: bool,
    /// Raw content (kept for fallback).
    pub raw_content: String,
}

/// Configuration for a single `ESLint` rule.
pub struct RuleConfig {
    /// Rule severity: "error", "warn", or "off".
    pub severity: String,
    /// Numeric option value if present (e.g., max-lines: 300).
    pub numeric_value: Option<u32>,
    /// Whether this rule appears in a test override block (files containing "test"/"spec").
    pub is_test_override: bool,
}

/// Create a fallback `EslintConfig` with only raw content (when parsing fails).
impl EslintConfig {
    pub const fn fallback(content: String) -> Self {
        Self {
            rules: BTreeMap::new(),
            presets: Vec::new(),
            has_boundaries: false,
            has_process_env_ban: false,
            has_route_wrappers: false,
            has_regexp_ban: false,
            raw_content: content,
        }
    }
}

/// Parse an `ESLint` flat config `.mjs` file and extract structured data.
///
/// Returns `None` only if tree-sitter itself fails to parse. Partial parse
/// failures return an `EslintConfig` with whatever was successfully extracted
/// plus the raw content for fallback.
pub fn parse_eslint_config(source: &str) -> Option<EslintConfig> {
    let tree = parse_javascript(source)?;
    let root = tree.root_node();
    let bytes = source.as_bytes();

    let mut config = EslintConfig {
        rules: BTreeMap::new(),
        presets: Vec::new(),
        has_boundaries: false,
        has_process_env_ban: false,
        has_route_wrappers: false,
        has_regexp_ban: false,
        raw_content: source.to_owned(),
    };

    // Pass 1: Scan import declarations for plugin detection
    scan_imports(&root, bytes, &mut config);

    // Pass 2: Find the default export and extract config data
    extract_from_default_export(&root, bytes, &mut config);

    Some(config)
}

// ---------------------------------------------------------------------------
// JavaScript parsing
// ---------------------------------------------------------------------------

/// Parse JavaScript source with tree-sitter-javascript.
fn parse_javascript(source: &str) -> Option<Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_javascript::LANGUAGE.into())
        .ok()?;
    parser.parse(source, None)
}

// ---------------------------------------------------------------------------
// Import scanning
// ---------------------------------------------------------------------------

/// Scan top-level import declarations for plugin names.
fn scan_imports(root: &Node<'_>, source: &[u8], config: &mut EslintConfig) {
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "import_statement" {
            let import_text = node_text(&child, source);
            if import_text.contains("boundaries")
                || import_text.contains("eslint-plugin-boundaries")
            {
                config.has_boundaries = true;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Default export extraction
// ---------------------------------------------------------------------------

/// Find `export default ...` and extract config data from it.
fn extract_from_default_export(root: &Node<'_>, source: &[u8], config: &mut EslintConfig) {
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if child.kind() == "export_statement" {
            // Look for the default keyword
            let stmt_text = node_text(&child, source);
            if !stmt_text.contains("default") {
                continue;
            }
            // The export value could be:
            // 1. `export default tseslint.config(...)` — call_expression
            // 2. `export default [...]` — array
            walk_for_config_data(&child, source, config, false);
        }
    }
}

/// Recursively walk a node looking for config-relevant structures.
fn walk_for_config_data(
    node: &Node<'_>,
    source: &[u8],
    config: &mut EslintConfig,
    in_test_block: bool,
) {
    match node.kind() {
        // Spread elements like `...tseslint.configs.strictTypeChecked`
        "spread_element" => {
            let text = node_text(node, source);
            extract_preset_from_spread(text, config);
        }

        // Object literals — could be config blocks with `rules:` and `files:`
        "object" => {
            let is_test = in_test_block || object_has_test_files(node, source);
            extract_rules_from_object(node, source, config, is_test);
            // Also check for marker patterns in rule values
            check_marker_patterns_in_object(node, source, config);
        }

        // Recurse into all other node types
        _ => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                walk_for_config_data(&child, source, config, in_test_block);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Preset extraction
// ---------------------------------------------------------------------------

/// Extract preset name from a spread like `...tseslint.configs.strictTypeChecked`.
fn extract_preset_from_spread(text: &str, config: &mut EslintConfig) {
    // Pattern: ...something.configs.PRESET_NAME or ...something.configs.PRESET_NAME(...)
    // The preset name is the last dotted segment before any `(` or end-of-string.
    if let Some(configs_idx) = text.find(".configs.") {
        let after = text
            .get(configs_idx.saturating_add(".configs.".len())..)
            .unwrap_or("");
        // Take until non-alphanumeric (handles trailing `,`, `)`, whitespace, etc.)
        let preset: String = after
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if !preset.is_empty() && !config.presets.contains(&preset) {
            config.presets.push(preset);
        }
    }
}

// ---------------------------------------------------------------------------
// Object analysis — files, rules, markers
// ---------------------------------------------------------------------------

/// Check if an object has a `files:` property matching test/spec patterns.
fn object_has_test_files(node: &Node<'_>, source: &[u8]) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "pair" {
            let key = pair_key_text(&child, source);
            if key == "files" {
                let value_text = pair_value_text(&child, source);
                return value_text.contains("test") || value_text.contains("spec");
            }
        }
    }
    false
}

/// Extract rules from an object's `rules: { ... }` property.
fn extract_rules_from_object(
    node: &Node<'_>,
    source: &[u8],
    config: &mut EslintConfig,
    is_test: bool,
) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "pair" {
            let key = pair_key_text(&child, source);
            if key == "rules" {
                // The value should be an object literal with rule entries
                if let Some(value_node) = pair_value_node(&child) {
                    if value_node.kind() == "object" {
                        extract_individual_rules(&value_node, source, config, is_test);
                    }
                }
            }
        }
    }
}

/// Extract individual rule entries from a rules object.
fn extract_individual_rules(
    rules_obj: &Node<'_>,
    source: &[u8],
    config: &mut EslintConfig,
    is_test: bool,
) {
    let mut cursor = rules_obj.walk();
    for child in rules_obj.children(&mut cursor) {
        if child.kind() == "pair" {
            let rule_name = unquote(&pair_key_text(&child, source));
            if rule_name.is_empty() {
                continue;
            }
            let value_text = pair_value_text(&child, source).trim().to_owned();
            let (severity, numeric_value) = parse_rule_value(&value_text);

            // Don't let test-override rules overwrite main rules.
            // Main rules always win; test overrides only stored if no main rule exists.
            let should_insert = !is_test
                || config
                    .rules
                    .get(&rule_name)
                    .is_none_or(|existing| existing.is_test_override);
            if should_insert {
                let _ = config.rules.insert(
                    rule_name,
                    RuleConfig {
                        severity,
                        numeric_value,
                        is_test_override: is_test,
                    },
                );
            }
        }
    }
}

/// Check for marker patterns (`process.env`, route wrappers, `RegExp` ban) in object values.
fn check_marker_patterns_in_object(node: &Node<'_>, source: &[u8], config: &mut EslintConfig) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "pair" {
            let key = pair_key_text(&child, source);
            let value_text = pair_value_text(&child, source);

            // Check for process.env ban in no-restricted-syntax / no-restricted-properties rules
            let unquoted_key = unquote(&key);
            if unquoted_key.contains("no-restricted") && value_text.contains("process.env") {
                config.has_process_env_ban = true;
            }

            // Check for route wrapper enforcement
            if value_text.contains("withBody") || value_text.contains("withRoute") {
                config.has_route_wrappers = true;
            }

            // Check for RegExp ban (no-restricted-globals or no-restricted-syntax with RegExp)
            if unquoted_key.contains("no-restricted") && value_text.contains("RegExp") {
                config.has_regexp_ban = true;
            }

            // Recurse into nested objects for deeper marker detection
            if let Some(val_node) = pair_value_node(&child) {
                if val_node.kind() == "object" || val_node.kind() == "array" {
                    check_marker_patterns_in_object(&val_node, source, config);
                }
            }
        }

        // Also recurse into array elements
        if child.kind() == "array" || child.kind() == "object" {
            check_marker_patterns_in_object(&child, source, config);
        }
    }
}

// ---------------------------------------------------------------------------
// Rule value parsing
// ---------------------------------------------------------------------------

/// Severity string paired with an optional numeric config value (e.g., max-lines threshold).
type RuleValue = (String, Option<u32>);

/// Parse a rule value into severity + optional numeric value.
///
/// Handles:
/// - `"error"` / `"warn"` / `"off"` (or numeric `2`/`1`/`0`)
/// - `["error", 300]` — severity + numeric
/// - `["error", { max: 300 }]` — severity + numeric from options object
#[allow(clippy::string_slice)] // reason: slicing known ASCII severity strings from ESLint config
fn parse_rule_value(value: &str) -> RuleValue {
    let trimmed = value.trim();

    // Direct string severity
    if let Some(sev) = extract_severity(trimmed) {
        return (sev, None);
    }

    // Direct numeric severity (0/1/2)
    if let Some(sev) = numeric_severity(trimmed) {
        return (sev, None);
    }

    // Array form: ["error", ...] or [2, ...]
    // Extract severity from first element and numeric from remaining text
    if trimmed.starts_with('[') {
        return parse_array_rule_value(trimmed);
    }

    // Fallback: unknown format
    ("error".to_owned(), None)
}

/// Parse an array-form rule value like `["error", 300]` or `["error", { max: 300 }]`.
fn parse_array_rule_value(text: &str) -> RuleValue {
    // Determine severity from the text
    let severity = if text.contains("\"warn\"") || text.contains("'warn'") {
        "warn".to_owned()
    } else if text.contains("\"off\"") || text.contains("'off'") {
        "off".to_owned()
    } else {
        "error".to_owned()
    };

    // Look for numeric: after severity, find first bare number
    // This handles both `["error", 300]` and `["error", { max: 300 }]`
    let numeric = extract_first_number(text);

    (severity, numeric)
}

/// Extract a severity string from a quoted value.
fn extract_severity(text: &str) -> Option<String> {
    let unquoted = unquote(text);
    match unquoted.as_str() {
        "error" | "warn" | "off" => Some(unquoted),
        _ => None,
    }
}

/// Convert numeric severity (0/1/2) to string.
fn numeric_severity(text: &str) -> Option<String> {
    match text.trim() {
        "0" => Some("off".to_owned()),
        "1" => Some("warn".to_owned()),
        "2" => Some("error".to_owned()),
        _ => None,
    }
}

/// Extract the first bare integer from text that looks like a config value (not 0/1/2 severity).
fn extract_first_number(text: &str) -> Option<u32> {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    // Skip past the severity part — find the first comma
    while i < len && chars.get(i).is_some_and(|c| *c != ',') {
        i = i.saturating_add(1);
    }

    while i < len {
        if chars.get(i).is_some_and(char::is_ascii_digit) {
            // Check it's not part of a word
            if i > 0
                && chars
                    .get(i.saturating_sub(1))
                    .is_some_and(|c| c.is_alphanumeric() || *c == '_')
            {
                i = i.saturating_add(1);
                continue;
            }
            let start = i;
            while i < len && chars.get(i).is_some_and(char::is_ascii_digit) {
                i = i.saturating_add(1);
            }
            let num_str: String = chars.get(start..i).unwrap_or(&[]).iter().collect();
            if let Ok(n) = num_str.parse::<u32>() {
                // Skip 0/1/2 — those are severity aliases, not config values
                if n > 2 {
                    return Some(n);
                }
            }
        } else {
            i = i.saturating_add(1);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Node helpers
// ---------------------------------------------------------------------------

/// Get the text content of a tree-sitter node.
fn node_text<'a>(node: &Node<'_>, source: &'a [u8]) -> &'a str {
    source
        .get(node.start_byte()..node.end_byte())
        .and_then(|b| std::str::from_utf8(b).ok())
        .unwrap_or("")
}

/// Get the key text from a `pair` node (property: value).
fn pair_key_text(pair: &Node<'_>, source: &[u8]) -> String {
    pair.child_by_field_name("key")
        .map(|k| node_text(&k, source).to_owned())
        .unwrap_or_default()
}

/// Get the value text from a `pair` node.
fn pair_value_text(pair: &Node<'_>, source: &[u8]) -> String {
    pair.child_by_field_name("value")
        .map(|v| node_text(&v, source).to_owned())
        .unwrap_or_default()
}

/// Get the value node from a `pair` node.
fn pair_value_node<'a>(pair: &Node<'a>) -> Option<Node<'a>> {
    pair.child_by_field_name("value")
}

/// Remove surrounding quotes (single or double) from a string.
fn unquote(s: &str) -> String {
    let trimmed = s.trim();
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        trimmed
            .get(1..trimmed.len().saturating_sub(1))
            .unwrap_or("")
            .to_owned()
    } else {
        trimmed.to_owned()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
