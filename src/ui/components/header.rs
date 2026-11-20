use gtk::prelude::*;
use gtk::{Box as GtkBox, Button, CssProvider, Image, Label, Orientation};
use std::cell::RefCell;
use std::io::Cursor;
use std::rc::Rc;

const APP_LOGO_PNG: &[u8] = include_bytes!("../../../img/logo/png/logo_mbo_128.png");

pub struct HeaderComponent {
    pub container: GtkBox,
    pub project_label: Label,
    pub theme_btn: Button,
}

pub fn build_header(
    project_name: &str,
    track_count: usize,
    css_provider: &CssProvider,
    is_dark_mode: &Rc<RefCell<bool>>,
    user_prefs: &Rc<RefCell<crate::models::UserPreferences>>,
    on_undo: impl Fn() + 'static,
    on_redo: impl Fn() + 'static,
) -> HeaderComponent {
    let header_box = GtkBox::new(Orientation::Horizontal, 8);

    // Title group: pack logo and project label together without expanding
    let title_group = GtkBox::new(Orientation::Horizontal, 6);

    // 1. Embedded Logo Image (FR-002)
    if let Ok(pixbuf) = gtk::gdk_pixbuf::Pixbuf::from_read(Cursor::new(APP_LOGO_PNG)) {
        if let Some(scaled) = pixbuf.scale_simple(24, 24, gtk::gdk_pixbuf::InterpType::Bilinear) {
            let logo_img = Image::from_pixbuf(Some(&scaled));
            title_group.pack_start(&logo_img, false, false, 0);
        }
    }

    // 2. Project Title Label
    let project_label = Label::new(None);
    project_label.set_markup(&format!(
        "<big><b>MusicBox Organizer</b></big> — <i>{}</i> ({} tracks)",
        project_name, track_count
    ));
    title_group.pack_start(&project_label, false, false, 0);
    header_box.pack_start(&title_group, false, false, 0);

    // 3. Toolbar Buttons
    let undo_btn = Button::with_label("↺ Undo");
    let redo_btn = Button::with_label("↻ Redo");
    let theme_btn = Button::with_label("☀ Theme: Dark");

    header_box.pack_end(&theme_btn, false, false, 0);
    header_box.pack_end(&redo_btn, false, false, 0);
    header_box.pack_end(&undo_btn, false, false, 0);

    // Undo action
    undo_btn.connect_clicked(move |_| {
        on_undo();
    });

    // Redo action
    redo_btn.connect_clicked(move |_| {
        on_redo();
    });

    // Theme toggle
    {
        let is_dark_mode = Rc::clone(is_dark_mode);
        let user_prefs = Rc::clone(user_prefs);
        let css_provider = css_provider.clone();
        let theme_btn_clone = theme_btn.clone();
        theme_btn.connect_clicked(move |_| {
            let mut dark = is_dark_mode.borrow_mut();
            *dark = !*dark;
            crate::ui::theme::apply_theme(&css_provider, &theme_btn_clone, *dark);
            let mut prefs = user_prefs.borrow_mut();
            prefs.dark_mode = *dark;
            prefs.save_to_dir(".").ok();
        });
    }

    HeaderComponent {
        container: header_box,
        project_label,
        theme_btn,
    }
}
