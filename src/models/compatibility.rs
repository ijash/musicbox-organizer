use serde::{Deserialize, Serialize};

/// Detailed report on device compatibility and ASCII sort safety
/// cite: data-model.md CompatibilityReport
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CompatibilityReport {
    pub total_files: usize,
    pub is_sorted: bool,
    pub ascii_violations: Vec<String>,
    pub length_violations: Vec<String>,
    pub format_violations: Vec<String>,
    pub overall_compatible: bool,
}

/// Target device constraints profile
/// cite: data-model.md DeviceProfile
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceProfile {
    pub name: String,
    pub max_path: usize,
    pub max_tracks: usize,
    pub ascii_only: bool,
}

impl DeviceProfile {
    pub fn portable_player() -> Self {
        Self {
            name: "Portable MP3 Player".to_string(),
            max_path: 255,
            max_tracks: 17_576,
            ascii_only: true,
        }
    }

    pub fn car_head_unit() -> Self {
        Self {
            name: "Car Head Unit".to_string(),
            max_path: 255,
            max_tracks: 9_999,
            ascii_only: true,
        }
    }

    pub fn microcontroller() -> Self {
        Self {
            name: "Microcontroller / Embedded".to_string(),
            max_path: 128,
            max_tracks: 1_000,
            ascii_only: true,
        }
    }
}
