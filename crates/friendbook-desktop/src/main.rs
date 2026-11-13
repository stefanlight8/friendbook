use tracing::{Level, debug};

use crate::{config::Config, dirs::Dirs};

mod config;
mod dirs;
mod error;
mod gui;
mod theme;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let dirs = Dirs::init()?;
    debug!(
        config_dir = %dirs.config_dir.display(),
        data_dir = %dirs.data_dir.display(),
        cache_dir = %dirs.cache_dir.display(),
        "resolved directories for the current runtime"
    );

    let mut config = Config::default();

    let error = config.load(&dirs.config_dir.join("config.toml"));

    Ok(())
}
