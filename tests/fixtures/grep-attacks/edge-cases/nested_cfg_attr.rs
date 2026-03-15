// Deeply nested cfg_attr with allow buried inside.
// grep sees #[allow(unused)] but the attribute is conditional —
// it only applies when BOTH feature "x" AND feature "y" are enabled.

#[cfg_attr(feature = "x", cfg_attr(feature = "y", allow(unused)))]
fn doubly_conditional() {
    let x = 42;
}

// Triple nesting — even more pathological
#[cfg_attr(
    feature = "a",
    cfg_attr(
        feature = "b",
        cfg_attr(feature = "c", allow(dead_code))
    )
)]
fn triply_conditional() {
    let y = 99;
}

// cfg_attr with multiple items including allow
#[cfg_attr(feature = "x", derive(Debug), allow(clippy::unwrap_used))]
struct ConditionalStruct {
    value: i32,
}

fn main() {
    doubly_conditional();
    triply_conditional();
    let _s = ConditionalStruct { value: 1 };
}
