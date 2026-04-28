use std::collections::BTreeSet;

use super::rust_hook_requirements;

#[test]
fn rust_hook_requirements_include_every_family_contract() {
    let owners = rust_hook_requirements()
        .into_iter()
        .map(|requirement| requirement.owner_family)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        owners,
        BTreeSet::from([
            "apparch".to_owned(),
            "arch".to_owned(),
            "cargo".to_owned(),
            "clippy".to_owned(),
            "code".to_owned(),
            "deny".to_owned(),
            "deps".to_owned(),
            "fmt".to_owned(),
            "garde".to_owned(),
            "release".to_owned(),
            "test".to_owned(),
            "toolchain".to_owned(),
            "topology".to_owned(),
        ])
    );
}
