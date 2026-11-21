# Quickstart & Verification Guide: Feature 009 (Cross-Platform CI & Releases)

## 1. Local Pre-Commit Validation

Before pushing workflow updates or new tags, run local checks to ensure the workflow definition is clean and functional:

### Step 1: Validate Workflow YAML Syntax
Validate that `.github/workflows/release.yml` conforms to standard YAML parsing rules without indentation errors:

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))" && echo "YAML syntax is valid."
```

### Step 2: Validate Local Release Gate (Linux)
Simulate the exact command sequence executed by the Linux runner:

```bash
# 1. Run all unit and integration tests
cargo test

# 2. Run headless GUI interaction tests
./tests/gui/run_gui_tests.sh

# 3. Build release binary
cargo build --release

# 4. Verify Constitution Principle I size gate (<2.5 MB / 2,621,440 bytes)
BIN_SIZE=$(stat -c%s target/release/musicbox)
echo "Binary size: $BIN_SIZE bytes"
[ "$BIN_SIZE" -lt 2621440 ] && echo "Principle I gate passed!" || (echo "Gate failed" && exit 1)

# 5. Verify packaging
tar -czvf /tmp/musicbox-linux-x86_64.tar.gz -C target/release musicbox
tar -tzvf /tmp/musicbox-linux-x86_64.tar.gz
```

---

## 2. On-Demand CI Dispatch (Smoke Testing)

To test the entire 3-OS build matrix without cutting a public release:

```bash
# Using GitHub CLI
gh workflow run release.yml
gh run watch
```

**Expected Outcome**:
- `ubuntu-latest`, `windows-latest`, and `macos-latest` jobs execute in parallel.
- All 3 jobs succeed and upload their respective artifacts.
- The `release` job is skipped because the run was not triggered by a `v*` tag.

---

## 3. Creating an Official GitHub Release

When ready to publish a new version:

```bash
# 1. Create annotated git tag
git tag -a v0.2.0 -m "Release v0.2.0"

# 2. Push tag to origin
git push origin v0.2.0
```

**Expected Outcome**:
- Workflow triggers automatically on tag push.
- All 3 OS builds complete.
- The `release` job triggers, downloads all 3 packages, generates `SHA256SUMS.txt`, and publishes a GitHub Release under tag `v0.2.0` with assets and auto-generated release notes.
