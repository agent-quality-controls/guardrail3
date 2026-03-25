use std::ffi::OsString;
use std::fmt;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

/// Portable filesystem error shape for trait boundaries.
#[derive(Debug, Clone)]
pub struct FsIoError {
    kind: io::ErrorKind,
    message: String,
}

impl FsIoError {
    #[must_use]
    pub const fn kind(&self) -> io::ErrorKind {
        self.kind
    }
}

impl fmt::Display for FsIoError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for FsIoError {}

impl From<io::Error> for FsIoError {
    fn from(error: io::Error) -> Self {
        Self {
            kind: error.kind(),
            message: error.to_string(),
        }
    }
}

/// Portable file type facts for a directory entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FsFileType {
    is_dir: bool,
    is_file: bool,
    is_symlink: bool,
}

impl FsFileType {
    #[must_use]
    pub fn from_std(file_type: std::fs::FileType) -> Self {
        Self {
            is_dir: file_type.is_dir(),
            is_file: file_type.is_file(),
            is_symlink: file_type.is_symlink(),
        }
    }

    #[must_use]
    pub const fn is_dir(self) -> bool {
        self.is_dir
    }

    #[must_use]
    pub const fn is_file(self) -> bool {
        self.is_file
    }

    #[must_use]
    pub const fn is_symlink(self) -> bool {
        self.is_symlink
    }
}

/// Portable permissions snapshot for filesystem metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FsPermissions {
    readonly: bool,
    #[cfg(unix)]
    mode: u32,
}

impl FsPermissions {
    #[must_use]
    pub fn from_std(permissions: std::fs::Permissions) -> Self {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            Self {
                readonly: permissions.readonly(),
                mode: permissions.mode(),
            }
        }

        #[cfg(not(unix))]
        {
            Self {
                readonly: permissions.readonly(),
            }
        }
    }

    #[must_use]
    pub const fn readonly(self) -> bool {
        self.readonly
    }

    #[cfg(unix)]
    #[must_use]
    pub const fn mode(self) -> u32 {
        self.mode
    }
}

/// Portable metadata snapshot for a filesystem object.
#[derive(Debug, Clone)]
pub struct FsMetadata {
    is_dir: bool,
    is_file: bool,
    len: u64,
    modified: Result<SystemTime, FsIoError>,
    permissions: FsPermissions,
}

impl FsMetadata {
    #[must_use]
    pub fn from_std(metadata: std::fs::Metadata) -> Self {
        Self {
            is_dir: metadata.is_dir(),
            is_file: metadata.is_file(),
            len: metadata.len(),
            modified: metadata.modified().map_err(FsIoError::from),
            permissions: FsPermissions::from_std(metadata.permissions()),
        }
    }

    #[must_use]
    pub const fn is_dir(&self) -> bool {
        self.is_dir
    }

    #[must_use]
    pub const fn is_file(&self) -> bool {
        self.is_file
    }

    #[must_use]
    pub const fn len(&self) -> u64 {
        self.len
    }

    pub fn modified(&self) -> Result<SystemTime, FsIoError> {
        self.modified.clone()
    }

    #[must_use]
    pub const fn permissions(&self) -> FsPermissions {
        self.permissions
    }
}

/// Portable directory entry snapshot for the shared trait surface.
#[derive(Debug, Clone)]
pub struct FsDirEntry {
    path: PathBuf,
    file_name: OsString,
    file_type: Result<FsFileType, FsIoError>,
    metadata: Result<FsMetadata, FsIoError>,
}

impl FsDirEntry {
    #[must_use]
    pub fn from_std(entry: std::fs::DirEntry) -> Self {
        Self {
            path: entry.path(),
            file_name: entry.file_name(),
            file_type: entry
                .file_type()
                .map(FsFileType::from_std)
                .map_err(FsIoError::from),
            metadata: entry
                .metadata()
                .map(FsMetadata::from_std)
                .map_err(FsIoError::from),
        }
    }

    #[must_use]
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    #[must_use]
    pub fn file_name(&self) -> OsString {
        self.file_name.clone()
    }

    pub fn file_type(&self) -> Result<FsFileType, FsIoError> {
        self.file_type.clone()
    }

    pub fn metadata(&self) -> Result<FsMetadata, FsIoError> {
        self.metadata.clone()
    }
}
