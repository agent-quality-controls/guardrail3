// Adversarial fixture: #[allow()] pattern inside a println!() macro string argument.
// Grep falsely flags this. A syn-based scanner should NOT flag it.

fn main() {
    println!("use #[allow(dead_code)] to suppress unused warnings");
}
