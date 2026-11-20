# Contract: Automated GUI Test Harness & Icon Loader

## 1. GUI Test Harness API (`tests/gui/harness.rs`)

```rust
pub struct GuiHarness {
    pub window: gtk::Window,
    pub project: Rc<RefCell<ProjectFile>>,
    pub session_state: Rc<RefCell<SessionState>>,
}

impl GuiHarness {
    pub fn new() -> Self;
    pub fn pump_events(&self);
    pub fn click_button(&self, btn: &gtk::Button);
    pub fn simulate_track_add(&self, path: &Path);
    pub fn simulate_track_move_down(&self, index: usize);
    pub fn simulate_track_remove(&self, index: usize);
    pub fn simulate_theme_toggle(&self);
}
```

## 2. Icon Loader API (`src/ui/icons.rs`)

```rust
pub enum AppIcon {
    FolderOpen,
    ListAdd,
    ListRemove,
    GoUp,
    GoDown,
    Play,
    Pause,
    Stop,
    Save,
    Open,
    ThemeDark,
    ThemeLight,
}

impl AppIcon {
    pub fn image(&self) -> gtk::Image;
    pub fn button_with_icon(&self, label: &str) -> gtk::Button;
}
```
