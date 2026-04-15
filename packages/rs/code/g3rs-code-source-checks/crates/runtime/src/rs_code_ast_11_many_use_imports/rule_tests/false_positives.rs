
#[test]
fn skips_below_warn_threshold() {
    let content = "use a::{b0,b1,b2,b3,b4,b5,b6,b7,b8,b9,b10,b11,b12,b13,b14};\n";

    let results = super::super::check_source("src/lib.rs", content, false);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn skips_error_band() {
    let content =
        "use a::{b0,b1,b2,b3,b4,b5,b6,b7,b8,b9,b10,b11,b12,b13,b14,b15,b16,b17,b18,b19,b20};\n";

    let results = super::super::check_source("src/lib.rs", content, false);

    assert!(results.is_empty(), "{results:#?}");
}
