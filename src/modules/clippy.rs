use super::Module;

pub const THRESHOLDS: &str = r"# Maximum lines per function before clippy::too_many_lines fires.
too-many-lines-threshold = 75

# Maximum cognitive complexity score per function.
cognitive-complexity-threshold = 15

# Maximum number of function parameters.
too-many-arguments-threshold = 7

# Maximum type nesting depth (e.g., Result<Option<Vec<Box<dyn Trait>>>>).
# 75 is the clippy default. Legitimate types like Option<BTreeMap<String, Vec<u8>>>
# score well under this. If legitimate types hit this, bump to 100.
type-complexity-threshold = 75

# Maximum number of bool fields in a struct before clippy::struct_excessive_bools fires.
max-struct-bools = 3";

// ---------------------------------------------------------------------------
// Disallowed method modules
// ---------------------------------------------------------------------------

pub const METHOD_ENV_VARS: Module = Module {
    name: "clippy/methods/env-vars",
    description: "Ban direct environment variable access (std::env::var*)",
    content: r#"    { path = "std::env::var", reason = "Use the centralized config module -- direct env access scatters configuration and is untestable" },
    { path = "std::env::var_os", reason = "Use the centralized config module -- direct env access scatters configuration and is untestable" },
    { path = "std::env::vars", reason = "Use the centralized config module -- direct env access scatters configuration and is untestable" },"#,
};

pub const METHOD_ENV_MUTATION: Module = Module {
    name: "clippy/methods/env-mutation",
    description: "Ban environment variable mutation (unsafe in multi-threaded contexts)",
    content: r#"    { path = "std::env::set_var", reason = "Unsafe in multi-threaded contexts -- environment mutation is not thread-safe" },
    { path = "std::env::remove_var", reason = "Unsafe in multi-threaded contexts -- environment mutation is not thread-safe" },"#,
};

pub const METHOD_PROCESS_CONTROL: Module = Module {
    name: "clippy/methods/process-control",
    description: "Ban process::exit and shell execution",
    content: r#"    { path = "std::process::exit", reason = "Use proper error propagation (return Result from main) -- process::exit skips destructors" },
    { path = "std::process::Command::new", reason = "Shell execution not permitted in this service" },"#,
};

pub const METHOD_BLOCKING_SLEEP: Module = Module {
    name: "clippy/methods/blocking-sleep",
    description: "Ban std::thread::sleep (use tokio::time::sleep)",
    content: r#"    { path = "std::thread::sleep", reason = "Use tokio::time::sleep for async context -- std::thread::sleep blocks the tokio runtime" },"#,
};

pub const METHOD_FILESYSTEM: Module = Module {
    name: "clippy/methods/filesystem",
    description: "Ban direct filesystem operations (create a centralized fs module)",
    content: r#"    # Reads
    { path = "std::fs::read_to_string", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },
    { path = "std::fs::read", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },
    { path = "std::fs::read_dir", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },
    { path = "std::fs::read_link", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },

    # Writes
    { path = "std::fs::write", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },

    # Destructive operations
    { path = "std::fs::remove_file", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },
    { path = "std::fs::remove_dir_all", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },

    # Directory creation
    { path = "std::fs::create_dir_all", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },

    # Move / copy
    { path = "std::fs::rename", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },
    { path = "std::fs::copy", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },

    # Metadata and inspection
    { path = "std::fs::metadata", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },
    { path = "std::fs::symlink_metadata", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },
    { path = "std::fs::canonicalize", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },
    { path = "std::fs::set_permissions", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },
    { path = "std::fs::hard_link", reason = "BANNED: Create a centralized fs module and route all filesystem operations through it -- no scattered std::fs calls" },"#,
};

pub const METHOD_HTTP_CLIENT: Module = Module {
    name: "clippy/methods/http-client",
    description: "Ban per-request HTTP client construction (use shared client via DI)",
    content: r#"    { path = "reqwest::Client::new", reason = "Use shared client from AppState -- per-request construction skips connection pooling" },
    { path = "reqwest::Client::builder", reason = "Use shared client from AppState -- construct once at startup and inject via DI" },"#,
};

// ---------------------------------------------------------------------------
// Disallowed type modules
// ---------------------------------------------------------------------------

pub const TYPE_COLLECTIONS: Module = Module {
    name: "clippy/types/collections",
    description: "Ban HashMap/HashSet (use BTree variants for deterministic ordering)",
    content: r#"    { path = "std::collections::HashMap", reason = "Use BTreeMap for deterministic iteration order" },
    { path = "std::collections::HashSet", reason = "Use BTreeSet for deterministic iteration order" },"#,
};

pub const TYPE_SYNC: Module = Module {
    name: "clippy/types/sync",
    description: "Ban std::sync::Mutex/RwLock (use parking_lot)",
    content: r#"    { path = "std::sync::Mutex", reason = "Use parking_lot::Mutex -- no poisoning, better performance" },
    { path = "std::sync::RwLock", reason = "Use parking_lot::RwLock -- no poisoning, better performance" },"#,
};

pub const TYPE_FILESYSTEM: Module = Module {
    name: "clippy/types/filesystem",
    description: "Ban std::fs::File (use centralized fs module)",
    content: r#"    { path = "std::fs::File", reason = "BANNED: Create a centralized fs module -- no direct file handle construction" },"#,
};

pub const TYPE_GLOBAL_STATE: Module = Module {
    name: "clippy/types/global-state",
    description: "Ban global state types in pure layers (LazyLock, OnceLock, once_cell)",
    content: r#"    { path = "std::sync::LazyLock", reason = "No global state in business logic -- inject dependencies instead" },
    { path = "std::sync::OnceLock", reason = "No global state in business logic -- inject dependencies instead" },
    { path = "once_cell::sync::Lazy", reason = "No global state in business logic -- inject dependencies instead" },
    { path = "once_cell::sync::OnceCell", reason = "No global state in business logic -- inject dependencies instead" },"#,
};

/// Returns the list of method modules included in the "service" profile.
pub fn service_profile_methods() -> Vec<&'static Module> {
    vec![
        &METHOD_ENV_VARS,
        &METHOD_ENV_MUTATION,
        &METHOD_PROCESS_CONTROL,
        &METHOD_BLOCKING_SLEEP,
        &METHOD_FILESYSTEM,
        &METHOD_HTTP_CLIENT,
    ]
}

/// Returns the list of type modules included in the "service" profile (workspace-wide).
pub fn service_profile_types() -> Vec<&'static Module> {
    vec![&TYPE_COLLECTIONS, &TYPE_SYNC, &TYPE_FILESYSTEM]
}

/// Returns extra type modules for "pure" layer crates (no global state).
pub fn pure_layer_extra_types() -> Vec<&'static Module> {
    vec![&TYPE_GLOBAL_STATE]
}

/// Returns the method modules for the "library" profile (same as service).
pub fn library_profile_methods() -> Vec<&'static Module> {
    service_profile_methods()
}

/// Returns the type modules for the "library" profile.
/// Always includes global-state bans (all crates are "pure" in a library).
pub fn library_profile_types() -> Vec<&'static Module> {
    vec![
        &TYPE_COLLECTIONS,
        &TYPE_SYNC,
        &TYPE_FILESYSTEM,
        &TYPE_GLOBAL_STATE,
    ]
}

/// Build the full clippy.toml content for a workspace root or crate.
/// `is_pure_layer` adds global-state type bans (for service/monorepo profiles).
/// `extra_methods` and `extra_types` are appended from local override files.
pub fn build_clippy_toml(
    profile: &str,
    is_pure_layer: bool,
    extra_methods: &str,
    extra_types: &str,
) -> String {
    let methods = match profile {
        "library" => library_profile_methods(),
        _ => service_profile_methods(),
    };

    let types = match profile {
        "library" => library_profile_types(),
        _ => {
            let mut t = service_profile_types();
            if is_pure_layer {
                t.extend(pure_layer_extra_types());
            }
            t
        }
    };

    let mut out = String::new();

    out.push_str(
        "# =============================================================================\n",
    );
    #[allow(clippy::format_push_string)] // reason: format! in push_str for template generation
    out.push_str(&format!(
        "# clippy.toml -- GENERATED by guardrail3 (profile: {profile})\n"
    ));
    out.push_str("# DO NOT EDIT -- regenerate with: guardrail3 generate\n");
    out.push_str(
        "# =============================================================================\n\n",
    );

    // Thresholds
    out.push_str("# THRESHOLDS\n");
    out.push_str(THRESHOLDS);
    out.push_str("\n\n");

    // Disallowed methods
    out.push_str("# DISALLOWED METHODS\n");
    out.push_str("disallowed-methods = [\n");
    for module in &methods {
        #[allow(clippy::format_push_string)] // reason: format! in push_str for template generation
        out.push_str(&format!("    # --- {} ---\n", module.description));
        out.push_str(module.content);
        out.push('\n');
    }
    if !extra_methods.trim().is_empty() {
        out.push_str("    # --- Local overrides ---\n");
        out.push_str(extra_methods);
        out.push('\n');
    }
    out.push_str("]\n\n");

    // Disallowed types
    out.push_str("# DISALLOWED TYPES\n");
    out.push_str("disallowed-types = [\n");
    for module in &types {
        #[allow(clippy::format_push_string)] // reason: format! in push_str for template generation
        out.push_str(&format!("    # --- {} ---\n", module.description));
        out.push_str(module.content);
        out.push('\n');
    }
    if !extra_types.trim().is_empty() {
        out.push_str("    # --- Local overrides ---\n");
        out.push_str(extra_types);
        out.push('\n');
    }
    out.push_str("]\n");

    out
}
