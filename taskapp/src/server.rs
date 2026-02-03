use crate::model::Task;
use leptos::prelude::*;

#[server(GetTasks)]
pub async fn get_tasks() -> Result<Vec<Task>, ServerFnError> {
    use crate::state::AppState;
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(state): Extension<AppState> = extract().await?;
    let tasks = state
        .tasks
        .read()
        .unwrap()
        .iter()
        .filter(|t| !t.is_deleted())
        .cloned()
        .collect();
    Ok(tasks)
}

#[server(CreateTask)]
pub async fn create_task(title: String, description: String) -> Result<Task, ServerFnError> {
    use crate::state::AppState;
    use axum::Extension;
    use chrono::Utc;
    use leptos_axum::extract;

    let Extension(state): Extension<AppState> = extract().await?;

    let mut next_id = state.next_id.write().unwrap();
    let id = *next_id;
    *next_id += 1;

    let now = Utc::now();
    let task = Task {
        id,
        title,
        description,
        created_at: now,
        completed_at: None,
        deleted_at: None,
    };

    state.tasks.write().unwrap().push(task.clone());
    Ok(task)
}

#[server(CompleteTask)]
pub async fn complete_task(id: u32, completed: bool) -> Result<Task, ServerFnError> {
    use crate::state::AppState;
    use axum::Extension;
    use chrono::Utc;
    use leptos_axum::extract;

    let Extension(state): Extension<AppState> = extract().await?;

    let mut tasks = state.tasks.write().unwrap();
    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| ServerFnError::new("Task not found"))?;

    let now = Utc::now();
    task.completed_at = if completed { Some(now) } else { None };

    Ok(task.clone())
}

#[server(DeleteTask)]
pub async fn delete_task(id: u32) -> Result<(), ServerFnError> {
    use crate::state::AppState;
    use axum::Extension;
    use chrono::Utc;
    use leptos_axum::extract;

    let Extension(state): Extension<AppState> = extract().await?;

    let mut tasks = state.tasks.write().unwrap();
    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| ServerFnError::new("Task not found"))?;

    let now = Utc::now();
    task.deleted_at = Some(now);

    Ok(())
}
