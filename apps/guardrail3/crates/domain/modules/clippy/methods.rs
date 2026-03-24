use crate::domain::modules::Module;

pub const SERVICE_METHOD_PATHS: &[&str] = &[
    "std::env::var",
    "std::env::var_os",
    "std::env::vars",
    "std::env::set_var",
    "std::env::remove_var",
    "std::process::exit",
    "std::process::abort",
    "std::process::Command::new",
    "std::thread::sleep",
    "std::fs::read_to_string",
    "std::fs::read",
    "std::fs::read_dir",
    "std::fs::read_link",
    "std::fs::write",
    "std::fs::remove_file",
    "std::fs::remove_dir_all",
    "std::fs::create_dir_all",
    "std::fs::rename",
    "std::fs::copy",
    "std::fs::metadata",
    "std::fs::symlink_metadata",
    "std::fs::canonicalize",
    "std::fs::set_permissions",
    "std::fs::hard_link",
    "reqwest::Client::new",
    "reqwest::Client::builder",
    "serde_json::from_str",
    "serde_json::from_slice",
    "serde_json::from_value",
    "serde_json::from_reader",
    "reqwest::Response::json",
    "toml::from_str",
    "serde_yaml::from_str",
    "serde_yaml::from_reader",
    "serde_qs::from_str",
    "serde_qs::from_bytes",
    "serde_urlencoded::from_str",
    "serde_urlencoded::from_bytes",
    "serde_urlencoded::from_reader",
    "ciborium::from_reader",
    "ciborium::de::from_reader",
    "rmp_serde::from_slice",
    "rmp_serde::from_read",
    "rmp_serde::decode::from_slice",
    "rmp_serde::decode::from_read",
    "bincode::deserialize",
    "bincode::deserialize_from",
    "bincode::serde::decode_from_slice",
    "bincode::serde::decode_from_reader",
    "csv::Reader::deserialize",
    "csv::StringRecord::deserialize",
    "csv::ByteRecord::deserialize",
    "serde_xml_rs::from_str",
    "serde_xml_rs::from_reader",
    "quick_xml::de::from_str",
    "quick_xml::de::from_reader",
    "ron::from_str",
    "ron::de::from_str",
    "serde_cbor::from_slice",
    "serde_cbor::from_reader",
    "postcard::from_bytes",
    "flexbuffers::from_slice",
    "serde_json::Deserializer::from_str",
    "serde_json::Deserializer::from_slice",
    "serde_json::Deserializer::from_reader",
    "toml_edit::de::from_str",
    "toml_edit::de::from_slice",
    "toml_edit::de::from_document",
    "config::Config::try_deserialize",
    "figment::Figment::extract",
];

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
    { path = "std::process::abort", reason = "Panic or return an error instead -- abort skips destructors and bypasses shutdown handling" },
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

pub const METHOD_GARDE_DESERIALIZATION: Module = Module {
    name: "clippy/methods/garde-deserialization",
    description: "Ban raw serde deserialization (use Validated<T>::new() for external data)",
    content: r#"    { path = "serde_json::from_str", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_json::from_slice", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_json::from_value", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_json::from_reader", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "reqwest::Response::json", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "toml::from_str", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_yaml::from_str", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_yaml::from_reader", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_qs::from_str", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_qs::from_bytes", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_urlencoded::from_str", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_urlencoded::from_bytes", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_urlencoded::from_reader", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "ciborium::from_reader", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "ciborium::de::from_reader", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "rmp_serde::from_slice", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "rmp_serde::from_read", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "rmp_serde::decode::from_slice", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "rmp_serde::decode::from_read", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "bincode::deserialize", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "bincode::deserialize_from", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "bincode::serde::decode_from_slice", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "bincode::serde::decode_from_reader", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "csv::Reader::deserialize", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "csv::StringRecord::deserialize", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "csv::ByteRecord::deserialize", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_xml_rs::from_str", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_xml_rs::from_reader", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "quick_xml::de::from_str", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "quick_xml::de::from_reader", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "ron::from_str", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "ron::de::from_str", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_cbor::from_slice", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_cbor::from_reader", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "postcard::from_bytes", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "flexbuffers::from_slice", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_json::Deserializer::from_str", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_json::Deserializer::from_slice", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "serde_json::Deserializer::from_reader", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "toml_edit::de::from_str", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "toml_edit::de::from_slice", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "toml_edit::de::from_document", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "config::Config::try_deserialize", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },
    { path = "figment::Figment::extract", reason = "BANNED: Use Validated<T>::new(value, &ctx) instead. (1) Add #[derive(garde::Validate)] to your response/data type. (2) Add garde rules to each field -- #[garde(length(chars, min = 1, max = N))] for strings, #[garde(range(min = 0))] for numbers, #[garde(dive)] for nested structs, #[garde(length(min = 1))] for non-empty vecs. (3) Deserialize into the raw type, then wrap with Validated::new()." },"#,
};

pub fn service_profile_methods() -> Vec<&'static Module> {
    vec![
        &METHOD_ENV_VARS,
        &METHOD_ENV_MUTATION,
        &METHOD_PROCESS_CONTROL,
        &METHOD_BLOCKING_SLEEP,
        &METHOD_FILESYSTEM,
        &METHOD_HTTP_CLIENT,
        &METHOD_GARDE_DESERIALIZATION,
    ]
}

pub fn library_profile_methods() -> Vec<&'static Module> {
    service_profile_methods()
}
