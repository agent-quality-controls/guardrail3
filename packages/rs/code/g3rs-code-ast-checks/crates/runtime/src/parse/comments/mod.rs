pub(crate) fn line_text<'a>(content: &'a str, line: usize) -> &'a str {
    content
        .lines()
        .nth(line.saturating_sub(1))
        .unwrap_or("")
        .trim()
}

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

pub(crate) fn same_line_has_comment(content: &str, line: usize) -> bool {
    content
        .lines()
        .nth(line.saturating_sub(1))
        .is_some_and(|source_line| {
            let stripped = strip_string_literals(source_line);
            stripped.contains("//") || stripped.contains("/*")
        })
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

fn raw_string_terminator(bytes: &[u8], index: usize, hashes: usize) -> bool {
    if bytes.get(index) != Some(&b'"') {
        return false;
    }
    (0..hashes).all(|offset| bytes.get(index.saturating_add(1 + offset)) == Some(&b'#'))
}
