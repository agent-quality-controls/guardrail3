use super::helpers::check_source;

#[test]
fn skips_test_files() {
    let content =
        "use a::{b0,b1,b2,b3,b4,b5,b6,b7,b8,b9,b10,b11,b12,b13,b14,b15,b16,b17,b18,b19,b20};\n";

    let results = check_source("tests/smoke.rs", content, true);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn skips_at_cap() {
    let content =
        "use a::{b0,b1,b2,b3,b4,b5,b6,b7,b8,b9,b10,b11,b12,b13,b14,b15,b16,b17,b18,b19};\n";

    let results = check_source("src/lib.rs", content, false);

    assert!(results.is_empty(), "{results:#?}");
}
