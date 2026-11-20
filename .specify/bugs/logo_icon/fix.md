# Bug Fix: App Icon / Favicon Embedded & GitHub Bottom Link

- **Slug**: logo_icon
- **Fixed**: 2026-11-20
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Embedded the 128px PNG application logo directly into the compiled binary via `include_bytes!` to ensure the window icon/favicon and top-left header logo render 100% reliably regardless of the current working directory. Removed the top-right `GitHub ↗` button and added a clean, interactive `LinkButton` at the bottom of the window.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/ui/gtk_ui.rs` | modified | Used `include_bytes!` for embedded logo pixbuf loading; removed top-right button; added footer `LinkButton` |

## Diff Highlights

```rust
// Embedded logo bytes:
const APP_LOGO_PNG: &[u8] = include_bytes!("../../img/logo/png/logo_mbo_128.png");

// 1. Set window icon / favicon:
if let Ok(pixbuf) = gtk::gdk_pixbuf::Pixbuf::from_read(Cursor::new(APP_LOGO_PNG)) {
    window.set_icon(Some(&pixbuf));
}

// 2. Render 24px header logo beside title:
if let Ok(pixbuf) = gtk::gdk_pixbuf::Pixbuf::from_read(Cursor::new(APP_LOGO_PNG)) {
    if let Some(scaled) = pixbuf.scale_simple(24, 24, gtk::gdk_pixbuf::InterpType::Bilinear) {
        let logo_img = Image::from_pixbuf(Some(&scaled));
        header_box.pack_start(&logo_img, false, false, 0);
    }
}

// 3. Bottom footer link:
let footer_box = GtkBox::new(Orientation::Horizontal, 4);
let github_link = LinkButton::with_label(
    "https://github.com/ijash/musicbox-organizer",
    "MusicBox Organizer on GitHub",
);
footer_box.pack_end(&github_link, false, false, 4);
root_box.pack_start(&footer_box, false, false, 0);
```

## Local Verification

- `cargo test` passes 49/49 tests.
- Release build compiled cleanly.
- Icon and header logo load directly from memory; bottom footer link opens repository URL in default browser.

## Deviations from Assessment

None.

## Follow-ups

- Run `/speckit-bug-test slug=logo_icon` to verify.
