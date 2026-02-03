use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://taskapp:taskapp@localhost:5432/taskapp".to_string()
        });

    let db = Database::connect(&database_url).await?;

    // Create the tasks table if it doesn't exist
    db.execute(Statement::from_string(
        db.get_database_backend(),
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id SERIAL PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            description TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            completed_at TIMESTAMPTZ,
            deleted_at TIMESTAMPTZ
        )
        "#.to_string(),
    ))
    .await?;

    Ok(db)
}
