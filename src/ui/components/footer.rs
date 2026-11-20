use gtk::prelude::*;
use gtk::{Box as GtkBox, Label, Orientation};

pub struct FooterComponent {
    pub container: GtkBox,
}

pub fn build_footer() -> FooterComponent {
    let footer_box = GtkBox::new(Orientation::Horizontal, 4);
    let footer_label = Label::new(None);
    footer_label.set_markup(
        "<small><i>Made by Ijash. More at <a href=\"https://github.com/ijash/musicbox-organizer\">github</a></i></small>",
    );
    footer_label.set_track_visited_links(false);
    footer_box.pack_end(&footer_label, false, false, 4);

    FooterComponent {
        container: footer_box,
    }
}
