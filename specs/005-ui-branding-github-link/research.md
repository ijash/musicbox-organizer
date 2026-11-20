# Research: UI Branding & GitHub Link Integration

**Feature Branch**: `005-ui-branding-github-link`
**Date**: 2026-11-20

## 1. Window Icon / Favicon Loading in GTK3

### Decision
Use `gtk::Window::set_icon_from_file("img/logo/png/logo_mbo_128.png")` (with fallback to `img/logo/ico/favicon.ico`).

### Rationale
- Standard GTK3 method supported by X11 and Wayland window managers.
- Pure platform API, zero additional crates.
- Gracefully ignores missing file (`.ok()`) so the app never panics if assets are missing.

---

## 2. Header Branding Logo Rendering

### Decision
Use `gtk::Image::from_file("img/logo/png/logo_mbo_128.png")` scaled with `gdk_pixbuf::Pixbuf::from_file_at_scale(path, 28, 28, true)` or an image widget embedded directly inside `header_box` next to `project_label`.

### Rationale
- Renders sharply at 24–28px height.
- Keeps startup latency <10ms.
- Matches header text baseline.

---

## 3. GitHub Link Launch Mechanism

### Decision
Use `gtk::show_uri_on_window(Some(&window), "https://github.com/ijash/musicbox-organizer", gtk::gdk::CURRENT_TIME)` (or `open::that()` / platform command fallback).

### Rationale
- GTK3 native function `gtk::show_uri_on_window` delegates directly to default desktop URI handler (XDG on Linux, default browser on Windows/macOS).
- Non-blocking, secure, and needs no new dependencies.
