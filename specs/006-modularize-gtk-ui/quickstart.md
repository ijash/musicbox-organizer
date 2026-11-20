# Quickstart: Modular UI Component Validation

## Validation Scenarios

### 1. Compile & Unit Tests
```bash
cargo test
```
Expected outcome: All 49 existing unit and integration tests compile cleanly and pass.

### 2. GUI Launch & Layout Check
```bash
cargo run -- gui
```
Expected outcome:
- Window renders in 740x520 default geometry.
- Logo icon and title render in header.
- Table, toolbar, audio player, export panel, and footer display identically to previous monolithic version.

### 3. File Size & Line Count Audit
```bash
wc -l src/ui/gtk_ui.rs src/ui/components/*.rs
```
Expected outcome:
- `src/ui/gtk_ui.rs` reduced to <200 lines.
- All files under `src/ui/components/` under 300 lines each.
