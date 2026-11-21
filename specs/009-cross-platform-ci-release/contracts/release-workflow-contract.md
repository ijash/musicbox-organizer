# Contract: GitHub Actions Release Workflow (`.github/workflows/release.yml`)

## 1. File Specification

- **Path**: `.github/workflows/release.yml`
- **Name**: `Release & Cross-Platform CI`
- **Schema**: GitHub Actions Workflow Schema v2

## 2. Trigger Specification

```yaml
on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:
```

- **Invariants**:
  - Pushes to branches (`main`, `Development`, `specs/*`) MUST NOT trigger this workflow.
  - Pull requests MUST NOT trigger this workflow.
  - Only git tags prefixed with `v` (e.g. `v0.1.0`, `v1.0.0-rc1`) or manual API/UI dispatch triggers execution.

## 3. Job Specifications

### Job 1: `build` (Matrix Strategy)

```yaml
jobs:
  build:
    name: Build (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        include:
          - os: ubuntu-latest
            artifact_name: musicbox-linux-x86_64.tar.gz
            target_asset: musicbox-linux-x86_64.tar.gz
          - os: windows-latest
            artifact_name: musicbox-windows-x86_64.zip
            target_asset: musicbox-windows-x86_64.zip
          - os: macos-latest
            artifact_name: musicbox-macos-arm64.tar.gz
            target_asset: musicbox-macos-arm64.tar.gz
```

#### Steps per OS:
1. `actions/checkout@v4`
2. Rust toolchain setup (stable)
3. `Swatinem/rust-cache@v2`
4. OS-specific preparation and testing:
   - **Linux**:
     - `sudo apt-get update && sudo apt-get install -y libgtk-3-dev libasound2-dev libdbus-1-dev xvfb`
     - `cargo test --verbose`
     - `chmod +x ./tests/gui/run_gui_tests.sh && ./tests/gui/run_gui_tests.sh`
     - `cargo build --release`
     - Size assertion: file size of `target/release/musicbox` must be strictly `< 2621440` bytes (2.5 MB). Exit code non-zero if >= 2.5 MB.
     - Compress: `tar -czvf musicbox-linux-x86_64.tar.gz -C target/release musicbox`
   - **Windows**:
     - `cargo test --lib --verbose`
     - `cargo build --release`
     - Compress: `Compress-Archive -Path target/release/musicbox.exe -DestinationPath musicbox-windows-x86_64.zip`
   - **macOS**:
     - `cargo test --lib --verbose`
     - `cargo build --release`
     - Compress: `tar -czvf musicbox-macos-arm64.tar.gz -C target/release musicbox`
5. `actions/upload-artifact@v4`:
   - `name: ${{ matrix.artifact_name }}`
   - `path: ${{ matrix.artifact_name }}`

---

### Job 2: `release` (Publication)

```yaml
  release:
    name: Publish GitHub Release
    needs: [build]
    if: startsWith(github.ref, 'refs/tags/v')
    runs-on: ubuntu-latest
    permissions:
      contents: write
```

#### Steps:
1. `actions/download-artifact@v4` with `merge-multiple: true`
2. Generate SHA256 checksums:
   - `sha256sum musicbox-linux-x86_64.tar.gz musicbox-windows-x86_64.zip musicbox-macos-arm64.tar.gz > SHA256SUMS.txt`
3. `softprops/action-gh-release@v2`:
   - `files`:
     - `musicbox-linux-x86_64.tar.gz`
     - `musicbox-windows-x86_64.zip`
     - `musicbox-macos-arm64.tar.gz`
     - `SHA256SUMS.txt`
   - `generate_release_notes: true`
