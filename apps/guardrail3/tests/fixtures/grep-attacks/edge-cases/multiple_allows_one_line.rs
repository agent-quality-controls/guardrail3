// Multiple lints suppressed in a single #[allow()] attribute.
// grep might count this as one allow. The AST should see three
// separate lint suppressions: unused, dead_code, clippy::unwrap_used.

#[allow(unused, dead_code, clippy::unwrap_used)] // reason: testing multiple lints in one attribute
fn multi_suppressed() {
    let x = 42;
    let y = Some(x);
    let _z = y.unwrap();
}

// Same but with allow and deny mixed via separate attributes on one item
#[allow(unused)] // reason: test
#[deny(clippy::panic)]
#[allow(dead_code)] // reason: test
fn mixed_attributes() {
    let _a = 1;
}

// Inline-style: multiple allows separated by commas, no spaces
#[allow(unused,dead_code,clippy::unwrap_used,clippy::todo)] // reason: compact form test
fn compact_form() {
    let _x = 1;
}

fn main() {
    multi_suppressed();
    mixed_attributes();
    compact_form();
}
