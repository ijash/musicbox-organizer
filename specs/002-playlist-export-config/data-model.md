# Data Model: Playlist Export Config & Theme Persistence

**Feature Branch**: `002-playlist-export-config`
**Date**: 2026-11-20

## Entities

### 1. UserPreferences
Persistent user configuration.

| Field | Type | Default | Description |
|---|---|---|---|
| `dark_mode` | `bool` | `true` | Active theme preference |
| `export_txt` | `bool` | `true` | Export plain text playlist |
| `export_rtf` | `bool` | `true` | Export rich text format playlist |

### 2. ExportFormatSelection
Represents checked format choices in export dialog.

| Field | Type | Description |
|---|---|---|
| `txt` | `bool` | Generate `playlist-text/playlist.txt` |
| `rtf` | `bool` | Generate `playlist-text/playlist.rtf` |

Validation rule: At least one format must be true to enable export.

### 3. AudioProcessingPlaceholder
Architectural stub representing planned audio DSP features.

| Field | Type | Description |
|---|---|---|
| `name` | `String` | Feature name (e.g. "Normalize Volume", "Trim Silence") |
| `is_available` | `bool` | Always `false` in v1 |
