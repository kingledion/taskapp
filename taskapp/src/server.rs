use crate::model::Task;
use leptos::prelude::*;

#[server(GetTasks)]
pub async fn get_tasks() -> Result<Vec<Task>, ServerFnError> {
    use crate::state::AppState;
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(state): Extension<AppState> = extract().await?;
    let tasks = state.tasks.read().unwrap().clone();
    Ok(tasks)
}

#[server(CreateTask)]
pub async fn create_task(title: String, description: String) -> Result<Task, ServerFnError> {
    use crate::state::AppState;
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(state): Extension<AppState> = extract().await?;

    let mut next_id = state.next_id.write().unwrap();
    let id = *next_id;
    *next_id += 1;

    let task = Task {
        id,
        title,
        description,
    };

    state.tasks.write().unwrap().push(task.clone());
    Ok(task)
}
