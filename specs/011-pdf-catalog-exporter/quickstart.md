# Quickstart & Validation Guide: Unified Printable Catalog Exporter (PDF, TXT, RTF)

**Feature**: `specs/011-pdf-catalog-exporter/spec.md`  
**Date**: 2026-11-22

## Prerequisites
- Rust 1.85+ toolchain (`rustc`, `cargo`).
- Working directory: repository root.

---

## 1. Automated Verification Commands

Run the full testing and quality suite:

```bash
# 1. Run unit and integration tests (layout solver, unit conversions, theme parsing, PDF rendering)
cargo test --test unit_tests --test integration_tests

# 2. Run headless GUI interaction tests via egui_kittest
cargo test --test gui_tests

# 3. Verify zero compiler warnings and formatting
cargo clippy --all-targets -- -D warnings
cargo fmt --check

# 4. Release build verification and binary footprint audit (< 20 MB ceiling)
cargo build --release
ls -lh target/release/musicbox
```

---

## 2. Interactive GUI Validation Workflow

1. Start application:
   ```bash
   cargo run
   ```
2. Load sample tracks by selecting directory `mp3-sample/`.
3. In the toolbar, click **"Export Catalog..."**:
   - Verify modal opens centered and clean in dark/light mode.
   - Verify default format is **PDF (`.pdf`)**.
   - Verify default toggles: **`Track Number (#)`** is checked, **`Title`** is checked; other toggles are unchecked.
   - Verify default dimensions are **`10.0 × 10.0 cm`** (Card preset).
   - Verify live layout summary displays estimated columns and font size (e.g. `2 cols • 7.4 pt • 10.0 × 10.0 cm`).
4. Readability Warning Check:
   - Change preset to `Custom` and type `1.0` cm width and `1.0` cm height.
   - Verify a bright **red warning** appears: `"Warning: font may not be readable to the naked eye"`.
   - Verify the "Export" button remains enabled and clickable.
5. Export PDF:
   - Reset size to `10.0 × 10.0 cm` Card, select `warm_paper` theme, check `Cut Border / Crop Marks`.
   - Click **"Export"** and save to `/tmp/catalog_test.pdf`.
   - Open `/tmp/catalog_test.pdf` in default PDF viewer; verify exact 10×10 cm page dimensions, clean Roboto typography, background tint, and outer border cut lines.
6. Multi-Format Verification:
   - Open dialog again, select **Plain Text (`.txt`)** and export to `/tmp/catalog_test.txt`.
   - Select **Rich Text (`.rtf`)** and export to `/tmp/catalog_test.rtf`.
   - Verify only enabled fields appear in all three formats.

---

## 3. CLI Validation Workflow

Test command line generation:

```bash
# Export 10x10 cm PDF from mp3-sample
cargo run -- export-catalog ./mp3-sample -o /tmp/cli_catalog.pdf

# Verify PDF header magic bytes
head -c 5 /tmp/cli_catalog.pdf
# Expected output: %PDF-
```
