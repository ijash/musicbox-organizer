# Contract: Audio DSP Service & UI Integration

## 1. Core Service Contract (`src/services/audio_editing.rs`)

```rust
use std::error::Error;
use std::path::Path;

pub trait AudioProcessor: Send + Sync {
    fn normalize_volume(&self, path: &Path, target_db: f32) -> Result<(), Box<dyn Error>>;
    fn trim_silence(&self, path: &Path, threshold_db: f32) -> Result<(), Box<dyn Error>>;
    fn is_available(&self) -> bool;
}

pub struct NativeAudioProcessor;

impl NativeAudioProcessor {
    pub fn new() -> Self;
    pub fn process_track(
        &self,
        path: &Path,
        trim: bool,
        trim_threshold_db: f32,
        normalize: bool,
        target_db: f32,
    ) -> Result<bool, Box<dyn Error>>;
}

impl AudioProcessor for NativeAudioProcessor {
    fn normalize_volume(&self, path: &Path, target_db: f32) -> Result<(), Box<dyn Error>>;
    fn trim_silence(&self, path: &Path, threshold_db: f32) -> Result<(), Box<dyn Error>>;
    fn is_available(&self) -> bool { true }
}

pub struct PlaceholderAudioProcessor;
impl AudioProcessor for PlaceholderAudioProcessor { ... }
```

## 2. DSP Dialog & UI Contract (`src/ui/components/playlist_table.rs` & `src/ui/icons.rs`)

```rust
// In src/ui/icons.rs
pub enum AppIcon {
    ...
    AudioTools,
}

// In src/ui/components/playlist_table.rs
pub fn show_dsp_dialog(
    parent: &gtk::Window,
    selected_paths: Vec<std::path::PathBuf>,
    on_complete: impl Fn() + 'static,
);
```
