pub(crate) struct Module {
    content: &'static str,
}

impl Module {
    pub(crate) const fn content(&self) -> &'static str {
        self.content
    }
}

pub(crate) const DENY_GRAPH: Module = Module {
    content: r"[graph]
all-features = true
no-default-features = false",
};

pub(crate) const DENY_BANS_BASE: Module = Module {
    content: r#"[bans]
multiple-versions = "deny"
wildcards = "allow"
allow-wildcard-paths = true
highlight = "all""#,
};

pub(crate) const DENY_BANS_JSON: Module = Module {
    content: r#"    { name = "simd-json", wrappers = [] },
    { name = "json5", wrappers = [] },
    { name = "sonic-rs", wrappers = [] },"#,
};

pub(crate) const DENY_BANS_TLS: Module = Module {
    content: r#"    { name = "openssl", wrappers = [] },
    { name = "openssl-sys", wrappers = [] },"#,
};

pub(crate) const DENY_BANS_HTTP: Module = Module {
    content: r#"    { name = "ureq", wrappers = [] },
    { name = "surf", wrappers = [] },
    { name = "isahc", wrappers = [] },"#,
};

pub(crate) const DENY_BANS_LOGGING: Module = Module {
    content: r#"    { name = "log4rs", wrappers = [] },
    { name = "env_logger", wrappers = [] },
    { name = "simple_logger", wrappers = [] },
    { name = "fern", wrappers = [] },"#,
};

pub(crate) const DENY_BANS_ASYNC: Module = Module {
    content: r#"    { name = "async-std", wrappers = [] },
    { name = "smol", wrappers = [] },"#,
};

pub(crate) const DENY_BANS_GLOBAL_STATE: Module = Module {
    content: r#"    { name = "lazy_static", wrappers = [] },"#,
};

pub(crate) const DENY_BANS_ERROR: Module = Module {
    content: r#"    { name = "anyhow", wrappers = [] },"#,
};

pub(crate) const DENY_BANS_WEB: Module = Module {
    content: r#"    { name = "actix-web", wrappers = [] },
    { name = "rocket", wrappers = [] },
    { name = "warp", wrappers = [] },
    { name = "poem", wrappers = [] },"#,
};

pub(crate) const DENY_BANS_DATETIME: Module = Module {
    content: r#"    { name = "chrono", wrappers = [] },"#,
};

pub(crate) const DENY_BANS_ORM: Module = Module {
    content: r#"    { name = "diesel", wrappers = [] },
    { name = "sea-orm", wrappers = [] },"#,
};

pub(crate) const DENY_BANS_SERIALIZATION: Module = Module {
    content: r#"    { name = "bincode", wrappers = [] },
    { name = "rmp-serde", wrappers = [] },
    { name = "prost", wrappers = [] },
    { name = "flatbuffers", wrappers = [] },"#,
};

pub(crate) const DENY_BANS_REGEX: Module = Module {
    content: r#"    { name = "regex", wrappers = ["tree-sitter", "globset", "ignore"] },
    { name = "fancy-regex", wrappers = [] },
    { name = "onig", wrappers = [] },
    { name = "pcre2", wrappers = [] },
    { name = "grep-cli", wrappers = [] },
    { name = "grep-regex", wrappers = [] },
    { name = "grep-matcher", wrappers = [] },"#,
};

pub(crate) const DENY_BANS_LIBRARY_IO: Module = Module {
    content: r#"    { name = "axum", wrappers = [] },
    { name = "tokio", wrappers = [] },
    { name = "async-std", wrappers = [] },
    { name = "reqwest", wrappers = [] },
    { name = "sqlx", wrappers = [] },
    { name = "hyper", wrappers = [] },
    { name = "warp", wrappers = [] },
    { name = "rocket", wrappers = [] },
    { name = "actix-web", wrappers = [] },
    { name = "poem", wrappers = [] },
    { name = "ureq", wrappers = [] },
    { name = "surf", wrappers = [] },
    { name = "isahc", wrappers = [] },"#,
};

pub(crate) const DENY_FEATURE_BANS_TOKIO: Module = Module {
    content: r#"[[bans.features]]
name = "tokio"
deny = ["full"]
allow = ["rt-multi-thread", "macros", "net", "sync", "signal", "bytes", "default", "io-util", "time"]"#,
};

pub(crate) const DENY_LICENSES: Module = Module {
    content: r#"[licenses]
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
    "Unicode-3.0",
    "Zlib",
    "CC0-1.0",
    "OpenSSL",
    "BSL-1.0",
    "MPL-2.0",
    "CDLA-Permissive-2.0",
]
confidence-threshold = 0.8

[licenses.private]
ignore = true"#,
};

pub(crate) const DENY_ADVISORIES: Module = Module {
    content: r#"[advisories]
unmaintained = "workspace"
yanked = "warn"
ignore = []"#,
};

pub(crate) const DENY_SOURCES: Module = Module {
    content: r#"[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["sparse+https://index.crates.io/"]
allow-git = []"#,
};

pub(crate) fn service_profile_ban_entries() -> Vec<&'static Module> {
    vec![
        &DENY_BANS_JSON,
        &DENY_BANS_TLS,
        &DENY_BANS_HTTP,
        &DENY_BANS_LOGGING,
        &DENY_BANS_ASYNC,
        &DENY_BANS_GLOBAL_STATE,
        &DENY_BANS_ERROR,
        &DENY_BANS_WEB,
        &DENY_BANS_DATETIME,
        &DENY_BANS_ORM,
        &DENY_BANS_SERIALIZATION,
        &DENY_BANS_REGEX,
    ]
}

pub(crate) fn library_profile_ban_entries() -> Vec<&'static Module> {
    let mut entries = service_profile_ban_entries();
    entries.push(&DENY_BANS_LIBRARY_IO);
    entries
}
