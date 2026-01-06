use anyhow::{Context, Result};
use dirs;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub data_path: PathBuf,
    pub subjects_path: PathBuf,
}

impl Config {
    fn new(data_path: PathBuf, subjects_path: PathBuf) -> Self {
        Self {
            data_path,
            subjects_path,
        }
    }

    pub fn load() -> Result<Self> {
        let config_file =
            Self::default_config_path().context("Failed to determine config file path")?;

        let content = fs::read_to_string(&config_file)
            .with_context(|| format!("Failed to read config file: {:?}", config_file))?;

        let mut subjects_path = None;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut parts = line.splitn(2, '=');
            let key = parts.next().unwrap().trim();
            let value = parts.next().unwrap_or("").trim();
            match key {
                "workspace" => subjects_path = Some(value.to_string()),
                _ => {}
            }
        }

        let subjects_path = subjects_path.context("Missing 'workspace' in config")?;

        let subjects_path = expand_tilde(&subjects_path)?;

        if !subjects_path.exists() {
            std::fs::create_dir_all(&subjects_path).with_context(|| {
                format!(
                    "Failed to create subjects_path directory: {:?}",
                    subjects_path
                )
            })?;
        }

        let data_path = subjects_path.join("workspace.db");

        Ok(Self::new(data_path, subjects_path))
    }

    fn default_config_path() -> Result<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            let config_dir = dirs::config_dir()
                .ok_or_else(|| anyhow::anyhow!("Не удалось найти папку конфигурации"))?;
            Ok(config_dir.join("mimir").join("config"))
        }

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            let config_dir = std::env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    let home = std::env::var("HOME").expect("Не удалось получить HOME");
                    PathBuf::from(home).join(".config")
                });

            Ok(config_dir.join("mimir").join("config"))
        }
    }
}
fn expand_tilde(path: &str) -> Result<PathBuf> {
    if path.starts_with("~/") || path == "~" {
        let home =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot determine HOME directory"))?;
        Ok(home.join(&path[2..]))
    } else {
        Ok(PathBuf::from(path))
    }
}
