# Phase 0 Research: Unified Printable Catalog Exporter (PDF, TXT, RTF)

**Feature**: `specs/011-pdf-catalog-exporter/spec.md`  
**Date**: 2026-11-22  
**Status**: Completed

## 1. Toolchain & PDF Engine Evaluation

### Decision
Use pure-Rust `krilla` v0.8.2 (with features `raster-images` and `simple-text`) as the primary PDF document engine.

### Rationale
- Local toolchain is `rustc 1.98.0` (2026 edition compatible), which cleanly satisfies `krilla` v0.8.2 requirements.
- `krilla` is built on top of low-level `pdf-writer`. It provides higher-level page layout, surface rendering, font embedding/subsetting, and raster image compositing without needing manual low-level PDF stream orchestration.
- Zero C/C++ dependencies, zero SVG/`resvg` runtime overhead.
- Release binary footprint impact is minimal (~+200 KB to +350 KB), keeping the final executable comfortably under the Constitution Principle I limit of `< 20 MB` (~8.2 MB estimated release binary).

### Alternatives Considered
- `pdf-writer` directly (raw streams): Extremely lightweight, but requires hand-crafting Type 1 / TrueType font subsetting tables, text encoding operators (`Tf`, `Tj`, `Tm`), and image XObject dictionaries manually. Unnecessary complexity when `krilla` provides clean, tested abstractions.
- `printpdf`: Older, less ergonomic API with stricter font format constraints and heavier transitive dependencies.
- `genpdf` / `headless_chrome`: Rejected due to heavy runtime dependencies and Chromium process spawning violating Constitution Principle I.

---

## 2. Cross-Platform Embedded Typography

### Decision
Bundle `Roboto-Regular.ttf` and `Roboto-Bold.ttf` (Apache-2.0 license) directly into the binary under `assets/fonts/` via `include_bytes!`.

### Rationale
- Guaranteed 100% visual consistency and identical font metric calculations across Linux, Windows, and macOS.
- Eliminates any dependency on host-installed system fonts or fontconfig.
- `Roboto` provides clean, modern sans-serif legibility at small sizes (down to 5.0 pt) with extensive character coverage and permissive Apache-2.0 licensing.
- Size footprint: ~330 KB total for both font files compressed into binary.

### Alternatives Considered
- Standard PDF Type 1 fonts (`Helvetica` / `Times-Roman`): While zero bytes in binary, PDF readers render Type 1 fonts using host substitute fonts, leading to metric inconsistencies across platforms (e.g. Foxit vs Preview vs Adobe Acrobat vs Linux Evince) and non-standard Unicode handling.
- System font discovery (`font-kit`): Rejected because target OSes have completely different font installations, breaking reproducible print scaling.

---

## 3. Physical Print Dimension Conversions

### Decision
Implement exact physical unit conversions in a dedicated `PhysicalDimensions` model:
- `1 inch = 72.0 pt`
- `1 cm = 28.3464567 pt` (`72.0 / 2.54`)
- `1 mm = 2.83464567 pt` (`72.0 / 25.4`)

Presets:
- `10×10 cm Card`: 10.0 cm × 10.0 cm (`283.46 × 283.46 pt`)
- `12×12 cm CD Cover`: 12.0 cm × 12.0 cm (`340.16 × 340.16 pt`)
- `A6`: 10.5 cm × 14.8 cm (`297.64 × 419.53 pt`)
- `A5`: 14.8 cm × 21.0 cm (`419.53 × 595.28 pt`)
- `A4`: 21.0 cm × 29.7 cm (`595.28 × 841.89 pt`)
- `Custom`: Width and height adjustable from minimum `1.0 × 1.0 cm` (`28.35 × 28.35 pt`) upwards.

Margin:
- Default: `0.5 cm` (`14.17 pt`). Configurable in active unit down to `0.0 cm`.

---

## 4. Smart Multi-Column Autofit Algorithm

### Decision
Pure, deterministic layout solver `CatalogLayoutSolver`:
1. Calculate printable canvas: `canvas_width = width_pt - 2 * margin_pt`, `canvas_height = height_pt - 2 * margin_pt`.
2. Header allocation: If `header_title` is enabled, reserve header height (`header_font_size * 1.5 + spacing`).
3. For candidate column counts $C \in \{1, 2, 3, 4, 5\}$:
   - Column width $W_c = \frac{canvas\_width - (C - 1) \times column\_gap}{C}$. If $W_c < 50\text{ pt}$, reject candidate.
   - Rows required: $R = \lceil \frac{total\_tracks}{C} \rceil$.
   - Available height per row: $H_{row} = \frac{available\_height}{R}$.
   - Computed font size: $S = \min(theme.font\_size, \frac{H_{row}}{theme.line\_height\_ratio})$.
   - Score candidate based on readability: maximize font size up to theme ceiling, penalize excessive columns.
4. If best computed font size $S < 5.0\text{ pt}$, flag `readability_warning = true` while preserving layout.
5. In each column row, format text line using enabled field tokens:
   - e.g. `[#] [Title] - [Artist]` or `[#] [Title] ([XXYY-filename.mp3])`.
   - Truncate text cleanly with `...` based on font glyph advances to strictly stay within $W_c$.

---

## 5. JSON Theme Architecture

### Decision
Define `CatalogTheme` with `serde::{Serialize, Deserialize}` and bundle starter themes under `assets/pdf_themes/` using `include_str!`:
- `monochrome_white.json` (Default fallback: clean white background, black typography, light grey separator lines)
- `warm_paper.json` (Soft cream background `#FDFBF7`, warm dark brown text `#2C2523`, subtle borders)
- `dark_slate.json` (Dark charcoal `#1E2022`, crisp white text `#F0F2F5`, subtle grey rules)

Raster Background Image Handling:
- Embedded via `image` crate (existing dependency, PNG/JPEG).
- Supported fit modes: `cover` (scale to fill, clipping overflow) and `contain` (scale to fit within sheet).
- Opacity multiplier `0.0 ..= 1.0` applied when drawing into the PDF content stream.
- Zero SVG/vector parsing dependencies to respect binary size constraints.

---

## 6. Atomic File Output & Error Handling

### Decision
All file writes (PDF, TXT, RTF) use `tempfile::NamedTempFile::new_in(target_dir)` followed by `.persist(target_path)`.

### Rationale
- Guarantees zero partial/corrupt files if write or generation fails mid-stream.
- Temporary files reside on the same filesystem/mount point, ensuring atomic replacement via OS rename.
- Leaves zero leftover temporary files on error (satisfies Constitution Principle IV).
