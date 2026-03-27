type NumberedLine = (usize, String);

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
        .and_then(|source_line| source_line.split("//").nth(1))
        .and_then(|comment| {
            let trimmed = comment.trim();
            let lower = trimmed.to_ascii_lowercase();
            if !lower.starts_with("reason:") {
                return None;
            }
            let reason = trimmed.get("reason:".len()..)?.trim();
            if reason.is_empty() {
                None
            } else {
                Some(reason.to_owned())
            }
        })
}

fn filter_non_comment_lines(content: &str) -> Vec<NumberedLine> {
    let mut result = Vec::new();
    let mut in_block_comment = false;

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim().to_owned();
        let for_comment_check = strip_string_literals(&trimmed);

        if in_block_comment {
            if let Some(end_pos) = for_comment_check.find("*/") {
                let after = trimmed
                    .get(end_pos.saturating_add(2)..)
                    .unwrap_or("")
                    .trim()
                    .to_owned();
                let after_for_check = strip_string_literals(&after);
                if after_for_check.contains("/*") {
                    in_block_comment = true;
                    if let Some(new_open) = after_for_check.find("/*") {
                        let before_new = after.get(..new_open).unwrap_or("").trim().to_owned();
                        if !before_new.is_empty() && !before_new.starts_with("//") {
                            result.push((line_num, before_new));
                        }
                    }
                } else {
                    in_block_comment = false;
                    if !after.is_empty() && !after.starts_with("//") {
                        result.push((line_num, after));
                    }
                }
            }
            continue;
        }

        let processed = strip_inline_block_comments(&trimmed);
        let processed_for_check = strip_string_literals(&processed);
        if let Some(open_pos) = processed_for_check.find("/*") {
            let before = processed.get(..open_pos).unwrap_or("").trim().to_owned();
            in_block_comment = true;
            if !before.is_empty() && !before.starts_with("//") {
                result.push((line_num, before));
            }
            continue;
        }

        let final_trimmed = processed.trim().to_owned();
        if final_trimmed.is_empty()
            || final_trimmed.starts_with("//")
            || final_trimmed.starts_with("///")
        {
            continue;
        }

        result.push((line_num, final_trimmed));
    }

    result
}

fn strip_inline_block_comments(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let mut remaining = line;

    loop {
        let remaining_for_check = strip_string_literals(remaining);
        match remaining_for_check.find("/*") {
            Some(start) => {
                result.push_str(remaining.get(..start).unwrap_or(""));
                let check_rest = strip_string_literals(remaining.get(start..).unwrap_or(""));
                match check_rest.find("*/") {
                    Some(end) => {
                        remaining = remaining
                            .get(start.saturating_add(end).saturating_add(2)..)
                            .unwrap_or("");
                    }
                    None => {
                        result.push_str(remaining.get(start..).unwrap_or(""));
                        break;
                    }
                }
            }
            None => {
                result.push_str(remaining);
                break;
            }
        }
    }

    result
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
