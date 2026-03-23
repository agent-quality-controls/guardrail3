use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskId(String);

impl TaskId {
    pub fn new(raw: impl Into<String>) -> Self {
        Self(raw.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Inbox,
    Scheduled,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskKind {
    Errand,
    Admin,
    Chore,
    School,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub kind: TaskKind,
    pub points: u16,
    pub status: TaskStatus,
    pub pinned: bool,
    pub carryover_count: u8,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeeklyPlan {
    pub tasks: Vec<Task>,
    pub focus_points: u16,
    pub overflow_tasks: Vec<Task>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum PlannerError {
    #[error("task title must not be blank")]
    BlankTitle,
    #[error("task points must be between 1 and 5")]
    InvalidPoints,
}

impl Task {
    pub fn try_new(
        id: impl Into<String>,
        title: impl Into<String>,
        kind: TaskKind,
        points: u16,
    ) -> Result<Self, PlannerError> {
        let title = title.into();
        if title.trim().is_empty() {
            return Err(PlannerError::BlankTitle);
        }
        if !(1..=5).contains(&points) {
            return Err(PlannerError::InvalidPoints);
        }
        Ok(Self {
            id: TaskId::new(id),
            title,
            kind,
            points,
            status: TaskStatus::Inbox,
            pinned: false,
            carryover_count: 0,
            notes: None,
        })
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn pinned(mut self) -> Self {
        self.pinned = true;
        self
    }

    pub fn carryover(mut self, carryover_count: u8) -> Self {
        self.carryover_count = carryover_count;
        self
    }
}
