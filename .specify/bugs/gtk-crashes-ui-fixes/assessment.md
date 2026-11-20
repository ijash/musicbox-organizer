# Bug Assessment: GTK UI Crashes on Move Down/Edit ID3, Logo Left Spacing, and Missing Remove Track

- **Slug**: gtk-crashes-ui-fixes
- **Created**: 2026-11-20
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

> "ada beberapa bug yang udh saya tes:
> - bug saat move down urutan. window langsung ke close semua.
> - bug saat keluar dari edit idv3. window langsung ke close semua.
> - logo tidak nempel ama judul di atas. nempelnya ke kiri window banget.
> - tidak ada remove mp3 dari list. cuma ada add mp3."

## Symptom

1. **Move Down Crash**: When clicking "▼ Down" on a selected row, the application aborts/exits immediately, closing all windows.
2. **Edit ID3 Crash**: When editing Title or Artist inline and finishing (Enter/focus out), the app crashes and closes all windows.
3. **Logo Header Alignment**: The 24x24 logo is pinned to the far-left border of the window while the title label expands across the center, leaving awkward whitespace between the logo and "MusicBox Organizer".
4. **Missing Remove Feature**: The toolbar only has `+ Add MP3` without any `✖ Remove` button to delete unwanted tracks from the playlist.

## Reproduction

1. Launch `cargo run`.
2. Add tracks (`+ Add MP3` or `Scan Folder`).
3. Select a track and click `▼ Down` -> Window closes (crash).
4. Double-click title to edit inline, type new text and press Enter -> Window closes (crash).
5. Look at header: Logo is aligned to far left margin (`pack_start(logo, false, false, 0)`), while `project_label` has `expand: true`, separating them.
6. Look at toolbar: No button exists to remove tracks.

## Suspected Code Paths

- `src/ui/components/playlist_table.rs:130-155` — `cell_title.connect_edited`: Mutates `project` and immediately invokes `refresh_table()`, which clears and rebuilds the `ListStore` inside the GTK signal callback of the cell renderer that is currently handling focus/editing. In GTK, clearing a tree view's model while an editing cell is committing its edit causes a fatal GTK assertion panic (`gtk_tree_view_...: assertion failed`), closing the process.
- `src/ui/components/playlist_table.rs:180-240` — `move_down_btn.connect_clicked`: After swapping rows and re-creating the tree path, calling `refresh_table()` resets the model, followed by `tree_view.scroll_to_cell(Some(&new_path), None, false, 0.0, 0.0)`. If the row indices or paths are momentarily invalid or unmapped, `scroll_to_cell` triggers a fatal unwrap/GTK crash.
- `src/ui/components/header.rs:20-40` — `header_box.pack_start(&project_label, true, true, 0)` sets `expand: true` on the label, which pushes the label center-right while the logo sits at far left. Packaging them together inside a dedicated `title_group = GtkBox::new(Horizontal, 6)` with `expand: false` keeps the logo snugly attached next to the title text.
- `src/ui/components/playlist_table.rs:25-50` — Toolbar only has `scan_dir_btn`, `add_track_btn`, `move_up_btn`, `move_down_btn`, etc., missing a `- Remove` or `✖ Remove` button.

## Root Cause Hypothesis

**Confidence: High**

1. **Signal Re-entrancy / Model Invalidation Panic**:
   In `cell_title.connect_edited` and `cell_artist.connect_edited`, calling `refresh_table()` synchronously clears the `ListStore` (`list_store.clear()`) while GTK's `CellRendererText` is still executing its editing-done teardown handler. Clearing the active model while the renderer finishes triggers a fatal GTK assertion, terminating the application.
   *Fix*: Update the specific row's values in `list_store` directly (or schedule `refresh_table` via `glib::idle_add_local`) without destroying the model during the callback.
2. **Move Down Panic**:
   `scroll_to_cell` or `select_path` crashes if `new_idx` path translation encounters unrendered rows immediately after `list_store.clear()`.
   *Fix*: Perform direct `list_store.swap()` or update row values in place instead of wiping the entire store on every move.
3. **Logo Spacing**:
   `header_box` packs the logo and label as independent siblings with the label set to expand, leaving a large gap between the logo and the title.
   *Fix*: Group the logo and project title together inside an inner `GtkBox` with tight 6px spacing.
4. **Missing Remove Feature**:
   `playlist_table.rs` simply lacks a `remove_btn` to delete the selected row from `project.tracks`.
   *Fix*: Add a `- Remove` button that deletes the selected track, updates subsequent indices, syncs the session, and refreshes the table.

## Proposed Remediation

1. In `src/ui/components/header.rs`:
   - Group the 24px logo image and `project_label` into an inner `title_box` (`GtkBox::new(Orientation::Horizontal, 6)`).
   - Pack `title_box` into `header_box` with `expand: false` so the logo is positioned right next to the title.
2. In `src/ui/components/playlist_table.rs`:
   - Fix inline editing crash: When `cell_title.connect_edited` or `cell_artist.connect_edited` fires, update the model row values in place (`list_store.set_value`) rather than calling `list_store.clear()`.
   - Fix move down crash: Guard `tree_view.selection().select_path` and `scroll_to_cell` to ensure safe tree path handling without GTK aborts.
   - Add `- Remove` button in the toolbar: Deletes selected row from `project.tracks`, recalculates indices (`0..N`), syncs `session_state`, and refreshes view safely.

**Files likely to change**:
- `src/ui/components/header.rs`
- `src/ui/components/playlist_table.rs`

## Risks & Considerations

- Low risk. Replacing destructive `list_store.clear()` calls with targeted row updates improves performance and prevents GTK crashes.

## Open Questions

None.
