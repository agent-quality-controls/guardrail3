#![no_main]
//! Fuzz target for guardrail3 project detection.
//!
//! Creates a temp directory with a fuzzed Cargo.toml and/or package.json,
//! then runs project detection on it. Goal: detect_project must never panic
//! regardless of file content.

use libfuzzer_sys::fuzz_target;

use guardrail3::app::discover;

fuzz_target!(|data: &[u8]| {
    // Only try valid UTF-8
    let Ok(content) = std::str::from_utf8(data) else {
        return;
    };

    // Skip very large inputs to avoid slow temp dir operations
    if content.len() > 4096 {
        return;
    }

    // Create a temp directory with fuzzed config files
    let Ok(dir) = tempfile::tempdir() else {
        return;
    };

    // Write as Cargo.toml — tests TOML parsing in detect_rust
    let _ = std::fs::write(dir.path().join("Cargo.toml"), content);

    // Run project detection — must not panic
    let _ = discover::detect_project(dir.path());

    // Also try with fuzzed package.json
    let _ = std::fs::write(dir.path().join("package.json"), content);
    let _ = discover::detect_project(dir.path());

    // Try with nested structure (monorepo pattern)
    let apps_backend = dir.path().join("apps").join("backend");
    let _ = std::fs::create_dir_all(&apps_backend);
    let _ = std::fs::write(apps_backend.join("Cargo.toml"), content);
    let _ = discover::detect_project(dir.path());
});
