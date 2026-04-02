use test_support::{entry, tree};

#[test]
fn undeclared_workspace_child_is_reported() {
    let tree = tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["crates"], &["Cargo.toml"])),
            ("apps/backend/crates", entry(&["api"], &[])),
            ("apps/backend/crates/api", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = []\nresolver = \"2\"\n",
            ),
            (
                "apps/backend/crates/api/Cargo.toml",
                "[package]\nname = \"api\"\n",
            ),
        ],
    );

    let results = crate::check_test_tree(&tree);
    let result = results
        .iter()
        .find(|result| {
            result.id() == "RS-TOPOLOGY-12"
                && result.file() == Some("apps/backend/crates/api/Cargo.toml")
        })
        .expect("expected missing-child RS-TOPOLOGY-12 result");

    assert!(result.title().contains("Workspace child"));
}

#[test]
fn extra_workspace_member_is_reported() {
    let tree = tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["crates"], &["Cargo.toml"])),
            ("apps/backend/crates", entry(&["api"], &[])),
            ("apps/backend/crates/api", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/api\", \"crates/ghost\"]\nresolver = \"2\"\n",
            ),
            (
                "apps/backend/crates/api/Cargo.toml",
                "[package]\nname = \"api\"\n",
            ),
        ],
    );

    let result = crate::check_test_tree(&tree)
        .into_iter()
        .find(|result| {
            result.id() == "RS-TOPOLOGY-12" && result.file() == Some("apps/backend/Cargo.toml")
        })
        .expect("expected extra-member RS-TOPOLOGY-12 result");

    assert!(result.title().contains("extra member"));
    assert!(result.message().contains("crates/ghost"));
}

#[test]
fn undeclared_package_workspace_child_is_reported() {
    let tree = tree(
        &[
            ("", entry(&["packages"], &[])),
            ("packages", entry(&["reason-policy"], &[])),
            ("packages/reason-policy", entry(&["crates"], &["Cargo.toml"])),
            ("packages/reason-policy/crates", entry(&["api"], &[])),
            ("packages/reason-policy/crates/api", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "packages/reason-policy/Cargo.toml",
                "[workspace]\nmembers = []\nresolver = \"2\"\n",
            ),
            (
                "packages/reason-policy/crates/api/Cargo.toml",
                "[package]\nname = \"reason-policy-api\"\n",
            ),
        ],
    );

    let result = crate::check_test_tree(&tree)
        .into_iter()
        .find(|result| {
            result.id() == "RS-TOPOLOGY-12"
                && result.file() == Some("packages/reason-policy/crates/api/Cargo.toml")
        })
        .expect("expected package missing-child RS-TOPOLOGY-12 result");

    assert!(result.title().contains("Workspace child"));
}

#[test]
fn extra_package_workspace_member_is_reported() {
    let tree = tree(
        &[
            ("", entry(&["packages"], &[])),
            ("packages", entry(&["reason-policy"], &[])),
            ("packages/reason-policy", entry(&["crates"], &["Cargo.toml"])),
            ("packages/reason-policy/crates", entry(&["api"], &[])),
            ("packages/reason-policy/crates/api", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "packages/reason-policy/Cargo.toml",
                "[workspace]\nmembers = [\"crates/api\", \"crates/ghost\"]\nresolver = \"2\"\n",
            ),
            (
                "packages/reason-policy/crates/api/Cargo.toml",
                "[package]\nname = \"reason-policy-api\"\n",
            ),
        ],
    );

    let result = crate::check_test_tree(&tree)
        .into_iter()
        .find(|result| {
            result.id() == "RS-TOPOLOGY-12"
                && result.file() == Some("packages/reason-policy/Cargo.toml")
        })
        .expect("expected package extra-member RS-TOPOLOGY-12 result");

    assert!(result.title().contains("extra member"));
    assert!(result.message().contains("crates/ghost"));
}

#[test]
fn exact_match_stays_clean() {
    let tree = tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["crates"], &["Cargo.toml"])),
            ("apps/backend/crates", entry(&["api"], &[])),
            ("apps/backend/crates/api", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/api\"]\nresolver = \"2\"\n",
            ),
            (
                "apps/backend/crates/api/Cargo.toml",
                "[package]\nname = \"api\"\n",
            ),
        ],
    );

    assert!(
        crate::check_test_tree(&tree)
            .into_iter()
            .all(|result| result.id() != "RS-TOPOLOGY-12")
    );
}

#[test]
fn glob_membership_covering_real_children_stays_clean() {
    let tree = tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["crates"], &["Cargo.toml"])),
            ("apps/backend/crates", entry(&["api", "core"], &[])),
            ("apps/backend/crates/api", entry(&[], &["Cargo.toml"])),
            ("apps/backend/crates/core", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]\nresolver = \"2\"\n",
            ),
            (
                "apps/backend/crates/api/Cargo.toml",
                "[package]\nname = \"api\"\n",
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n",
            ),
        ],
    );

    assert!(
        crate::check_test_tree(&tree)
            .into_iter()
            .all(|result| result.id() != "RS-TOPOLOGY-12")
    );
}

#[test]
fn missing_and_extra_are_both_reported() {
    let tree = tree(
        &[
            ("", entry(&["apps"], &[])),
            ("apps", entry(&["backend"], &[])),
            ("apps/backend", entry(&["crates"], &["Cargo.toml"])),
            ("apps/backend/crates", entry(&["api", "core"], &[])),
            ("apps/backend/crates/api", entry(&[], &["Cargo.toml"])),
            ("apps/backend/crates/core", entry(&[], &["Cargo.toml"])),
        ],
        &[
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/api\", \"crates/ghost\"]\nresolver = \"2\"\n",
            ),
            (
                "apps/backend/crates/api/Cargo.toml",
                "[package]\nname = \"api\"\n",
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n",
            ),
        ],
    );

    let results = crate::check_test_tree(&tree);
    assert_eq!(
        results.iter().filter(|result| result.id() == "RS-TOPOLOGY-12").count(),
        2
    );
    assert!(results.iter().any(|result| {
        result.id() == "RS-TOPOLOGY-12"
            && result.file() == Some("apps/backend/crates/core/Cargo.toml")
    }));
    assert!(results.iter().any(|result| {
        result.id() == "RS-TOPOLOGY-12" && result.file() == Some("apps/backend/Cargo.toml")
    }));
}
