use eslint_directive_parser_types::document::{
    EslintDirectiveDocument, EslintDirectiveFileState, EslintDirectiveFinding, EslintDirectiveKind,
    EslintDirectiveParseState, EslintDisabledRuleSet,
};

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized ESLint directive parser"
)]
pub fn parse(input: &str, rel_path: &str) -> Result<EslintDirectiveFileState, crate::error::Error> {
    Ok(normalize_file_state(input, rel_path))
}

#[allow(
    clippy::disallowed_methods,
    reason = "parser.rs IS the centralized ESLint directive parser"
)]
pub fn parse_document(
    input: &str,
    rel_path: &str,
) -> Result<EslintDirectiveDocument, crate::error::Error> {
    Ok(EslintDirectiveDocument {
        raw: input.to_owned(),
        typed: normalize_file_state(input, rel_path),
    })
}

pub fn from_path(
    path: impl AsRef<std::path::Path>,
    rel_path: &str,
) -> Result<EslintDirectiveFileState, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse(&content, rel_path)
}

pub fn from_path_document(
    path: impl AsRef<std::path::Path>,
    rel_path: &str,
) -> Result<EslintDirectiveDocument, crate::error::Error> {
    let content = crate::fs::read_to_string(path)?;
    parse_document(&content, rel_path)
}

fn normalize_file_state(input: &str, rel_path: &str) -> EslintDirectiveFileState {
    let state = if is_mdx(rel_path) {
        if mdx_has_potential_directive(input) {
            EslintDirectiveParseState::Ambiguous {
                reason: "MDX directive parsing requires an MDX-aware comment parser".to_owned(),
            }
        } else {
            EslintDirectiveParseState::Parsed {
                findings: Vec::new(),
            }
        }
    } else if !is_supported_source(rel_path) {
        EslintDirectiveParseState::Unsupported {
            reason: format!("unsupported ESLint directive source file: {rel_path}"),
        }
    } else if is_astro(rel_path) {
        parse_astro_source(input, rel_path)
    } else {
        parse_supported_source(input, rel_path)
    };

    EslintDirectiveFileState {
        rel_path: rel_path.to_owned(),
        state,
    }
}

fn is_mdx(rel_path: &str) -> bool {
    rel_path.ends_with(".mdx")
}

fn is_supported_source(rel_path: &str) -> bool {
    [
        ".js", ".jsx", ".mjs", ".cjs", ".ts", ".tsx", ".mts", ".cts", ".astro",
    ]
    .iter()
    .any(|suffix| rel_path.ends_with(suffix))
}

fn is_astro(rel_path: &str) -> bool {
    rel_path.ends_with(".astro")
}

fn mdx_has_potential_directive(input: &str) -> bool {
    let EslintDirectiveParseState::Parsed { findings } = parse_supported_source(input, "probe.js")
    else {
        return true;
    };
    !findings.is_empty()
}

fn parse_astro_source(input: &str, rel_path: &str) -> EslintDirectiveParseState {
    let mut findings = Vec::new();
    if input.starts_with("---\n") {
        let frontmatter_start = 4usize;
        if let Some(frontmatter_end) = input[frontmatter_start..].find("\n---") {
            let frontmatter = &input[frontmatter_start..frontmatter_start + frontmatter_end];
            match parse_supported_source(frontmatter, rel_path) {
                EslintDirectiveParseState::Parsed {
                    findings: frontmatter_findings,
                } => findings.extend(frontmatter_findings.into_iter().map(|mut finding| {
                    finding.line += 1;
                    finding.target_line = finding.target_line.map(|target| target + 1);
                    finding
                })),
                state @ (EslintDirectiveParseState::ParseError { .. }
                | EslintDirectiveParseState::Unsupported { .. }
                | EslintDirectiveParseState::Ambiguous { .. }) => return state,
            }
            let template_start = frontmatter_start + frontmatter_end + 4;
            return parse_html_comments_only(input, template_start, rel_path, findings);
        }
    }
    parse_html_comments_only(input, 0, rel_path, findings)
}

fn parse_html_comments_only(
    input: &str,
    mut idx: usize,
    rel_path: &str,
    mut findings: Vec<EslintDirectiveFinding>,
) -> EslintDirectiveParseState {
    let bytes = input.as_bytes();
    let mut line = count_newlines(&input[..idx]) + 1;

    while idx < bytes.len() {
        if starts_with(bytes, idx, b"<!--") {
            let Some(end) = find_marker(bytes, idx + 4, b"-->") else {
                return EslintDirectiveParseState::ParseError {
                    reason: "unterminated HTML comment".to_owned(),
                };
            };
            let comment_end_line = line + count_newlines(&input[idx..end + 3]);
            if let Err(reason) = collect_directive_text(
                &input[idx + 4..end],
                rel_path,
                line,
                comment_end_line + 1,
                &mut findings,
            ) {
                return EslintDirectiveParseState::ParseError { reason };
            }
            line = comment_end_line;
            idx = end + 3;
            continue;
        }
        if bytes[idx] == b'\n' {
            line += 1;
        }
        idx += 1;
    }

    EslintDirectiveParseState::Parsed { findings }
}

fn parse_supported_source(input: &str, rel_path: &str) -> EslintDirectiveParseState {
    let bytes = input.as_bytes();
    let mut idx = 0usize;
    let mut line = 1u32;
    let mut findings = Vec::new();

    while idx < bytes.len() {
        if starts_with(bytes, idx, b"//") && !is_escaped(bytes, idx) {
            let end = find_line_end(bytes, idx);
            if let Err(reason) = collect_directive_text(
                &input[idx + 2..end],
                rel_path,
                line,
                line + 1,
                &mut findings,
            ) {
                return EslintDirectiveParseState::ParseError { reason };
            }
            idx = end;
            continue;
        }

        if starts_with(bytes, idx, b"/*") && !is_escaped(bytes, idx) {
            let Some(end) = find_marker(bytes, idx + 2, b"*/") else {
                return EslintDirectiveParseState::ParseError {
                    reason: "unterminated block comment".to_owned(),
                };
            };
            let comment_end_line = line + count_newlines(&input[idx..end + 2]);
            if let Err(reason) = collect_directive_text(
                &input[idx + 2..end],
                rel_path,
                line,
                comment_end_line + 1,
                &mut findings,
            ) {
                return EslintDirectiveParseState::ParseError { reason };
            }
            line = comment_end_line;
            idx = end + 2;
            continue;
        }

        if starts_with(bytes, idx, b"<!--") {
            let Some(end) = find_marker(bytes, idx + 4, b"-->") else {
                return EslintDirectiveParseState::ParseError {
                    reason: "unterminated HTML comment".to_owned(),
                };
            };
            let comment_end_line = line + count_newlines(&input[idx..end + 3]);
            if let Err(reason) = collect_directive_text(
                &input[idx + 4..end],
                rel_path,
                line,
                comment_end_line + 1,
                &mut findings,
            ) {
                return EslintDirectiveParseState::ParseError { reason };
            }
            line = comment_end_line;
            idx = end + 3;
            continue;
        }

        match bytes[idx] {
            b'\'' | b'"' => {
                let quote = bytes[idx];
                let Some((next_idx, extra_lines)) = skip_quoted(bytes, idx + 1, quote) else {
                    return EslintDirectiveParseState::ParseError {
                        reason: "unterminated string literal".to_owned(),
                    };
                };
                line += extra_lines;
                idx = next_idx;
            }
            b'`' => {
                let Ok((next_idx, extra_lines)) =
                    skip_template(bytes, input, idx + 1, line, rel_path, &mut findings)
                else {
                    return EslintDirectiveParseState::ParseError {
                        reason: "unterminated template literal".to_owned(),
                    };
                };
                line += extra_lines;
                idx = next_idx;
            }
            b'\n' => {
                line += 1;
                idx += 1;
            }
            _ => idx += 1,
        }
    }

    EslintDirectiveParseState::Parsed { findings }
}

fn starts_with(bytes: &[u8], idx: usize, marker: &[u8]) -> bool {
    bytes.get(idx..idx + marker.len()) == Some(marker)
}

fn is_escaped(bytes: &[u8], idx: usize) -> bool {
    idx > 0 && bytes.get(idx - 1) == Some(&b'\\')
}

fn find_line_end(bytes: &[u8], start: usize) -> usize {
    bytes[start..]
        .iter()
        .position(|byte| *byte == b'\n')
        .map_or(bytes.len(), |offset| start + offset)
}

fn find_marker(bytes: &[u8], start: usize, marker: &[u8]) -> Option<usize> {
    bytes[start..]
        .windows(marker.len())
        .position(|window| window == marker)
        .map(|offset| start + offset)
}

fn skip_quoted(bytes: &[u8], mut idx: usize, quote: u8) -> Option<(usize, u32)> {
    let mut lines = 0u32;
    while idx < bytes.len() {
        match bytes[idx] {
            b'\\' => idx += 2,
            byte if byte == quote => return Some((idx + 1, lines)),
            b'\n' => {
                lines += 1;
                idx += 1;
            }
            _ => idx += 1,
        }
    }
    None
}

fn skip_template(
    bytes: &[u8],
    input: &str,
    mut idx: usize,
    base_line: u32,
    rel_path: &str,
    findings: &mut Vec<EslintDirectiveFinding>,
) -> Result<(usize, u32), String> {
    let mut lines = 0u32;
    while idx < bytes.len() {
        match bytes[idx] {
            b'\\' => idx += 2,
            b'`' => return Ok((idx + 1, lines)),
            b'$' if starts_with(bytes, idx, b"${") => {
                let expression_start = idx + 2;
                let (expression_end, expression_lines) =
                    find_template_expression_end(bytes, expression_start)?;
                let expression_text = &input[expression_start..expression_end];
                let expression_state = parse_supported_source(expression_text, rel_path);
                match expression_state {
                    EslintDirectiveParseState::Parsed {
                        findings: expression_findings,
                    } => {
                        findings.extend(expression_findings.into_iter().map(|mut finding| {
                            finding.line += base_line + lines - 1;
                            finding.target_line = finding
                                .target_line
                                .map(|target| target + base_line + lines - 1);
                            finding
                        }));
                    }
                    EslintDirectiveParseState::ParseError { reason }
                    | EslintDirectiveParseState::Unsupported { reason }
                    | EslintDirectiveParseState::Ambiguous { reason } => return Err(reason),
                }
                lines += expression_lines;
                idx = expression_end + 1;
            }
            b'\n' => {
                lines += 1;
                idx += 1;
            }
            _ => idx += 1,
        }
    }
    Err("unterminated template literal".to_owned())
}

fn find_template_expression_end(bytes: &[u8], mut idx: usize) -> Result<(usize, u32), String> {
    let mut depth = 1u32;
    let mut lines = 0u32;
    while idx < bytes.len() {
        match bytes[idx] {
            b'\'' | b'"' => {
                let quote = bytes[idx];
                let Some((next_idx, extra_lines)) = skip_quoted(bytes, idx + 1, quote) else {
                    return Err("unterminated string literal".to_owned());
                };
                lines += extra_lines;
                idx = next_idx;
            }
            b'`' => {
                let Some((next_idx, extra_lines)) =
                    skip_template_without_expression(bytes, idx + 1)
                else {
                    return Err("unterminated template literal".to_owned());
                };
                lines += extra_lines;
                idx = next_idx;
            }
            b'{' => {
                depth += 1;
                idx += 1;
            }
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok((idx, lines));
                }
                idx += 1;
            }
            b'\n' => {
                lines += 1;
                idx += 1;
            }
            _ => idx += 1,
        }
    }
    Err("unterminated template expression".to_owned())
}

fn skip_template_without_expression(bytes: &[u8], mut idx: usize) -> Option<(usize, u32)> {
    let mut lines = 0u32;
    while idx < bytes.len() {
        match bytes[idx] {
            b'\\' => idx += 2,
            b'`' => return Some((idx + 1, lines)),
            b'\n' => {
                lines += 1;
                idx += 1;
            }
            _ => idx += 1,
        }
    }
    None
}

fn count_newlines(text: &str) -> u32 {
    text.bytes().filter(|byte| *byte == b'\n').count() as u32
}

fn collect_directive_text(
    text: &str,
    rel_path: &str,
    base_line: u32,
    disable_next_line_target: u32,
    findings: &mut Vec<EslintDirectiveFinding>,
) -> Result<(), String> {
    for (offset, line) in text.lines().enumerate() {
        let line_number = base_line + offset as u32;
        collect_directive_line(
            line.trim(),
            rel_path,
            line_number,
            disable_next_line_target,
            findings,
        )?;
    }
    Ok(())
}

fn collect_directive_line(
    text: &str,
    rel_path: &str,
    line: u32,
    disable_next_line_target: u32,
    findings: &mut Vec<EslintDirectiveFinding>,
) -> Result<(), String> {
    let text = normalize_comment_line(text);
    for (marker, kind) in [
        (
            "eslint-disable-next-line",
            EslintDirectiveKind::DisableNextLine,
        ),
        ("eslint-disable-line", EslintDirectiveKind::DisableLine),
        ("eslint-disable", EslintDirectiveKind::Disable),
        ("eslint-enable", EslintDirectiveKind::Enable),
        ("eslint", EslintDirectiveKind::InlineConfig),
    ] {
        let Some(rest) = text.strip_prefix(marker) else {
            continue;
        };
        if !rest.is_empty() && !rest.starts_with(char::is_whitespace) {
            continue;
        }
        let disabled_rules = if kind == EslintDirectiveKind::InlineConfig {
            parse_inline_config_rules(rest.trim())?
        } else {
            parse_disabled_rules(rest.trim())?
        };
        let target_line = match kind {
            EslintDirectiveKind::DisableLine => Some(line),
            EslintDirectiveKind::DisableNextLine => Some(disable_next_line_target),
            EslintDirectiveKind::Disable
            | EslintDirectiveKind::Enable
            | EslintDirectiveKind::InlineConfig => None,
        };
        findings.push(EslintDirectiveFinding {
            rel_path: rel_path.to_owned(),
            directive_kind: kind,
            disabled_rules,
            line,
            target_line,
        });
        return Ok(());
    }
    Ok(())
}

fn normalize_comment_line(text: &str) -> &str {
    let trimmed = text.trim();
    trimmed.strip_prefix('*').map_or(trimmed, str::trim_start)
}

fn parse_disabled_rules(rest: &str) -> Result<EslintDisabledRuleSet, String> {
    let rules_text = rest
        .split_once("--")
        .map_or(rest, |(rules, _description)| rules);
    let rules_text = rules_text.trim();
    if rules_text.is_empty() {
        return Ok(EslintDisabledRuleSet::AllRules);
    }
    let rules = parse_comma_separated_rule_names(rules_text)?;
    if rules.is_empty() {
        Ok(EslintDisabledRuleSet::AllRules)
    } else {
        Ok(EslintDisabledRuleSet::Rules(rules))
    }
}

fn parse_inline_config_rules(rest: &str) -> Result<EslintDisabledRuleSet, String> {
    let config_text = rest
        .split_once("--")
        .map_or(rest, |(rules, _description)| rules);
    let config_text = config_text.trim();
    if config_text.is_empty() {
        return Err("malformed ESLint inline config directive".to_owned());
    }
    let mut rules = Vec::new();
    for entry in config_text.split(',') {
        let entry = entry.trim();
        if entry.is_empty() {
            return Err("malformed ESLint inline config directive".to_owned());
        }
        let Some((rule_name, _setting)) = entry.split_once(':') else {
            return Err("malformed ESLint inline config directive".to_owned());
        };
        let rule_name = rule_name.trim();
        if rule_name.is_empty() || rule_name.chars().any(char::is_whitespace) {
            return Err("malformed ESLint inline config directive".to_owned());
        }
        rules.push(rule_name.to_owned());
    }
    Ok(EslintDisabledRuleSet::Rules(rules))
}

fn parse_comma_separated_rule_names(rules_text: &str) -> Result<Vec<String>, String> {
    let mut rules = Vec::new();
    for raw_rule in rules_text.split(',') {
        let rule = raw_rule.trim();
        if rule.is_empty() || rule.chars().any(char::is_whitespace) {
            return Err("malformed ESLint directive rule list".to_owned());
        }
        rules.push(rule.to_owned());
    }
    Ok(rules)
}

#[cfg(test)]
#[path = "parser_tests/mod.rs"]
mod parser_tests;
