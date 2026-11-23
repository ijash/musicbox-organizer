# Quickstart Validation Guide: Unified Cross-Platform Rust GUI (Feature 010)

This guide documents the procedures for compiling, testing, and validating Feature 010 locally and across target platforms.

## Prerequisites

- **Rust Toolchain**: Stable Rust 1.85+ (Edition 2024 support). Verify with:
  ```bash
  rustc --version
  cargo --version
  ```
- **Linux Build Dependencies**:
  ```bash
  # Only ALSA and D-Bus required; libgtk-3-dev is NO LONGER NEEDED!
  sudo apt-get install -y libasound2-dev libdbus-1-dev
  ```

---

## Validation Scenario 1: Automated Unit & Headless GUI Verification

Run the full cross-platform test suite (including `egui_kittest` GUI tests) directly from the terminal without requiring `xvfb-run` or an active X11/Wayland display server:

```bash
# 1. Run all tests including headless GUI interaction tests
cargo test

# 2. Verify clean clippy linting
cargo clippy --all-targets -- -D warnings

# 3. Verify standard code formatting
cargo fmt --check
```

**Expected Outcome**:
- All tests pass 100% with zero failures.
- Headless GUI harness verifies playlist mutations, undo/redo, theme toggles, and modal dialog state changes.
- Zero compiler or clippy warnings.

---

## Validation Scenario 2: Release Binary Compilation & Size Gate

Compile the optimized release binary and verify the strict Constitution Principle I ceiling (`< 20 MB`):

```bash
# 1. Build optimized release binary
cargo build --release

# 2. Check binary size on Linux
ls -lh target/release/musicbox
# Verify size is strictly < 20 MB (20,971,520 bytes):
python3 -c "import os; size = os.path.getsize('target/release/musicbox'); print(f'Binary size: {size} bytes ({size/1024/1024:.2f} MB)'); assert size < 20971520, 'Binary exceeds 20 MB limit!'"
```

**Expected Outcome**:
- Release binary builds successfully with LTO and symbol stripping.
- Executable size is well below 20 MB (expected ~8-14 MB).

---

## Validation Scenario 3: CLI Subcommand Compatibility Verification

Verify that all existing command-line interfaces continue to execute seamlessly:

```bash
# Test help display
./target/release/musicbox help

# Test status display
./target/release/musicbox status

# Test directory scan
./target/release/musicbox scan assets/mp3-sample

# Test primitive device compatibility check
./target/release/musicbox check-compat assets/mp3-sample
```

**Expected Outcome**:
- Standard output prints formatted tables and metadata directly to terminal.
- On Windows, standard output is visible in CMD/PowerShell via `AttachConsole`.

---

## Validation Scenario 4: Interactive Desktop GUI Smoke Test

Launch the desktop interface to manually inspect visuals, themes, and audio playback:

```bash
# Launch GUI window
./target/release/musicbox
```

**Manual Verification Steps**:
1. **Window Display**: Window opens immediately with title "MusicBox Organizer".
2. **Theme Toggle**: Click `🌙 Dark` / `☀ Light` in header; observe immediate theme switch with crisp contrast on all buttons and text.
3. **Add Tracks**: Click `➕ Add MP3` or `📁 Scan Folder...`; native file picker opens; select files in `assets/mp3-sample/`.
4. **Inline Edit**: Double-click or click into "Title" or "Artist" cell; edit text; verify table updates immediately.
5. **Reorder & Undo**: Select a row; click `⬇ Down`; track swaps position; click `⮌ Undo`; track reverts position.
6. **Audio Preview**: Select a track; click `▶ Play`; verify preview status label updates and audio plays; click `⏹ Stop`.
7. **Audio DSP Dialog**: Select a track; click `⚙ Audio Tools...`; toggle Normalize and Trim; click `Cancel` or `Apply`.
8. **Export Panel**: Navigate to removable media section; click `Scan`; verify detected storage drives (or clean empty state).
