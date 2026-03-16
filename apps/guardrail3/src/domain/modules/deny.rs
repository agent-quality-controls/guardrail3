use std::fmt::Write;

use super::Module;

// ---------------------------------------------------------------------------
// deny.toml section modules
// ---------------------------------------------------------------------------

pub const DENY_GRAPH: Module = Module {
    name: "deny/graph",
    description: "Graph traversal settings",
    content: r"[graph]
all-features = true
no-default-features = false",
};

pub const DENY_BANS_BASE: Module = Module {
    name: "deny/bans-base",
    description: "Base ban settings (multiple-versions, wildcards)",
    content: r#"[bans]
multiple-versions = "deny"
# Workspace path dependencies (e.g., domain = { path = "../domain" }) have no version spec.
# cargo-deny 0.19+ flags these as wildcards. Allow them.
wildcards = "allow"
allow-wildcard-paths = true
highlight = "all""#,
};

pub const DENY_BANS_JSON: Module = Module {
    name: "deny/bans/json",
    description: "Ban competing JSON libraries (standardize on serde_json)",
    content: r#"    # --- JSON: standardize on serde_json ---
    { name = "simd-json", wrappers = [] },
    { name = "json5", wrappers = [] },
    { name = "sonic-rs", wrappers = [] },"#,
};

pub const DENY_BANS_TLS: Module = Module {
    name: "deny/bans/tls",
    description: "Ban OpenSSL (standardize on rustls)",
    content: r#"    # --- TLS: standardize on rustls (no OpenSSL) ---
    { name = "openssl", wrappers = [] },
    { name = "openssl-sys", wrappers = [] },"#,
};

pub const DENY_BANS_HTTP: Module = Module {
    name: "deny/bans/http",
    description: "Ban competing HTTP clients (standardize on reqwest)",
    content: r#"    # --- HTTP: standardize on reqwest ---
    { name = "ureq", wrappers = [] },
    { name = "surf", wrappers = [] },
    { name = "isahc", wrappers = [] },"#,
};

pub const DENY_BANS_LOGGING: Module = Module {
    name: "deny/bans/logging",
    description: "Ban competing logging frameworks (standardize on tracing)",
    content: r#"    # --- Logging: standardize on tracing ---
    { name = "log4rs", wrappers = [] },
    { name = "env_logger", wrappers = [] },
    { name = "simple_logger", wrappers = [] },
    { name = "fern", wrappers = [] },"#,
};

pub const DENY_BANS_ASYNC: Module = Module {
    name: "deny/bans/async",
    description: "Ban competing async runtimes (standardize on tokio)",
    content: r#"    # --- Async runtime: standardize on tokio ---
    { name = "async-std", wrappers = [] },
    { name = "smol", wrappers = [] },"#,
};

pub const DENY_BANS_ERROR: Module = Module {
    name: "deny/bans/error",
    description: "Ban anyhow (standardize on thiserror)",
    content: r#"    # --- Error handling: standardize on thiserror ---
    { name = "anyhow", wrappers = [] },"#,
};

pub const DENY_BANS_WEB: Module = Module {
    name: "deny/bans/web",
    description: "Ban competing web frameworks (standardize on axum)",
    content: r#"    # --- Web framework: standardize on axum ---
    { name = "actix-web", wrappers = [] },
    { name = "rocket", wrappers = [] },
    { name = "warp", wrappers = [] },
    { name = "poem", wrappers = [] },"#,
};

pub const DENY_BANS_DATETIME: Module = Module {
    name: "deny/bans/datetime",
    description: "Ban chrono (standardize on time crate)",
    content: r#"    # --- Datetime: standardize on time crate ---
    { name = "chrono", wrappers = [] },"#,
};

pub const DENY_BANS_ORM: Module = Module {
    name: "deny/bans/orm",
    description: "Ban competing ORMs (standardize on sqlx)",
    content: r#"    # --- ORM: standardize on sqlx ---
    { name = "diesel", wrappers = [] },
    { name = "sea-orm", wrappers = [] },"#,
};

pub const DENY_BANS_SERIALIZATION: Module = Module {
    name: "deny/bans/serialization",
    description: "Ban competing binary serialization formats",
    content: r#"    # --- Serialization: no competing binary formats ---
    { name = "bincode", wrappers = [] },
    { name = "rmp-serde", wrappers = [] },
    { name = "prost", wrappers = [] },
    { name = "flatbuffers", wrappers = [] },"#,
};

pub const DENY_FEATURE_BANS_TOKIO: Module = Module {
    name: "deny/feature-bans/tokio",
    description: "Ban tokio 'full' feature (require explicit feature selection)",
    content: r#"[[bans.features]]
name = "tokio"
deny = ["full"]
# Force explicit feature selection instead of enabling everything
allow = ["rt-multi-thread", "macros", "net", "sync", "signal", "bytes", "default", "io-util", "time"]"#,
};

pub const DENY_LICENSES: Module = Module {
    name: "deny/licenses",
    description: "License allowlist",
    content: r#"[licenses]
# All licenses are denied unless explicitly listed here
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

# Workspace crates with `publish = false` don't need a license field
[licenses.private]
ignore = true"#,
};

pub const DENY_ADVISORIES: Module = Module {
    name: "deny/advisories",
    description: "Security advisory checking settings",
    content: r#"[advisories]
# unmaintained: check workspace deps only (transitive unmaintained crates are not actionable)
unmaintained = "workspace"
yanked = "warn" # EXCEPTION: yanked crates in transitive deps are not actionable -- warn instead of deny
ignore = []"#,
};

pub const DENY_SOURCES: Module = Module {
    name: "deny/sources",
    description: "Source registry restrictions",
    content: r#"[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
allow-git = []"#,
};

/// Returns all ban entry modules for the "service" profile.
pub fn service_profile_ban_entries() -> Vec<&'static Module> {
    vec![
        &DENY_BANS_JSON,
        &DENY_BANS_TLS,
        &DENY_BANS_HTTP,
        &DENY_BANS_LOGGING,
        &DENY_BANS_ASYNC,
        &DENY_BANS_ERROR,
        &DENY_BANS_WEB,
        &DENY_BANS_DATETIME,
        &DENY_BANS_ORM,
        &DENY_BANS_SERIALIZATION,
    ]
}

/// Library IO bans -- ban all I/O framework crates in library crates.
pub const DENY_BANS_LIBRARY_IO: Module = Module {
    name: "deny/bans/library-io",
    description: "Ban I/O framework crates in library crates",
    content: r#"    # --- Library IO bans: no I/O frameworks in pure libraries ---
    { name = "axum", wrappers = [] },
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

/// Returns all ban entry modules for the "library" profile.
/// Includes all service bans plus library-io bans.
pub fn library_profile_ban_entries() -> Vec<&'static Module> {
    let mut entries = service_profile_ban_entries();
    entries.push(&DENY_BANS_LIBRARY_IO);
    entries
}

/// Build the full deny.toml content using default service profile entries.
/// `extra_bans`, `extra_skip`, and `extra_feature_bans` come from local override files.
pub fn build_deny_toml(
    profile: &str,
    extra_bans: &str,
    extra_skip: &str,
    extra_feature_bans: &str,
) -> String {
    build_deny_toml_with_entries(
        profile,
        &service_profile_ban_entries(),
        Some(DENY_FEATURE_BANS_TOKIO.content),
        extra_bans,
        extra_skip,
        extra_feature_bans,
    )
}

/// Build the full deny.toml content with explicit ban entries and optional feature ban.
pub fn build_deny_toml_with_entries(
    profile: &str,
    ban_entries: &[&Module],
    feature_ban_content: Option<&str>,
    extra_bans: &str,
    extra_skip: &str,
    extra_feature_bans: &str,
) -> String {
    let mut out = String::new();

    out.push_str(
        "# =============================================================================\n",
    );
    let _ = writeln!(out, "# deny.toml -- GENERATED by guardrail3 (profile: {profile})");
    out.push_str("# DO NOT EDIT -- regenerate with: guardrail3 generate\n");
    out.push_str(
        "# =============================================================================\n\n",
    );

    // Graph
    out.push_str(DENY_GRAPH.content);
    out.push_str("\n\n");

    // Bans section
    out.push_str("# ============================================================\n");
    out.push_str("# DEPENDENCY BANS\n");
    out.push_str("# ============================================================\n");
    out.push_str(DENY_BANS_BASE.content);
    out.push_str("\n\n");

    // Skip entries
    out.push_str("skip = [\n");
    if !extra_skip.trim().is_empty() {
        out.push_str(extra_skip);
        out.push('\n');
    }
    out.push_str("]\n\n");

    // Deny entries
    out.push_str("deny = [\n");
    for module in ban_entries {
        out.push_str(module.content);
        out.push('\n');
    }
    if !extra_bans.trim().is_empty() {
        out.push_str("    # --- Local overrides ---\n");
        out.push_str(extra_bans);
        out.push('\n');
    }
    out.push_str("]\n\n");

    // Feature bans
    if let Some(fb_content) = feature_ban_content {
        out.push_str(
            "# Feature-level bans -- prevent agents from enabling kitchen-sink features\n",
        );
        out.push_str(fb_content);
        out.push('\n');
    }
    if !extra_feature_bans.trim().is_empty() {
        out.push('\n');
        out.push_str(extra_feature_bans);
        out.push('\n');
    }
    out.push('\n');

    // Licenses
    out.push_str("# ============================================================\n");
    out.push_str("# LICENSE CHECKING\n");
    out.push_str("# ============================================================\n");
    out.push_str(DENY_LICENSES.content);
    out.push_str("\n\n");

    // Advisories
    out.push_str("# ============================================================\n");
    out.push_str("# SECURITY ADVISORY CHECKING\n");
    out.push_str("# ============================================================\n");
    out.push_str(DENY_ADVISORIES.content);
    out.push_str("\n\n");

    // Sources
    out.push_str("# ============================================================\n");
    out.push_str("# SOURCE RESTRICTIONS\n");
    out.push_str("# ============================================================\n");
    out.push_str(DENY_SOURCES.content);
    out.push('\n');

    out
}
