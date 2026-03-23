use super::super::super::test_support::copy_fixture;
use super::super::super::test_support::run_family;

#[test]
fn inventories_real_publish_dry_run_success_from_richer_fixture() {
    let tmp = copy_fixture();
    let results = run_family(tmp.path(), true);
    let shared_types = results
        .iter()
        .find(|result| {
            result.id == "RS-PUB-09"
                && result.inventory
                && result.file.as_deref() == Some("packages/shared-types/Cargo.toml")
        })
        .expect("expected RS-PUB-09 inventory for shared-types");

    assert_eq!(shared_types.title, "shared-types: publish dry-run passed");
    assert_eq!(shared_types.message, "`cargo publish --dry-run` succeeded.");
}
