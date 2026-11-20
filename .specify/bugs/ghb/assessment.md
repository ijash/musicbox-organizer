# Bug Assessment: Replace LinkButton with Plain Clickable Text Link in Bottom Footer

- **Slug**: ghb
- **Created**: 2026-11-20
- **Source**: pasted text
- **Verdict**: valid
- **Severity**: low

## Report (verbatim or summarized)

> "- githubnya jg aku maunya jangan lah button. bagusnya ada link. ganti ke teks dengan link di bawah app aja. slug=ghb"

## Symptom

The GitHub link at the bottom of the window is rendered using `gtk::LinkButton`, which inherits native GTK button styles (borders, padding, background highlights, and button box rendering) rather than displaying as a simple, clean, inline text link (`Label` with markup/underline).

## Reproduction

1. Run `cargo run`.
2. Inspect the bottom footer of the window.
3. Observed: The link is styled like a button (`LinkButton` with borders/background). Expected: A clean, discrete text label hyperlink (`<a href="...">GitHub</a>`) without button chrome or padding.

## Suspected Code Paths

- `src/ui/gtk_ui.rs:350-360` — `LinkButton::with_label(...)` creates a button widget.
- `src/ui/gtk_ui.rs:30-115` — CSS stylesheet applies button styling to all button classes including `LinkButton`.

## Root Cause Hypothesis

**Confidence: High**

`LinkButton` in GTK3 is a subclass of `GtkButton`, so it carries full button styling (frames, background, border) in both light and dark CSS themes. Replacing it with a `gtk::Label` using Pango markup `<a href="https://github.com/ijash/musicbox-organizer">GitHub</a>` produces a clean, true plain-text hyperlink that opens the default browser without any button borders.

## Proposed Remediation

**Preferred**:
In `src/ui/gtk_ui.rs`:
1. In the bottom `footer_box`, replace `LinkButton` with a `gtk::Label`.
2. Configure the label with `markup`: `<a href="https://github.com/ijash/musicbox-organizer">MusicBox Organizer on GitHub</a>`.
3. Set `track_visited_links = false` so it retains uniform theme colors.
4. Center or right-align the link cleanly in the bottom footer.

**Files likely to change**:
- `src/ui/gtk_ui.rs`

**Tests to add or update**:
- Run `cargo test` and verify release build.

## Risks & Considerations

- Zero risk. `Label` with standard markup hyperlinks is supported natively in GTK3 and handles system browser launching automatically.

## Open Questions

None.
