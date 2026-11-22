---
description: "Task list for Feature 011: Unified Printable Catalog Exporter (PDF, TXT, RTF)"
---

# Tasks: Unified Printable Catalog Exporter (PDF, TXT, RTF)

**Input**: Design documents from `specs/011-pdf-catalog-exporter/`  
**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/](contracts/)  
**Branch**: `feat/011-pdf-catalog-exporter`  

## Format: `- [ ] [TaskID] [P?] [Story?] Description with file path`
- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: User story identifier ([US1], [US2], [US3], [US4])
- Every task specifies exact file paths.

---

## Phase 1: Setup & Assets Gate

**Purpose**: Dependency registration, bundled fonts, embedded starter themes, and compilation gate.

- [X] T001 Add `krilla` v0.8.2 (`raster-images`, `simple-text`) dependency to `Cargo.toml`
- [X] T002 [P] Bundle Apache-2.0 `Roboto-Regular.ttf` and `Roboto-Bold.ttf` in `assets/fonts/`
- [X] T003 [P] Create default starter JSON themes (`monochrome_white.json`, `warm_paper.json`, `dark_slate.json`) in `assets/pdf_themes/`
- [X] T004 Run `cargo check` to verify toolchain compatibility and zero warnings

---

## Phase 2: Foundational Models & Core Layout Engine

**Purpose**: Core data models, physical measurement conversions, and pure layout solver required by all export workflows.

- [X] T005 [P] Create catalog models and units (`LengthUnit`, `CatalogPagePreset`, `CatalogFormat`, `CatalogFieldSelection`, `PhysicalDimensions`, `CatalogTheme`, `CatalogExportConfig`, `ComputedCatalogLayout`) citing `data-model.md` (`CatalogFormat`, `LengthUnit`, `PhysicalDimensions.width`, `PhysicalDimensions.height`, `PhysicalDimensions.margin`, `CatalogPagePreset`, `CatalogFieldSelection.track_number`, `CatalogFieldSelection.title`, `CatalogFieldSelection.artist`, `CatalogFieldSelection.export_filename`, `CatalogFieldSelection.header_title`, `CatalogTheme.bg_color`, `CatalogTheme.font_color`, `CatalogTheme.font_size`, `CatalogTheme.bg_opacity`, `CatalogExportConfig.cut_border`) in `src/models/catalog_config.rs`
- [X] T006 [P] Update `src/models/mod.rs` and `src/models/preferences.rs` to persist `CatalogExportConfig` in `.musicbox_config.json`
- [X] T007 Implement theme registry (`CatalogThemeRegistry`) embedding starter themes via `include_str!` with fallback to hardcoded White Monochrome in `src/services/catalog_theme.rs`
- [X] T008 Implement unit tests for `LengthUnit` conversions (`cm`/`mm`/`in` <-> `pt`), presets, and theme JSON deserialization in `tests/unit/catalog_config_test.rs`
- [X] T009 Implement pure `CatalogLayoutSolver` (multi-column candidate solver, row height calculation, font scaling down to 1×1 cm, and `< 5.0 pt` readability threshold) in `src/services/catalog_layout.rs`
- [X] T010 Implement unit tests for `CatalogLayoutSolver` (single-track, 10×10 cm card, multi-column balancing, tiny 1×1 cm sheet, text truncation with `...`) in `tests/unit/catalog_layout_test.rs`

**Checkpoint**: Foundation ready. Layout math, models, and themes verified by unit tests.

---

## Phase 3: User Story 1 - Print-Ready Physical PDF Catalog with Smart Autofit (Priority: P1) 🎯 MVP

**Goal**: Generate exact-dimension physical PDF catalogs with embedded Roboto typography, multi-column layout, optional crop/cut marks, and atomic output.

**Independent Test**: Export a playlist to a 10×10 cm PDF and verify with tests that the PDF starts with `%PDF-`, matches 283.46 × 283.46 pt MediaBox, has zero leftover temp files, and formats tracks into columns without overflow.

### Tests for User Story 1
- [X] T011 [P] [US1] Create integration test verifying PDF generation, MediaBox dimensions, embedded Roboto subsetting, crop marks, and atomic `NamedTempFile` persistence in `tests/integration/pdf_export_test.rs`

### Implementation for User Story 1
- [X] T012 [US1] Implement `PdfCatalogExporter` using `krilla` v0.8.2 (`Page`, `Surface`, `Font` embedding via `include_bytes!`, text layout, column drawing, crop guidelines, and atomic file persist via `tempfile::NamedTempFile::new_in`) in `src/services/pdf_exporter.rs`
- [X] T013 [US1] Add CLI subcommand `export-catalog` / `export-pdf` supporting path, preset, width, height, unit, margin, cut-border, and output options in `src/main.rs`
- [X] T014 [US1] Register `catalog_exporter` module in `src/services/mod.rs` and `src/lib.rs`

**Checkpoint**: User Story 1 (CLI & PDF service) functional and verifiable independently.

---

## Phase 4: User Story 2 - Unified Multi-Format Catalog Export with Field Toggles (Priority: P2)

**Goal**: Consolidate PDF, Plain Text (`.txt`), and Rich Text (`.rtf`) exports under a unified interface sharing universal field toggles (`#`, `Title`, `Artist`, `Export Filename`, `Header Title`).

**Independent Test**: Select different combinations of field toggles and verify that TXT, RTF, and PDF exports all produce documents containing exactly the requested fields.

### Tests for User Story 2
- [X] T015 [P] [US2] Create integration test validating that universal field toggles apply consistently across TXT, RTF, and PDF outputs in `tests/integration/catalog_exporter_test.rs`

### Implementation for User Story 2
- [X] T016 [US2] Update `RtfExporter` in `src/services/rtf_exporter.rs` to honor `CatalogFieldSelection` (track number, title, artist, export filename, header title)
- [X] T017 [US2] Implement unified `CatalogExportService` dispatching to `PdfCatalogExporter`, `RtfExporter`, or TXT writer based on `CatalogFormat` and `CatalogFieldSelection` in `src/services/catalog_exporter.rs`

**Checkpoint**: User Story 2 complete. All 3 formats work through a single unified service.

---

## Phase 5: User Story 3 - Visual Readability Warning and Small Size Handling (Priority: P3)

**Goal**: Provide live visual feedback in the GUI showing computed columns, font size, and dimensions, surfacing a prominent red warning when font drops below 5.0 pt while keeping export functional down to 1.0 × 1.0 cm.

**Independent Test**: Open export dialog via headless `egui_kittest`, set size to 1.0 × 1.0 cm, verify the red warning appears and export remains clickable.

### Tests for User Story 3
- [X] T018 [P] [US3] Create headless GUI test with `egui_kittest` verifying live layout summary, 1.0 cm clamping, and red readability warning label display in `tests/gui/catalog_readability_warning_test.rs`

### Implementation for User Story 3
- [X] T019 [US3] Implement `ExportCatalogDialog` state and controller with live layout calculation, unit conversions, clamping to minimum 1.0 × 1.0 cm, and `< 5.0 pt` warning badge logic in `src/ui/export_catalog_dialog.rs`

**Checkpoint**: User Story 3 complete. Live feedback and warnings functional.

---

## Phase 6: User Story 4 - PDF Theming & Visual Presentation (Priority: P4)

**Goal**: Enable styled catalog printing using embedded JSON themes, supporting background colors, accent colors, and optional PNG/JPG raster background images with opacity and cover/contain fitting.

**Independent Test**: Export PDF using `warm_paper` and `dark_slate` themes; verify background colors and styling in generated documents.

### Tests for User Story 4
- [X] T020 [P] [US4] Create integration test verifying raster background image decoding (`image` crate), opacity scaling, and theme styling in PDF output in `tests/integration/pdf_theme_test.rs`

### Implementation for User Story 4
- [X] T021 [US4] Add raster background image rendering (`bg_opacity`, `bg_fit` cover/contain via `image` crate) and theme color fills to `src/services/pdf_exporter.rs`
- [X] T022 [US4] Add theme selector dropdown to `src/ui/export_catalog_dialog.rs`

**Checkpoint**: User Story 4 complete. Custom themes and raster backgrounds operational.

---

## Phase 7: GUI Integration & Repo-Wide Alignment

**Purpose**: Wire unified modal dialog into the application toolbar, update repository references from DOCX to PDF, and pass all quality gates.

- [X] T023 Replace existing text/RTF export buttons in `src/ui/toolbar.rs` with a single unified `Export Catalog...` button that opens `ExportCatalogDialog`
- [X] T024 Wire `ExportCatalogDialog` rendering into `src/ui/app.rs`, opening `rfd::FileDialog` in the current working directory (`std::env::current_dir()`)
- [X] T025 [P] Create headless GUI test via `egui_kittest` verifying unified dialog interaction (switching formats, toggles, preset changes, narrow 300×400px layout) in `tests/gui/export_catalog_dialog_test.rs`
- [X] T026 Update active DOCX references to PDF in `.specify/memory/constitution.md`, `README.md` (Roadmap & Features), and documentation
- [X] T027 Add brief deprecation note in past completed specs (`specs/001-mp3-file-organizer/`, `specs/002-playlist-export-config/`, `specs/008-audio-dsp-editing/`) that planned DOCX was superseded by Feature 011 PDF export
- [X] T028 Run full verification suite: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`, and verify release binary ceiling (`cargo build --release`, `< 20 MB`) on branch `feat/011-pdf-catalog-exporter`

---

## Dependencies & Execution Order

### Phase Dependencies
- **Phase 1 (Setup)**: No dependencies.
- **Phase 2 (Foundational)**: Depends on Phase 1 completion. Blocks all user stories.
- **Phase 3 (US1 - PDF Engine)**: Depends on Phase 2. Delivers MVP.
- **Phase 4 (US2 - Multi-Format)**: Depends on Phase 2 & Phase 3 core models.
- **Phase 5 (US3 - Readability & GUI)**: Depends on Phase 2 layout solver & Phase 3 PDF exporter.
- **Phase 6 (US4 - Theming)**: Depends on Phase 2 theme registry & Phase 3 PDF exporter.
- **Phase 7 (Integration & Gates)**: Depends on Phases 1–6 completion.

### Parallel Opportunities
- T002 and T003 can run in parallel during Setup.
- T005, T006, and T007 can proceed in parallel during Foundational.
- T011 and T012 can proceed once Foundational is complete.
- Integration tests (T015, T018, T020) can run alongside their respective feature implementation tasks.

---

## Implementation Strategy: MVP First

1. **Step 1**: Complete Phase 1 (Setup) and Phase 2 (Foundational).
2. **Step 2**: Implement Phase 3 (User Story 1 - PDF core exporter & CLI). Validate independently via `cargo test --test integration_tests`. **This is the MVP.**
3. **Step 3**: Incrementally add User Story 2 (Multi-format TXT/RTF), User Story 3 (GUI Dialog & warning), and User Story 4 (Themes).
4. **Step 4**: Finish Phase 7 (Toolbar wiring, DOCX repository-wide alignment, and release gates).
