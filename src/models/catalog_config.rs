use serde::{Deserialize, Serialize};

/// Supported catalog export file formats.
/// cite: data-model.md CatalogFormat
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CatalogFormat {
    #[default]
    Pdf,
    Txt,
    Rtf,
}

impl CatalogFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Txt => "txt",
            Self::Rtf => "rtf",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Pdf => "PDF (.pdf)",
            Self::Txt => "Plain Text (.txt)",
            Self::Rtf => "Rich Text (.rtf)",
        }
    }
}

/// Universal field toggles applicable across PDF, TXT, and RTF formats.
/// cite: data-model.md CatalogFieldSelection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatalogFieldSelection {
    pub track_number: bool,    // Default: true (#)
    pub title: bool,           // Default: true (Track Title)
    pub artist: bool,          // Default: false (Artist Name)
    pub export_filename: bool, // Default: false ([XXYY-filename.mp3])
    pub header_title: bool,    // Default: false (Playlist Header / Name)
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

impl CatalogFieldSelection {
    pub fn is_valid(&self) -> bool {
        self.track_number || self.title || self.artist || self.export_filename
    }
}

/// Physical measurement unit for real-world print sizing.
/// cite: data-model.md LengthUnit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LengthUnit {
    #[default]
    Cm,
    Mm,
    Inches,
}

impl LengthUnit {
    pub fn all() -> &'static [LengthUnit] {
        &[Self::Cm, Self::Mm, Self::Inches]
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Cm => "cm",
            Self::Mm => "mm",
            Self::Inches => "in",
        }
    }

    pub fn to_points(self, value: f32) -> f32 {
        match self {
            Self::Cm => value * 28.346_457,
            Self::Mm => value * 2.834_645_7,
            Self::Inches => value * 72.0,
        }
    }

    pub fn from_points(self, points: f32) -> f32 {
        match self {
            Self::Cm => points / 28.346_457,
            Self::Mm => points / 2.834_645_7,
            Self::Inches => points / 72.0,
        }
    }
}

/// Physical dimension presets for catalog pages.
/// cite: data-model.md CatalogPagePreset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CatalogPagePreset {
    #[default]
    Card10x10,
    CdCover12x12,
    A6,
    A5,
    A4,
    Custom,
}

impl CatalogPagePreset {
    pub fn all() -> &'static [CatalogPagePreset] {
        &[
            Self::Card10x10,
            Self::CdCover12x12,
            Self::A6,
            Self::A5,
            Self::A4,
            Self::Custom,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Card10x10 => "10×10 cm Card",
            Self::CdCover12x12 => "12×12 cm CD Cover",
            Self::A6 => "A6 (10.5×14.8 cm)",
            Self::A5 => "A5 (14.8×21.0 cm)",
            Self::A4 => "A4 (21.0×29.7 cm)",
            Self::Custom => "Custom",
        }
    }

    pub fn dimensions_cm(&self) -> (f32, f32) {
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

/// Real-world physical dimensions and margins.
/// cite: data-model.md PhysicalDimensions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalDimensions {
    pub unit: LengthUnit,
    pub width: f32,
    pub height: f32,
    pub margin: f32,
    pub preset: CatalogPagePreset,
}

impl Default for PhysicalDimensions {
    fn default() -> Self {
        Self {
            unit: LengthUnit::Cm,
            width: 10.0,
            height: 10.0,
            margin: 0.5,
            preset: CatalogPagePreset::Card10x10,
        }
    }
}

impl PhysicalDimensions {
    pub fn min_dimension_in_unit(unit: LengthUnit) -> f32 {
        match unit {
            LengthUnit::Cm => 1.0,
            LengthUnit::Mm => 10.0,
            LengthUnit::Inches => 1.0 / 2.54, // ~0.3937 in
        }
    }

    pub fn clamp_dimensions(&mut self) {
        let min = Self::min_dimension_in_unit(self.unit);
        if self.width < min {
            self.width = min;
        }
        if self.height < min {
            self.height = min;
        }
        if self.margin < 0.0 {
            self.margin = 0.0;
        }
        let max_margin = (self.width.min(self.height) / 2.0 - 0.1).max(0.0);
        if self.margin > max_margin {
            self.margin = max_margin;
        }
    }

    pub fn width_pt(&self) -> f32 {
        self.unit.to_points(self.width)
    }

    pub fn height_pt(&self) -> f32 {
        self.unit.to_points(self.height)
    }

    pub fn margin_pt(&self) -> f32 {
        self.unit.to_points(self.margin)
    }

    pub fn set_unit(&mut self, target_unit: LengthUnit) {
        if self.unit == target_unit {
            return;
        }
        let w_pt = self.width_pt();
        let h_pt = self.height_pt();
        let m_pt = self.margin_pt();

        self.unit = target_unit;
        self.width = target_unit.from_points(w_pt);
        self.height = target_unit.from_points(h_pt);
        self.margin = target_unit.from_points(m_pt);
        self.clamp_dimensions();
    }
}

/// Image fit mode for PDF background raster images.
/// cite: data-model.md ImageFitMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ImageFitMode {
    #[default]
    Cover,
    Contain,
}

/// Theme definition for PDF catalogs.
/// cite: data-model.md CatalogTheme
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogTheme {
    pub name: String,
    pub bg_color: [u8; 3],
    pub font_color: [u8; 3],
    pub accent_color: [u8; 3],
    pub border_color: [u8; 3],
    pub bg_image: Option<String>,
    pub bg_opacity: f32,
    pub bg_fit: ImageFitMode,
    pub font_family: String,
    pub font_size: f32,
    pub header_font_size: f32,
    pub line_height: f32,
    pub column_gap: f32,
}

impl Default for CatalogTheme {
    fn default() -> Self {
        Self {
            name: "White Monochrome".to_string(),
            bg_color: [255, 255, 255],
            font_color: [20, 20, 20],
            accent_color: [100, 100, 100],
            border_color: [210, 210, 210],
            bg_image: None,
            bg_opacity: 1.0,
            bg_fit: ImageFitMode::Cover,
            font_family: "Roboto".to_string(),
            font_size: 8.0,
            header_font_size: 11.0,
            line_height: 1.3,
            column_gap: 10.0,
        }
    }
}

/// Unified Catalog export configuration.
/// cite: data-model.md CatalogExportConfig
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogExportConfig {
    pub format: CatalogFormat,
    pub fields: CatalogFieldSelection,
    pub dimensions: PhysicalDimensions,
    pub cut_border: bool,
    pub theme_name: String,
}

impl Default for CatalogExportConfig {
    fn default() -> Self {
        Self {
            format: CatalogFormat::Pdf,
            fields: CatalogFieldSelection::default(),
            dimensions: PhysicalDimensions::default(),
            cut_border: false,
            theme_name: "White Monochrome".to_string(),
        }
    }
}

/// Calculated catalog layout result.
/// cite: data-model.md ComputedCatalogLayout
#[derive(Debug, Clone, PartialEq)]
pub struct ComputedCatalogLayout {
    pub column_count: usize,
    pub column_width_pt: f32,
    pub column_widths: Vec<f32>,
    pub row_height_pt: f32,
    pub computed_font_size_pt: f32,
    pub total_pages: usize,
    pub readability_warning: bool,
    pub physical_width_cm: f32,
    pub physical_height_cm: f32,
}
