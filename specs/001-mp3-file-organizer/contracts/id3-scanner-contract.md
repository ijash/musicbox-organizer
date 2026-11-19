# Contract: ID3 Scanner & Audio Management

## Scanner API
Scans a folder for `.mp3` files without recursing into subdirectories (per FR-001).

```rust
pub trait AudioScanner {
    fn scan_directory(&self, dir: &Path) -> Result<Vec<OrganizerTrack>, Box<dyn Error>>;
}
```

## Tag Editor API
Reads and updates ID3 tags on disk.

```rust
pub trait TagEditor {
    fn read_tags(&self, path: &Path) -> Result<TrackMetadata, Box<dyn Error>>;
    fn write_tags(&self, path: &Path, metadata: &TrackMetadata) -> Result<(), Box<dyn Error>>;
}
```

## Audio Player API
Manages preview playback for verification.

```rust
pub trait AudioPreviewPlayer {
    fn load(&mut self, path: &Path) -> Result<(), Box<dyn Error>>;
    fn play(&mut self) -> Result<(), Box<dyn Error>>;
    fn pause(&mut self);
    fn stop(&mut self);
    fn is_playing(&self) -> bool;
    fn seek(&mut self, position_secs: f64) -> Result<(), Box<dyn Error>>;
}
```
