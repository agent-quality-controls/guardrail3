use std::collections::BTreeMap;

use crate::domain::modules::clippy::build_clippy_toml;
use crate::domain::report::Severity;

use super::super::super::test_support::{
    canonical_clippy_toml, copy_fixture, run_family, write_file,
};

#[test]
fn inventories_exact_covering_config_for_each_rust_root_in_multi_root_fixture() {
    let tmp = copy_fixture();
    write_file(tmp.path(), "clippy.toml", &canonical_clippy_toml());
    write_file(
        tmp.path(),
        "apps/devctl/clippy.toml",
        &build_clippy_toml("service", false, true, "", ""),
    );
    write_file(
        tmp.path(),
        "packages/shared-types/clippy.toml",
        &build_clippy_toml("library", false, true, "", ""),
    );

    let results = run_family(tmp.path());
    let coverage = results
        .iter()
        .filter(|result| result.id == "RS-CLIPPY-01")
        .collect::<Vec<_>>();

    let actual = coverage
        .iter()
        .map(|result| {
            (
                result.message.clone(),
                (
                    result.severity,
                    result.inventory,
                    result.file.clone(),
                    result.title.clone(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let expected = BTreeMap::from([
        (
            "standalone package root `packages/shared-types` is covered by `packages/shared-types/clippy.toml`."
                .to_owned(),
            (
                Severity::Info,
                true,
                Some("packages/shared-types/clippy.toml".to_owned()),
                "Rust unit covered by clippy.toml".to_owned(),
            ),
        ),
        (
            "workspace root `apps/backend` is covered by `clippy.toml`.".to_owned(),
            (
                Severity::Info,
                true,
                Some("clippy.toml".to_owned()),
                "Rust unit covered by clippy.toml".to_owned(),
            ),
        ),
        (
            "workspace root `apps/devctl` is covered by `apps/devctl/clippy.toml`.".to_owned(),
            (
                Severity::Info,
                true,
                Some("apps/devctl/clippy.toml".to_owned()),
                "Rust unit covered by clippy.toml".to_owned(),
            ),
        ),
        (
            "workspace root `apps/worker` is covered by `clippy.toml`.".to_owned(),
            (
                Severity::Info,
                true,
                Some("clippy.toml".to_owned()),
                "Rust unit covered by clippy.toml".to_owned(),
            ),
        ),
        (
            "workspace root is covered by `clippy.toml`.".to_owned(),
            (
                Severity::Info,
                true,
                Some("clippy.toml".to_owned()),
                "Rust unit covered by clippy.toml".to_owned(),
            ),
        ),
    ]);

    assert_eq!(actual, expected);
}
