type NumberedLine = (usize, ());

pub fn effective_non_comment_line_count(content: &str) -> usize {
    filter_non_comment_lines(content).len()
}

pub fn line_text<'a>(content: &'a str, line: usize) -> &'a str {
    content
        .lines()
        .nth(line.saturating_sub(1))
        .unwrap_or("")
        .trim()
}

pub fn same_line_reason(content: &str, line: usize) -> Option<String> {
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

pub fn same_line_has_comment(content: &str, line: usize) -> bool {
    content
        .lines()
        .nth(line.saturating_sub(1))
        .is_some_and(|source_line| {
            let stripped = strip_string_literals(source_line);
            stripped.contains("//") || stripped.contains("/*")
        })
}

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

fn starts_line_comment(bytes: &[u8], index: usize) -> bool {
    bytes.get(index) == Some(&b'/') && bytes.get(index.saturating_add(1)) == Some(&b'/')
}

fn starts_block_comment(bytes: &[u8], index: usize) -> bool {
    bytes.get(index) == Some(&b'/') && bytes.get(index.saturating_add(1)) == Some(&b'*')
}

fn ends_block_comment(bytes: &[u8], index: usize) -> bool {
    bytes.get(index) == Some(&b'*') && bytes.get(index.saturating_add(1)) == Some(&b'/')
}

fn starts_byte_or_plain_string(bytes: &[u8], index: usize) -> bool {
    bytes.get(index) == Some(&b'"')
        || (bytes.get(index) == Some(&b'b') && bytes.get(index.saturating_add(1)) == Some(&b'"'))
}

fn starts_byte_or_plain_char(bytes: &[u8], index: usize) -> bool {
    bytes.get(index) == Some(&b'\'')
        || (bytes.get(index) == Some(&b'b') && bytes.get(index.saturating_add(1)) == Some(&b'\''))
}

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

fn raw_string_prefix(bytes: &[u8], index: usize) -> Option<(usize, usize)> {
    let prefix_offset = if bytes.get(index) == Some(&b'r') {
        0
    } else if bytes.get(index) == Some(&b'b') && bytes.get(index.saturating_add(1)) == Some(&b'r') {
        1
    } else {
        return None;
    };

    let mut hashes = 0usize;
    let mut cursor = index.saturating_add(prefix_offset).saturating_add(1);
    while bytes.get(cursor) == Some(&b'#') {
        hashes = hashes.saturating_add(1);
        cursor = cursor.saturating_add(1);
    }
    if bytes.get(cursor) != Some(&b'"') {
        return None;
    }

    let prefix_len = cursor.saturating_sub(index).saturating_add(1);
    Some((prefix_len, hashes))
}

fn raw_string_terminator(bytes: &[u8], index: usize, hashes: usize) -> bool {
    if bytes.get(index) != Some(&b'"') {
        return false;
    }
    (0..hashes).all(|offset| bytes.get(index.saturating_add(1 + offset)) == Some(&b'#'))
}

fn strip_string_literals(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars.get(i).copied() == Some('r') {
            let mut hashes = 0usize;
            let mut j = i.saturating_add(1);
            while chars.get(j).copied() == Some('#') {
                hashes = hashes.saturating_add(1);
                j = j.saturating_add(1);
            }
            if chars.get(j).copied() == Some('"') && (hashes > 0 || j == i.saturating_add(1)) {
                if i > 0
                    && chars
                        .get(i.saturating_sub(1))
                        .copied()
                        .is_some_and(|ch| ch.is_alphanumeric() || ch == '_')
                {
                    if let Some(ch) = chars.get(i).copied() {
                        result.push(ch);
                    }
                    i = i.saturating_add(1);
                    continue;
                }
                let terminator = format!("\"{}", "#".repeat(hashes));
                let start_idx = line
                    .char_indices()
                    .nth(j)
                    .map_or(line.len(), |(idx, _)| idx);
                let rest = line.get(start_idx..).unwrap_or("");
                if let Some(end) = rest.find(&terminator) {
                    i = j.saturating_add(
                        rest.get(..end).unwrap_or("").chars().count() + terminator.chars().count(),
                    );
                } else {
                    break;
                }
                continue;
            }
        }

        if chars.get(i).copied() == Some('"') {
            i = i.saturating_add(1);
            let mut escaped = false;
            while i < len {
                let Some(ch) = chars.get(i).copied() else {
                    break;
                };
                if escaped {
                    escaped = false;
                } else if ch == '\\' {
                    escaped = true;
                } else if ch == '"' {
                    i = i.saturating_add(1);
                    break;
                }
                i = i.saturating_add(1);
            }
            continue;
        }

        if let Some(ch) = chars.get(i).copied() {
            result.push(ch);
        }
        i = i.saturating_add(1);
    }

    result
}

#[cfg(test)]

mod comments_tests;
