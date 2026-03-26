mod classification;
mod overlap;
mod roots;

pub use classification::{
    RustArchitectureOwner, RustRootClassification, RustRootPlacementRootFacts,
};
pub use overlap::{RustZoneOverlapFacts, collect_overlaps};
pub use roots::{
    RustRootPlacementFacts, RustRootPlacementInputFailureFacts, collect, is_excluded_live_root_dir,
};
