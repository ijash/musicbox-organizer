# Bug Assessment: egui UI Parity & Responsive Layout Regressions

- **Slug**: egui-ui-parity-responsive
- **Created**: 2026-11-22
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

Triage and propose remediation for 9 `eframe`/`egui` GUI regressions found during visual QA on Feature 010:

1. **Undo/Redo missing glyphs (`□`)**: `egui`'s default font lacks the current Unicode codepoints used for Undo/Redo (`⮌` / `⮎`). Replace with glyphs guaranteed supported by `egui`'s built-in font (e.g. `↩` / `↪` or `←` / `→`).
2. **Scan Folder initial directory**: Set `rfd::FileDialog::new().set_directory(std::env::current_dir())` so "Scan Folder..." opens directly in the current working directory.
3. **Auto-load current folder on startup**: On app initialization (`MusicBoxApp::new`), auto-load existing `playlist.txt` or scan MP3s in the current working directory (matching pre-010 startup behavior).
4. **Missing window favicon & header logo (Feature 005 regression)**: Inspect `specs/005-ui-branding-github-link/` and `assets/img/logo/` assets; embed and attach the app icon via `ViewportBuilder::with_icon(...)` and display the small app logo in the header bar.
5. **Auto-detect removable devices on startup**: Trigger initial device discovery (`refresh_devices()`) automatically inside `MusicBoxApp::new()` so connected USB/SD drives appear immediately without clicking "Scan Devices".
6. **Minimum window size `300x400px`**: Allow resizing the window down to `300x400px` (`ViewportBuilder::with_min_inner_size([300.0, 400.0])`); wrap the playlist area in a `ScrollArea` so it shrinks gracefully.
7. **Responsive wrapping / stacking (no overlap or clipping)**: Use `ui.horizontal_wrapped(...)` across Header, Toolbar, Audio Preview Bar, and Export to Removable Media panel so buttons stack cleanly onto new lines when narrow — fixing `Export Text/RTF...` overlapping `Open` and fixing `Export Playlist` / `Eject` getting clipped.
8. **More compact buttons & slightly smaller typography**: Tighten global `egui::Style` (`TextStyle::Body` / `Button` slightly smaller e.g. `12.0–12.5px`, `button_padding: [5.0, 2.0]`, `item_spacing: [4.0, 4.0]`) so the entire UI feels crisp and dense.
9. **Footer GitHub attribution format**: Update footer right-aligned text to italic: `Made by Ijash. Github: ` followed by a clickable italic link `musicbox` pointing to `https://github.com/ijash/musicbox-organizer`.

## Symptom

- Undo and Redo buttons in the header render placeholder box characters (`□`) instead of arrows due to missing glyph coverage in `egui`'s default proportional font.
- File dialogs open in arbitrary OS-default directories rather than the user's active working directory.
- The desktop app starts with an empty playlist table and empty removable device selector even when `playlist.txt` or MP3 files and USB drives exist in the environment.
- The window title bar has no favicon icon and the header bar has no visual logo graphic, regressing Feature 005.
- On smaller display resolutions or narrow window dimensions (<720px), the toolbar actions overlap `Save` and `Open`, the export actions get clipped off-screen, and window resizing is artificially constrained to a 720x480 minimum.
- Button padding and font sizes appear bloated compared to the desktop app standard.
- Footer repository attribution does not match the requested format.

## Reproduction

1. Run `cargo run` in a directory containing `playlist.txt` and MP3 files.
2. Observe header: Undo/Redo buttons display `□ Redo` and `□ Undo`.
3. Observe window title bar: generic default window icon instead of MusicBox logo.
4. Observe playlist table: empty initial state (does not auto-load `playlist.txt` or scan local MP3s).
5. Observe export panel: device dropdown displays "No device selected" until manual click on "🔄 Scan Devices".
6. Resize window to minimum width: window stops at 720px width; when made smaller programmatically, toolbar buttons collide and export buttons clip outside viewport.

## Suspected Code Paths

- `src/ui/header.rs:36` & `src/ui/header.rs:48` — Unicode glyphs `⮎` and `⮌` unsupported in default `egui` fonts; header title lacks logo image texture.
- `src/ui/toolbar.rs:48` — `rfd::FileDialog::new().pick_folder()` lacks `.set_directory(...)` initialization with `std::env::current_dir()`.
- `src/ui/app.rs:50` & `src/ui/app.rs:80` — `MusicBoxApp::default()` and `MusicBoxApp::new()` initialize empty `ProjectFile` without attempting `auto_load_project()` or local directory scan.
- `src/ui/app.rs:200` — `eframe::run_native` viewport configuration enforces `min_inner_size([720.0, 480.0])` and omits `with_icon(...)`.
- `src/ui/export_panel.rs:25` — `ExportPanelState::default()` initializes empty device list without running initial enumeration.
- `src/ui/header.rs:4`, `src/ui/toolbar.rs:6`, `src/ui/audio_bar.rs:6`, `src/ui/export_panel.rs:37` — Non-wrapping `ui.horizontal(...)` layout calls cause overlap and clipping under horizontal constraints.
- `src/ui/app.rs:135` — Global `egui::Style` does not customize `button_padding`, `item_spacing`, or typography size.
- `src/ui/footer.rs:5` — Right-aligned layout uses plain `hyperlink_to` without italic attribution styling.

## Root Cause Hypothesis

During the rapid migration from GTK3 to `eframe`/`egui` in Feature 010, the core focus was functional parity, compilation, and headless tests. As a result:
1. Glyphs were chosen from the Extended Miscellaneous Symbols block (`U+2B8C`, `U+2B8E`) which are absent from egui's embedded fonts (which only bundle basic arrow and symbol subsets like `↩` `U+21A9`, `↪` `U+21AA`).
2. Directory context was omitted from `rfd` dialog builders.
3. Startup hydration hooks present in CLI (`auto_load_project`) and GTK UI were not wired into `MusicBoxApp::default()` / `new()`.
4. The window icon asset (`assets/img/logo/png/logo_mbo_128.png`) was not decoded into `egui::IconData` on `ViewportBuilder`.
5. Fixed `ui.horizontal(...)` containers with fixed spacers and right-to-left sub-layouts were used instead of responsive `ui.horizontal_wrapped(...)`.
6. High confidence: all 9 issues stem from straightforward missing configurations and layout primitives in `src/ui/`.

## Proposed Remediation

**Preferred**:
1. Replace Undo/Redo glyphs with standard `↩ Undo` / `↪ Redo`.
2. Configure all `rfd::FileDialog` invocations with `.set_directory(cwd)`.
3. In `MusicBoxApp::new()` and `MusicBoxApp::default()`, attempt loading `playlist.txt` from current working directory; if absent, scan current directory for MP3 files using `DirectoryScanner`.
4. Embed `assets/img/logo/png/logo_mbo_128.png` with `include_bytes!`; decode RGBA using `image::load_from_memory`; configure `ViewportBuilder::with_icon(...)`; render a 20x20 logo image in `render_header`.
5. Trigger initial `scan_devices()` during `MusicBoxApp` initialization so connected devices populate on launch.
6. Lower `min_inner_size` to `[300.0, 400.0]` in `run_gui()`. Ensure table columns shrink gracefully.
7. Replace `ui.horizontal` with `ui.horizontal_wrapped` across Header, Toolbar, Audio Bar, and Export Panel. Avoid conflicting `right_to_left` layouts in tight containers.
8. Set global `egui::Style` with `button_padding = [5.0, 2.0]`, `item_spacing = [4.0, 4.0]`, and `TextStyle::Body` / `TextStyle::Button` font size `12.5px`.
9. Update `src/ui/footer.rs` to render `RichText::new("Made by Ijash. Github: ").italics()` and `RichText::new("musicbox").italics()` pointing to repository URL.

**Files likely to change**:
- `src/ui/app.rs`
- `src/ui/header.rs`
- `src/ui/toolbar.rs`
- `src/ui/audio_bar.rs`
- `src/ui/export_panel.rs`
- `src/ui/footer.rs`
- `tests/gui/playlist_tests.rs`
- `tests/gui/theme_tests.rs`

**Tests to add or update**:
- Test startup auto-load and device discovery in `tests/gui/playlist_tests.rs`.
- Test responsive layout rendering at 300x400 in `tests/gui/theme_tests.rs`.
- Test footer text styling in `tests/gui/theme_tests.rs`.

## Risks & Considerations

- Decoding PNG icon at startup should be done once using `LazyLock` or during `run_gui()` to prevent startup lag.
- Auto-scanning current directory on startup should handle empty folders gracefully without blocking.

## Open Questions

- None. All 9 visual QA items are unambiguous.
