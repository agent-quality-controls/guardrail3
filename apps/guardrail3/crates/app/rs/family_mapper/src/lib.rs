mod rs;
mod scoped_files;
mod views;

// DirEntry re-export removed. Families don't get directory access.
pub use rs::FamilyMapper;
pub type RsTopologyOverlapView = views::RsTopologyOverlapView;
pub type RsTopologyRootView = views::RsTopologyRootView;
pub type RsTopologyRoute = views::RsTopologyRoute;
pub type RsTopologyIssueKindView = views::RsTopologyIssueKindView;
pub type RsTopologyIssueView = views::RsTopologyIssueView;
pub type RsArchRoute = views::RsArchRoute;
pub type RsCargoRoute = views::RsCargoRoute;
pub type RsClippyRoute = views::RsClippyRoute;
pub type RsCodeRoute = views::RsCodeRoute;
pub type RsDenyRoute = views::RsDenyRoute;
pub type RsDepsRoute = views::RsDepsRoute;
pub type RsFamilyFileAttachmentView = views::RsFamilyFileAttachmentView;
pub type RsFamilyFilePlacementView = views::RsFamilyFilePlacementView;
pub type RsFamilyFileView = views::RsFamilyFileView;
pub type RsFmtRoute = views::RsFmtRoute;
pub type RsGardeRoute = views::RsGardeRoute;
pub type RsHexarchRoute = views::RsHexarchRoute;
// RsProjectSurface removed. Families receive FamilyView from legality.
pub type RsReleaseRoute = views::RsReleaseRoute;
pub type RsRootInputFailureView = views::RsRootInputFailureView;
pub type RsRootView = views::RsRootView;
pub type RsScopedRootView = views::RsScopedRootView;
pub type RsScopedSourceRoute = views::RsScopedSourceRoute;
pub type RsTestRoute = views::RsTestRoute;
pub type RsToolchainRoute = views::RsToolchainRoute;
