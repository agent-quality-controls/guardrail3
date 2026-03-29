use super::same_line_reason;
use guardrail3_app_rs_family_code_assertions::comments::assert_same_line_reason;

#[test]
fn accepts_only_exact_same_line_reason_prefix() {
    let content = "#[allow(clippy::unwrap_used)] // reason: documented seam\n#[allow(clippy::panic)] // REASON: uppercase\n#[allow(clippy::expect_used)] //reason: tight\n";

    assert_same_line_reason(
        same_line_reason(content, 1),
        Some("documented seam"),
    );
    assert_same_line_reason(same_line_reason(content, 2), None);
    assert_same_line_reason(same_line_reason(content, 3), None);
}

#[test]
fn ignores_comment_markers_inside_strings_and_raw_strings() {
    let content = concat!(
        "let url = \"https://example.com/policy\"; #[allow(clippy::unwrap_used)] // reason: external policy link https://example.com/policy\n",
        "let snippet = r#\"// reason: forged\"#; #[allow(clippy::panic)] // reason: raw string stays inert\n",
    );

    assert_same_line_reason(
        same_line_reason(content, 1),
        Some("external policy link https://example.com/policy"),
    );
    assert_same_line_reason(
        same_line_reason(content, 2),
        Some("raw string stays inert"),
    );
}
