use std::collections::BTreeSet;
use std::path::Path;
const FIXTURE: super::HexarchFixture = super::HexarchFixture;

fn rust_apps() -> &'static [&'static str] {
    FIXTURE.apps()
}

fn inner_hex() -> &'static str {
    FIXTURE.inner_hex_root()
}

use super::{create_dir, write_file};

pub const OUTER_CONTAINERS: &[&str] = &[
    "crates/app",
    "crates/domain",
    "crates/adapters/inbound",
    "crates/adapters/outbound",
    "crates/ports/inbound",
    "crates/ports/outbound",
];

pub const INNER_CONTAINERS: &[&str] = &["app", "domain", "adapters/inbound"];

pub const NESTED_CONTAINERS: &[&str] = &[
    "app",
    "domain",
    "adapters/inbound",
    "adapters/outbound",
    "ports/inbound",
    "ports/outbound",
];

pub fn owned_leaf_dirs(root: &Path, name: &str) -> BTreeSet<String> {
    let mut expected = BTreeSet::new();
    for app in rust_apps() {
        for container in OUTER_CONTAINERS {
            let rel = format!("apps/{app}/{container}/{name}");
            create_dir(root, &rel);
            let _ = expected.insert(rel);
        }
    }
    for container in INNER_CONTAINERS {
        let rel = format!("{}/{container}/{name}", inner_hex());
        create_dir(root, &rel);
        let _ = expected.insert(rel);
    }
    expected
}

pub fn nested_hex_everywhere(root: &Path, name: &str) -> BTreeSet<String> {
    let leaves = owned_leaf_dirs(root, name);
    for rel in &leaves {
        write_file(root, &format!("{rel}/.gitkeep"), "");
        for container in NESTED_CONTAINERS {
            write_file(root, &format!("{rel}/crates/{container}/.gitkeep"), "");
        }
    }
    leaves
}
