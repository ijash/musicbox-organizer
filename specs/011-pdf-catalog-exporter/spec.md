# Feature Specification: Unified Printable Catalog Exporter (PDF, TXT, RTF)

**Feature Branch**: `feat/011-pdf-catalog-exporter`

**Created**: 2026-11-22

**Status**: Draft

**Input**: User description: "Implement Feature 011: Unified Printable Catalog Exporter (PDF, TXT, RTF) with custom physical print sizing, smart multi-column autofit, embedded Roboto typography, and JSON-driven PDF themes — superseding the previously planned DOCX catalog exporter across the entire repository."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Export Print-Ready Physical PDF Catalog with Smart Autofit (Priority: P1)

A user has organized a playlist and wants to print a compact physical tracklist card (e.g., 10×10 cm card for a small music box, or 12×12 cm CD jewel case sleeve). The user opens the unified catalog export dialog (which renders in a content-driven auto-height OS child viewport with zero vertical scrollbars), chooses PDF, selects physical dimensions and units (or picks a preset), and exports. The layout solver measures exact OpenType glyph advance widths from the embedded Roboto font, couples header metrics between solver and renderer, and distributes horizontal space into inter-column gaps so the rightmost column flushes to the right margin with zero overlap or clipping.

**Why this priority**: Core value of the feature. Solves physical printing and tracklist insertion for hardware music boxes and CD cases with exact real-world dimensions.

**Independent Test**: Can be tested independently by exporting a 20-track playlist to a 10×10 cm PDF file and verifying physical page dimensions (283.46 × 283.46 pt) and complete track visibility with correct column layout.

**Acceptance Scenarios**:

1. **Given** an organized playlist of tracks, **When** the user clicks "Export Catalog...", selects PDF format, leaves default size 10×10 cm, and confirms export, **Then** a valid PDF file matching exact 10×10 cm dimensions is produced containing all tracks formatted into readable columns with page margins.
2. **Given** a user selecting physical size units (`cm`, `mm`, `in`), **When** the user switches units, **Then** width, height, and margin numeric inputs immediately convert accurately preserving physical scale (`1 in = 72 pt`, `1 cm = 28.3465 pt`, `1 mm = 2.83465 pt`). Margin honors down to `0.00 cm` (with an internal safety inset applied only when cut border is drawn).
3. **Given** a large playlist (e.g., 40 tracks) on a small physical dimension, **When** the layout engine computes the sheet, **Then** it measures exact glyph advance widths from the embedded `Roboto-Regular.ttf` font (`cmap`+`hmtx`), couples header metrics directly, balances track columns, and scales font size so all tracks fit with zero clipping, truncating only as an absolute last resort if font size < 1.0 pt.
4. **Given** cut borders enabled, **When** the PDF is generated, **Then** outer dashed cut guidelines (`0.75 pt` width, `[4.0, 2.5]` dash pattern, `0.5 pt` edge inset) are rendered with high contrast against the theme background to guide physical trimming without overlapping text.

---

### User Story 2 - Unified Multi-Format Catalog Export with Field Toggles (Priority: P2)

A user wants to export their playlist as plain text (`.txt`), rich text (`.rtf`), or printable document (`.pdf`) using a single, consolidated dialog. The dialog runs in a content-driven auto-height OS child viewport that dynamically resizes without vertical scrollbars. The user chooses which fields appear in the output (Track Number, Title, Artist, Export Filename, Header Title) and can edit the header title text directly.

**Why this priority**: Eliminates disparate export buttons and fragmented configurations; unifies catalog export into one predictable user experience across all supported formats.

**Independent Test**: Can be tested independently by opening the export dialog, switching between PDF, TXT, and RTF, toggling fields (e.g., disabling Track Number or enabling Artist), and verifying generated files contain exactly the requested fields.

**Acceptance Scenarios**:

1. **Given** the main application window, **When** the user clicks "Export Catalog...", **Then** an auto-height OS child viewport opens displaying format options (PDF, Plain Text, Rich Text), field toggles, and dynamically resizes on format switches without vertical scrollbars.
2. **Given** field toggles with defaults (`Track Number` ON, `Title` ON, `Artist` OFF, `Export Filename` OFF, `Header Title` OFF), **When** exporting to TXT or RTF or PDF, **Then** only the enabled fields are rendered in the resulting file. When `Export Filename` is enabled, filenames follow Constitution Principle IX (`XXYY-<sanitized_id3_title>.mp3`, no brackets, lowercase underscore slug).
3. **Given** `Header Title` enabled, **When** the user checks the checkbox, **Then** an inline editable `Title Text` field appears, pre-populated with the playlist name or folder name (fallback `"Playlist Catalog"`), allowing custom heading text in the exported document.
4. **Given** user preferences stored locally, **When** the user alters format selection or field toggles, **Then** their choices persist across application restarts.

---

### User Story 3 - Visual Readability Warning and Small Size Handling (Priority: P3)

A user enters very small custom physical dimensions (e.g., down to 1.0 × 1.0 cm) or packs an excessive number of tracks onto a small card. The layout engine calculates that readable rendering requires a font smaller than 5.0 pt. The system alerts the user with a distinct real-time visual warning indicating that text may not be legible to the naked eye, but never blocks or refuses the export.

**Why this priority**: Prevents user frustration from illegible prints while respecting user agency to export any custom dimension down to 1.0 cm.

**Independent Test**: Can be tested independently by entering `1.0` × `1.0` cm in the dimensions fields and verifying that a live warning appears in the dialog and export still succeeds.

**Acceptance Scenarios**:

1. **Given** the PDF export options view, **When** dimensions or track density cause the calculated font size to fall below 5.0 pt, **Then** a prominent warning label is displayed stating that the font may not be legible to the naked eye.
2. **Given** the readability warning is active, **When** the user clicks Export, **Then** the export proceeds normally without error dialogs or blocking.
3. **Given** dimensions above 1.0 × 1.0 cm, **When** the user attempts to enter a dimension smaller than 1.0 cm, **Then** the value clamps to the 1.0 cm minimum.

---

### User Story 4 - PDF Theming and Visual Presentation (Priority: P4)

A user wants a styled or branded physical tracklist card (e.g., dark theme, warm paper, or custom palette with an optional background image). The user selects a theme from the preset dropdown, and the exported PDF applies the theme's colors, background fill, background image opacity/fitting, and font styling.

**Why this priority**: Allows customized aesthetic presentation for gifts, music box hardware inserts, and album packaging.

**Independent Test**: Can be tested independently by selecting different themes (e.g., White Monochrome, Warm Paper, Dark Slate) and verifying color attributes and background elements in the generated PDF.

**Acceptance Scenarios**:

1. **Given** the PDF export dialog, **When** the user selects a theme from the dropdown, **Then** the exported document applies the background color, text color, accent color, and border styling defined by that theme.
2. **Given** a theme containing a background image with opacity and fit mode (`cover` / `contain`), **When** exported, **Then** the image is scaled and composited behind text according to theme specifications.
3. **Given** missing or invalid external theme files, **When** the application runs, **Then** it gracefully falls back to the embedded White Monochrome theme.

---

### Edge Cases

- **Zero tracks in playlist**: System warns user that playlist is empty before opening file dialog; export does not proceed.
- **Single track playlist**: Layout engine handles single-item catalog without division-by-zero or empty-column layout glitches.
- **Extreme track titles**: Overlong titles (e.g., 200+ characters) are cleanly truncated with ellipsis (`...`) within column bounds and never collide with adjacent columns.
- **Special characters & Unicode**: Non-ASCII titles, foreign scripts, and symbols in track names are handled gracefully using embedded unicode fonts or safe glyph fallback.
- **Destination write permission error**: If target path is read-only or locked, an informative error message is displayed to the user; no corrupted or partial files are created.
- **Existing file overwrite**: Export atomically replaces target file only upon successful document generation.
- **Screen viewport constraints**: Dialog contents remain fully operable and layout does not break in narrow viewport sizes down to 300 × 400 px.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a unified "Export Catalog..." action in the main interface that opens an auto-height OS child viewport dialog without vertical scrollbars.
- **FR-002**: System MUST support exporting catalogs in three formats: PDF (`.pdf`), Plain Text (`.txt`), and Rich Text (`.rtf`).
- **FR-003**: System MUST provide universal column/field toggles applicable to all formats: Track Number (default ON), Title (default ON), Artist (default OFF), Export Filename (default OFF), and Header Title (default OFF). When Header Title is enabled, an editable title text input MUST be provided defaulting to the playlist name or folder name.
- **FR-004**: System MUST allow users to select physical measurement units (`cm`, `mm`, `in`) for PDF export, converting values accurately to standard PDF points (`1 in = 72 pt`, `1 cm = 28.3465 pt`, `1 mm = 2.83465 pt`).
- **FR-005**: System MUST provide physical dimension presets for PDF export: `10×10 cm Card` (default), `12×12 cm CD Cover`, `A6`, `A5`, `A4`, and `Custom`.
- **FR-006**: System MUST allow custom width and height inputs for PDF export with an absolute minimum allowable size of 1.0 × 1.0 cm.
- **FR-007**: System MUST allow configuring outer margins in the active unit down to 0.00 cm, with an automatic internal safety inset applied only when cut border is drawn.
- **FR-008**: System MUST provide an optional toggle to render dashed outer cut/crop guidelines (`0.75 pt`, `[4.0, 2.5]` dash pattern, `0.5 pt` edge inset) on the PDF.
- **FR-009**: System MUST automatically compute optimal column count and scaled font size using exact glyph advance widths from embedded `Roboto-Regular.ttf` (`cmap`+`hmtx`), coupling header metrics directly, and distributing leftover width into inter-column gaps so the rightmost column flushes to the right margin with zero overlap or clipping.
- **FR-010**: System MUST display a live layout summary indicating estimated columns, calculated font size, and physical sheet dimensions.
- **FR-011**: System MUST display a clear visual readability warning whenever calculated font size falls below 5.0 pt, while still allowing the user to export.
- **FR-012**: System MUST support selectable visual themes for PDF output, including an embedded fallback White Monochrome theme and clean starter themes.
- **FR-013**: System MUST embed cross-platform typography ensuring identical document rendering across Linux, Windows, and macOS without requiring system fonts.
- **FR-014**: System MUST write exported files atomically to prevent partial or corrupted outputs on failure and leave zero temporary files behind.
- **FR-015**: System MUST provide command-line catalog export capability supporting format, sizing, and theme options with sensible defaults.
- **FR-016**: System MUST persist the user's export format, field toggles, unit, and theme preferences across sessions.
- **FR-017**: All documentation, roadmap entries, and specifications across the repository MUST reflect PDF export as superseding previously planned DOCX export.
- **FR-018**: Export filenames rendered in the catalog MUST follow Constitution Principle IX (`XXYY-<sanitized_id3_title>.mp3`, no square brackets, lowercase underscore slug, falling back to original filename stem).

### Key Entities *(include if feature involves data)*

- **CatalogExportConfig**: Represents the active export settings, including selected format (PDF, TXT, RTF), universal field toggles (track number, title, artist, filename, header), physical dimensions, unit of measure, margins, cut marks toggle, and active theme identifier.
- **PhysicalDimensions**: Real-world printable dimensions including width, height, and margin, with conversion methods between physical units (`cm`, `mm`, `in`) and PDF points.
- **CatalogTheme**: Styling specification defining background color, primary font color, accent color, optional background image with opacity and fit mode, font sizes, line height, column spacing, and border styling.
- **CatalogLayout**: Computed layout solution containing resolved column count, column width, line height, font size, and item placement for rendering.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can configure and export a custom-sized PDF catalog in under 15 seconds from the main window.
- **SC-002**: 100% of exported PDF documents match specified physical print dimensions within 0.1 mm accuracy when printed at 100% scale.
- **SC-003**: 100% of playlist tracks fit inside the configured sheet boundaries without text overlapping column borders or neighboring tracks.
- **SC-004**: PDF export produces identical visual layout and font metrics across Linux, Windows, and macOS test environments.
- **SC-005**: Real-time layout calculations and font size adjustments update within 50 ms of user input changes in the export dialog.
- **SC-006**: Export failure or cancellation leaves 0 bytes of temporary files on disk.
- **SC-007**: Export modal dialog remains completely usable and fully visible on screen viewports down to 300 × 400 px.

## Assumptions

- Users print documents at 100% ("Actual Size") scale in standard PDF viewers to achieve true physical dimensions.
- Track metadata (title, artist) originates from the current loaded project state and ID3 tags.
- Default export destination opens in the current project or working directory.
- Standard A4 / A5 / A6 preset sizes follow ISO 216 dimensions.
