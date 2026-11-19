# Contract: Playlist Text Exporter

```rust
pub trait PlaylistTextExporter {
    fn export_playlist(
        &self,
        base_dir: &Path,
        selection: &ExportFormatSelection,
        project: &ProjectFile,
    ) -> Result<Vec<PathBuf>, Box<dyn Error>>;
}
```
