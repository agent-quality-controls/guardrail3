#![allow(
    clippy::expect_used,
    clippy::missing_const_for_fn,
    clippy::missing_panics_doc,
    reason = "assertion helpers are reusable panic-based proof sites for test harnesses"
)]

use deny_toml_parser::DenyToml;

pub fn assert_empty_toml(cfg: &DenyToml) {
    assert_eq!(cfg.graph, None, "graph should be None");
    assert_eq!(cfg.advisories, None, "advisories should be None");
    assert_eq!(cfg.bans, None, "bans should be None");
    assert_eq!(cfg.licenses, None, "licenses should be None");
    assert_eq!(cfg.sources, None, "sources should be None");
    assert_eq!(cfg.output, None, "output should be None");
    assert!(cfg.extra.is_empty(), "extra should be empty");
}

pub fn assert_parse_error(err: impl std::fmt::Display) {
    let msg = err.to_string();
    assert!(
        msg.contains("invalid deny.toml"),
        "expected error message prefix, got: {msg}",
    );
}
