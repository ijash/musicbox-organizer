# Bug Fix: Dark Mode Icon Contrast and Compact Button Styling

- **Slug**: dark-icons-compact-buttons
- **Fixed**: 2026-11-21
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Added theme-aware SVG rasterization in `AppIcon` replacing `currentColor` with high-contrast hex fills (`#f0f0f0` for dark, `#2e3436` for light), wired dynamic in-place button icon refreshing on theme toggle, and applied compact button padding (`2px 6px`) and min-height (`24px`) in GTK CSS.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/ui/icons.rs` | modified | Added `pixbuf_with_color`, `pixbuf_themed`, `image_themed`, `button_themed`, and `update_button_icon` |
| `src/ui/theme.rs` | modified | Added `padding: 2px 6px; min-height: 24px;` to `button` rules in both `LIGHT_CSS` and `DARK_CSS` |
| `src/ui/components/playlist_table.rs` | modified | Implemented `update_icons` and exposed buttons on `PlaylistTableComponent` |
| `src/ui/components/audio_bar.rs` | modified | Implemented `update_icons` on `AudioBarComponent` |
| `src/ui/gtk_ui.rs` | modified | Added initial icon theme sync and wired `theme_btn.connect_clicked` refresh |
| `tests/gui/harness.rs` | modified | Synced initial icon theme and connected theme toggle handler in test harness |
| `tests/gui_tests.rs` | modified | Added test assertions for themed icon generation and `update_button_icon` |

## Diff Highlights

```rust
// src/ui/icons.rs
pub fn pixbuf_with_color(&self, size: i32, hex_color: &str) -> Option<gtk::gdk_pixbuf::Pixbuf> {
    let svg = self.svg_data().replace("currentColor", hex_color);
    let pixbuf = gtk::gdk_pixbuf::Pixbuf::from_read(Cursor::new(svg.into_bytes())).ok()?;
    pixbuf.scale_simple(size, size, gtk::gdk_pixbuf::InterpType::Bilinear)
}

pub fn update_button_icon(btn: &Button, icon: AppIcon, is_dark: bool) {
    if let Some(child) = btn.child() {
        if let Ok(box_container) = child.downcast::<gtk::Box>() {
            if let Some(first) = box_container.children().first() {
                if let Ok(img) = first.clone().downcast::<Image>() {
                    if let Some(pixbuf) = icon.pixbuf_themed(14, is_dark) {
                        img.set_from_pixbuf(Some(&pixbuf));
                    }
                }
            }
        }
    }
}
```

```css
/* src/ui/theme.rs */
button {
    padding: 2px 6px;
    min-height: 24px;
}
```

## Tests Added or Updated

- `tests/gui_tests.rs::test_gui_interactive_workflow_headless` — Asserts `image_themed(14, true)` and `image_themed(14, false)` produce valid non-empty pixbufs across all 12 `AppIcon` variants, verifies `update_button_icon` across dark/light transitions, and tests theme toggle re-renders without GTK errors.

## Local Verification

- Commands run:
  - `cargo check` → 0 errors, clean check.
  - `xvfb-run -a cargo test` → 56 passed, 0 failed (21 lib unit tests, 1 headless GUI test, 5 integration tests, 29 unit tests).
  - `cargo build --release` → 2.2 MB binary size (<2.5 MB ceiling maintained).

## Deviations from Assessment

None. Implementation followed proposed remediation directly.

## Follow-ups

- None. Ready for `/speckit-bug-test slug=dark-icons-compact-buttons`.
