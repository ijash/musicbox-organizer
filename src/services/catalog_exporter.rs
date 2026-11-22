use crate::models::{CatalogExportConfig, CatalogFormat, OrganizerTrack};
use crate::services::{CatalogThemeRegistry, PdfCatalogExporter, RtfExporter};
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Unified exporter for PDF, TXT, and RTF catalogs.
/// cite: contracts/service-api-contract.md CatalogExportService
pub struct CatalogExportService {
    pdf_exporter: PdfCatalogExporter,
}

impl Default for CatalogExportService {
    fn default() -> Self {
        Self::new()
    }
}

impl CatalogExportService {
    pub fn new() -> Self {
        Self {
            pdf_exporter: PdfCatalogExporter::new(),
        }
    }

    /// Exports tracks according to CatalogExportConfig to the specified destination path.
    pub fn export_catalog(
        &self,
        dest_path: &Path,
        tracks: &[OrganizerTrack],
        playlist_title: Option<&str>,
        config: &CatalogExportConfig,
    ) -> Result<(), Box<dyn Error>> {
        if tracks.is_empty() {
            return Err("Cannot export empty playlist catalog".into());
        }

        match config.format {
            CatalogFormat::Pdf => {
                let theme = CatalogThemeRegistry::get_theme(&config.theme_name);
                self.pdf_exporter
                    .export_pdf(dest_path, tracks, playlist_title, config, &theme)
            }
            CatalogFormat::Txt => self.export_txt(dest_path, tracks, playlist_title, config),
            CatalogFormat::Rtf => self.export_rtf(dest_path, tracks, playlist_title, config),
        }
    }

    fn export_txt(
        &self,
        dest_path: &Path,
        tracks: &[OrganizerTrack],
        playlist_title: Option<&str>,
        config: &CatalogExportConfig,
    ) -> Result<(), Box<dyn Error>> {
        let parent_dir = dest_path.parent().unwrap_or_else(|| Path::new("."));
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }

        let mut lines = Vec::new();
        if config.fields.header_title {
            lines.push(format!(
                "=== {} ===",
                playlist_title.unwrap_or("Playlist Catalog")
            ));
            lines.push(String::new());
        }

        for (idx, t) in tracks.iter().enumerate() {
            let mut parts = Vec::new();
            if config.fields.track_number {
                parts.push(format!("{:02}.", idx + 1));
            }
            if config.fields.title {
                let title = if !t.metadata.title.trim().is_empty() {
                    t.metadata.title.as_str()
                } else {
                    t.original_filename.as_str()
                };
                parts.push(title.to_string());
            }
            if config.fields.artist && !t.metadata.artist.trim().is_empty() {
                parts.push(format!("- {}", t.metadata.artist));
            }
            if config.fields.export_filename {
                parts.push(format!("({})", t.formatted_filename()));
            }

            if parts.is_empty() {
                lines.push(format!("{:02}. Track {}", idx + 1, idx + 1));
            } else {
                lines.push(parts.join(" "));
            }
        }

        let content = lines.join("\n");
        let mut temp_file = tempfile::NamedTempFile::new_in(parent_dir)?;
        temp_file.write_all(content.as_bytes())?;
        temp_file.persist(dest_path)?;

        Ok(())
    }

    fn export_rtf(
        &self,
        dest_path: &Path,
        tracks: &[OrganizerTrack],
        playlist_title: Option<&str>,
        config: &CatalogExportConfig,
    ) -> Result<(), Box<dyn Error>> {
        let parent_dir = dest_path.parent().unwrap_or_else(|| Path::new("."));
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir)?;
        }

        let title = playlist_title.unwrap_or("Playlist Catalog");
        let content = RtfExporter::generate_rtf_with_fields(title, tracks, &config.fields);

        let mut temp_file = tempfile::NamedTempFile::new_in(parent_dir)?;
        temp_file.write_all(content.as_bytes())?;
        temp_file.persist(dest_path)?;

        Ok(())
    }
}
