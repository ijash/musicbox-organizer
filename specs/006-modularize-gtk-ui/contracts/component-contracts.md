# Contract: Modular UI Components API

## Component Signatures

```rust
pub mod theme {
    pub const LIGHT_CSS: &str;
    pub const DARK_CSS: &str;
    pub fn apply_theme(dark: bool);
}

pub mod components {
    pub struct HeaderComponent {
        pub container: gtk::Box,
        pub project_label: gtk::Label,
    }

    pub struct PlaylistTableComponent {
        pub container: gtk::Box,
        pub tree_view: gtk::TreeView,
        pub list_store: gtk::ListStore,
    }

    pub struct AudioBarComponent {
        pub container: gtk::Frame,
        pub status_label: gtk::Label,
    }

    pub struct ExportPanelComponent {
        pub container: gtk::Frame,
        pub status_label: gtk::Label,
    }

    pub struct FooterComponent {
        pub container: gtk::Box,
    }
}
```
