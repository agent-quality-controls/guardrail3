//! Test-only filesystem helpers. Centralizes the disallowed `std::fs`
//! calls so production callers can stay routed through a shared layer.

#![allow(
    clippy::disallowed_methods,
    reason = "Test fixtures must materialize on-disk trees; the centralized fs module does not service tests."
)]

use std::path::Path;

pub(crate) fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent directories for test fixture file");
    }
    std::fs::write(path, content).expect("write fixture file to temp directory");
}

pub(crate) fn mkdir_p(path: &Path) {
    std::fs::create_dir_all(path).expect("create fixture directory tree under tempdir");
}

pub(crate) fn write_package_json(dir: &Path) {
    write_file(&dir.join("package.json"), "{}\n");
}

pub(crate) fn write_guardrail3_ts_toml(dir: &Path) {
    write_file(&dir.join("guardrail3-ts.toml"), "");
}
