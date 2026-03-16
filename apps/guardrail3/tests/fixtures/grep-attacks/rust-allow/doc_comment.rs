// Adversarial fixture: #[allow()] pattern inside a doc comment.
// Grep falsely flags this. A syn-based scanner should NOT flag it.

/// Example usage:
/// ```
/// #[allow(unused)]
/// fn example() {}
/// ```
fn main() {}
