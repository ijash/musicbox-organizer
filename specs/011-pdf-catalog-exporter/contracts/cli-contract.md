# Contract: CLI Catalog Export Interface

**Specification**: `specs/011-pdf-catalog-exporter/spec.md`  
**Command**: `musicbox export-catalog` / `musicbox export-pdf`  
**Date**: 2026-11-22

## Command Overview

The CLI provides headless generation of printable catalogs directly from project files or track directories.

```bash
musicbox export-catalog [OPTIONS] <PATH>
```

Alias: `musicbox export-pdf [OPTIONS] <PATH>`

## Arguments & Options

| Parameter | Type | Default | Description |
|---|---|---|---|
| `<PATH>` | Positional String | Required | Path to directory containing organized MP3s or `playlist.txt` project file. |
| `--format` | Enum (`pdf`, `txt`, `rtf`) | `pdf` | Target catalog export format. |
| `--output` / `-o` | String | Auto | Destination file path (defaults to `<PATH>/playlist.<ext>`). |
| `--preset` | Enum (`10x10`, `12x12`, `a6`, `a5`, `a4`, `custom`) | `10x10` | Physical page dimension preset. |
| `--width` | Float | `10.0` | Custom physical width (requires unit or defaults to `--unit`). |
| `--height` | Float | `10.0` | Custom physical height (requires unit or defaults to `--unit`). |
| `--unit` | Enum (`cm`, `mm`, `in`) | `cm` | Physical measurement unit. |
| `--margin` | Float | `0.5` | Outer margin in active unit. |
| `--theme` | String | `monochrome_white` | Name of embedded JSON theme or path to custom `.json` theme file. |
| `--cut-border` | Flag | `false` | Draw thin cut/crop marks on the outer boundary. |
| `--include-artist` | Flag | `false` | Include artist column/text in track entries. |
| `--include-filename` | Flag | `false` | Include `[XXYY-filename.mp3]` sequence filenames. |
| `--include-header` | Flag | `false` | Print header with playlist title and total track count. |

## Exit Codes

| Code | Meaning |
|---|---|
| `0` | Success: catalog file generated and verified atomically. |
| `1` | General error: invalid arguments, missing input path, or write permissions failure. |
| `2` | Empty playlist error: input directory contains 0 audio tracks. |
| `3` | Theme error: custom theme file could not be parsed or found. |

## Example Invocations

```bash
# Default 10x10 cm White Monochrome PDF catalog
musicbox export-catalog /media/usb/music

# CD jewel case cover (12x12 cm) with artist names and warm paper theme
musicbox export-catalog /media/usb/music --preset 12x12 --theme warm_paper --include-artist

# A4 printable sheet with cut marks
musicbox export-catalog /media/usb/music --preset a4 --cut-border -o ./my_catalog.pdf
```
