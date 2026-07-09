//! Persisted application settings — `settings.json` under the `fla-dynoview`
//! data folder. Currently the UI language and the unit system.

use crate::error::CommandError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// Locale code, e.g. `"en"`, `"de"`, `"fi"`.
    pub language: String,
    /// `"metric"` (kW / Nm / °C / hPa) or `"imperial"` (bhp / lb·ft / °F / mbar).
    pub unit_system: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            language: "en".into(),
            unit_system: "metric".into(),
        }
    }
}

fn settings_path(root: &Path) -> PathBuf {
    root.join("settings.json")
}

/// Load settings, falling back to defaults if the file is missing/invalid.
pub fn load(root: &Path) -> Settings {
    fs::read_to_string(settings_path(root))
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

/// Persist settings (creates the folder if needed).
pub fn save(root: &Path, s: &Settings) -> Result<(), CommandError> {
    fs::create_dir_all(root).map_err(|e| CommandError::Io(format!("create settings dir: {e}")))?;
    let json = serde_json::to_string_pretty(s).map_err(|e| CommandError::Other(e.to_string()))?;
    fs::write(settings_path(root), json)
        .map_err(|e| CommandError::Io(format!("write settings: {e}")))
}
