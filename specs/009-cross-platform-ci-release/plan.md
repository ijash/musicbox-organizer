# Implementation Plan: Cross-Platform CI & Automated GitHub Releases (Feature 009)

**Branch**: `009-cross-platform-ci-release` | **Date**: 2026-11-21 | **Spec**: [specs/009-cross-platform-ci-release/spec.md](spec.md)

**Input**: Feature specification from `specs/009-cross-platform-ci-release/spec.md`

## Summary

Implement automated multi-platform CI and release distribution via `.github/workflows/release.yml`. Protect runner quotas by triggering matrix builds exclusively on `v*` tag pushes and manual `workflow_dispatch`. Compile, test, and package standalone binaries across Linux (`x86_64`), Windows (`x86_64`), and macOS (`arm64`), strictly enforcing Constitution Principle I (<2.5 MB binary size ceiling), computing cryptographic `SHA256SUMS.txt`, and publishing official GitHub Releases via `softprops/action-gh-release@v2`.

## Technical Context

**Language/Version**: GitHub Actions YAML / Rust 2021 edition (stable)

**Primary Dependencies**:
- Actions: `actions/checkout@v4`, `Swatinem/rust-cache@v2`, `actions/upload-artifact@v4`, `actions/download-artifact@v4`, `softprops/action-gh-release@v2`
- Linux build deps: `libgtk-3-dev`, `libasound2-dev`, `libdbus-1-dev`, `xvfb`

**Storage**: GitHub Actions artifacts (staged archives) & GitHub Releases (permanent public release assets)

**Testing**: `cargo test`, `cargo test --lib`, `./tests/gui/run_gui_tests.sh` under `xvfb-run`

**Target Platform**: Linux (`ubuntu-latest`), Windows (`windows-latest`), macOS (`macos-latest` ARM64)

**Project Type**: Desktop application CI/CD and release automation

**Performance Goals**: Full 3-OS build matrix and release publication completed in <15 minutes total runner time

**Constraints**:
- Strict quota protection: zero Windows/macOS runner minutes on regular branch pushes
- Principle I binary size gate: Linux release binary must be `< 2.5 MB`
- Reliable multi-archive integrity with `SHA256SUMS.txt`

**Scale/Scope**: 1 workflow file (`.github/workflows/release.yml`) + `README.md` release instructions

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Principle I: Lightweight First**: PASS. Linux build asserts binary size strictly `< 2.5 MB` (`2621440` bytes) before packaging.
- **Principle II: Cross-Platform Native**: PASS. Native standalone executables built for Linux, Windows, and macOS.
- **Principle IV: Offline-First & Portable**: PASS. Releases distribute portable standalone archives with zero runtime installer requirements.
- **Principle V: Testable Core Logic & GUI Interaction Verification**: PASS. Linux runner executes unit tests and full headless GUI test suite under `xvfb-run` (`./tests/gui/run_gui_tests.sh`).
- **Principle VI: Git-First Workflow & Conventional Commits**: PASS. Workflow triggers on standard semver git tags (`v*`).

## Project Structure

### Documentation (this feature)

```text
specs/009-cross-platform-ci-release/
├── plan.md              # Implementation plan
├── research.md          # Trigger, matrix, caching, and release findings
├── data-model.md        # Entities, matrix definitions, state transitions
├── quickstart.md        # Local dry-run, dispatch, and release verification guide
└── contracts/
    └── release-workflow-contract.md # .github/workflows/release.yml declarative contract
```

### Source Code (repository root)

```text
.github/
└── workflows/
    └── release.yml      # Multi-platform build matrix, testing gates, and release publisher

README.md                # Added "Creating a Release" developer guide
```

**Structure Decision**: Workflow placed at standard `.github/workflows/release.yml` with clear separation of matrix build jobs (`build`) and publication job (`release`). Documentation updated in project root `README.md`.

## Complexity Tracking

*No constitutional violations. Zero complexity penalties incurred.*
