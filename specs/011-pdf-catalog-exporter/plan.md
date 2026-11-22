# Implementation Plan: Unified Printable Catalog Exporter (PDF, TXT, RTF)

**Branch**: `feat/011-pdf-catalog-exporter` | **Date**: 2026-11-22 | **Spec**: [specs/011-pdf-catalog-exporter/spec.md](spec.md)

**Input**: Feature specification from `specs/011-pdf-catalog-exporter/spec.md`

## Summary

Implement a unified printable playlist catalog export subsystem supporting PDF, Plain Text, and Rich Text with custom physical sizing, smart multi-column autofit layout, bundled Roboto typography, and JSON-driven styling themes. Supersedes all planned DOCX export references repository-wide.

Key implementation components:
1. **Repository-Wide Alignment**: Update `.specify/memory/constitution.md`, `README.md`, specs, and code to supersede DOCX with PDF catalog export.
2. **Pure-Rust PDF Engine**: Integrate `krilla` v0.8.2 (`pdf-writer` backend) and bundle Apache-2.0 `Roboto-Regular.ttf` and `Roboto-Bold.ttf` under `assets/fonts/` for cross-platform visual consistency.
3. **Smart Autofit Layout Engine (`src/services/catalog_layout.rs`)**: Solve optimal column count (1 to 5) and font sizing to fit playlists on exact physical print dimensions (`10×10 cm`, `12×12 cm`, `A6`, `A5`, `A4`, down to `1×1 cm`).
4. **JSON Theme System (`assets/pdf_themes/`)**: Embedded starter themes (`monochrome_white`, `warm_paper`, `dark_slate`) with background colors, borders, and raster image compositing.
5. **Unified Export Modal Dialog (`src/ui/export_catalog_dialog.rs`)**: Responsive modal dialog (`300×400px` safe) with format switching, universal field toggles, physical dimension controls, live layout feedback, and red readability warning below 5.0 pt.
6. **CLI & Testing Gates**: CLI command `export-catalog` / `export-pdf`, unit tests, integration tests, and headless `egui_kittest` verification under `cargo test`.

## Technical Context

**Language/Version**: Rust 1.85+ (tested on rustc 1.98.0, Edition 2024)

**Primary Dependencies**:
- `krilla` v0.8.2 (pure-Rust PDF generation on top of `pdf-writer`, features: `raster-images`, `simple-text`)
- `serde`, `serde_json` (theme serialization)
- `image` (existing dependency, PNG raster rendering)
- `tempfile` (existing dependency, atomic file replacement)
- `eframe`, `egui`, `rfd` (existing GUI framework)

**Storage**: Local filesystem only; zero database or network dependencies. Single `.musicbox_config.json` stores user preferences.

**Testing**: `cargo test` (`unit_tests.rs`, `integration_tests.rs`, `gui_tests.rs` with `egui_kittest`).

**Target Platform**: Windows, Linux, macOS.

**Project Type**: Desktop GUI + CLI application.

**Performance Goals**:
- PDF document generation under 100 ms for 100 tracks.
- Live GUI layout calculation under 50 ms per slider/input edit.

**Constraints**:
- Standalone release binary ceiling strictly `< 20 MB` (Principle I).
- Zero C dependencies, zero external rendering processes, zero temporary files left on disk (Principle IV).
- Native cross-platform GUI with `eframe`/`egui` + `rfd` (Principle II).

**Scale/Scope**: Playlists from 1 to 500+ tracks; physical sheet dimensions from 1.0 × 1.0 cm to A4.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle / Gate | Assessment | Status |
|---|---|---|
| **I. Lightweight First** | `krilla` + bundled Roboto TTF adds ~500 KB to release binary. Final binary remains ~8.3 MB, well under the `< 20 MB` ceiling. | **PASS** |
| **II. Cross-Platform Native** | Pure-Rust implementation with bundled fonts; compiles and renders identically on Linux, Windows, macOS. | **PASS** |
| **III. User-Centric Simplicity** | Single "Export Catalog..." dialog replaces fragmented export buttons. Clear defaults (# and Title checked, 10x10cm preset). | **PASS** |
| **IV. Offline-First & Portable** | 100% offline; atomic write via `tempfile::NamedTempFile` ensures zero temp files left on disk. | **PASS** |
| **V. Testable Core Logic & GUI** | Pure layout solver tested via unit tests; headless GUI tested via `egui_kittest`. | **PASS** |
| **VI. Git-First & Conventional Commits** | Dedicated branch `feat/011-pdf-catalog-exporter` active off `Development`. Conventional Commits v1.0.0. No auto-merge without approval. | **PASS** |
| **VII. Rapid Rebuild** | `cargo check` clean in ~1.7s. Binary rebuild verified upon completion. | **PASS** |
| **VIII. DRY & KISS** | Universal field selection shared between PDF, TXT, and RTF. Minimalist theme schema without unnecessary abstraction layers. | **PASS** |
| **IX. Device Compatibility** | Sequence numbering [XXYY-filename.mp3] preserved and selectable. | **PASS** |

## Project Structure

### Documentation (this feature)

```text
specs/011-pdf-catalog-exporter/
├── spec.md              # Feature specification
├── plan.md              # This implementation plan
├── research.md          # Phase 0 research findings
├── data-model.md        # Phase 1 data models and domain entities
├── quickstart.md        # Phase 1 quickstart & validation workflow
├── contracts/           # Phase 1 interface contracts
│   ├── cli-contract.md
│   └── service-api-contract.md
├── checklists/
│   └── requirements.md  # Specification quality checklist
└── tasks.md             # Phase 2 output (/speckit-tasks command)
```

### Source Code & Asset Layout

```text
assets/
├── fonts/
│   ├── Roboto-Regular.ttf
│   └── Roboto-Bold.ttf
└── pdf_themes/
    ├── monochrome_white.json
    ├── warm_paper.json
    └── dark_slate.json

src/
├── models/
│   ├── catalog_config.rs   # Format, LengthUnit, Presets, FieldSelection, Theme, Dimensions
│   ├── mod.rs
│   └── preferences.rs      # UserPreferences updated for catalog export
├── services/
│   ├── catalog_exporter.rs # Unified CatalogExportService (PDF, TXT, RTF)
│   ├── catalog_layout.rs   # Pure CatalogLayoutSolver (multi-column autofit & font scaling)
│   ├── pdf_exporter.rs     # Krilla-based PDF rendering engine
│   ├── rtf_exporter.rs     # Updated with universal field toggles
│   └── mod.rs
├── ui/
│   ├── export_dialog.rs    # Media export dialog (renamed or decoupled)
│   ├── export_catalog_dialog.rs # Unified Catalog Export modal dialog
│   ├── toolbar.rs          # Single "Export Catalog..." toolbar button
│   └── app.rs
└── main.rs                 # CLI export-catalog / export-pdf subcommand handling

tests/
├── unit/
│   ├── catalog_layout_test.rs
│   ├── length_unit_test.rs
│   └── theme_test.rs
├── integration/
│   └── pdf_export_test.rs
└── gui/
    └── export_catalog_dialog_test.rs
```

**Structure Decision**: Add assets under `assets/fonts/` and `assets/pdf_themes/`. Group catalog export logic into `src/services/` and `src/models/catalog_config.rs`. Expose the unified UI modal in `src/ui/export_catalog_dialog.rs`.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|---|---|---|
| None | All principles satisfied within constraints. | N/A |
