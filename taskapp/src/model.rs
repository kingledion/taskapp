use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

#[cfg(feature = "ssr")]
impl From<crate::entity::task::Model> for Task {
    fn from(model: crate::entity::task::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            description: model.description,
            created_at: model.created_at,
            completed_at: model.completed_at,
            deleted_at: model.deleted_at,
        }
    }
}
