mod classification;
mod overlap;
mod roots;

#[cfg(feature = "api")]
pub use classification::{
    RustTopologyRole, RustTopologyOwner, RustRootClassification, RustRootPlacementRootFacts,
};
#[cfg(feature = "api")]
pub use overlap::{RustZoneOverlapFacts, collect_overlaps};
#[cfg(feature = "api")]
pub use roots::{
    RustRootPlacementFacts, RustRootPlacementInputFailureFacts, collect, is_excluded_live_root_dir,
};
