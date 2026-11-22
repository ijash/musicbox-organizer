# Phase 1 Data Model: Unified Printable Catalog Exporter (PDF, TXT, RTF)

**Feature**: `specs/011-pdf-catalog-exporter/spec.md`  
**Date**: 2026-11-22  
**Status**: Completed

## 1. Domain Entities & Value Objects

### 1.1 `CatalogFormat` (Enum)
Supported export formats:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CatalogFormat {
    #[default]
    Pdf,
    Txt,
    Rtf,
}
```

### 1.2 `CatalogFieldSelection` (Struct)
Universal field/column toggles shared across PDF, TXT, and RTF formats:
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogFieldSelection {
    pub track_number: bool,     // Default: true (#)
    pub title: bool,            // Default: true (Track Title)
    pub artist: bool,           // Default: false (Artist Name)
    pub export_filename: bool,   // Default: false ([XXYY-filename.mp3])
    pub header_title: bool,      // Default: false (Playlist Header / Name)
}

impl Default for CatalogFieldSelection {
    fn default() -> Self {
        Self {
            track_number: true,
            title: true,
            artist: false,
            export_filename: false,
            header_title: false,
        }
    }
}
```

### 1.3 `LengthUnit` (Enum)
Measurement unit for real-world print sizing:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LengthUnit {
    #[default]
    Cm,
    Mm,
    Inches,
}

impl LengthUnit {
    pub fn to_points(self, value: f32) -> f32 {
        match self {
            LengthUnit::Cm => value * 28.3464567,
            LengthUnit::Mm => value * 2.83464567,
            LengthUnit::Inches => value * 72.0,
        }
    }

    pub fn from_points(self, points: f32) -> f32 {
        match self {
            LengthUnit::Cm => points / 28.3464567,
            LengthUnit::Mm => points / 2.83464567,
            LengthUnit::Inches => points / 72.0,
        }
    }
}
```

### 1.4 `CatalogPagePreset` (Enum)
Physical dimension presets:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CatalogPagePreset {
    #[default]
    Card10x10,      // 10.0 x 10.0 cm
    CdCover12x12,   // 12.0 x 12.0 cm
    A6,             // 10.5 x 14.8 cm
    A5,             // 14.8 x 21.0 cm
    A4,             // 21.0 x 29.7 cm
    Custom,
}

impl CatalogPagePreset {
    pub fn dimensions_cm(self) -> (f32, f32) {
        match self {
            Self::Card10x10 => (10.0, 10.0),
            Self::CdCover12x12 => (12.0, 12.0),
            Self::A6 => (10.5, 14.8),
            Self::A5 => (14.8, 21.0),
            Self::A4 => (21.0, 29.7),
            Self::Custom => (10.0, 10.0),
        }
    }
}
```

### 1.5 `PhysicalDimensions` (Struct)
Stores dimensions and margins in points with conversions to user units:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalDimensions {
    pub unit: LengthUnit,
    pub width: f32,       // Stored in active unit (min 1.0 cm)
    pub height: f32,      // Stored in active unit (min 1.0 cm)
    pub margin: f32,      // Stored in active unit (default 0.5 cm)
    pub preset: CatalogPagePreset,
}
```

### 1.6 `CatalogTheme` (Struct)
JSON-driven theme definition for PDF catalogs:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogTheme {
    pub name: String,
    pub bg_color: [u8; 3],          // RGB
    pub font_color: [u8; 3],        // RGB
    pub accent_color: [u8; 3],      // RGB
    pub border_color: [u8; 3],      // RGB
    pub bg_image: Option<String>,   // Optional relative path or base64 raster
    pub bg_opacity: f32,            // 0.0 ..= 1.0
    pub bg_fit: ImageFitMode,       // Cover | Contain
    pub font_family: String,        // "Roboto"
    pub font_size: f32,             // Base font size in pt (default: 8.0)
    pub header_font_size: f32,      // Header font size in pt (default: 11.0)
    pub line_height: f32,           // Line height ratio (default: 1.3)
    pub column_gap: f32,            // Gap between columns in pt (default: 10.0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ImageFitMode {
    #[default]
    Cover,
    Contain,
}
```

### 1.7 `CatalogExportConfig` (Struct)
Root configuration struct integrating all export parameters:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogExportConfig {
    pub format: CatalogFormat,
    pub fields: CatalogFieldSelection,
    pub dimensions: PhysicalDimensions,
    pub cut_border: bool,
    pub theme_name: String,
}
```

### 1.8 `ComputedCatalogLayout` (Struct)
Solver output for rendering and live GUI feedback:
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ComputedCatalogLayout {
    pub column_count: usize,
    pub column_width_pt: f32,
    pub row_height_pt: f32,
    pub computed_font_size_pt: f32,
    pub total_pages: usize,
    pub readability_warning: bool, // true if computed_font_size_pt < 5.0 pt
    pub physical_width_cm: f32,
    pub physical_height_cm: f32,
}
```

---

## 2. Validation & Boundary Invariants

- **Minimum Dimensions**: Width and height cannot be less than `1.0 cm` (or equivalent in mm/in: `10.0 mm`, `0.3937 in`). Values below this are clamped to `1.0 cm`.
- **Minimum Margin**: Margins cannot be negative and cannot exceed `width / 2.0` or `height / 2.0`. Clamped to `[0.0, min(width, height) / 2.0 - 0.2]`.
- **Readability Threshold**: If computed track font size drops below `5.0 pt`, `readability_warning` is activated to alert the user without blocking export.
- **Empty Playlist**: Export action is disabled if the playlist contains 0 tracks.
- **Atomic Persistence**: Files are written via temporary files before renaming; target file remains untouched if any error occurs.
