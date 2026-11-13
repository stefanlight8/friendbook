use std::{fs, path::PathBuf};

use crate::error::DirsError;

#[derive(Debug)]
pub struct Dirs {
    pub(crate) config_dir: PathBuf,
    pub(crate) data_dir: PathBuf,
    pub(crate) cache_dir: PathBuf,
}

impl Dirs {
    #[cfg(target_os = "linux")]
    pub fn init() -> Result<Dirs, DirsError> {
        let config_dir = dirs::config_dir()
            .ok_or(DirsError::ResolveFailed)?
            .join("friendbook");
        let data_dir = dirs::data_dir()
            .ok_or(DirsError::ResolveFailed)?
            .join("friendbook");
        let cache_dir = dirs::cache_dir()
            .ok_or(DirsError::ResolveFailed)?
            .join("friendbook");

        let _ = fs::create_dir(&config_dir);
        let _ = fs::create_dir(&data_dir);
        let _ = fs::create_dir(&cache_dir);

        Ok(Dirs {
            config_dir,
            data_dir,
            cache_dir,
        })
    }

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    pub fn init() -> Result<Dirs, DirsError> {
        let application_dir = dirs::data_dir()
            .ok_or(DirsError::ResolveFailed)?
            .join("Friendbook");
        let cache_dir = dirs::cache_dir()
            .ok_or(DirsError::ResolveFailed)?
            .join("Friendbook");

        let _ = fs::create_dir(&application_dir);
        let _ = fs::create_dir(&cache_dir);

        Ok(Dirs {
            config_dir: application_dir.clone(),
            data_dir: application_dir,
            cache_dir,
        })
    }

    fn media(&self) -> PathBuf {
        self.data_dir.join("media")
    }
}
