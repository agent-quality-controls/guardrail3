//! Centralized filesystem port for the jscpd ingestion runtime.
//!
//! Production runtime code accesses the filesystem through helpers in this
//! module so that the project-wide ban on direct `std::fs` use is enforced
//! everywhere except this single boundary file.

#![expect(
    clippy::disallowed_types,
    reason = "This module is the centralized fs port required by the project policy; \
              `std::fs::File` is intentionally constructed here and only here so that \
              the ban applies to every other call site"
)]

use std::fs::File;
use std::path::Path;

/// Returns `true` when `path` can be opened for reading.
pub(crate) fn is_readable_file(path: &Path) -> bool {
    File::open(path).is_ok()
}
