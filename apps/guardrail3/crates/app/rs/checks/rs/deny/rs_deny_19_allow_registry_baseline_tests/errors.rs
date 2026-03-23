use std::collections::BTreeSet;

use crate::domain::report::Severity;

use super::super::super::inputs::ConfigDenyInput;
use super::super::super::test_support::{canonical_deny_toml_service, config_facts};
use super::super::check;

#[test]
fn errors_when_sources_section_is_missing_or_crates_io_is_not_allowed() {
    let missing_sources =
        config_facts(&canonical_deny_toml_service().replace("[sources]\n", "[removed]\n"));
    let missing_crates_io = config_facts(&canonical_deny_toml_service().replace(
        "allow-registry = [\"https://github.com/rust-lang/crates.io-index\"]",
        "allow-registry = []",
    ));
    let unexpected_registry = config_facts(&canonical_deny_toml_service().replace(
        "allow-registry = [\"https://github.com/rust-lang/crates.io-index\"]",
        "allow-registry = [\"https://github.com/rust-lang/crates.io-index\", \"https://example.com/index\"]",
    ));
    let mut results = Vec::new();

    check(
        &ConfigDenyInput {
            config: &missing_sources,
        },
        &mut results,
    );
    check(
        &ConfigDenyInput {
            config: &missing_crates_io,
        },
        &mut results,
    );
    check(
        &ConfigDenyInput {
            config: &unexpected_registry,
        },
        &mut results,
    );

    let actual = results
        .iter()
        .map(|result| (result.title.clone(), result.message.clone()))
        .collect::<BTreeSet<_>>();
    let expected = BTreeSet::from([
        (
            "[sources] allow-registry missing".to_owned(),
            "`deny.toml` has no valid crates.io registry allow-list.".to_owned(),
        ),
        (
            "crates.io registry not allowed".to_owned(),
            "`deny.toml` must include crates.io in `[sources].allow-registry`.".to_owned(),
        ),
        (
            "unexpected registry allowed".to_owned(),
            "`deny.toml` allows unexpected registries: https://example.com/index.".to_owned(),
        ),
    ]);

    assert_eq!(actual, expected);
    assert!(results.iter().all(|result| {
        result.id == "RS-DENY-19"
            && result.severity == Severity::Error
            && result.file.as_deref() == Some("deny.toml")
    }));
}
