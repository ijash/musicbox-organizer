# Research: Cross-Platform CI & Automated GitHub Releases (Feature 009)

## 1. Triggers & Private Repository Quota Optimization

- **Decision**: Configure `on.push.tags: ['v*']` and `on.workflow_dispatch` only.
- **Rationale**:
  - Private GitHub repositories have finite free runner minutes (2,000 min/month). Windows runners consume 2x minutes and macOS runners consume 10x minutes.
  - Routine branch pushes (`main`, `Development`, feature branches) and pull requests MUST NOT invoke the 3-OS matrix.
  - Manual `workflow_dispatch` allows on-demand matrix testing and artifact inspection prior to tagging.
  - Pushing official version tags (`v*`, e.g., `v0.2.0`) builds the matrix and proceeds to publish a GitHub Release.
- **Alternatives Considered**:
  - Running on every PR: Rejected due to runner quota waste on multi-platform VMs when local Linux test harness is primary.
  - Scheduled nightly builds: Rejected as unnecessary overhead for current project stage.

## 2. Rust Build Matrix & Caching

- **Decision**: Use `Swatinem/rust-cache@v2` on each matrix job runner.
- **Rationale**:
  - `Swatinem/rust-cache` automatically hashes `Cargo.lock` and caches `target/` and cargo registry cache, cutting workflow runtime by 50–70% on subsequent runs.
  - Native support across Linux, Windows, and macOS without extra configuration.
- **Alternatives Considered**:
  - `actions/cache`: Rejected because manual cache key setup is brittle across multiple OSes compared to `rust-cache`.

## 3. Platform-Specific Build & Packaging Details

### Linux (`ubuntu-latest`)
- **System Dependencies**:
  ```bash
  sudo apt-get update
  sudo apt-get install -y libgtk-3-dev libasound2-dev libdbus-1-dev xvfb
  ```
- **Verification Gates**:
  - `cargo test`
  - `./tests/gui/run_gui_tests.sh` (executes GUI test suite off-screen under `xvfb-run`)
  - `cargo build --release`
  - Binary Size Gate (Constitution Principle I):
    ```bash
    BIN_SIZE=$(stat -c%s target/release/musicbox)
    echo "Linux release binary size: $BIN_SIZE bytes"
    if [ "$BIN_SIZE" -ge 2621440 ]; then
      echo "Error: Binary size exceeds 2.5 MB ceiling" >&2
      exit 1
    fi
    ```
- **Packaging**:
  - `tar -czvf musicbox-linux-x86_64.tar.gz -C target/release musicbox`

### Windows (`windows-latest`)
- **Verification Gates**:
  - `cargo test --lib` (tests core cross-platform modules without native GTK3 display)
  - `cargo build --release`
- **Packaging**:
  - PowerShell:
    ```powershell
    Compress-Archive -Path target/release/musicbox.exe -DestinationPath musicbox-windows-x86_64.zip
    ```

### macOS (`macos-latest` - Apple Silicon ARM64)
- **Verification Gates**:
  - `cargo test --lib` (tests core cross-platform modules)
  - `cargo build --release`
- **Packaging**:
  - `tar -czvf musicbox-macos-arm64.tar.gz -C target/release musicbox`

## 4. Release Publication & Checksum Generation

- **Decision**: Dedicated downstream job `release` with `needs: [build]`, conditioned on `if: startsWith(github.ref, 'refs/tags/v')`.
- **Permissions**: `permissions: contents: write`.
- **Artifact Coordination**:
  - Each build job uploads its compressed archive via `actions/upload-artifact@v4`.
  - The `release` job downloads all archives into a common staging directory via `actions/download-artifact@v4` with `merge-multiple: true`.
  - Generates `SHA256SUMS.txt`:
    ```bash
    sha256sum musicbox-linux-x86_64.tar.gz musicbox-windows-x86_64.zip musicbox-macos-arm64.tar.gz > SHA256SUMS.txt
    ```
  - Publishes using `softprops/action-gh-release@v2`:
    ```yaml
    with:
      files: |
        musicbox-linux-x86_64.tar.gz
        musicbox-windows-x86_64.zip
        musicbox-macos-arm64.tar.gz
        SHA256SUMS.txt
      generate_release_notes: true
    ```
- **Alternatives Considered**:
  - Publishing release assets in each matrix job: Rejected because `action-gh-release` would create race conditions and cannot produce a single unified `SHA256SUMS.txt`.

## 5. Documentation & Release Workflow

- **Decision**: Add a dedicated "Creating a Release" section to `README.md`.
- **Command Steps**:
  1. `git tag -a v0.2.0 -m "Release v0.2.0"`
  2. `git push origin v0.2.0`
- **Verification**: Document manual verification via `gh workflow run release.yml` or GitHub Actions web UI.
