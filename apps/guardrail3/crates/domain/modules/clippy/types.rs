use crate::domain::modules::Module;

pub const BASE_TYPE_PATHS: &[&str] = &[
    "std::collections::HashMap",
    "std::collections::HashSet",
    "std::sync::Mutex",
    "std::sync::RwLock",
    "std::fs::File",
    "axum::extract::Json",
    "axum::Json",
    "axum::extract::Query",
    "axum::extract::Form",
    "std::any::Any",
];

pub const LIBRARY_EXTRA_TYPE_PATHS: &[&str] = &[
    "std::sync::LazyLock",
    "std::sync::OnceLock",
    "once_cell::sync::Lazy",
    "once_cell::sync::OnceCell",
];

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

pub const TYPE_DYNAMIC: Module = Module {
    name: "clippy/types/dynamic",
    description: "Ban std::any::Any (type erasure bypasses typed boundaries)",
    content: r#"    { path = "std::any::Any", reason = "Avoid type erasure -- std::any::Any weakens typed boundaries and reviewability" },"#,
};

pub const TYPE_GLOBAL_STATE: Module = Module {
    name: "clippy/types/global-state",
    description: "Ban global state types in pure layers (LazyLock, OnceLock, once_cell)",
    content: r#"    { path = "std::sync::LazyLock", reason = "No global state in business logic -- inject dependencies instead" },
    { path = "std::sync::OnceLock", reason = "No global state in business logic -- inject dependencies instead" },
    { path = "once_cell::sync::Lazy", reason = "No global state in business logic -- inject dependencies instead" },
    { path = "once_cell::sync::OnceCell", reason = "No global state in business logic -- inject dependencies instead" },"#,
};

pub const TYPE_GARDE_EXTRACTORS: Module = Module {
    name: "clippy/types/garde-extractors",
    description: "Ban raw Axum extractors (use ValidatedJson/ValidatedQuery/ValidatedForm)",
    content: r#"    { path = "axum::extract::Json", reason = "BANNED: Use ValidatedJson<T>/ValidatedQuery<T>/ValidatedForm<T> instead. Requires #[derive(garde::Validate)] on the request type." },
    { path = "axum::Json", reason = "BANNED: Use ValidatedJson<T>/ValidatedQuery<T>/ValidatedForm<T> instead. Requires #[derive(garde::Validate)] on the request type." },
    { path = "axum::extract::Query", reason = "BANNED: Use ValidatedJson<T>/ValidatedQuery<T>/ValidatedForm<T> instead. Requires #[derive(garde::Validate)] on the request type." },
    { path = "axum::extract::Form", reason = "BANNED: Use ValidatedJson<T>/ValidatedQuery<T>/ValidatedForm<T> instead. Requires #[derive(garde::Validate)] on the request type." },"#,
};

pub fn service_profile_types() -> Vec<&'static Module> {
    vec![
        &TYPE_COLLECTIONS,
        &TYPE_SYNC,
        &TYPE_FILESYSTEM,
        &TYPE_DYNAMIC,
        &TYPE_GARDE_EXTRACTORS,
    ]
}

pub fn pure_layer_extra_types() -> Vec<&'static Module> {
    vec![&TYPE_GLOBAL_STATE]
}

pub fn library_profile_types() -> Vec<&'static Module> {
    vec![
        &TYPE_COLLECTIONS,
        &TYPE_SYNC,
        &TYPE_FILESYSTEM,
        &TYPE_DYNAMIC,
        &TYPE_GLOBAL_STATE,
        &TYPE_GARDE_EXTRACTORS,
    ]
}
