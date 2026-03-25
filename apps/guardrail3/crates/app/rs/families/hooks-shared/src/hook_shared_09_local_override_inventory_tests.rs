use super::check;

#[test]
fn inventories_no_overrides() {
    let mut results = Vec::new();
    check(&[], &mut results);
    assert!(results[0].inventory);
    assert_eq!(results[0].title, "no local hook overrides");
}

#[test]
fn inventories_override_names() {
    let overrides = vec!["99-local.sh".to_owned(), "20-extra.sh".to_owned()];
    let mut results = Vec::new();
    check(&overrides, &mut results);
    assert!(results[0].inventory);
    assert!(results[0].message.contains("99-local.sh"));
}
