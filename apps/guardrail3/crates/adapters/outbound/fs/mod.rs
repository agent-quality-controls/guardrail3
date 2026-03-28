//! Real filesystem adapter implementing the `FileSystem` port.

use std::path::Path;

use guardrail3_outbound_traits::{FileSystem, FsDirEntry, FsMetadata};

/// Production filesystem implementation that delegates to `guardrail3_shared_fs`.
#[derive(Debug)]
pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn read_file(&self, path: &Path) -> Option<String> {
        guardrail3_shared_fs::read_file(path)
    }

    fn read_file_err(&self, path: &Path) -> Result<String, std::io::Error> {
        guardrail3_shared_fs::read_file_err(path)
    }

    fn list_dir(&self, path: &Path) -> Vec<FsDirEntry> {
        guardrail3_shared_fs::list_dir(path)
            .into_iter()
            .map(FsDirEntry::from_std)
            .collect()
    }

    fn metadata(&self, path: &Path) -> Option<FsMetadata> {
        guardrail3_shared_fs::metadata(path).map(FsMetadata::from_std)
    }
}
