// Adversarial fixture: R58 false positive
// "use std::fs" appears inside a string literal, not as an actual import.
// Grep-based R58 will flag this. AST-based check should NOT.

fn main() {
    let msg = "use std::fs";
    println!("{msg}");
}
