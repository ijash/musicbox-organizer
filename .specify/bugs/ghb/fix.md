# Bug Fix: Italic Footer Text Link with Custom Attribution

- **Slug**: ghb
- **Fixed**: 2026-11-20
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Replaced the `gtk::LinkButton` in `src/ui/gtk_ui.rs` with a clean `gtk::Label` using Pango markup. The text is formatted as requested: `<small><i>made by ijash. more at <a href="...">github</a></i></small>`.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/ui/gtk_ui.rs` | modified | Removed `LinkButton`; added italic `Label` with anchor markup `made by ijash. more at <github>` |

## Diff Highlights

```rust
// Bottom Footer Bar with Italic Text Link
let footer_box = GtkBox::new(Orientation::Horizontal, 4);
let footer_label = Label::new(None);
footer_label.set_markup(
    "<small><i>Made by Ijash. More at <a href=\"https://github.com/ijash/musicbox-organizer\">github</a></i></small>",
);
footer_label.set_track_visited_links(false);
footer_box.pack_end(&footer_label, false, false, 4);
root_box.pack_start(&footer_box, false, false, 0);
```

## Local Verification

- `cargo test`: 49/49 tests passed.
- Clean build without warnings.
- Clicking the link in the footer opens the repository in the user's default browser.

## Deviations from Assessment

Incorporated user's specific text wording: `made by ijash. more at <github>` styled in italics.

## Follow-ups

- Run `/speckit-bug-test slug=ghb`.
