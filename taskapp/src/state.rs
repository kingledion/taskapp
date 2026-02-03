use std::sync::{Arc, RwLock};

use crate::model::Task;

#[derive(Clone, Default)]
pub struct AppState {
    pub tasks: Arc<RwLock<Vec<Task>>>,
    pub next_id: Arc<RwLock<u32>>,
}
