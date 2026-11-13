use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("failed to connect to database")]
    Connection(#[from] sqlx::Error),

    #[error("failed to run migrations")]
    Migration(#[from] sqlx::migrate::MigrateError),
}
