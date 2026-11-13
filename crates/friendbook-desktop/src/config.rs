use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{error::ConfigError, theme::Theme};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: Theme::default(),
        }
    }
}

impl Config {
    pub fn load(&mut self, path: &Path) -> Result<(), ConfigError> {
        if !path.exists() {
            let value = toml::to_string_pretty(self)?;

            fs::write(path, value)?;
        } else {
            let data = fs::read(path)?;
            let config = toml::from_slice(&data)?;

            self.merge(config);
        }

        Ok(())
    }

    fn merge(&mut self, config: Config) {
        self.theme = config.theme;
    }
}
