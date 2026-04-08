pub(crate) fn line_text(content: &str, line: usize) -> &str {
    content
        .lines()
        .nth(line.saturating_sub(1))
        .map(str::trim)
        .unwrap_or("")
}
