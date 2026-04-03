use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RustValidateFamily {
    Topology,
    Arch,
    Fmt,
    Toolchain,
    Clippy,
    Deny,
    Cargo,
    Code,
    Hexarch,
    Deps,
    Garde,
    Test,
    Release,
    HooksShared,
    HooksRs,
}

impl RustValidateFamily {
    pub const ALL: [Self; 15] = [
        Self::Topology,
        Self::Arch,
        Self::Fmt,
        Self::Toolchain,
        Self::Clippy,
        Self::Deny,
        Self::Cargo,
        Self::Code,
        Self::Hexarch,
        Self::Deps,
        Self::Garde,
        Self::Test,
        Self::Release,
        Self::HooksShared,
        Self::HooksRs,
    ];

    #[must_use]
    pub fn all() -> &'static [Self] {
        &Self::ALL
    }
}

#[derive(Debug, Clone, Default)]
pub struct RustFamilySelection {
    enabled: BTreeSet<RustValidateFamily>,
}

impl RustFamilySelection {
    #[must_use]
    pub fn new(enabled: BTreeSet<RustValidateFamily>) -> Self {
        Self { enabled }
    }

    #[must_use]
    pub fn all() -> Self {
        Self::new(RustValidateFamily::all().iter().copied().collect())
    }

    #[must_use]
    pub fn contains(&self, family: RustValidateFamily) -> bool {
        self.enabled.contains(&family)
    }

    pub fn insert(&mut self, family: RustValidateFamily) {
        let _ = self.enabled.insert(family);
    }

    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = RustValidateFamily> + '_ {
        self.enabled.iter().copied()
    }
}
