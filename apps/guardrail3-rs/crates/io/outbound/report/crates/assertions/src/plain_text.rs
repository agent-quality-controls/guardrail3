/// Checks that inventory findings are hidden while warnings remain visible.
///
/// # Panics
///
/// Panics if the rendered output does not keep the warning lines while hiding inventory lines.
pub fn assert_inventory_hidden(output: &str) {
    assert!(
        output.contains("== fmt =="),
        "expected fmt family header in rendered output: {output}"
    );
    assert!(
        output.contains("RS-FMT-CONFIG-02"),
        "expected visible warning result in rendered output: {output}"
    );
    assert!(
        !output.contains("RS-FMT-CONFIG-01"),
        "expected inventory-only finding to stay hidden: {output}"
    );
}

/// Checks that the renderer prints the clean no-findings line.
///
/// # Panics
///
/// Panics if the renderer output is not the exact clean no-findings text.
pub fn assert_no_findings(output: &str) {
    assert_eq!(
        output, "No findings.\n",
        "expected exact clean output line"
    );
}

/// Checks that the rendered output keeps the rule message line.
///
/// # Panics
///
/// Panics if the warning title or its message line is missing.
pub fn assert_includes_rule_message(output: &str) {
    assert!(
        output.contains("[Warn] RS-DENY-CONFIG-27 deny.toml managed ban wrappers changed"),
        "expected deny warning title in rendered output: {output}"
    );
    assert!(
        output.contains("`deny.toml` ban `regex` adds local wrappers `tree-sitter`."),
        "expected deny warning message line in rendered output: {output}"
    );
}
