use std::path::Path;

#[expect(
    clippy::disallowed_methods,
    reason = "test fixture creation is isolated here so production code keeps centralized filesystem access"
)]
pub(super) fn write(path: &Path, content: &str) {
    std::fs::write(path, content).expect("write temporary workspace fixture file");
}
