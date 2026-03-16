// Adversarial fixture: #[allow()] pattern inside a byte string literal.
// Grep falsely flags this. A syn-based scanner should NOT flag it.

fn main() {
    let b = b"#[allow(dead_code)]";
    assert!(!b.is_empty());
}
