use thiserror::Error;

#[derive(Debug, Error)]
pub enum DirsError {
    #[error("failed to resolve user directory")]
    ResolveFailed,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error(transparent)]
    Serialization(#[from] toml::ser::Error),
    #[error(transparent)]
    Deserialization(#[from] toml::de::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
