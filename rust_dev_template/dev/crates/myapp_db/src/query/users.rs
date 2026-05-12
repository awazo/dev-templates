use chrono::NaiveDateTime;
use serde::Serialize;
use tracing::instrument;

use crate::db::Db;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

impl User {
    #[instrument(skip(db))]
    pub async fn select_all(db: &Db) -> Vec<User> {
        let query = sqlx::query_as::<_, User>(
            r#"
                SELECT id, name, created_at
                FROM users
                ORDER BY created_at DESC, id ASC
            "#,
        );
        query.fetch_all(&db.pool).await.unwrap_or_default()
    }
}
