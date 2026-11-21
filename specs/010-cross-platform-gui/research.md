# Phase 0 Research: Unified Cross-Platform Rust GUI (Feature 010)

## 1. Crate Selection & Feature Flags

### Decision: `eframe` with `glow` Backend (No `wgpu`, No `tokio`)
- **Decision**: Add `eframe = { version = "0.31", default-features = false, features = ["default_fonts", "glow"] }`, `egui = "0.31"`, and `egui_extras = { version = "0.31", features = [] }` (only minimal features needed). In `[dev-dependencies]`, add `egui_kittest = { version = "0.31", default-features = false, features = ["winit"] }`.
- **Rationale**:
  - `wgpu` brings heavy DirectX/Vulkan/Metal shader compilers that inflate binary size by 15-30 MB and increase compilation times significantly.
  - `glow` uses lightweight cross-platform OpenGL/OpenGL ES/ANGLE bindings that compile rapidly and keep standalone release binaries strictly under `< 20 MB` (typically ~8-12 MB stripped with LTO).
  - Heavy async runtimes like `tokio` are avoided per Constitution Principle I & `rust-gui` skill; background work uses `std::thread::spawn` and `std::sync::mpsc`.
- **Alternatives Considered**:
  - `eframe` with `wgpu`: Rejected due to binary bloat exceeding the < 20 MB ceiling and extended compile times.
  - Slint / Iced / Tauri: Rejected per Constitution Principle II (only pure-Rust `eframe`/`egui` permitted).

### Decision: Native File Dialogs via `rfd` with `xdg-portal`
- **Decision**: Add `rfd = { version = "0.15", default-features = false, features = ["xdg-portal"] }`.
- **Rationale**:
  - On Linux, `rfd` defaults to `gtk3`. By specifying `default-features = false, features = ["xdg-portal"]`, `rfd` uses D-Bus Desktop Portals (`org.freedesktop.portal.FileChooser`), which work across GNOME, KDE, and Wayland without linking or installing `libgtk-3-dev`.
  - On Windows and macOS, `rfd` automatically uses native Win32 `IFileDialog` and Cocoa `NSOpenPanel` respectively without extra configuration.
- **Alternatives Considered**:
  - `rfd` with default GTK3 feature: Rejected because it forces `libgtk-3-dev` in CI and runtime GTK3 libraries on Linux desktops.

### Decision: Rust Edition 2024 Upgrade
- **Decision**: Update `Cargo.toml` to `edition = "2024"` and `version = "0.3.0"`.
- **Rationale**:
  - `eframe` 0.31+ and modern ecosystem dependencies seamlessly leverage the 2024 edition resolver and idioms.
  - Supported natively by the existing `rustc 1.98.0` toolchain.
- **Alternatives Considered**:
  - Staying on `edition = "2021"`: Rejected per clarification decision to adopt modern edition standards and resolver behavior.

---

## 2. Windows Desktop Launch vs. CLI Subcommands

### Decision: `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` + Conditional `AttachConsole`
- **Decision**:
  - Mark crate root with `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`.
  - In `src/main.rs`, inspect CLI arguments before dispatching. If arguments are present (`args.len() > 1` and not `"gui"`):
    ```rust
    #[cfg(windows)]
    unsafe {
        use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }
    ```
- **Rationale**:
  - Launching from Windows Explorer (double-clicking `musicbox.exe`) creates zero terminal windows.
  - Launching from PowerShell / CMD (`musicbox.exe status`, `musicbox.exe scan .`, `musicbox.exe export`) attaches to the existing calling terminal and prints standard output directly.
- **Alternatives Considered**:
  - Subsystem `"console"`: Flashes an unsightly black console window on every double-click before opening the GUI.
  - Pure subsystem `"windows"` without `AttachConsole`: Completely silences CLI output in Windows terminals, breaking all CLI subcommands on Windows.

---

## 3. UI State Architecture & Concurrency

### Decision: Direct State Ownership in `MusicBoxApp` with `mpsc` Background Channels
- **Decision**:
  - Replace Relm4 / GTK widgets and `Rc<RefCell<...>>` patterns with a single idiomatic immediate-mode struct:
    ```rust
    pub struct MusicBoxApp {
        pub project: ProjectFile,
        pub session: SessionState,
        pub active_device_id: Option<String>,
        pub available_devices: Vec<Device>,
        pub preview_player: Option<AudioPreviewPlayer>,
        pub playing_track_idx: Option<usize>,
        pub dsp_dialog: DspDialogState,
        pub export_in_progress: bool,
        pub status_message: String,
        pub dark_mode: bool,
        // Channel receivers for non-blocking worker threads
        pub dsp_rx: Option<std::sync::mpsc::Receiver<DspProgressMessage>>,
        pub export_rx: Option<std::sync::mpsc::Receiver<ExportProgressMessage>>,
    }
    ```
  - State rendering occurs in `eframe::App::update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)`.
  - While background operations run, the worker thread sends progress events over `std::sync::mpsc::channel`. The UI checks `try_recv()` each frame and calls `ctx.request_repaint()` to maintain smooth updates at 60 FPS without UI freezes.
- **Rationale**:
  - Eliminates pointer aliasing and runtime borrow panics.
  - Follows immediate-mode GUI best practices (`rust-gui` skill).
  - Stops `preview_player` before dispatching any DSP processing to prevent file locking and playback stutter.
- **Alternatives Considered**:
  - Multi-threaded shared state (`Arc<Mutex<MusicBoxApp>>`): Rejected as unnecessarily complex; UI loop stays single-threaded while long-running jobs run detached with simple message passing.

---

## 4. Theme Accessibility & Icon Rendering

### Decision: Unicode Symbol Glyphs with Theme-Inherited Text Color
- **Decision**:
  - Instead of loading heavy icon fonts or parsing SVG trees into textures each frame, buttons use compact Unicode glyphs:
    - Add: `➕ Add MP3`
    - Folder: `📁 Scan Folder...`
    - Remove: `🗑 Remove`
    - Up: `⬆ Up`
    - Down: `⬇ Down`
    - Audio Tools: `⚙ Audio Tools...`
    - Export Text: `📄 Export Text/RTF...`
    - Play: `▶ Play`
    - Pause: `⏸ Pause`
    - Stop: `⏹ Stop`
    - Undo: `⮌ Undo`
    - Redo: `⮎ Redo`
    - Dark/Light Toggle: `🌙 Dark` / `☀ Light`
- **Rationale**:
  - Unicode glyphs are rendered as native font characters by `egui`, automatically inheriting `ui.visuals().text_color()` in both `egui::Visuals::dark()` and `egui::Visuals::light()`.
  - 100% immune to contrast mismatch bugs; adds 0 KB to binary size.
- **Alternatives Considered**:
  - SVG via `egui_extras`: Adds extra texture allocations and rasterization dependencies without adding functional benefit over crisp standard glyphs.

---

## 5. Windows Removable Drive Enumeration

### Decision: Direct Win32 Storage API in `src/platform/windows.rs`
- **Decision**:
  - Use `windows::Win32::Storage::FileSystem::{GetLogicalDrives, GetDriveTypeW, GetDiskFreeSpaceExW, GetVolumeInformationW}`.
  - Iterate drive bits (0..26 for 'A'..'Z').
  - Check `GetDriveTypeW(drive_path)`: if `DRIVE_REMOVABLE` (or `DRIVE_FIXED` where appropriate for mounted flash memory), query volume label and capacity.
  - Populate `Device` struct with `path`, `label`, `total_space`, `free_space`, `is_removable: true`.
- **Rationale**:
  - Fulfills Clarification decision Q5 without introducing heavy WMI or external crates.
  - Cleanly implements `DeviceManager` trait already defined in `src/services/device_manager.rs`.

---

## 6. Headless Testing with `egui_kittest`

### Decision: `egui_kittest::Harness` for Cross-Platform Automated UI Verification
- **Decision**:
  - In `tests/gui_tests.rs`, instantiate `Harness::new_ui` wrapping `MusicBoxApp`.
  - Simulate clicks (`harness.get_by_label("...").click()`), text editing, and state changes.
  - Call `harness.run()` and assert state mutations on `app.project`, `app.session`, and `app.dsp_dialog`.
  - Run via standard `cargo test` in CI across Linux, Windows, and macOS without `xvfb-run`.
- **Rationale**:
  - Works headlessly across all 3 operating systems.
  - Eliminates Linux-only X11/Xvfb display server dependencies.
