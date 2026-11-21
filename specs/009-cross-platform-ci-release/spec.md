# Feature Specification: Cross-Platform CI & Automated GitHub Releases

**Feature Branch**: `009-cross-platform-ci-release`

**Created**: 2026-11-21

**Status**: Draft

**Input**: User description: "Implement Feature 009: Cross-Platform CI & Automated GitHub Releases via `.github/workflows/release.yml` for `musicbox-organizer` (quota-friendly for private repos: manual `workflow_dispatch` + `v*` tag releases)."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Automated Cross-Platform Release Distribution via Git Tags (Priority: P1)

Maintainers and users want automated, multi-platform binary compilation and distribution whenever an official version tag (`v*`) is pushed to the repository. The workflow automatically builds optimized release executables across Linux (`x86_64`), Windows (`x86_64`), and macOS (`arm64`), validates all automated tests and constitution binary size limits (< 2.5 MB), generates cryptographic checksums (`SHA256SUMS.txt`), and publishes a draft-free public GitHub Release with automated release notes and downloadable assets.

**Why this priority**: Eliminates manual cross-compilation errors and ensures end-users can reliably download verified standalone executables for their platform immediately upon release.

**Independent Test**: Can be tested by pushing a test version tag (e.g. `v0.2.0-rc1`) or simulating tag-triggered job conditions, confirming all 3 operating systems build and the release job publishes all 3 archives plus checksums.

**Acceptance Scenarios**:

1. **Given** a pushed git tag matching `v*` (e.g., `v0.2.0`), **When** the workflow triggers, **Then** all three operating system jobs (Linux, Windows, macOS) execute in parallel and compile release binaries.
2. **Given** Linux release build, **When** compilation finishes, **Then** the workflow validates that `target/release/musicbox` is strictly `< 2.5 MB` and packages `musicbox-linux-x86_64.tar.gz`.
3. **Given** Windows and macOS release builds, **When** compilation finishes, **Then** `musicbox.exe` is packaged as `musicbox-windows-x86_64.zip` and macOS binary is packaged as `musicbox-macos-arm64.tar.gz`.
4. **Given** all 3 OS build jobs succeed on a tag push, **When** the release publication job executes, **Then** `SHA256SUMS.txt` is generated and a GitHub Release is published containing all 3 archives, checksums, and auto-generated changelog notes.

---

### User Story 2 - On-Demand CI Matrix Verification & Artifact Staging via Manual Dispatch (Priority: P2)

Developers and maintainers want the ability to run the full cross-platform build matrix on demand without publishing a public GitHub Release. Triggering a manual `workflow_dispatch` builds and tests all three platforms and uploads the resulting archives as workflow artifacts for smoke-testing and verification, preserving GitHub Actions quota by avoiding automatic runs on routine commits.

**Why this priority**: Allows verifying that cross-platform code changes compile and pass tests across Windows and macOS before cutting an official release tag, without leaking unreleased binaries as public releases.

**Independent Test**: Can be tested by triggering the workflow manually via GitHub Actions UI / CLI `workflow_dispatch` and confirming that 3 OS jobs succeed and artifacts are uploaded, while the GitHub Release publication step is skipped.

**Acceptance Scenarios**:

1. **Given** a manual `workflow_dispatch` trigger, **When** the workflow runs, **Then** all three platform build matrix jobs execute.
2. **Given** successful completion of matrix builds on `workflow_dispatch`, **When** the release job evaluates its condition, **Then** the release job skips publication and does not create a GitHub Release.
3. **Given** completed matrix runs, **When** inspecting the workflow summary, **Then** build artifacts for all 3 platforms are available for download by repository collaborators.

---

### User Story 3 - Quota-Friendly CI Guardrails & Developer Release Documentation (Priority: P3)

Maintainers of private or resource-bounded repositories need strict quota protection so that standard daily branch pushes (`git push origin main` or feature branches) do not spin up expensive Windows and macOS runner minutes. Additionally, repository documentation must provide clear instructions explaining how to tag and cut an automated release.

**Why this priority**: Prevents rapid exhaustion of private GitHub Actions runner quotas while establishing a standardized, repeatable release procedure for all contributors.

**Independent Test**: Can be tested by pushing regular commits to branches without tags and verifying zero Windows/macOS runner jobs are scheduled, and by reviewing the release documentation in `README.md`.

**Acceptance Scenarios**:

1. **Given** a normal commit push to a feature branch or `Development`, **When** pushed to GitHub, **Then** the release workflow does not trigger.
2. **Given** build jobs on all three platforms, **When** compiling dependencies, **Then** `Swatinem/rust-cache@v2` caches dependencies to minimize subsequent build runtimes.
3. **Given** a developer reading `README.md`, **When** navigating to the release instructions, **Then** a concise section documents the exact git tag and push commands (`git tag v* && git push origin v*`).

---

### Edge Cases

- **Build failure on one OS**: If any platform build (e.g. macOS runner) fails its compilation or tests, the downstream release job MUST NOT execute and no partial or broken GitHub Release is published.
- **Binary size ceiling exceeded**: If Linux release binary size reaches or exceeds 2.5 MB, the Linux job MUST fail immediately with an explicit error, preventing an oversized binary from being published per Constitution Principle I.
- **Workflow syntax invalidity**: The YAML workflow file MUST pass GitHub Actions linting and schema validation without malformed keys, incorrect permissions, or unescaped expressions.
- **Missing or non-semver tag**: If a tag does not begin with `v` (e.g. `test-tag`), the workflow MUST ignore it.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Workflow MUST trigger exclusively on `push` of tags matching `v*` and on manual `workflow_dispatch`.
- **FR-002**: Workflow MUST NOT trigger on routine branch pushes or pull requests, protecting runner minute quotas.
- **FR-003**: All workflow jobs MUST use `Swatinem/rust-cache@v2` to cache Rust dependency artifacts and speed up repeated builds.
- **FR-004**: The Linux job (`ubuntu-latest`) MUST install required system libraries (`libgtk-3-dev`, `libasound2-dev`, `libdbus-1-dev`, `xvfb`), execute unit tests (`cargo test`), execute the headless GUI interaction test harness (`./tests/gui/run_gui_tests.sh`), build an optimized release binary (`cargo build --release`), verify the executable size is `< 2.5 MB`, and package the binary as `musicbox-linux-x86_64.tar.gz`.
- **FR-005**: The Windows job (`windows-latest`) MUST run library unit tests (`cargo test --lib`), build an optimized release binary (`cargo build --release`), and package the executable as `musicbox-windows-x86_64.zip`.
- **FR-006**: The macOS job (`macos-latest`) MUST run library unit tests (`cargo test --lib`), build an optimized release binary for ARM64 (`cargo build --release`), and package the executable as `musicbox-macos-arm64.tar.gz`.
- **FR-007**: Matrix jobs MUST upload their packaged archives as workflow artifacts so they can be consumed by downstream steps or inspected during manual dispatch runs.
- **FR-008**: The release publishing job MUST depend on all three OS matrix jobs (`needs: [...]`), require `permissions: contents: write`, and execute ONLY when the ref is an official release tag (`startsWith(github.ref, 'refs/tags/v')`).
- **FR-009**: The release publishing job MUST download all three platform packages, compute cryptographic SHA256 checksums into `SHA256SUMS.txt`, and publish the release using `softprops/action-gh-release@v2` with `generate_release_notes: true`.
- **FR-010**: Project documentation in `README.md` MUST include a "Creating a Release" section detailing the version tagging and push steps.

### Key Entities

- **Release Workflow (`.github/workflows/release.yml`)**: GitHub Actions workflow orchestrating the 3-OS build matrix, Constitution validation gates, artifact packaging, and GitHub Release publication.
- **Platform Release Package**: Compressed distribution archive containing the compiled platform-native executable (`musicbox-linux-x86_64.tar.gz`, `musicbox-windows-x86_64.zip`, `musicbox-macos-arm64.tar.gz`).
- **Checksum Manifest (`SHA256SUMS.txt`)**: Text manifest containing cryptographic SHA-256 hashes of all three packaged archives.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Multi-platform release pipeline completes execution across all 3 platforms in under 15 minutes total runner duration.
- **SC-002**: Regular code pushes consume exactly zero Windows and macOS Actions runner minutes.
- **SC-003**: 100% of published Linux binaries verify against Constitution Principle I (< 2.5 MB release size limit).
- **SC-004**: Official GitHub Releases include all three target platform packages and verifiable `SHA256SUMS.txt`.
- **SC-005**: `README.md` provides complete, error-free documentation for releasing a new version in < 3 command steps.

## Assumptions

- Repository has GitHub Actions enabled with default permissions permitting `contents: write` for release actions when configured in workflow.
- Linux build runs on Ubuntu 22.04 / 24.04 (`ubuntu-latest`) using system GTK-3, ALSA, and D-Bus packages.
- Windows and macOS runners do not have GTK3 installed by default; therefore, library test verification (`cargo test --lib`) validates core cross-platform modules while headless GTK GUI tests run on Linux under `xvfb`.
- Apple Silicon (`arm64`) is the standard architecture for modern `macos-latest` runners.
