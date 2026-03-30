#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RustArchitectureOwner {
    Hexarch,
    Libarch,
}

impl RustArchitectureOwner {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Hexarch => "app",
            Self::Libarch => "package",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustRootClassification {
    App,
    Package,
    Auxiliary,
    Other,
    Ambiguous,
}

#[derive(Debug, Clone)]
pub struct RustRootPlacementRootFacts {
    rel_dir: String,
    cargo_rel_path: String,
    classification: RustRootClassification,
    arch_role: Option<RustArchRole>,
    app_zone_candidates: Vec<String>,
    package_zone_candidates: Vec<String>,
    owner_families: Vec<RustArchitectureOwner>,
}

impl RustRootPlacementRootFacts {
    #[must_use]
    pub fn new(
        rel_dir: String,
        cargo_rel_path: String,
        classification: RustRootClassification,
        arch_role: Option<RustArchRole>,
        app_zone_candidates: Vec<String>,
        package_zone_candidates: Vec<String>,
        owner_families: Vec<RustArchitectureOwner>,
    ) -> Self {
        Self {
            rel_dir,
            cargo_rel_path,
            classification,
            arch_role,
            app_zone_candidates,
            package_zone_candidates,
            owner_families,
        }
    }

    #[must_use]
    pub fn rel_dir(&self) -> &str {
        &self.rel_dir
    }

    #[must_use]
    pub fn cargo_rel_path(&self) -> &str {
        &self.cargo_rel_path
    }

    #[must_use]
    pub const fn classification(&self) -> RustRootClassification {
        self.classification
    }

    #[must_use]
    pub const fn arch_role(&self) -> Option<RustArchRole> {
        self.arch_role
    }

    #[must_use]
    pub fn app_zone_candidates(&self) -> &[String] {
        &self.app_zone_candidates
    }

    #[must_use]
    pub fn package_zone_candidates(&self) -> &[String] {
        &self.package_zone_candidates
    }

    #[must_use]
    pub fn owner_families(&self) -> &[RustArchitectureOwner] {
        &self.owner_families
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustArchRole {
    Auxiliary,
}

#[must_use]
pub fn has_governed_zone_candidate(rel_dir: &str) -> bool {
    !zone_candidates(rel_dir, "apps").is_empty() || !zone_candidates(rel_dir, "packages").is_empty()
}

#[must_use]
pub fn classify_root(
    rel_dir: String,
    cargo_rel_path: String,
    placement_rel_dir: &str,
    arch_role: Option<RustArchRole>,
) -> RustRootPlacementRootFacts {
    let app_zone_candidates = zone_candidates(placement_rel_dir, "apps");
    let package_zone_candidates = zone_candidates(placement_rel_dir, "packages");

    let classification = match (
        app_zone_candidates.len(),
        package_zone_candidates.len(),
        arch_role,
    ) {
        (0, 0, Some(RustArchRole::Auxiliary)) => RustRootClassification::Auxiliary,
        (0, 0, _) => RustRootClassification::Other,
        (1, 0, _) => RustRootClassification::App,
        (0, 1, _) => RustRootClassification::Package,
        _ => RustRootClassification::Ambiguous,
    };

    let mut owner_families = Vec::new();
    if !app_zone_candidates.is_empty() {
        owner_families.push(RustArchitectureOwner::Hexarch);
    }
    if !package_zone_candidates.is_empty() {
        owner_families.push(RustArchitectureOwner::Libarch);
    }

    RustRootPlacementRootFacts::new(
        rel_dir,
        cargo_rel_path,
        classification,
        arch_role,
        app_zone_candidates,
        package_zone_candidates,
        owner_families,
    )
}

fn zone_candidates(rel_dir: &str, zone_dir: &str) -> Vec<String> {
    let segments: Vec<_> = rel_dir
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    if segments.len() < 2 {
        return Vec::new();
    }

    segments
        .iter()
        .enumerate()
        .filter_map(|(index, segment)| {
            if *segment != zone_dir || index + 1 >= segments.len() {
                return None;
            }
            Some(segments[..=index + 1].join("/"))
        })
        .collect()
}
