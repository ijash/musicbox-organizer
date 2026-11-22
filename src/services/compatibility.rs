use crate::models::{CompatibilityReport, DeviceProfile};
use std::error::Error;
use std::fs;
use std::path::Path;

/// Service for verifying primitive device compatibility and ASCII sort safety
/// cite: contracts/compatibility-contract.md, spec.md FR-001..FR-008
pub struct CompatibilityService {
    profile: DeviceProfile,
}

impl Default for CompatibilityService {
    fn default() -> Self {
        Self::new()
    }
}

impl CompatibilityService {
    pub fn new() -> Self {
        Self {
            profile: DeviceProfile::portable_player(),
        }
    }

    pub fn with_profile(profile: DeviceProfile) -> Self {
        Self { profile }
    }

    /// Check if filename contains only ASCII characters (FR-002)
    pub fn is_ascii_only(&self, text: &str) -> bool {
        text.is_ascii()
    }

    /// Sanitize filename by replacing non-ASCII chars with underscores or safe ASCII
    pub fn sanitize_filename(&self, raw: &str) -> String {
        raw.chars()
            .map(|ch| {
                if ch.is_ascii() && !ch.is_control() {
                    ch
                } else {
                    '_'
                }
            })
            .collect()
    }

    /// Validate list of filenames against compatibility rules
    pub fn validate_filenames(&self, filenames: &[String]) -> CompatibilityReport {
        let total_files = filenames.len();
        let mut ascii_violations = Vec::new();
        let mut length_violations = Vec::new();
        let mut format_violations = Vec::new();

        let mut sorted_copy = filenames.to_vec();
        sorted_copy.sort();

        // Verify prefix uniqueness (no sequence collisions)
        let mut seen_prefixes = std::collections::HashSet::new();
        let mut has_prefix_collision = false;
        for name in &sorted_copy {
            if name.len() >= 5
                && name.as_bytes()[..4].iter().all(|b| b.is_ascii_uppercase())
                && name.as_bytes()[4] == b'-'
            {
                let prefix = &name[..4];
                if !seen_prefixes.insert(prefix) {
                    has_prefix_collision = true;
                    break;
                }
            } else if name.len() > 5
                && name.starts_with('[')
                && let Some(hyphen) = name.find('-')
            {
                let prefix = &name[1..hyphen];
                if !seen_prefixes.insert(prefix) {
                    has_prefix_collision = true;
                    break;
                }
            }
        }

        let is_sorted = (filenames == sorted_copy.as_slice()) && !has_prefix_collision;

        for name in filenames {
            // ASCII check (FR-002)
            if !self.is_ascii_only(name) {
                ascii_violations.push(name.clone());
            }

            // Length check (FAT32 limit 255 bytes)
            if name.len() > self.profile.max_path {
                length_violations.push(name.clone());
            }

            // Pattern check XXYY-... (Constitution v1.9.0) or legacy [XXYY-...]
            let has_valid_prefix = (name.len() >= 5
                && name.as_bytes()[..4].iter().all(|b| b.is_ascii_uppercase())
                && name.as_bytes()[4] == b'-')
                || (name.starts_with('[') && name.len() > 6 && name.chars().nth(5) == Some('-'));
            let has_valid_ext = name.ends_with(".mp3") || name.ends_with(".mp3]");
            if !has_valid_prefix || !has_valid_ext {
                format_violations.push(name.clone());
            }
        }

        let overall_compatible = is_sorted
            && ascii_violations.is_empty()
            && length_violations.is_empty()
            && format_violations.is_empty();

        CompatibilityReport {
            total_files,
            is_sorted,
            ascii_violations,
            length_violations,
            format_violations,
            overall_compatible,
        }
    }

    /// Inspect a directory of audio files on disk (e.g. mounted USB/SD card)
    pub fn validate_directory(&self, dir: &Path) -> Result<CompatibilityReport, Box<dyn Error>> {
        let mut files = Vec::new();

        if !dir.is_dir() {
            return Err(format!("'{}' is not a directory", dir.display()).into());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let path_str = path.to_string_lossy();
                let is_mp3 = path_str.ends_with(".mp3")
                    || path_str.ends_with(".mp3]")
                    || path
                        .extension()
                        .is_some_and(|e| e.eq_ignore_ascii_case("mp3"));
                if is_mp3 && let Some(name) = path.file_name() {
                    files.push(name.to_string_lossy().to_string());
                }
            }
        }

        files.sort();
        Ok(self.validate_filenames(&files))
    }
}
