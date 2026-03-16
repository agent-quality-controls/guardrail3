//! Real filesystem adapter implementing the `FileSystem` port.

use std::path::Path;

use crate::ports::outbound::FileSystem;

/// Production filesystem implementation that delegates to `crate::fs`.
pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn read_file(&self, path: &Path) -> Option<String> {
        crate::fs::read_file(path)
    }

    fn read_file_err(&self, path: &Path) -> Result<String, std::io::Error> {
        crate::fs::read_file_err(path)
    }

    fn list_dir(&self, path: &Path) -> Vec<std::fs::DirEntry> {
        crate::fs::list_dir(path)
    }

    fn metadata(&self, path: &Path) -> Option<std::fs::Metadata> {
        crate::fs::metadata(path)
    }
}
