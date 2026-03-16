// Adversarial fixture: #[allow()] pattern inside a block comment.
// Grep falsely flags this. A syn-based scanner should NOT flag it.

/*
 * You can suppress warnings with #[allow(clippy::panic)]
 * but this is discouraged.
 */
fn main() {}
