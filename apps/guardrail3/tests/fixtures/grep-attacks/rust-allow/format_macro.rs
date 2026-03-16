// Adversarial fixture: #[allow()] pattern inside a format!() macro string argument.
// Grep falsely flags this. A syn-based scanner should NOT flag it.

fn main() {
    let msg = format!("Use #[allow(clippy::unwrap_used)] to suppress this lint");
    assert!(!msg.is_empty());
}
