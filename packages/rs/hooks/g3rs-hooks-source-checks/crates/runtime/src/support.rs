pub(crate) fn inline_comment_text(line: &str) -> Option<&str> {
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut escaped = false;

    for (index, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        match ch {
            '\\' if double_quoted => {
                escaped = true;
            }
            '\'' if !double_quoted => {
                single_quoted = !single_quoted;
            }
            '"' if !single_quoted => {
                double_quoted = !double_quoted;
            }
            '#' if !single_quoted && !double_quoted => {
                if index == 0 && line.starts_with("#!") {
                    return None;
                }
                return Some(&line[index + 1..]);
            }
            _ => {}
        }
    }

    None
}
