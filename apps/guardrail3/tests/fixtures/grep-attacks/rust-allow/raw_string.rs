// Adversarial fixture: #[allow()] pattern inside a raw string literal.
// Grep falsely flags this. A syn-based scanner should NOT flag it.

fn main() {
    let s = r#"#[allow(dead_code)]"#;
    assert!(!s.is_empty());
}
