//! Persisted application settings — `settings.json` under the `fla-dynoview`
//! data folder. Currently the UI language and the unit system.

use crate::error::CommandError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Which curves are drawn in the preview/print chart. All channel data is always
/// saved regardless; this only controls visibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurveVisibility {
    pub engine: bool,
    pub wheel: bool,
    pub loss: bool,
    pub torque: bool,
}

impl Default for CurveVisibility {
    fn default() -> Self {
        // Classic dyno view: engine power + torque. Wheel/loss are opt-in.
        CurveVisibility {
            engine: true,
            wheel: false,
            loss: false,
            torque: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// Locale code, e.g. `"en"`, `"de"`, `"fi"`.
    pub language: String,
    /// `"metric"` (kW / Nm / °C / hPa) or `"imperial"` (bhp / lb·ft / °F / mbar).
    pub unit_system: String,
    /// User overrides for field captions, keyed by i18n label key (e.g.
    /// `"abbr.pmax"` → `"DIN 70020 corrected engine power"`). Empty = use default.
    #[serde(default)]
    pub label_overrides: BTreeMap<String, String>,
    /// Which curves to draw (remembered globally).
    #[serde(default)]
    pub curve_visibility: CurveVisibility,
    /// Optional logo shown in the print header, stored as a `data:` URI so it is
    /// self-contained across machines.
    #[serde(default)]
    pub logo_data_uri: Option<String>,
    /// Optional custom print-header text; overrides the shop name on the printout
    /// when set (leaves the name field free for other text).
    #[serde(default)]
    pub print_header_text: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            language: "en".into(),
            unit_system: "metric".into(),
            label_overrides: BTreeMap::new(),
            curve_visibility: CurveVisibility::default(),
            logo_data_uri: None,
            print_header_text: None,
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
