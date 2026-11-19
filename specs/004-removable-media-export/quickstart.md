# Quickstart: Removable Media Export Validation

This guide documents end-to-end validation scenarios to prove Feature 004 (Removable Media Export) works correctly. These are manual test procedures; implementation phase will add corresponding unit and integration tests.

## Prerequisites

- **Hardware**: At least 2 USB drives or SD cards (different brands/sizes recommended)
- **Test MP3 files**: 10–20 small MP3 files (~5 MB each, ~100 MB total) in a local folder
- **Test platform**: Windows, macOS, or Linux (repeat per platform for full validation)
- **Time**: ~30 minutes per platform

---

## Validation Scenario 1: Enumerate Connected Devices

**Objective**: Verify app detects and displays removable storage devices correctly.

**Setup**:
1. Connect 1 USB drive or SD card to computer
2. Launch musicbox-organizer
3. Open Export dialog

**Expected Outcome**:
```
Device List:
✓ Kingston USB Drive - 8.0 GB (7.2 GB available, FAT32, Mounted)
[Refresh] button
```

**Validation Checklist**:
- [ ] Device appears in list with name, capacity, available space
- [ ] Filesystem type detected (FAT32 / exFAT / Unknown)
- [ ] Status badge shows green (Ready) if writable and sufficient space
- [ ] Refresh button reloads list without freezing UI
- [ ] Connect second device while dialog open → appears in list after refresh
- [ ] Disconnect device → list updates on next refresh (or auto-detects within 5s)

**Pass Criteria**: All items checked.

---

## Validation Scenario 2: Select Device and View Playlist

**Objective**: Verify device selection and playlist summary displayed.

**Setup** (continuing from Scenario 1):
1. Select Kingston USB Drive from list
2. Load test MP3 playlist (10 tracks, ~100 MB)

**Expected Outcome**:
```
Selected: Kingston USB Drive (8.0 GB, 7.2 GB available)

Playlist: Test Playlist (10 tracks, 98 MB)
├ Track 1: Song1.mp3 (3:45, 5.2 MB)
├ Track 2: Song2.mp3 (4:12, 6.1 MB)
...
└ Track 10: Song10.mp3 (3:30, 4.8 MB)

Space available: 7.2 GB ✓ (sufficient for 98 MB)

[Export]  [Cancel]
```

**Validation Checklist**:
- [ ] Selected device displayed at top
- [ ] Playlist name and track count shown
- [ ] Total size calculated correctly (sum of track sizes)
- [ ] Available space on device shown
- [ ] Warning if insufficient space (e.g., "Only 50 MB available, need 98 MB")
- [ ] Export button enabled if space available, disabled otherwise

**Pass Criteria**: All items checked; space calculation accurate within ±1 MB.

---

## Validation Scenario 3: Export Without Reformat

**Objective**: Verify files copied to device with correct [XXYY-filename] naming and playable sequence.

**Setup**:
1. From Scenario 2, ensure "Reformat device" is unchecked
2. Click [Export]
3. Monitor progress panel

**Expected Outcome** (Progress panel):
```
Exporting to Kingston USB Drive...

[████████████████░░░] 82%

Files: 8 / 10 copied
Size: 80 MB / 98 MB written
Time elapsed: 0m 45s
Estimated time: 0m 10s

[Cancel Export]
```

**After Export Completes** (Success dialog):
```
✓ Export Complete

10 tracks exported to Kingston USB Drive

Time taken: 0m 55s
Space used: 98 MB / 8.0 GB

Device is ready to use.

[Open Device]  [OK]
```

**Device Content Verification**:
Connect device to file explorer; verify:
```
Kingston USB Drive (USB)
├── AAAA - Song1.mp3      (5.2 MB)
├── AAAB - Song2.mp3      (6.1 MB)
├── AAAC - Song3.mp3      (5.8 MB)
...
└── AAAJ - Song10.mp3     (4.8 MB)

Total: 98 MB
Free: 7.1 GB (approx, ~98 MB used)
```

**Validation Checklist**:
- [ ] Files copied without error (completion dialog shows 10/10)
- [ ] File naming follows [XXYY-filename] convention (AAAA, AAAB, ..., AAAJ)
- [ ] Original filenames preserved after hyphen
- [ ] Files on device are identical to source (size matches)
- [ ] Progress bar updates smoothly (at least every 500ms)
- [ ] Total time <2 minutes for 100 MB on typical USB 3.0
- [ ] Device space updated after export (df shows ~98 MB used)
- [ ] Files readable on computer file explorer

**Playback Verification** (on primitive device):
1. Connect Kingston USB Drive to portable MP3 player, car head unit, or similar
2. Select "Play all" or navigate to /AAAA... folder
3. Verify tracks play in sequence (1, 2, 3, ..., 10) without user intervention

**Pass Criteria**: All items checked; playback sequence correct on at least one device.

---

## Validation Scenario 4: Reformat Device Before Export

**Objective**: Verify reformat warning, device formatting, and export after reformat.

**Setup**:
1. Connect same Kingston USB Drive (now contains files from Scenario 3)
2. Open Export dialog
3. Check "Reformat device before export"
4. Select FAT32 from dropdown
5. Click [Export]

**Expected Outcome** (Warning dialog):
```
⚠ WARNING: Device Will Be Erased

Device: Kingston USB Drive (8.0 GB)

All data on this device will be PERMANENTLY DELETED.

This action cannot be undone.

[CANCEL]  [I UNDERSTAND - FORMAT & EXPORT]
```

**Validation Checklist (Warning)**:
- [ ] Warning dialog appears before format
- [ ] Device name and capacity displayed in warning
- [ ] Cancel button aborts and returns to Export dialog (no changes)
- [ ] "I UNDERSTAND..." button required (cannot just close dialog)

**After Clicking "I UNDERSTAND..."** (Progress panel):
```
Formatting device...
[████░░░░░░░░░░░░░░] 25%
```

Then:
```
Exporting to Kingston USB Drive...
[██████████████░░░░] 70%
...
```

**Validation Checklist (Format + Export)**:
- [ ] Device formatted to FAT32 (or selected filesystem)
- [ ] Format completes before export starts
- [ ] Reformat time <30 seconds on typical USB drive
- [ ] Export then proceeds normally
- [ ] Final device contains only new playlist files (old files erased)
- [ ] Completion shows "Format & Export complete"

**Device Content Verification**:
```
Kingston USB Drive (USB) [AFTER REFORMAT]
├── AAAA - Song1.mp3
├── AAAB - Song2.mp3
...
└── AAAJ - Song10.mp3

(Only new playlist; old files gone)
```

**Pass Criteria**: Warning appears, user can cancel, format succeeds, export proceeds, device clean and populated.

---

## Validation Scenario 5: Device Disconnection During Export

**Objective**: Verify graceful handling of device disconnection mid-operation and error reporting.

**Setup**:
1. Start export with Scenario 3 (10 tracks, no reformat)
2. After progress reaches ~50% (5–10 files copied), physically unplug device or eject it

**Expected Behavior** (immediate, <5s):
```
⚠ Device Disconnected

Kingston USB Drive was disconnected during export.

Files already written: 5 / 10

Please reconnect the device to resume, or [Close] to cancel.

[Reconnect to Resume]  [Close]
```

**Validation Checklist**:
- [ ] Disconnection detected within 5 seconds of unplugging
- [ ] Export pauses (no crash or hang)
- [ ] Alert dialog shows files written so far (accurate count)
- [ ] User can click "Close" to abort (partial files remain on device)
- [ ] User can reconnect device and click "Reconnect to Resume" to continue

**After "Reconnect to Resume"**:
1. Reconnect Kingston USB Drive to same USB port
2. Wait for system to remount device
3. Click "Reconnect to Resume" in alert dialog

**Expected Outcome**:
```
Exporting to Kingston USB Drive...
[████████████░░░░░░] 50%

Files: 5 / 10 copied
(resumed, not restarted from 0)
```

Export should complete normally, copying remaining 5 files. Final device should contain all 10 files (5 old + 5 new, no duplicates).

**Validation Checklist (Resume)**:
- [ ] Export resumes from file 6 (not restarting from file 1)
- [ ] No duplicate files on device
- [ ] Completion dialog shows 10/10 files (total, not incremental)
- [ ] All 10 files present and playable on device

**Pass Criteria**: Disconnection detected <5s, resume works, no duplicates or missing files.

---

## Validation Scenario 6: Device Write-Protected or Full

**Objective**: Verify error detection for read-only and full-capacity devices.

**Test 6a: Write-Protected Device**

1. Enable write-protect on Kingston USB Drive (physical switch or OS-level)
2. Open Export dialog
3. Select write-protected device

**Expected Outcome**:
```
Selected: Kingston USB Drive (8.0 GB)
Status: ✗ Read-Only

[Export] button DISABLED (grayed out)

Reason: Device is write-protected. Check the physical write-protect switch or 
        disable write-protection in your operating system settings.
```

**Validation Checklist**:
- [ ] Write-protected status detected
- [ ] Export button disabled
- [ ] Clear error message shown
- [ ] No export attempted

**Test 6b: Device Full**

1. Connect Kingston USB Drive with only 50 MB free space
2. Try to export 100 MB playlist

**Expected Outcome**:
```
Selected: Kingston USB Drive (8.0 GB, 50 MB available)
Playlist: Test Playlist (10 tracks, 98 MB)

⚠ Insufficient Space
Space available: 50 MB
Space needed: 98 MB
Shortage: 48 MB

[Export] button DISABLED
```

**Validation Checklist**:
- [ ] Space shortage calculated correctly
- [ ] Export button disabled
- [ ] Warning shown before export attempt
- [ ] User cannot proceed until space issue resolved

**Pass Criteria**: Both read-only and full-capacity states detected and blocked.

---

## Validation Scenario 7: Multiple Devices

**Objective**: Verify multi-device enumeration and isolation (export targets only selected device).

**Setup**:
1. Connect Kingston USB Drive (Device A) and SanDisk SD Card (Device B)
2. Open Export dialog

**Expected Outcome**:
```
Device List:
✓ Kingston USB Drive - 8.0 GB (7.2 GB available, FAT32, Mounted)
✓ SanDisk SD Card - 16.0 GB (15.5 GB available, exFAT, Mounted)

[Refresh]
```

**Validation Checklist**:
- [ ] Both devices enumerate in list
- [ ] Different names, capacities, filesystems shown
- [ ] User can select either device

**Test**:
1. Select Kingston USB Drive
2. Click [Export]
3. Monitor completion; verify files only on Kingston (not on SanDisk)
4. Eject Kingston
5. Verify SanDisk still empty
6. Connect Kingston again, repeat with SanDisk selected

**Validation Checklist (Export)**:
- [ ] Export targets only selected device
- [ ] Non-selected devices untouched
- [ ] No cross-device contamination

**Pass Criteria**: Both devices detected, exports target selected device only.

---

## Summary Checklist

| Scenario | Status | Notes |
|----------|--------|-------|
| 1. Device Enumeration | ✓ or ✗ | All connected devices appear |
| 2. Playlist Selection | ✓ or ✗ | Correct size/track count calculated |
| 3. Export Without Reformat | ✓ or ✗ | Files copied, correct naming, playable |
| 4. Reformat Before Export | ✓ or ✗ | Warning shown, format succeeds, clean export |
| 5. Disconnection Handling | ✓ or ✗ | Detected <5s, resume works, no duplicates |
| 6. Write-Protected / Full | ✓ or ✗ | Detected, export blocked, error shown |
| 7. Multiple Devices | ✓ or ✗ | Enumerated separately, exports isolated |

**Overall Pass Criteria**: All scenarios pass on at least one platform (Windows, macOS, or Linux). Integration tests in Phase 2 will formalize these procedures and expand to additional device types and edge cases.

---

## Next Steps (Phase 2 Implementation)

After this manual validation succeeds:
1. Automate scenarios 1–7 as integration tests (where possible)
2. Add unit tests for naming sequence generator (AAAA → ZZZZ)
3. Add unit tests for filesystem detection logic
4. Create mock DeviceManager for CI environments (no live USB required)
5. Profile export performance (target: <3 minutes for 100 MB on USB 2.0)
