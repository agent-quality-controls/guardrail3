use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobId(String);

impl JobId {
    pub fn new(raw: impl Into<String>) -> Result<Self, JobError> {
        let value = raw.into();
        if value.trim().is_empty() {
            return Err(JobError::MissingId);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    InFlight,
    Retrying,
    Failed,
    Completed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobKind {
    BillingDigest,
    ContentReindex,
    NotificationFanout,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Job {
    pub id: JobId,
    pub tenant_slug: String,
    pub kind: JobKind,
    pub status: JobStatus,
    pub attempts: u8,
    pub max_attempts: u8,
    pub locked_by: Option<String>,
    pub payload_hint: String,
}

impl Job {
    pub fn new(
        id: JobId,
        tenant_slug: impl Into<String>,
        kind: JobKind,
        payload_hint: impl Into<String>,
    ) -> Result<Self, JobError> {
        let tenant_slug = tenant_slug.into();
        if tenant_slug.trim().is_empty() {
            return Err(JobError::MissingTenantSlug);
        }

        Ok(Self {
            id,
            tenant_slug,
            kind,
            status: JobStatus::Pending,
            attempts: 0,
            max_attempts: 3,
            locked_by: None,
            payload_hint: payload_hint.into(),
        })
    }

    pub fn with_attempts(mut self, attempts: u8, max_attempts: u8) -> Self {
        self.attempts = attempts;
        self.max_attempts = max_attempts;
        self
    }

    pub fn lock_to(mut self, worker_id: impl Into<String>) -> Self {
        self.locked_by = Some(worker_id.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessedJob {
    pub id: JobId,
    pub disposition: JobDisposition,
    pub attempts_after_run: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobDisposition {
    Acked,
    Requeued,
    DeadLetter,
    Skipped,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum JobError {
    #[error("job id must not be empty")]
    MissingId,
    #[error("tenant slug must not be empty")]
    MissingTenantSlug,
}
