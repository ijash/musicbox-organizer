use crate::models::catalog_config::CatalogExportConfig;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const CONFIG_FILE_NAME: &str = ".musicbox_config.json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportFormatSelection {
    pub txt: bool,
    pub rtf: bool,
}

impl Default for ExportFormatSelection {
    fn default() -> Self {
        Self {
            txt: true,
            rtf: true,
        }
    }
}

impl ExportFormatSelection {
    pub fn is_valid(&self) -> bool {
        self.txt || self.rtf
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPreferences {
    pub dark_mode: bool,
    pub formats: ExportFormatSelection,
    #[serde(default)]
    pub catalog_config: CatalogExportConfig,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            dark_mode: true, // Dark by default per FR-001
            formats: ExportFormatSelection::default(),
            catalog_config: CatalogExportConfig::default(),
        }
    }
}

impl UserPreferences {
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Self {
        let path = dir.as_ref().join(CONFIG_FILE_NAME);
        if path.exists()
            && let Ok(content) = fs::read_to_string(&path)
            && let Ok(prefs) = serde_json::from_str(&content)
        {
            return prefs;
        }
        Self::default()
    }

    pub fn save_to_dir<P: AsRef<Path>>(&self, dir: P) -> Result<(), String> {
        let path = dir.as_ref().join(CONFIG_FILE_NAME);
        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, content).map_err(|e| e.to_string())
    }
}
