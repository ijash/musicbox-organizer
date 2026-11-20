#[cfg(target_os = "linux")]
use gtk::prelude::*;
#[cfg(target_os = "linux")]
use gtk::{Button, CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_USER};

pub const LIGHT_CSS: &str = "
    window, dialog, popover {
        background-color: #f6f8fa;
        color: #24292f;
    }
    frame, frame > border {
        border-color: #d0d7de;
    }
    treeview {
        background-color: #ffffff;
        color: #24292f;
    }
    treeview:selected, treeview:selected:focus {
        background-color: #0969da;
        color: #ffffff;
    }
    treeview header button {
        background-color: #eaeef2;
        color: #24292f;
        font-weight: bold;
        border-color: #d0d7de;
    }
    entry, combobox button {
        background-color: #ffffff;
        color: #24292f;
        border-color: #d0d7de;
    }
    button {
        background-color: #f3f4f6;
        color: #24292f;
        border-color: #d0d7de;
    }
    button:hover {
        background-color: #e5e7eb;
    }
    progressbar progress {
        background-color: #0969da;
    }
    progressbar trough {
        background-color: #eaeef2;
    }
    label {
        color: #24292f;
    }
";

pub const DARK_CSS: &str = "
    window, dialog, popover {
        background-color: #1e2227;
        color: #abb2bf;
    }
    frame, frame > border {
        border-color: #3e4451;
    }
    treeview {
        background-color: #282c34;
        color: #abb2bf;
    }
    treeview:selected, treeview:selected:focus {
        background-color: #2c5a85;
        color: #ffffff;
    }
    treeview header button {
        background-color: #21252b;
        color: #abb2bf;
        font-weight: bold;
        border-color: #3e4451;
    }
    entry, combobox button {
        background-color: #1b1d23;
        color: #ffffff;
        border-color: #3e4451;
    }
    button {
        background-color: #2c313a;
        color: #dcdfe4;
        border-color: #3e4451;
    }
    button:hover {
        background-color: #3e4451;
    }
    progressbar progress {
        background-color: #61afef;
    }
    progressbar trough {
        background-color: #1b1d23;
    }
    label {
        color: #abb2bf;
    }
";

#[cfg(target_os = "linux")]
pub fn apply_theme(css_provider: &CssProvider, theme_btn: &Button, dark: bool) {
    if let Some(screen) = gtk::gdk::Screen::default() {
        let css = if dark { DARK_CSS } else { LIGHT_CSS };
        css_provider.load_from_data(css.as_bytes()).ok();
        StyleContext::add_provider_for_screen(&screen, css_provider, STYLE_PROVIDER_PRIORITY_USER);
    }
    if dark {
        theme_btn.set_label("☀ Theme: Dark");
    } else {
        theme_btn.set_label("🌙 Theme: Light");
    }
}
