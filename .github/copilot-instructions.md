# Copilot Instructions: MusicBox Organizer

## Constitution Precedence
`.specify/memory/constitution.md` is the supreme law of this project. It always takes precedence over any generic advice, default framework patterns, or suggestions found inside external skills or documentation.

## Agent Skills Routing
Agents MUST invoke the appropriate skill before executing task phases:
- **`rust-gui`**: Load before writing, refactoring, or reviewing Rust application logic, data models, services, or `eframe`/`egui` GUI code.
- **`egui-gui-testing`**: Load before writing, running, or debugging headless GUI tests (`egui_kittest`) and service regression tests.

## Key Project Rules
- **GUI & Graphics**: Pure-Rust `eframe`/`egui` with `rfd` native dialogs. No Electron, WebView, GTK, Iced, or Slint. Prefer `glow` backend for cross-platform compatibility.
- **Binary Footprint**: Standalone release binary ceiling is `< 20 MB`. Minimalist dependencies; avoid heavy runtimes like `tokio` when `std` threads/channels suffice.
- **Audio Processing**: Native DSP and decoding only. Zero FFmpeg runtime dependency. Zero temp files left behind.
- **Testing**: Cross-platform headless GUI testing via `egui_kittest` under `cargo test`. No `xvfb-run` required.
- **Git Workflow**: Agent commits using Conventional Commits v1.0.0 format with Co-authored-by trailer. Auto-commit upon feature completion per Constitution Principle VI.
