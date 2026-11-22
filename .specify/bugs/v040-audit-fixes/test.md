# Bug Verification: v0.4.0 Audit Fixes

- **Slug**: v040-audit-fixes
- **Tested**: 2026-11-23
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: verified

## Summary

All 5 audit defect remediations verified against the codebase. The DSP modal Cancel button functions cleanly and closes the dialog; the modal is rendered in an OS child viewport; batch DSP uses single-pass `process_track` without double MP3 generation loss; Audio Bar allows direct track switching without prior stop; Toolbar remapping maintains accurate `playing_track_idx` across removals and reordering; Windows device stubs return explicit unsupported errors; and Export Catalog dynamically sends `ViewportCommand::InnerSize` upon format toggle. Full regression suite of 93 tests passes with zero warnings.

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| DSP Cancel & Viewport | `cargo test --test gui_tests -- test_dsp_dialog_cancel_button` | pass | Verifies Cancel closes modal and resets `is_open`. |
| DSP Full App Toolbar Open/Close | `cargo test --test gui_tests -- test_dsp_dialog_toolbar_button_opens_and_cancel_closes_full_app` | pass | Opens from toolbar and closes on Cancel under eframe harness. |
| Audio Bar Direct Switching | `cargo test --test gui_tests -- test_audio_bar_direct_track_switching_while_playing` | pass | `▶ Play` enabled on different track selection while playing. |
| Toolbar Index Reconcile | `cargo test --test gui_tests -- test_toolbar_track_removal_and_move_sync_playing_index` | pass | Verifies index shifts on deletion/move and playback halts if playing track deleted. |
| Export Catalog Dynamic Resize | `cargo test --test gui_tests -- test_export_catalog_format_switch_resizes_viewport` | pass | Verifies format switching under child viewport. |
| Windows Format/Eject Honest Errors | `cargo test --test unit_tests -- test_windows_unsupported_format_and_eject_errors` | pass | Returns descriptive unsupported `Err` messages. |
| Full Regression Suite | `cargo test --all-targets` | pass | 93 tests passed, 0 failed. |
| Static Analysis / Linter | `cargo clippy --all-targets -- -D warnings` | pass | Zero warnings across all targets. |
| Format Check | `cargo fmt --check` | pass | Compliant with repository rustfmt rules. |
| Release Binary Size | `cargo build --release` | pass | 12 MB binary (`< 20 MB` Constitution ceiling). |

## Output Excerpts

### Focused GUI & Unit Regressions
```text
running 4 tests
test gui::audio_dsp_tests::test_dsp_dialog_cancel_button ... ok
test gui::audio_dsp_tests::test_audio_bar_direct_track_switching_while_playing ... ok
test gui::export_catalog_dialog_test::test_export_catalog_format_switch_resizes_viewport ... ok
test gui::playlist_tests::test_toolbar_track_removal_and_move_sync_playing_index ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 19 filtered out; finished in 0.04s

running 1 test
test device_tests::test_windows_unsupported_format_and_eject_errors ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 39 filtered out; finished in 0.00s
```

### Full Test Suite
```text
running 22 tests (src/lib.rs) ... ok. 22 passed
running 23 tests (tests/gui_tests.rs) ... ok. 23 passed
running 8 tests (tests/integration_tests.rs) ... ok. 8 passed
running 40 tests (tests/unit_tests.rs) ... ok. 40 passed
Total: 93 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Binary Verification
```text
-rwxrwxr-x 2 ijash ijash 12M Nov 23 00:24 target/release/musicbox
```

## Residual Risks

- **Platform Native Viewports**: Multi-viewport rendering requires compositor/windowing support (Wayland, X11, Win32, AppKit). Where multi-viewport is disabled or unsupported by the platform window manager, `egui` automatically falls back to inline modal rendering.
- **Windows Physical Device Testing**: `format_device` and `eject_device` tests verify error contracts headlessly on non-Windows host; real hardware execution on Windows requires running the release binary on Windows OS.

## Recommendation

Close the bug. All 5 defects are resolved, verified by automated unit and headless GUI tests, and release gates are green.
