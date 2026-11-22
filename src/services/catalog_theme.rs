use crate::models::CatalogTheme;
use std::collections::BTreeMap;
use std::sync::LazyLock;

/// Theme registry providing built-in JSON themes and custom theme parsing.
pub struct CatalogThemeRegistry;

static EMBEDDED_THEMES: LazyLock<BTreeMap<&'static str, CatalogTheme>> = LazyLock::new(|| {
    let mut map = BTreeMap::new();

    let white_str = include_str!("../../assets/pdf_themes/monochrome_white.json");
    if let Ok(theme) = serde_json::from_str::<CatalogTheme>(white_str) {
        map.insert("White Monochrome", theme);
    }

    let warm_str = include_str!("../../assets/pdf_themes/warm_paper.json");
    if let Ok(theme) = serde_json::from_str::<CatalogTheme>(warm_str) {
        map.insert("Warm Paper", theme);
    }

    let dark_str = include_str!("../../assets/pdf_themes/dark_slate.json");
    if let Ok(theme) = serde_json::from_str::<CatalogTheme>(dark_str) {
        map.insert("Dark Slate", theme);
    }

    map
});

impl CatalogThemeRegistry {
    /// Return the list of available built-in theme names.
    pub fn available_themes() -> Vec<&'static str> {
        EMBEDDED_THEMES.keys().copied().collect()
    }

    /// Retrieve a theme by name, falling back to White Monochrome if not found.
    pub fn get_theme(name: &str) -> CatalogTheme {
        if let Some(theme) = EMBEDDED_THEMES.get(name) {
            theme.clone()
        } else {
            CatalogTheme::default()
        }
    }

    /// Parse a custom JSON theme string.
    pub fn parse_theme_json(json_str: &str) -> Result<CatalogTheme, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}
