use crate::core::PackageManagerKind;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub default_manager: Option<String>,
    pub auto_yes: bool,
    pub colors: bool,
    pub parallel_search: bool,
    pub show_summary: bool,
    pub full_log: bool,
    pub search_order: Vec<String>,
    pub aliases: std::collections::HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "kawaii".to_string(),
            default_manager: None,
            auto_yes: false,
            colors: true,
            parallel_search: true,
            show_summary: true,
            full_log: false,
            search_order: PackageManagerKind::all()
                .iter()
                .map(|k| k.to_string())
                .collect(),
            aliases: std::collections::HashMap::new(),
        }
    }
}

impl Config {
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("kawaii").join("config.toml"))
    }

    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            return Self::default();
        };
        match std::fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let Some(path) = Self::config_path() else {
            anyhow::bail!("Could not determine config directory");
        };
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(&path, toml::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn ensure_exists(&self) -> PathBuf {
        let path = Self::config_path().expect("Could not determine config dir");
        if !path.exists() {
            let _ = std::fs::create_dir_all(path.parent().unwrap());
            let _ = std::fs::write(&path, toml::to_string_pretty(self).unwrap());
        }
        path
    }

    pub fn open_in_editor(&self) -> anyhow::Result<()> {
        let path = self.ensure_exists();

        let editor = std::env::var("EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .unwrap_or_else(|_| "nano".to_string());

        let status = std::process::Command::new(&editor)
            .arg(&path)
            .status()?;

        if !status.success() {
            anyhow::bail!("Editor '{editor}' exited with status {status}");
        }

        Ok(())
    }
}
