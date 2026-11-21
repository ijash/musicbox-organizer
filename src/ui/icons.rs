use gtk::prelude::*;
use gtk::{Button, Image};
use std::io::Cursor;

// Clean, standard GNOME / Relm4 symbolic SVG icons embedded directly into binary
const SVG_FOLDER: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path fill="currentColor" d="M1 2.5A1.5 1.5 0 0 1 2.5 1h3A1.5 1.5 0 0 1 7 2.5V3h6.5A1.5 1.5 0 0 1 15 4.5v9a1.5 1.5 0 0 1-1.5 1.5h-11A1.5 1.5 0 0 1 1 13.5zM2 4v9.5a.5.5 0 0 0 .5.5h11a.5.5 0 0 0 .5-.5V5a1 1 0 0 0-1-1H2.5A.5.5 0 0 0 2 4.5z"/></svg>"#;
const SVG_ADD: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path fill="currentColor" d="M8 2a1 1 0 0 1 1 1v4h4a1 1 0 1 1 0 2H9v4a1 1 0 1 1-2 0V9H3a1 1 0 0 1 0-2h4V3a1 1 0 0 1 1-1"/></svg>"#;
const SVG_REMOVE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path fill="currentColor" d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708"/></svg>"#;
const SVG_UP: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path fill="currentColor" d="m7.247 4.86-4.796 5.481c-.566.647-.106 1.659.753 1.659h9.592a1 1 0 0 0 .753-1.659l-4.796-5.48a1 1 0 0 0-1.506 0z"/></svg>"#;
const SVG_DOWN: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path fill="currentColor" d="M7.247 11.14 2.451 5.658C1.885 5.013 2.345 4 3.204 4h9.592a1 1 0 0 1 .753 1.659l-4.796 5.48a1 1 0 0 1-1.506 0z"/></svg>"#;
const SVG_SAVE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path fill="currentColor" d="M2 1a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V4.414a1 1 0 0 0-.293-.707l-2.414-2.414A1 1 0 0 0 11.586 1zm2 1h7v3H4zm7 5v6H5V7z"/></svg>"#;
const SVG_OPEN: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path fill="currentColor" d="M.54 3.87.5 3a2 2 0 0 1 2-2h3.672a2 2 0 0 1 1.414.586l.828.828A2 2 0 0 0 9.828 3h3.982a2 2 0 0 1 1.992 2.181l-.637 7A2 2 0 0 1 13.174 14H2.826a2 2 0 0 1-1.991-1.819l-.637-7a2 2 0 0 1 .342-1.31"/></svg>"#;
const SVG_PLAY: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path fill="currentColor" d="m11.596 8.697-6.363 3.692c-.54.313-1.233-.066-1.233-.697V4.308c0-.63.692-1.01 1.233-.696l6.363 3.692a.802.802 0 0 1 0 1.393"/></svg>"#;
const SVG_PAUSE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path fill="currentColor" d="M5.5 3.5A1.5 1.5 0 0 1 7 5v6a1.5 1.5 0 0 1-3 0V5a1.5 1.5 0 0 1 1.5-1.5m5 0A1.5 1.5 0 0 1 12 5v6a1.5 1.5 0 0 1-3 0V5a1.5 1.5 0 0 1 1.5-1.5"/></svg>"#;
const SVG_STOP: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path fill="currentColor" d="M5 3.5h6A1.5 1.5 0 0 1 12.5 5v6a1.5 1.5 0 0 1-1.5 1.5H5A1.5 1.5 0 0 1 3.5 11V5A1.5 1.5 0 0 1 5 3.5"/></svg>"#;
const SVG_EXPORT: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path fill="currentColor" d="M14 4.5V14a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V2a2 2 0 0 1 2-2h5.5zm-3 0A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V4.5z"/></svg>"#;

#[derive(Debug, Clone, Copy)]
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
    ExportText,
}

impl AppIcon {
    pub fn svg_data(&self) -> &'static str {
        match self {
            AppIcon::FolderOpen => SVG_FOLDER,
            AppIcon::ListAdd => SVG_ADD,
            AppIcon::ListRemove => SVG_REMOVE,
            AppIcon::GoUp => SVG_UP,
            AppIcon::GoDown => SVG_DOWN,
            AppIcon::Play => SVG_PLAY,
            AppIcon::Pause => SVG_PAUSE,
            AppIcon::Stop => SVG_STOP,
            AppIcon::Save => SVG_SAVE,
            AppIcon::Open => SVG_OPEN,
            AppIcon::ExportText => SVG_EXPORT,
        }
    }

    pub fn image(&self, size: i32) -> Option<Image> {
        let pixbuf = gtk::gdk_pixbuf::Pixbuf::from_read(Cursor::new(self.svg_data().as_bytes())).ok()?;
        let scaled = pixbuf.scale_simple(size, size, gtk::gdk_pixbuf::InterpType::Bilinear)?;
        Some(Image::from_pixbuf(Some(&scaled)))
    }

    pub fn button(&self, label: &str) -> Button {
        let btn = Button::new();
        let box_container = gtk::Box::new(gtk::Orientation::Horizontal, 4);

        if let Some(img) = self.image(14) {
            box_container.pack_start(&img, false, false, 0);
        }

        let lbl = gtk::Label::new(Some(label));
        box_container.pack_start(&lbl, false, false, 0);
        btn.add(&box_container);
        btn
    }
}
