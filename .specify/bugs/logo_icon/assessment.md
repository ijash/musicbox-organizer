# Bug Assessment: App Icon / Favicon Missing & GitHub Button Style Change

- **Slug**: logo_icon
- **Created**: 2026-11-20
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: medium

## Report (verbatim or summarized)

> "- icon & faviconnya ga muncul.
> - githubnya jg aku maunya jangan lah button. bagusnya ada link. ganti ke teks dengan link di bawah app aja. slug=logo_icon"

## Symptom

1. When running the application, the window icon / favicon does not appear on the desktop taskbar or window title bar, and the in-app logo image fails to display.
2. The GitHub link is currently rendered as a button in the top-right header, crowding the header controls, instead of being a discrete text link placed at the bottom footer of the window.

## Reproduction

1. Run `cargo run` (or launch the standalone executable from a different directory or desktop launcher).
2. Observe window title bar/taskbar: icon remains generic application icon instead of the custom MusicBox icon.
3. Observe header: header image is missing if current working directory doesn't match relative `assets/img/` search path, or icon failed pixbuf loading.
4. Top-right header contains a `GitHub ↗` button that takes up toolbar space instead of being a subtle bottom footer link.

## Suspected Code Paths

- `src/ui/gtk_ui.rs:140-155` — Window icon uses relative path `"assets/img/logo/..."` which fails if current working directory is not the repository root, or when the `.ico` file is attempted on Linux X11/Wayland without GdkPixbuf loader for ico. Embedded bytes via `include_bytes!` should be used so the binary carries its own icons self-contained without filesystem dependencies.
- `src/ui/gtk_ui.rs:165-175` — Header logo image also relies on relative path `"assets/img/logo/png/logo_mbo_128.png"`. Using `include_bytes!` ensures the logo displays 100% reliably regardless of execution working directory.
- `src/ui/gtk_ui.rs:185-215` — `github_btn` (`Button::with_label("GitHub ↗")`) is packed into `header_box.pack_end(...)`.
- `src/ui/gtk_ui.rs:370-390` — Bottom of `root_box` lacks a footer bar for hyperlinks.

## Root Cause Hypothesis

**Confidence: High**

1. **Icon / Favicon Failure**:
   - `window.set_icon_from_file("assets/img/logo/...")` depends strictly on runtime current working directory (`std::env::current_dir()`). If launched from desktop application launchers, terminal subfolders, or installed paths, relative path resolution fails. Furthermore, `.ico` format support varies on Linux GdkPixbuf loaders compared to standard PNG.
   - Solution: Compile the PNG logo into the binary using `include_bytes!("../../../assets/img/logo/png/logo_mbo_128.png")` via `gdk_pixbuf::Pixbuf::from_read(std::io::Cursor::new(...))` and `window.set_icon(Some(&pixbuf))`. This guarantees the icon and favicon exist unconditionally in memory.
2. **GitHub Link Placement & Format**:
   - The user requested removing the top-right button entirely and replacing it with a text hyperlink at the bottom of the application window (e.g. `<a href="...">GitHub Repository</a>` or a subtle bottom footer text label).

## Proposed Remediation

**Preferred**:
1. In `src/ui/gtk_ui.rs`:
   - Embed `assets/img/logo/png/logo_mbo_128.png` at compile time using `include_bytes!` so that window icon (`window.set_icon(...)`) and header logo (`Image::from_pixbuf(...)`) load reliably with zero filesystem path dependency.
   - Remove `github_btn` from the top `header_box`.
   - Add a subtle bottom footer `HBox` at the end of `root_box` below the export frame containing an interactive link label (e.g., `<a href="https://github.com/ijash/musicbox-organizer">MusicBox Organizer on GitHub</a>` using GTK markup `use_markup = true`).
   - Clean up top header spacing so `project_label`, `undo_btn`, `redo_btn`, and `theme_btn` sit cleanly without clutter.

**Files likely to change**:
- `src/ui/gtk_ui.rs`

**Tests to add or update**:
- Verify GUI compiles cleanly and manual execution displays window icon, header logo, and bottom footer link.

## Risks & Considerations

- Low risk. Embedding a 128px PNG asset adds only ~15 KB to the compiled binary while eliminating runtime path missing errors.

## Open Questions

None.
