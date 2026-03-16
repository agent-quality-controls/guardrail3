// Adversarial fixture: R58 false positive
// "use std::fs" appears inside a line comment, not as an actual import.
// Grep-based R58 will flag this. AST-based check should NOT.

// Don't use std::fs directly
fn main() {}
