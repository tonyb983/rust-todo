use config::{Config, ConfigError, Environment, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{env, fmt::Debug, path::PathBuf};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppSettings {
    config_dir: Option<String>,
    data_dir: Option<String>,
    use_backup: bool,
    use_service: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        let my_dir = ProjectDirs::from("io", "imtony", "ThingsTodo");

        Self {
            config_dir: my_dir
                .as_ref()
                .map(|pd| pd.config_dir())
                .and_then(|p| p.to_str())
                .map(|st| st.to_owned()),
            data_dir: my_dir
                .as_ref()
                .map(|pd| pd.data_dir())
                .and_then(|p| p.to_str())
                .map(|st| st.to_owned()),
            use_backup: true,
            use_service: false,
        }
    }
}
