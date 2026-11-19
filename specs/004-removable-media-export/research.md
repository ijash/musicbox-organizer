# Research: USB/SD Device Enumeration & Formatting

**Feature**: 004-removable-media-export | **Date**: 2026-11-19 | **Decision Gate**: LOCKED

---

## 1. Device Enumeration: Platform Strategy

### Decision: Platform-Native APIs (No Third-Party Crate)

**Rationale**: Binary size constraint (<50MB). Platform-specific code already committed (Win32, Cocoa, GTK in Cargo.toml). Native APIs available on all three platforms; no abstraction layer needed—each platform implements directly behind `DeviceManager` trait.

### Windows: Win32 SetupAPI

**Approach**: Use `windows` crate (already dependency) + SetupAPI to enumerate disk devices.

```rust
// windows crate feature set
windows::Win32::Devices::DeviceAndDriverInstallation::{
    SetupDiGetClassDevsW,
    SetupDiEnumDeviceInfo,
    SetupDiGetDeviceInstanceIdW,
    SetupDiGetDevicePropertyW,
    DIGCF_PRESENT,
}
// + GUID_DEVCLASS_DISKDRIVE

// Enumerate: HDEVINFO → device list → filter removable (StorageDeviceNumber property)
// Capacity: via IOCTL_DISK_GET_LENGTH_INFO
// Filesystem: WMI query or registry (less reliable; defer to detection on mount)
// Disconnection: polling (SetupDiEnumDeviceInfo repeat calls, ~1s interval)
```

**Alternatives Considered**:
- WMI (more discoverable but heavier, requires COM initialization)
- Registry scan (fragile, does not reflect hot-plug)
- rusb crate (USB-only, overkill for mount enumeration)

**Chosen**: SetupAPI. Minimal, already available via `windows` crate.

---

### macOS: IOKit + DADiskArbitration

**Approach**: Use `cocoa`/`objc` (already dependency) + DADiskArbitration framework (lighter than IOKit).

```rust
// objc FFI to DADiskArbitration
// DADiskCreateFromBSDName → DADiskCopyDescription
// Extract: kDADiskDescriptionMediaNameKey, kDADiskDescriptionMediaSizeKey, 
//          kDADiskDescriptionDeviceModelKey

// Enumerate all disks → filter removable (kDADiskDescriptionMediaRemovableKey == true)
// Capacity: from DADiskCopyDescription dict
// Filesystem: statfs(mount_point) → f_fstypename
// Disconnection: DADiskAppearedCallback + DADiskDisappearedCallback (event-driven, <100ms latency)
```

**Alternatives Considered**:
- IOKit (more detailed but verbose API)
- mount points scan (statfs loop; stale after umount)
- Disk Utility CLI parsing (fragile, slow)

**Chosen**: DADiskArbitration. Native, event-driven, minimal boilerplate.

---

### Linux: udisks2 DBus (with /sys/block fallback)

**Approach**: DBus call to udisks2 service (most Linux distros have it; fallback to /sys/block parsing).

**Option A (Preferred): udisks2 DBus**
```rust
// dbus crate integration (already used implicitly; add if needed)
// org.freedesktop.UDisks2.Manager.GetBlockDevices()
// Filter: Drive.Removable == true, Block.ReadOnly == false
// Capacity, filesystem, mount path all from Properties

// Disconnection: subscribe to org.freedesktop.DBus.Properties.PropertiesChanged
// signal (event-driven, immediate)
```

**Option B (Fallback): /sys/block direct parse**
```rust
// Read /sys/block/sd* and /sys/block/mmc* → check:
// - /sys/block/sdX/removable (1 = removable)
// - /sys/block/sdX/size (sectors)
// - /proc/mounts (mount path, filesystem)
// Polling: scan every ~1s; simple but less efficient
```

**Alternatives Considered**:
- lsblk parser (CLI fragile, version-dependent)
- pyudev wrapper (Python dep, overkill)
- pure /sys parsing (no DBus; works on minimal systems)

**Chosen**: udisks2 DBus with /sys/block fallback. DBus preferred for event-driven disconnect detection; fallback for environments without udisks2 (embedded, minimal containers).

---

## 2. Formatting: FAT32 / exFAT

### Decision: Platform-Native CLI Tools (mkfs via subprocess)

**Rationale**: Pure Rust FAT32/exFAT impl unavailable or immature. Binary size = avoid embedding full filesystem lib. All three platforms have native mkfs tools. Trade: subprocess latency (~1s), acceptable for infrequent reformat operation. No network calls; offline-capable. ponytail: swap to `fatfs` crate (pure Rust) if binary bloat becomes issue after profiling.

### Windows: `format.exe` or PowerShell

```rust
// Subprocess: format /FS:FAT32 /Y X:
// or PowerShell Format-Volume -FileSystem FAT32 -Force -Confirm:$false

// Check write-protect: via SetupAPI property query before attempt
```

**Alternatives Considered**:
- fatfs crate (Rust impl; untested for exFAT, adds ~100KB)
- WinAPI FormatEx (undocumented, risky)

**Chosen**: `format.exe`. Stable, built-in, reliable.

---

### macOS: `diskutil`

```rust
// Subprocess: diskutil eraseVolume FAT32 MBRFormat /Volumes/DEVICE
// or: diskutil secureErase freespace 0 FAT32 /Volumes/DEVICE

// Check write-protect: DADiskArbitration kDADiskDescriptionMediaWriteProtectedKey
```

**Alternatives Considered**:
- IOKit formatting (verbose C FFI)
- newfs_msdos (low-level; requires superuser)
- fatfs crate (Rust; not battle-tested on macOS)

**Chosen**: `diskutil`. Apple's official tool; handles device logic.

---

### Linux: `mkfs.fat` or `mkfs.exfat`

```rust
// Subprocess: mkfs.fat -F 32 /dev/sdX1
// or: mkfs.exfat -L PLAYLIST /dev/sdX1

// Check write-protect: /sys/block/sdX/device/vendor (read-only attribute)
```

**Alternatives Considered**:
- fatfs crate (Rust; testing on Linux incomplete)
- libblkid (C library; extra dep)
- pure /sys write (unsafe, no error recovery)

**Chosen**: `mkfs.fat` / `mkfs.exfat`. Ubiquitous, battle-tested.

---

## 3. Device Disconnection Detection

### Decision: Platform-Specific Events + Polling Fallback

| Platform | Primary | Fallback | Latency |
|----------|---------|----------|---------|
| **Windows** | Polling SetupDiEnumDeviceInfo (~1s) | Monitor device node removal messages | ~1–2s |
| **macOS** | DADiskDisappearedCallback (event) | Fallback to polling if callback fails | <100ms |
| **Linux (DBus)** | PropertiesChanged signal (event) | Fallback to stat(mount_point) check | <100ms |
| **Linux (/sys)** | Polling /sys/block/sdX/ existence | — | ~1s |

**Implementation**:
- Primary: use platform events (most responsive, <5s requirement met)
- Fallback: ~1s polling loop if event channel fails
- User feedback: prompt retry within 5s before aborting export

**Rationale**: Events are free (OS delivers notification); polling is simple fallback. 5s tolerance in spec easily satisfied.

---

## 4. Binary Size Impact

| Component | Estimated Size |
|-----------|-----------------|
| SetupAPI (Win32) | 0 KB (in `windows` crate already) |
| DADiskArbitration (macOS) | 0 KB (system framework, FFI only) |
| udisks2 DBus (Linux) | +50–100 KB (if dbus crate needed) |
| subprocess overhead | <10 KB (std::process only) |
| **Total add** | ~50–100 KB |
| **Total binary post-strip** | <50 MB ✓ |

No new heavy dependencies needed. Subprocess approach avoids embedding filesystem libs.

---

## 5. Testing Strategy

**Unit Tests**:
- Device struct parsing from platform-specific outputs
- Mock DeviceManager responses
- Filesystem type detection logic

**Integration Tests** (require live hardware):
- Connect USB/SD → enumerate → verify device appears
- Reformat → verify filesystem change
- Hot-unplug → verify disconnection detected <5s

**Mocking** (for CI):
- Fake /sys/block files (Linux)
- Mock SetupAPI calls (Windows)
- Stub DADiskArbitration (macOS)

---

## 6. Crate Dependencies: Final List

**Add to Cargo.toml**:
```toml
[target.'cfg(target_os = "linux")'.dependencies]
dbus = "0.9"  # optional; use if udisks2 DBus preferred; fallback to /sys if omitted
```

**Existing (reuse)**:
- `windows` (Win32 SetupAPI)
- `cocoa` + `objc` (macOS DADiskArbitration)
- `gtk` (Linux UI; no device code dependency)

---

## 7. Unknowns Resolved

| Unknown | Resolution |
|---------|-----------|
| Best Rust crate for enumeration? | Platform-native APIs (SetupAPI/DADiskArbitration/udisks2 DBus). No third-party crate. |
| FAT32/exFAT formatting approach? | Subprocess (format.exe / diskutil / mkfs.fat/exfat). Binary size safe, offline-capable. |
| Disconnection detection? | Platform events (callback/signal) + polling fallback; <5s latency guaranteed. |

---

## 8. Next Steps

1. **Phase 1 (Design)**: Implement `DeviceManager` trait + platform-specific backends.
2. **Phase 1 (Design)**: Sketches for UI (device list widget, reformat warning dialog).
3. **Phase 2 (Implement)**: Coding per tasks.md.
4. **Phase 3 (Validate)**: Integration tests on real USB/SD devices (3 different types per spec).

**Go/No-Go**: READY. No blocking unknowns. Proceed to planning phase.
