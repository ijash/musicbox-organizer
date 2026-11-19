# UI Contract: Export Dialog

## Overview
The Export Dialog is the primary user interface for the removable media export workflow. It enumerates connected USB/SD devices, allows device selection, optional reformatting, and triggers export with real-time progress feedback.

## Components

### Device List Widget
Displays all connected removable storage devices.

**Input State**:
- `devices: Vec<Device>` (from `DeviceManager::enumerate()`)
- `selected_device_id: Option<String>` (user selection)
- `is_loading: bool` (during enumeration or disconnection check)

**Output**:
- User selects device → `selected_device_id = Some(id)`
- User clicks Refresh → trigger re-enumerate

**UI Elements**:
- List container (scrollable if >5 devices)
- Per-device row:
  - Device name (e.g., "Kingston USB Drive")
  - Capacity (e.g., "8.0 GB")
  - Available space (e.g., "7.2 GB available")
  - Filesystem (e.g., "FAT32")
  - Status badge:
    - ✓ Green: Ready (mounted, writable, sufficient space)
    - ⚠ Yellow: Caution (low space, filesystem unknown)
    - ✗ Red: Error (read-only, disconnected, full)
  - Refresh button (reload device list)

**Empty State**:
```
[No removable devices detected]
Connect a USB drive or SD card to proceed.
[Refresh] button active
```

**Error States**:
```
[Device Disconnected]
Kingston USB Drive is no longer available.
[OK]
```

---

### Export Options Panel

**Controls**:

1. **Reformat Device** (checkbox)
   - Label: "Reformat device before export"
   - Default: unchecked
   - When checked:
     - Show warning: "This will erase all data on the device."
     - Require explicit confirmation before export
   - When unchecked:
     - Export directly to existing filesystem

2. **Filesystem Selection** (radio buttons, only if Reformat checked)
   - Options: FAT32 (default), exFAT
   - Default: FAT32 (maximum compatibility with older players)

3. **Export Path** (optional, advanced)
   - Text field: `/path/on/device/` (default: `/` = root)
   - Allow user to specify subdirectory (e.g., `/Playlists/`)

---

### Reformat Warning Dialog

**Trigger**: User clicks "Export" with Reformat enabled and not yet confirmed.

**Content**:
```
⚠ WARNING: Device Will Be Erased

Device: Kingston USB Drive (8.0 GB)

All data on this device will be PERMANENTLY DELETED.

This action cannot be undone.

[CANCEL]  [I UNDERSTAND - FORMAT & EXPORT]
```

**Validation**:
- User must click "I UNDERSTAND..." button (not just dismiss)
- Checkbox optional but recommended: "Do not warn me again"

**Outcome**:
- Cancel: Return to Export Dialog, no changes
- Confirm: Set `ExportJob.reformat_confirmed = true`, proceed to format + export

---

### Progress Panel

**Display** (during export):
```
Exporting to Kingston USB Drive...

[████████████░░░░░] 65%

Files: 65 / 100 copied
Size: 450 MB / 700 MB written
Time elapsed: 2m 15s
Estimated time: 1m 5s

[Cancel Export]
```

**Elements**:
- Progress bar (visual 0–100%)
- Files written / total
- Bytes written / total
- Elapsed time (running counter)
- Estimated remaining (calculated from throughput)
- Cancel button (abort export, leave partial files on device)

**Disconnection Detected**:
```
⚠ Device Disconnected

Kingston USB Drive was disconnected during export.

Files already written: 65 / 100

Please reconnect the device to resume, or [Close] to cancel.

[Reconnect to Resume]  [Cancel]
```

---

### Export Complete Dialog

**On Success**:
```
✓ Export Complete

100 tracks exported to Kingston USB Drive

Time taken: 3m 42s
Space used: 700 MB / 8.0 GB

Device is ready to use.

[Open Device]  [OK]
```

**On Failure (partial)**:
```
✗ Export Failed

65 / 100 tracks exported to Kingston USB Drive
Reason: Device disconnected during export

Partial files remain on device. Please:
1. Manually delete the incomplete files, or
2. Reformat the device and retry

[Retry Export]  [Close]
```

**On Failure (format error)**:
```
✗ Format Failed

Device: Kingston USB Drive

Reason: Permission denied (check write-protect switch)

[Retry]  [Cancel]
```

---

## State Machine (UI Flow)

```
[Export Dialog Open]
  ├─ Device list empty
  │   └─ Show "No devices" message, Refresh active
  │
  ├─ Devices enumerated
  │   ├─ User selects device
  │   │   └─ Show export options
  │   │
  │   ├─ User checks "Reformat device"
  │   │   └─ Show filesystem selector (FAT32 / exFAT)
  │   │
  │   ├─ User clicks "Export" (no reformat)
  │   │   └─ Start ExportJob → Progress panel → Complete/Fail
  │   │
  │   └─ User clicks "Export" (reformat enabled, not confirmed)
  │       └─ Show warning dialog
  │           ├─ User clicks "Cancel"
  │           │   └─ Return to Export Dialog
  │           │
  │           └─ User clicks "I UNDERSTAND - FORMAT & EXPORT"
  │               └─ Set reformat_confirmed=true
  │               └─ Start format → Progress panel → Complete/Fail
  │
  ├─ Export in progress
  │   ├─ Device disconnects
  │   │   └─ Show "Disconnected" prompt; user can retry
  │   │
  │   └─ Device stays connected
  │       └─ Progress updates every 500ms
  │           └─ Export completes → Complete dialog
  │
  └─ Export aborted or failed
      └─ Show Failure dialog; user can retry or close
```

---

## Key Behaviors

| Scenario | Behavior |
|----------|----------|
| Multiple devices connected | User selects one; export targets only that device |
| Device disconnects mid-export | UI shows alert; user can reconnect + retry or cancel |
| Device full during export | Export fails at file that doesn't fit; partial files left on device |
| Device read-only | Detected before export; error shown; cannot proceed |
| Reformat enabled but then device removed | Format cancels; alert shown; return to device list |
| User force-closes app mid-export | Partial files remain on device; user must manually clean or reformat |

---

## Accessibility

- All buttons and labels have tooltips (hover text)
- Error messages use both color (red) and text (never color-only)
- Progress bar has numeric percentage + text label
- Keyboard shortcuts:
  - Tab: cycle through fields
  - Enter: confirm (Export, OK)
  - Esc: cancel (abort export, close dialog)
- Screen reader: all state changes announce via live regions (e.g., "Export started", "65% complete")

---

## Platform-Specific Notes

- **Windows**: Dialog may appear briefly in taskbar during format; expected behavior
- **macOS**: Reformat uses `diskutil` (requires admin password first-time only); password prompt handled by macOS
- **Linux**: `mkfs.fat` may require sudo; show password dialog before attempt

