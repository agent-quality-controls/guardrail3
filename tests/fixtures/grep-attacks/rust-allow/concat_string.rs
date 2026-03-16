// Adversarial fixture: #[allow()] pattern constructed via string concatenation.
// Grep falsely flags this. A syn-based scanner should NOT flag it.

fn main() {
    let attr = String::from("#[allow(") + "clippy::todo)]";
    assert!(attr.contains("#[allow("));
}
