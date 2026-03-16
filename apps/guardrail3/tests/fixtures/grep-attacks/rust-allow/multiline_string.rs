// Adversarial fixture: #[allow()] pattern split across lines in a multi-line string.
// Grep falsely flags this. A syn-based scanner should NOT flag it.

fn main() {
    let s = "\
#[allow(\
clippy::unwrap_used\
)]";
    assert!(!s.is_empty());
}
