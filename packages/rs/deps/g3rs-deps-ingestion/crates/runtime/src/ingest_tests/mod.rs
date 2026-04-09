mod basic;
mod deps;
mod pipeline;

use std::path::Path;

use tempfile::TempDir;

fn write_file(workspace: &Path, rel_path: &str, content: &str) {
    let abs_path = workspace.join(rel_path);
    if let Some(parent) = abs_path.parent() {
        #[allow(clippy::disallowed_methods, reason = "test fixture setup")]
        std::fs::create_dir_all(parent).expect("fixture parent directory should be created");
    }
    #[allow(clippy::disallowed_methods, reason = "test fixture setup")]
    std::fs::write(abs_path, content).expect("fixture file should be written");
}

fn temp_workspace() -> TempDir {
    TempDir::new().expect("temp workspace should be created")
}
