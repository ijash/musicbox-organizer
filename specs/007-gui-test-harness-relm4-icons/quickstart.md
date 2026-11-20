# Quickstart: Automated GUI Test Harness & Relm4 Icons Validation

## Validation Scenarios

### 1. Headless GUI Test Execution
```bash
./tests/gui/run_gui_tests.sh
```
Expected: `xvfb-run` spawns headless display, runs `gui_tests` exercising button clicks, reordering, removal, and theme toggling; passes with 0 failures in <10 seconds.

### 2. Standalone Binary Rebuild & Size Audit
```bash
cargo build --release
ls -lh target/release/musicbox
```
Expected: Binary compiles cleanly with embedded Relm4 icons, remaining <2.5 MB in size (per Constitution Principle I).

### 3. Normal Test Suite Regression Pass
```bash
cargo test
```
Expected: All existing unit and integration tests (49/49) pass cleanly.
