use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TenantSlug(String);

impl TenantSlug {
    pub fn new(raw: impl Into<String>) -> Option<Self> {
        let value = raw.into();
        if value.trim().is_empty() {
            return None;
        }

        Some(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditStamp {
    pub actor: String,
    pub changed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceMode {
    Healthy,
    Degraded,
    Maintenance,
}
