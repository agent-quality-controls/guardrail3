use guardrail3_app_rs_family_deny_assertions::licenses::rs_deny_17_license_exceptions_inventory as assertions;

use super::super::{build_fixture_deny_toml, set_license_exceptions};

fn exception_entry(fields: impl IntoIterator<Item = (&'static str, toml::Value)>) -> toml::Value {
    toml::Value::Table(toml::map::Map::from_iter(
        fields
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value)),
    ))
}

#[test]
fn errors_for_malformed_missing_reason_and_non_string_reason_exception_entries() {
    let deny = set_license_exceptions(
        &build_fixture_deny_toml("service"),
        vec![
            exception_entry([
                (
                    "allow",
                    toml::Value::Array(vec![toml::Value::String("MIT".to_owned())]),
                ),
                (
                    "reason",
                    toml::Value::String("good enough reason text".to_owned()),
                ),
            ]),
            exception_entry([
                ("crate", toml::Value::String("demo".to_owned())),
                (
                    "allow",
                    toml::Value::Array(vec![toml::Value::String("MIT".to_owned())]),
                ),
            ]),
            exception_entry([
                ("crate", toml::Value::String("   ".to_owned())),
                (
                    "allow",
                    toml::Value::Array(vec![toml::Value::String("MIT".to_owned())]),
                ),
                (
                    "reason",
                    toml::Value::String("good enough reason text".to_owned()),
                ),
            ]),
            exception_entry([
                (
                    "crate",
                    toml::Value::String("demo-blank-license".to_owned()),
                ),
                (
                    "allow",
                    toml::Value::Array(vec![
                        toml::Value::String("".to_owned()),
                        toml::Value::String("MIT".to_owned()),
                    ]),
                ),
                (
                    "reason",
                    toml::Value::String("good enough reason text".to_owned()),
                ),
            ]),
            exception_entry([
                ("crate", toml::Value::String("demo-legacy".to_owned())),
                (
                    "allow",
                    toml::Value::Array(vec![toml::Value::String("MIT".to_owned())]),
                ),
                ("reason", toml::Value::Integer(7)),
            ]),
        ],
    );
    let results = super::super::run_check(&deny);

    assertions::assert_findings(
        &results,
        &[
            assertions::error(
                "malformed license exception entry",
                "`deny.toml` has `[[licenses.exceptions]]` entry without a valid crate identifier.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "license exception missing reason",
                "`deny.toml` has license exception `demo` without a `reason`.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "malformed license exception entry",
                "`deny.toml` has `[[licenses.exceptions]]` entry without a valid crate identifier.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "malformed license exception entry",
                "`deny.toml` has `[[licenses.exceptions]]` entry `demo-blank-license` with blank allowed license name.",
                "deny.toml",
                false,
            ),
            assertions::error(
                "license exception reason must be a string",
                "`deny.toml` has `[[licenses.exceptions]]` entry `demo-legacy` with a non-string `reason`.",
                "deny.toml",
                false,
            ),
            assertions::warn_no_file(
                "license exception count",
                "`deny.toml` has 2 license exceptions (0 documented, 2 missing or invalid reasons, 0 weak reasons).",
                false,
            ),
        ],
    );
}
