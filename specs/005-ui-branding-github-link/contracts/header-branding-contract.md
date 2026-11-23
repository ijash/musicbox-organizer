# Contract: UI Header Branding and GitHub Link

## UI Header Elements

```text
HeaderBox (Horizontal)
├── [LogoImage] (26x26 Pixbuf)
├── [ProjectTitleLabel] ("MusicBox Organizer — <name> (<N> tracks)")
├── [Spacer / Flexible Area]
├── [UndoButton] ("↺ Undo")
├── [RedoButton] ("↻ Redo")
├── [ThemeButton] ("☀ Theme: Dark" / "🌙 Theme: Light")
└── [GitHubButton] ("GitHub ↗")
```

## Behavior Contract

- **Window Icon**: On `window.show_all()`, taskbar/window icon shows `logo_mbo_128.png`.
- **GitHub Action**: Clicking `GitHubButton` opens `https://github.com/ijash/musicbox-organizer` via desktop URI handler.
- **Asset Fallback**: If `assets/img/logo/` is absent, logo widget is omitted silently without breaking layout or throwing errors.
