use crate::model::Task;
use leptos::prelude::*;

#[server(GetTasks)]
pub async fn get_tasks() -> Result<Vec<Task>, ServerFnError> {
    use crate::entity::task;
    use crate::state::AppState;
    use axum::Extension;
    use leptos_axum::extract;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let Extension(state): Extension<AppState> = extract().await?;

    let tasks = task::Entity::find()
        .filter(task::Column::DeletedAt.is_null())
        .all(&state.db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(tasks.into_iter().map(Task::from).collect())
}

#[server(CreateTask)]
pub async fn create_task(title: String, description: String) -> Result<Task, ServerFnError> {
    use crate::entity::task;
    use crate::state::AppState;
    use axum::Extension;
    use chrono::Utc;
    use leptos_axum::extract;
    use sea_orm::{ActiveModelTrait, Set};

    let Extension(state): Extension<AppState> = extract().await?;

    let now = Utc::now();
    let new_task = task::ActiveModel {
        title: Set(title),
        description: Set(description),
        created_at: Set(now),
        completed_at: Set(None),
        deleted_at: Set(None),
        ..Default::default()
    };

    let result = new_task
        .insert(&state.db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(Task::from(result))
}

#[server(CompleteTask)]
pub async fn complete_task(id: i32, completed: bool) -> Result<Task, ServerFnError> {
    use crate::entity::task;
    use crate::state::AppState;
    use axum::Extension;
    use chrono::Utc;
    use leptos_axum::extract;
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};

    let Extension(state): Extension<AppState> = extract().await?;

    let existing = task::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Task not found"))?;

    let now = Utc::now();
    let mut active: task::ActiveModel = existing.into();
    active.completed_at = Set(if completed { Some(now) } else { None });

    let result = active
        .update(&state.db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(Task::from(result))
}

#[server(DeleteTask)]
pub async fn delete_task(id: i32) -> Result<(), ServerFnError> {
    use crate::entity::task;
    use crate::state::AppState;
    use axum::Extension;
    use chrono::Utc;
    use leptos_axum::extract;
    use sea_orm::{ActiveModelTrait, EntityTrait, Set};

    let Extension(state): Extension<AppState> = extract().await?;

    let existing = task::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Task not found"))?;

    let now = Utc::now();
    let mut active: task::ActiveModel = existing.into();
    active.deleted_at = Set(Some(now));

    active
        .update(&state.db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
