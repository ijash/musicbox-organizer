# Research: Modularize GTK UI Architecture (Tech Debt D3)

**Feature Branch**: `006-modularize-gtk-ui`
**Date**: 2026-11-20

## 1. Component State Sharing & Ownership Pattern

### Decision
Use explicit builder/constructor functions returning the component's root widget (`gtk::Box` or `gtk::Frame`), taking `Rc<RefCell<ProjectFile>>`, `Rc<RefCell<SessionState>>`, and necessary coordination callbacks.

### Rationale
- Idiomatic Rust GTK3 architecture.
- Eliminates circular dependencies and complex event buses.
- Maintains clean separation of concerns:
  - `header.rs`: Constructs top bar, logo, project title, undo, redo, and theme toggle.
  - `playlist_table.rs`: Constructs TreeView and action toolbar (Scan, Add, Up, Down, Save, Open, Export Text).
  - `audio_bar.rs`: Constructs audio preview bar (Play, Pause, Stop, status label).
  - `export_panel.rs`: Constructs USB/SD media export frame (device dropdown, refresh, reformat warning modal, export trigger).
  - `footer.rs`: Constructs bottom attribution label with repository hyperlink.

---

## 2. Global Stylesheets & CSS Scoping

### Decision
Keep `LIGHT_CSS` and `DARK_CSS` definitions in a dedicated `src/ui/theme.rs` module, applied globally to `gtk::gdk::Screen::default()` with `STYLE_PROVIDER_PRIORITY_USER`.

### Rationale
- Centralizes styling and palette rules.
- Avoids scattering CSS strings across components.
- Guarantees immediate switching across all modular subwidgets.

---

## 3. Backward Compatibility & Zero Regressions

### Decision
`src/ui/gtk_ui.rs` remains the primary entry point exporting `run_gtk_app()`, acting as the top-level window assembler.

### Rationale
- Call sites in `src/main.rs` and `src/ui/mod.rs` remain completely untouched.
- All 49 existing unit and integration tests remain intact.
