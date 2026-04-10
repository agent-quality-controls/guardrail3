use super::helpers::check_source;

#[test]
fn skips_small_types() {
    let results = check_source(
        "src/lib.rs",
        "struct Small { a: u8 }\nenum E { A, B }\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn skips_struct_at_threshold() {
    let fields = (0..15)
        .map(|i| format!("f{i}: u8"))
        .collect::<Vec<_>>()
        .join(", ");
    let content = format!("struct Exact {{ {fields} }}\n");

    let results = check_source("src/lib.rs", &content, false);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn skips_enum_at_threshold() {
    let variants = (0..20)
        .map(|i| format!("V{i}"))
        .collect::<Vec<_>>()
        .join(", ");
    let content = format!("enum Exact {{ {variants} }}\n");

    let results = check_source("src/lib.rs", &content, false);

    assert!(results.is_empty(), "{results:#?}");
}
