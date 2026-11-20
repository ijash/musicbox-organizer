# Bug Fix: GTK UI Crashes on Move Down/Edit ID3, Logo Left Spacing, and Missing Remove Track

- **Slug**: gtk-crashes-ui-fixes
- **Fixed**: 2026-11-20
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

1. **Inline ID3 Editing Crash Fixed**: Replaced destructive `refresh_table()` calls in `cell_title` and `cell_artist` `connect_edited` with targeted `list_store.set_value` in-place updates. This prevents model invalidation during GTK cell editing.
2. **Move Down Crash Fixed**: Removed conflicting `scroll_to_cell` race conditions on unmapped tree paths after store reloads. Selection persists smoothly.
3. **Logo Header Spacing Fixed**: Grouped 24px logo and project label in `title_group` (`GtkBox::new(Horizontal, 6)`) packed without expanding, keeping the logo right next to the title text.
4. **Remove Track Feature Added**: Added `✖ Remove` button to the playlist toolbar allowing deletion of selected tracks with undo support and automatic sequence re-indexing.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `src/ui/components/header.rs` | modified | Grouped logo and title inside tight horizontal container |
| `src/ui/components/playlist_table.rs` | modified | Added in-place `set_value` for inline editing, removed crash-inducing scroll on move down, added `✖ Remove` button |

## Diff Highlights

```rust
// 1. Group logo and title in header.rs:
let title_group = GtkBox::new(Orientation::Horizontal, 6);
title_group.pack_start(&logo_img, false, false, 0);
title_group.pack_start(&project_label, false, false, 0);
header_box.pack_start(&title_group, false, false, 0);

// 2. In-place model update in playlist_table.rs:
let tree_path = gtk::TreePath::from_indicesv(&[idx as i32]);
if let Some(iter) = list_store.iter(&tree_path) {
    list_store.set_value(&iter, 2, &new_text.to_value());
}

// 3. Remove track button:
remove_track_btn.connect_clicked(move |_| {
    // removes track, re-indexes, records undo checkpoint, updates list_store
});
```

## Local Verification

- Commands run:
  - `cargo check`: clean compilation
  - `cargo test`: all 49 tests passed
  - `cargo build --release`: built release binary

## Deviations from Assessment

None. All four items resolved.

## Follow-ups

- Run `/speckit-bug-test slug=gtk-crashes-ui-fixes`.
