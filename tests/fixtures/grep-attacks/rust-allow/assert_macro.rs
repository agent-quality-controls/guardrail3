// Adversarial fixture: #[allow()] pattern inside an assert_eq!() macro string argument.
// Grep falsely flags this. A syn-based scanner should NOT flag it.

fn main() {
    let s = String::from("suppress with allow");
    let expected = "#[allow(unused)]";
    assert_eq!(expected, "#[allow(unused)]");
    assert!(!s.is_empty());
}
