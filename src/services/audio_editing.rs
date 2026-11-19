use std::error::Error;
use std::path::Path;

/// Extensibility trait for future audio DSP editing features
/// cite: spec.md FR-011, FR-012, FR-013 (architecture placeholder for v2+)
pub trait AudioProcessor {
    fn normalize_volume(&self, path: &Path, target_db: f32) -> Result<(), Box<dyn Error>>;
    fn trim_silence(&self, path: &Path, threshold_db: f32) -> Result<(), Box<dyn Error>>;
    fn is_available(&self) -> bool {
        false // Deferred to v2 per spec
    }
}

pub struct PlaceholderAudioProcessor;

impl AudioProcessor for PlaceholderAudioProcessor {
    fn normalize_volume(&self, _path: &Path, _target_db: f32) -> Result<(), Box<dyn Error>> {
        Err("Audio editing features are coming in v2".into())
    }

    fn trim_silence(&self, _path: &Path, _threshold_db: f32) -> Result<(), Box<dyn Error>> {
        Err("Audio editing features are coming in v2".into())
    }
}
