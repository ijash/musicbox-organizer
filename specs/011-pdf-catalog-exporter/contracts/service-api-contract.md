# Contract: Catalog Export Service Public API

**Specification**: `specs/011-pdf-catalog-exporter/spec.md`  
**Date**: 2026-11-22

## Module: `crate::services::catalog_exporter`

### `PdfCatalogExporter`
Renders structured multi-column PDF documents via `krilla` and embedded Roboto fonts.

```rust
pub struct PdfCatalogExporter;

impl PdfCatalogExporter {
    pub fn new() -> Self;

    /// Renders tracks into a PDF catalog using config and theme, writing atomically to dest_path
    pub fn export_pdf(
        &self,
        dest_path: &Path,
        tracks: &[OrganizerTrack],
        playlist_title: Option<&str>,
        config: &CatalogExportConfig,
        theme: &CatalogTheme,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
```

### `CatalogLayoutSolver`
Computes optimal column layout, row dimensions, and scaled font sizes for physical sheets.

```rust
pub struct CatalogLayoutSolver;

impl CatalogLayoutSolver {
    /// Solves the column count and font size to maximize readability within physical dimensions
    pub fn solve(
        width_pt: f32,
        height_pt: f32,
        margin_pt: f32,
        track_count: usize,
        has_header: bool,
        theme: &CatalogTheme,
    ) -> ComputedCatalogLayout;
}
```

### `CatalogThemeRegistry`
Manages embedded and user-provided JSON themes.

```rust
pub struct CatalogThemeRegistry;

impl CatalogThemeRegistry {
    /// Returns list of available theme names
    pub fn available_themes() -> &'static [&'static str];

    /// Retrieves theme by name, falling back to embedded White Monochrome if unknown
    pub fn get_theme(name: &str) -> CatalogTheme;

    /// Loads custom theme from JSON string
    pub fn parse_theme_json(json_str: &str) -> Result<CatalogTheme, serde_json::Error>;
}
```

### `CatalogExportService`
Unified dispatcher across PDF, TXT, and RTF formats:

```rust
pub struct CatalogExportService;

impl CatalogExportService {
    pub fn new() -> Self;

    /// Exports tracks according to CatalogExportConfig to the specified destination path
    pub fn export_catalog(
        &self,
        dest_path: &Path,
        tracks: &[OrganizerTrack],
        playlist_title: Option<&str>,
        config: &CatalogExportConfig,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
```
