// Adversarial fixture: R58 false positive
// "std::fs" appears inside a doc comment, not as an actual import.
// Grep-based R58 may flag this. AST-based check should NOT.

/// Uses std::fs for file operations
///
/// This module wraps filesystem access through a centralized helper.
fn main() {}
