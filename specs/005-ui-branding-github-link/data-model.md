# Data Model: UI Branding & GitHub Link

**Feature Branch**: `005-ui-branding-github-link`
**Date**: 2026-11-20

## Entities

### 1. BrandingConfig
Visual branding assets and remote URLs.

| Field | Type | Default Value | Description |
|---|---|---|---|
| `icon_path` | `&str` | `"assets/img/logo/png/logo_mbo_128.png"` | Window favicon / desktop icon |
| `logo_header_path` | `&str` | `"assets/img/logo/png/logo_mbo_128.png"` | In-app header logo |
| `github_url` | `&str` | `"https://github.com/ijash/musicbox-organizer"` | Repository & docs link |
| `logo_size` | `(i32, i32)` | `(26, 26)` | Header logo render dimensions |

## State & Interactions

```
[App Startup]
       │
       ├─► Window::set_icon_from_file(icon_path)
       │
       └─► HeaderBox.pack_start(LogoImage, false, false, 4)
           HeaderBox.pack_start(ProjectLabel, true, true, 0)
           HeaderBox.pack_end(GitHubButton, false, false, 0)
                 │
                 ▼ (User clicks GitHub ↗)
           gtk::show_uri_on_window(window, github_url)
```
