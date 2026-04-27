use cargo_config_toml_parser_runtime_assertions::parser as assertions;
use helpers::{parse_fixture, parse_from_tempfile};

use super::helpers;

#[test]
fn empty_string_yields_empty_config() {
    let cfg = parse_fixture("");

    assertions::assert_core_fields_empty(&cfg);
    assertions::assert_extra_empty(&cfg);
}

#[test]
fn env_section_parses_simple_and_detailed_values() {
    let cfg = parse_fixture(
        r#"
[env]
CLIPPY_CONF_DIR = "."
OPENSSL_DIR = { value = "vendor/openssl", force = true, relative = true }
"#,
    );

    assertions::assert_simple_env_value(cfg.env.get("CLIPPY_CONF_DIR"), ".", "CLIPPY_CONF_DIR");
    assertions::assert_detailed_env_value(
        cfg.env.get("OPENSSL_DIR"),
        "vendor/openssl",
        Some(true),
        Some(true),
        "OPENSSL_DIR",
    );
    assertions::assert_extra_empty(&cfg);
}

#[test]
fn include_entries_require_toml_paths() {
    let err = super::super::parse(
        r#"
include = ["shared.txt"]
"#,
    )
    .expect_err("non-.toml include path should be rejected");

    assertions::assert_include_path_error(err);
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "one realistic fixture is the clearest way to prove section coverage"
)]
fn realistic_config_parses_known_sections() {
    let cfg = parse_fixture(
        r#"
paths = ["vendor/crates"]
include = ["shared.toml", { path = "optional.toml", optional = true }]
future-key = "keep-me"
future-int = 42

[alias]
xtask = ["run", "-p", "xtask", "--"]

[build]
jobs = 4
target = ["x86_64-unknown-linux-gnu", "wasm32-unknown-unknown"]
rustflags = ["-Dwarnings"]
target-dir = "target/custom"

[doc]
browser = ["open", "-a", "Firefox"]

[future-incompat-report]
frequency = "never"

[cache]
auto-clean-frequency = "1 day"

[cargo-new]
vcs = "git"

[http]
timeout = 20
ssl-version = { min = "tlsv1.2", max = "tlsv1.3" }
user-agent = "guardrail3-tests"

[install]
root = "tools/cargo-bin"

[net]
retry = 5
git-fetch-with-cli = true
[net.ssh]
known-hosts = ["example.com ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAItestkey"]

[registries.custom]
index = "https://example.com/index"
credential-provider = ["cargo-credential-test", "--flag"]

[registry]
default = "custom"
global-credential-providers = ["cargo:token"]

[source.vendored-sources]
directory = "vendor"

[target.x86_64-unknown-linux-gnu]
linker = "clang"
runner = ["qemu-x86_64", "-L", "/usr/x86_64-linux-gnu"]
rustflags = ["-Clink-arg=-fuse-ld=lld"]
[target.x86_64-unknown-linux-gnu.foo]
rustc-link-lib = ["foo"]

[term]
color = "always"
[term.progress]
when = "never"
width = 80
"#,
    );

    assertions::assert_string_list(&cfg.paths, &["vendor/crates"], "paths");
    assertions::assert_command_list(
        cfg.alias.get("xtask"),
        &["run", "-p", "xtask", "--"],
        "alias.xtask",
    );
    assertions::assert_target_selector_list(
        cfg.build.as_ref().and_then(|build| build.target.as_ref()),
        &["x86_64-unknown-linux-gnu", "wasm32-unknown-unknown"],
        "build.target",
    );
    assertions::assert_command_list(
        cfg.build
            .as_ref()
            .and_then(|build| build.rustflags.as_ref()),
        &["-Dwarnings"],
        "build.rustflags",
    );
    assertions::assert_command_list(
        cfg.doc.as_ref().and_then(|doc| doc.browser.as_ref()),
        &["open", "-a", "Firefox"],
        "doc.browser",
    );
    assertions::assert_tls_range(
        cfg.http.as_ref().and_then(|http| http.ssl_version.as_ref()),
        Some("tlsv1.2"),
        Some("tlsv1.3"),
    );
    assertions::assert_command_list(
        cfg.registries
            .get("custom")
            .and_then(|registry| registry.credential_provider.as_ref()),
        &["cargo-credential-test", "--flag"],
        "registries.custom.credential-provider",
    );
    assertions::assert_string_list(
        &cfg.registry
            .as_ref()
            .expect("registry section should exist")
            .global_credential_providers,
        &["cargo:token"],
        "registry.global-credential-providers",
    );
    assertions::assert_command_list(
        cfg.target
            .get("x86_64-unknown-linux-gnu")
            .and_then(|target| target.runner.as_ref()),
        &["qemu-x86_64", "-L", "/usr/x86_64-linux-gnu"],
        "target.x86_64-unknown-linux-gnu.runner",
    );
    assertions::assert_nested_extra_table(
        &cfg.target
            .get("x86_64-unknown-linux-gnu")
            .expect("target section should exist")
            .extra,
        "foo",
    );
    assertions::assert_top_level_string_extra(&cfg, "future-key", "keep-me");
    assertions::assert_top_level_integer_extra(&cfg, "future-int", 42);
}

#[test]
fn unknown_keys_land_in_extra() {
    let cfg = parse_fixture(
        r#"
future-key = "value"

[env]
CLIPPY_CONF_DIR = "."
FUTURE_ENV = { value = "x", custom = { nested = true } }
"#,
    );

    assertions::assert_simple_env_value(cfg.env.get("CLIPPY_CONF_DIR"), ".", "CLIPPY_CONF_DIR");
    assertions::assert_top_level_string_extra(&cfg, "future-key", "value");
    assertions::assert_detailed_env_extra_table(cfg.env.get("FUTURE_ENV"), "FUTURE_ENV", "custom");
}

#[test]
fn config_roundtrips() {
    let cfg = parse_fixture(
        r#"
[env]
CLIPPY_CONF_DIR = "."

[registry]
default = "custom"
global-credential-providers = ["cargo:token"]

[target.'cfg(target_os = "linux")']
runner = "env"
"#,
    );

    let serialized = toml::to_string(&cfg).expect("serialization should succeed");
    let cfg2 = parse_fixture(&serialized);
    assertions::assert_tomls_equal(&cfg, &cfg2);
}

#[test]
fn from_path_reads_and_parses_file() {
    let cfg = parse_from_tempfile(
        r#"
[env]
CLIPPY_CONF_DIR = "."
"#,
    );

    assertions::assert_simple_env_value(cfg.env.get("CLIPPY_CONF_DIR"), ".", "CLIPPY_CONF_DIR");
}

#[test]
fn parse_error_on_invalid_toml() {
    let err = super::super::parse("this is not [[[valid toml");
    assertions::assert_parse_error(err.expect_err("invalid TOML should produce an error"));
}
