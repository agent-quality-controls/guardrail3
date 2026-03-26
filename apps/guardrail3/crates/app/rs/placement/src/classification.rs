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
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub classification: RustRootClassification,
    pub arch_role: Option<RustArchRole>,
    pub app_zone_candidates: Vec<String>,
    pub package_zone_candidates: Vec<String>,
    pub owner_families: Vec<RustArchitectureOwner>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustArchRole {
    Auxiliary,
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

    RustRootPlacementRootFacts {
        rel_dir,
        cargo_rel_path,
        classification,
        arch_role,
        app_zone_candidates,
        package_zone_candidates,
        owner_families,
    }
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
