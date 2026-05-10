#![allow(
    clippy::excessive_nesting,
    clippy::missing_docs_in_private_items,
    clippy::wildcard_enum_match_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::question_mark,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::needless_pass_by_value,
    clippy::expect_used,
    clippy::panic,
    clippy::format_collect,
    clippy::format_in_format_args,
    clippy::option_if_let_else,
    clippy::map_unwrap_or,
    clippy::if_same_then_else,
    clippy::match_same_arms,
    clippy::match_like_matches_macro,
    clippy::nonminimal_bool,
    clippy::single_match_else,
    clippy::items_after_statements,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::needless_for_each,
    clippy::manual_let_else,
    clippy::redundant_else,
    clippy::shadow_unrelated,
    clippy::struct_excessive_bools,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::module_name_repetitions,
    clippy::large_enum_variant,
    clippy::large_types_passed_by_value,
    clippy::ptr_arg,
    clippy::needless_collect,
    clippy::branches_sharing_code,
    clippy::unused_self,
    reason = "code-source-checks parse/visitor walks every variant of large external syntax-tree enums (syn::Type, syn::Item, syn::Expr, syn::Pat, etc.) and the ban-detection visitors mirror the source structure they are looking for; the rule modules accept the schema-versioned shape verbatim because the per-rule findings depend on the exact spans and the rule ids embed the schema."
)]

/// Type alias `NumberedLine` used by this module.
type NumberedLine = (usize, ());

/// Implements `effective non comment line count`.
pub(crate) fn effective_non_comment_line_count(content: &str) -> usize {
    filter_non_comment_lines(content).len()
}

/// Implements `line text`.
pub(crate) fn line_text(content: &str, line: usize) -> &str {
    content
        .lines()
        .nth(line.saturating_sub(1))
        .unwrap_or("")
        .trim()
}

/// Implements `same line reason`.
pub(crate) fn same_line_reason(content: &str, line: usize) -> Option<String> {
    content
        .lines()
        .nth(line.saturating_sub(1))
        .and_then(trailing_line_comment)
        .and_then(|comment| {
            if !comment.starts_with(" reason:") {
                return None;
            }
            let reason = comment.get(" reason:".len()..)?.trim();
            if reason.is_empty() {
                None
            } else {
                Some(reason.to_owned())
            }
        })
}

/// Implements `same line has comment`.
pub(crate) fn same_line_has_comment(content: &str, line: usize) -> bool {
    content
        .lines()
        .nth(line.saturating_sub(1))
        .is_some_and(|source_line| {
            let stripped = strip_string_literals(source_line);
            stripped.contains("//") || stripped.contains("/*")
        })
}

/// Implements `filter non comment lines`.
fn filter_non_comment_lines(content: &str) -> Vec<NumberedLine> {
    let mut result = Vec::new();
    let bytes = content.as_bytes();
    let mut i = 0usize;
    let mut line_num = 0usize;
    let mut line_has_code = false;

    #[derive(Clone, Copy)]
    enum ScanState {
        Normal,
        LineComment,
        BlockComment(usize),
        String { escaped: bool },
        RawString { hashes: usize },
    }

    let mut state = ScanState::Normal;

    while i < bytes.len() {
        let byte = bytes[i];
        if byte == b'\n' {
            if line_has_code {
                result.push((line_num, ()));
            }
            line_num = line_num.saturating_add(1);
            line_has_code = false;
            if matches!(state, ScanState::LineComment) {
                state = ScanState::Normal;
            }
            i = i.saturating_add(1);
            continue;
        }

        state = match state {
            ScanState::Normal => {
                if starts_line_comment(bytes, i) {
                    i = i.saturating_add(2);
                    ScanState::LineComment
                } else if starts_block_comment(bytes, i) {
                    i = i.saturating_add(2);
                    ScanState::BlockComment(1)
                } else if let Some((prefix_len, hashes)) = raw_string_prefix(bytes, i) {
                    line_has_code = true;
                    i = i.saturating_add(prefix_len);
                    ScanState::RawString { hashes }
                } else if starts_byte_or_plain_string(bytes, i) {
                    line_has_code = true;
                    i = i.saturating_add(if bytes[i] == b'b' { 2 } else { 1 });
                    ScanState::String { escaped: false }
                } else {
                    if !bytes[i].is_ascii_whitespace() {
                        line_has_code = true;
                    }
                    i = i.saturating_add(1);
                    ScanState::Normal
                }
            }
            ScanState::LineComment => {
                i = i.saturating_add(1);
                ScanState::LineComment
            }
            ScanState::BlockComment(depth) => {
                if starts_block_comment(bytes, i) {
                    i = i.saturating_add(2);
                    ScanState::BlockComment(depth.saturating_add(1))
                } else if ends_block_comment(bytes, i) {
                    i = i.saturating_add(2);
                    if depth == 1 {
                        ScanState::Normal
                    } else {
                        ScanState::BlockComment(depth.saturating_sub(1))
                    }
                } else {
                    i = i.saturating_add(1);
                    ScanState::BlockComment(depth)
                }
            }
            ScanState::String { escaped } => {
                if escaped {
                    i = i.saturating_add(1);
                    ScanState::String { escaped: false }
                } else if byte == b'\\' {
                    i = i.saturating_add(1);
                    ScanState::String { escaped: true }
                } else if byte == b'"' {
                    i = i.saturating_add(1);
                    ScanState::Normal
                } else {
                    i = i.saturating_add(1);
                    ScanState::String { escaped: false }
                }
            }
            ScanState::RawString { hashes } => {
                if raw_string_terminator(bytes, i, hashes) {
                    i = i.saturating_add(1 + hashes);
                    ScanState::Normal
                } else {
                    i = i.saturating_add(1);
                    ScanState::RawString { hashes }
                }
            }
        };
    }

    if line_has_code {
        result.push((line_num, ()));
    }

    result
}

/// Implements `starts line comment`.
fn starts_line_comment(bytes: &[u8], index: usize) -> bool {
    bytes.get(index) == Some(&b'/') && bytes.get(index.saturating_add(1)) == Some(&b'/')
}

/// Implements `starts block comment`.
fn starts_block_comment(bytes: &[u8], index: usize) -> bool {
    bytes.get(index) == Some(&b'/') && bytes.get(index.saturating_add(1)) == Some(&b'*')
}

/// Implements `ends block comment`.
fn ends_block_comment(bytes: &[u8], index: usize) -> bool {
    bytes.get(index) == Some(&b'*') && bytes.get(index.saturating_add(1)) == Some(&b'/')
}

/// Implements `starts byte or plain string`.
fn starts_byte_or_plain_string(bytes: &[u8], index: usize) -> bool {
    bytes.get(index) == Some(&b'"')
        || (bytes.get(index) == Some(&b'b') && bytes.get(index.saturating_add(1)) == Some(&b'"'))
}

/// Implements `starts byte or plain char`.
fn starts_byte_or_plain_char(bytes: &[u8], index: usize) -> bool {
    bytes.get(index) == Some(&b'\'')
        || (bytes.get(index) == Some(&b'b') && bytes.get(index.saturating_add(1)) == Some(&b'\''))
}

/// Implements `trailing line comment`.
fn trailing_line_comment(line: &str) -> Option<&str> {
    let bytes = line.as_bytes();
    let mut i = 0usize;

    #[derive(Clone, Copy)]
    enum ScanState {
        Normal,
        BlockComment(usize),
        String { escaped: bool },
        Char { escaped: bool },
        RawString { hashes: usize },
    }

    let mut state = ScanState::Normal;

    while i < bytes.len() {
        state = match state {
            ScanState::Normal => {
                if starts_line_comment(bytes, i) {
                    return line.get(i.saturating_add(2)..);
                }
                if starts_block_comment(bytes, i) {
                    i = i.saturating_add(2);
                    ScanState::BlockComment(1)
                } else if let Some((prefix_len, hashes)) = raw_string_prefix(bytes, i) {
                    i = i.saturating_add(prefix_len);
                    ScanState::RawString { hashes }
                } else if starts_byte_or_plain_string(bytes, i) {
                    i = i.saturating_add(if bytes[i] == b'b' { 2 } else { 1 });
                    ScanState::String { escaped: false }
                } else if starts_byte_or_plain_char(bytes, i) {
                    i = i.saturating_add(if bytes[i] == b'b' { 2 } else { 1 });
                    ScanState::Char { escaped: false }
                } else {
                    i = i.saturating_add(1);
                    ScanState::Normal
                }
            }
            ScanState::BlockComment(depth) => {
                if starts_block_comment(bytes, i) {
                    i = i.saturating_add(2);
                    ScanState::BlockComment(depth.saturating_add(1))
                } else if ends_block_comment(bytes, i) {
                    i = i.saturating_add(2);
                    if depth == 1 {
                        ScanState::Normal
                    } else {
                        ScanState::BlockComment(depth.saturating_sub(1))
                    }
                } else {
                    i = i.saturating_add(1);
                    ScanState::BlockComment(depth)
                }
            }
            ScanState::String { escaped } => {
                let byte = bytes[i];
                if escaped {
                    i = i.saturating_add(1);
                    ScanState::String { escaped: false }
                } else if byte == b'\\' {
                    i = i.saturating_add(1);
                    ScanState::String { escaped: true }
                } else if byte == b'"' {
                    i = i.saturating_add(1);
                    ScanState::Normal
                } else {
                    i = i.saturating_add(1);
                    ScanState::String { escaped: false }
                }
            }
            ScanState::Char { escaped } => {
                let byte = bytes[i];
                if escaped {
                    i = i.saturating_add(1);
                    ScanState::Char { escaped: false }
                } else if byte == b'\\' {
                    i = i.saturating_add(1);
                    ScanState::Char { escaped: true }
                } else if byte == b'\'' {
                    i = i.saturating_add(1);
                    ScanState::Normal
                } else {
                    i = i.saturating_add(1);
                    ScanState::Char { escaped: false }
                }
            }
            ScanState::RawString { hashes } => {
                if raw_string_terminator(bytes, i, hashes) {
                    i = i.saturating_add(1 + hashes);
                    ScanState::Normal
                } else {
                    i = i.saturating_add(1);
                    ScanState::RawString { hashes }
                }
            }
        };
    }

    None
}

/// Implements `strip string literals`.
fn strip_string_literals(line: &str) -> String {
    let bytes = line.as_bytes();
    let mut out = String::with_capacity(line.len());
    let mut i = 0usize;

    #[derive(Clone, Copy)]
    enum ScanState {
        Normal,
        String { escaped: bool },
        Char { escaped: bool },
        RawString { hashes: usize },
    }

    let mut state = ScanState::Normal;

    while i < bytes.len() {
        state = match state {
            ScanState::Normal => {
                if let Some((prefix_len, hashes)) = raw_string_prefix(bytes, i) {
                    for _ in 0..prefix_len {
                        out.push(' ');
                    }
                    i = i.saturating_add(prefix_len);
                    ScanState::RawString { hashes }
                } else if starts_byte_or_plain_string(bytes, i) {
                    let prefix_len = if bytes[i] == b'b' { 2 } else { 1 };
                    for _ in 0..prefix_len {
                        out.push(' ');
                    }
                    i = i.saturating_add(prefix_len);
                    ScanState::String { escaped: false }
                } else if starts_byte_or_plain_char(bytes, i) {
                    let prefix_len = if bytes[i] == b'b' { 2 } else { 1 };
                    for _ in 0..prefix_len {
                        out.push(' ');
                    }
                    i = i.saturating_add(prefix_len);
                    ScanState::Char { escaped: false }
                } else {
                    out.push(char::from(bytes[i]));
                    i = i.saturating_add(1);
                    ScanState::Normal
                }
            }
            ScanState::String { escaped } => {
                let byte = bytes[i];
                out.push(' ');
                if escaped {
                    i = i.saturating_add(1);
                    ScanState::String { escaped: false }
                } else if byte == b'\\' {
                    i = i.saturating_add(1);
                    ScanState::String { escaped: true }
                } else if byte == b'"' {
                    i = i.saturating_add(1);
                    ScanState::Normal
                } else {
                    i = i.saturating_add(1);
                    ScanState::String { escaped: false }
                }
            }
            ScanState::Char { escaped } => {
                let byte = bytes[i];
                out.push(' ');
                if escaped {
                    i = i.saturating_add(1);
                    ScanState::Char { escaped: false }
                } else if byte == b'\\' {
                    i = i.saturating_add(1);
                    ScanState::Char { escaped: true }
                } else if byte == b'\'' {
                    i = i.saturating_add(1);
                    ScanState::Normal
                } else {
                    i = i.saturating_add(1);
                    ScanState::Char { escaped: false }
                }
            }
            ScanState::RawString { hashes } => {
                out.push(' ');
                if raw_string_terminator(bytes, i, hashes) {
                    for _ in 0..hashes {
                        out.push(' ');
                    }
                    i = i.saturating_add(1 + hashes);
                    ScanState::Normal
                } else {
                    i = i.saturating_add(1);
                    ScanState::RawString { hashes }
                }
            }
        };
    }

    out
}

/// Implements `raw string prefix`.
fn raw_string_prefix(bytes: &[u8], index: usize) -> Option<(usize, usize)> {
    let starts_with_r = bytes.get(index) == Some(&b'r');
    let starts_with_br =
        bytes.get(index) == Some(&b'b') && bytes.get(index.saturating_add(1)) == Some(&b'r');
    if !starts_with_r && !starts_with_br {
        return None;
    }

    let mut cursor = index + if starts_with_br { 2 } else { 1 };
    let mut hashes = 0usize;
    while bytes.get(cursor) == Some(&b'#') {
        hashes = hashes.saturating_add(1);
        cursor = cursor.saturating_add(1);
    }
    if bytes.get(cursor) != Some(&b'"') {
        return None;
    }
    Some((cursor.saturating_sub(index).saturating_add(1), hashes))
}

/// Implements `raw string terminator`.
fn raw_string_terminator(bytes: &[u8], index: usize, hashes: usize) -> bool {
    if bytes.get(index) != Some(&b'"') {
        return false;
    }
    (0..hashes).all(|offset| bytes.get(index.saturating_add(1 + offset)) == Some(&b'#'))
}
