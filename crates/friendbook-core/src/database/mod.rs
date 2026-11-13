use sqlx::{SqlitePool, migrate};

use crate::error::DatabaseError;

pub async fn open(path: &str) -> Result<SqlitePool, DatabaseError> {
    let pool = SqlitePool::connect(path).await?;

    migrate!().run(&pool).await?;

    Ok(pool)
}
