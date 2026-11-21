# Tasks: Cross-Platform CI & Automated GitHub Releases (Feature 009)

**Input**: Design documents from `specs/009-cross-platform-ci-release/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

## Format: `- [ ] [TaskID] [P?] [Story?] Description with file path`

- **[P]**: Parallelizable (distinct files, independent scope)
- **[Story]**: User story identifier ([US1], [US2], [US3])

---

## Phase 1: Setup

**Purpose**: Workflow directory initialization and syntax validation setup.

- [X] T001 Ensure `.github/workflows` directory exists and configure YAML syntax validation command in `specs/009-cross-platform-ci-release/quickstart.md`

---

## Phase 2: Foundational (Workflow Skeleton & Quota Triggers)

**Purpose**: Establish workflow trigger boundaries, permissions, and top-level schema per `contracts/release-workflow-contract.md`.

- [X] T002 Define `.github/workflows/release.yml` skeleton with restrictive triggers (`push.tags: ['v*']` and `workflow_dispatch`), workflow name, and base permissions

---

## Phase 3: User Story 1 - Automated Cross-Platform Release Distribution via Git Tags (Priority: P1) 🎯 MVP

**Goal**: Automatically build, test, package, checksum, and publish standalone binaries for Linux, Windows, and macOS upon `v*` tag push.

**Independent Test**: Push an official version tag (`v*`) or simulate tag trigger conditions; confirm all 3 OS jobs compile, pass tests, generate artifacts, create `SHA256SUMS.txt`, and invoke `softprops/action-gh-release@v2`.

- [X] T003 [US1] Implement Linux (`ubuntu-latest`) matrix build step in `.github/workflows/release.yml` installing `libgtk-3-dev`, `libasound2-dev`, `libdbus-1-dev`, `xvfb`, running `cargo test` and `./tests/gui/run_gui_tests.sh`, building release binary, asserting `< 2.5 MB` size limit (`test $(stat -c%s target/release/musicbox) -lt 2621440`), packaging `musicbox-linux-x86_64.tar.gz`, and uploading artifact
- [X] T004 [US1] Implement Windows (`windows-latest`) matrix build step in `.github/workflows/release.yml` running `cargo test --lib`, building release binary, packaging `musicbox-windows-x86_64.zip` via PowerShell `Compress-Archive`, and uploading artifact
- [X] T005 [US1] Implement macOS (`macos-latest` ARM64) matrix build step in `.github/workflows/release.yml` running `cargo test --lib`, building release binary, packaging `musicbox-macos-arm64.tar.gz`, and uploading artifact
- [X] T006 [US1] Implement publication job (`release`) in `.github/workflows/release.yml` with `needs: [build]`, `if: startsWith(github.ref, 'refs/tags/v')`, `permissions: contents: write`, downloading all 3 archives, computing `SHA256SUMS.txt`, and publishing release via `softprops/action-gh-release@v2` with `generate_release_notes: true`

---

## Phase 4: User Story 2 - On-Demand CI Matrix Verification & Artifact Staging via Manual Dispatch (Priority: P2)

**Goal**: Enable running the full 3-OS build matrix manually via `workflow_dispatch` to stage downloadable workflow artifacts without publishing public GitHub Releases.

**Independent Test**: Trigger workflow via `workflow_dispatch`; verify all 3 OS build jobs succeed and upload artifacts while the `release` publication job is skipped.

- [X] T007 [US2] Verify and enforce conditional execution in `.github/workflows/release.yml` ensuring manual `workflow_dispatch` uploads build artifacts but strictly bypasses the `release` publication job

---

## Phase 5: User Story 3 - Quota-Friendly CI Guardrails & Developer Release Documentation (Priority: P3)

**Goal**: Protect private repository runner minutes using `Swatinem/rust-cache@v2` and provide concise release instructions in `README.md`.

**Independent Test**: Verify cache step presence across all 3 OS matrix jobs, and confirm `README.md` documents exact release tagging commands.

- [X] T008 [P] [US3] Integrate `Swatinem/rust-cache@v2` in every matrix build job in `.github/workflows/release.yml`
- [X] T009 [P] [US3] Add "Creating a Release" section in `README.md` documenting git tag creation, push procedure, and GitHub Actions release verification

---

## Phase 6: Polish & Quality Gates

**Purpose**: Syntax linting, local Linux release gate validation, and cross-artifact consistency checks.

- [X] T010 Validate YAML syntax of `.github/workflows/release.yml` using `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"`
- [X] T011 Run local Linux release simulation (`cargo test`, `./tests/gui/run_gui_tests.sh`, `cargo build --release`, binary size verification `< 2.5 MB`) per `specs/009-cross-platform-ci-release/quickstart.md`

---

## Dependencies & Execution Order

```
[Phase 1: Setup] (T001)
       │
       ▼
[Phase 2: Foundational] (T002)
       │
       ▼
[Phase 3: User Story 1 (P1) MVP]
  (T003 Linux Build Matrix)
  (T004 Windows Build Matrix)
  (T005 macOS Build Matrix)
       │
       ▼
  (T006 Release Publication Job)
       │
       ▼
[Phase 4: User Story 2 (P2)] (T007 Dispatch Verification)
       │
       ▼
[Phase 5: User Story 3 (P3)] (T008 Rust-Cache [P], T009 README Docs [P])
       │
       ▼
[Phase 6: Polish & Quality Gates] (T010 Syntax Validation, T011 Local Simulation)
```

### Parallel Opportunities

- In Phase 5: T008 (`.github/workflows/release.yml`) and T009 (`README.md`) can execute in parallel.
- In Phase 3: Matrix definitions (T003, T004, T005) represent distinct runner platforms and can be configured concurrently within the matrix definition.

---

## Implementation Strategy

### MVP First (User Story 1 Only)
1. Complete T001 and T002 to scaffold the workflow skeleton.
2. Complete T003–T006 to establish the 3-OS build matrix and automated GitHub Release publishing.
3. Validate MVP by simulating the Linux build gates locally and checking workflow syntax.

### Incremental Delivery
1. Foundation & MVP: Multi-OS build matrix + automated release on tag.
2. US2 verification: Ensure `workflow_dispatch` acts as a non-publishing smoke test.
3. US3 polish: Optimize compilation times via `rust-cache` and document release procedures in `README.md`.
