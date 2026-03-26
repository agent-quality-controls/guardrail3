const APP_WORKSPACE_CARGO: &str = "[workspace]\nmembers = []\nresolver = \"2\"\n";
const PACKAGE_CARGO: &str = "[package]\nname = \"shared\"\nedition = \"2024\"\n";

pub(super) enum CargoFixture {
    AppWorkspace,
    Package,
}

pub(super) fn cargo_fixture(kind: CargoFixture) -> &'static str {
    match kind {
        CargoFixture::AppWorkspace => APP_WORKSPACE_CARGO,
        CargoFixture::Package => PACKAGE_CARGO,
    }
}

pub(crate) use test_support::{entry, tree, tree_at};

mod golden;
mod ownership_coherence;
mod package_coherence;
