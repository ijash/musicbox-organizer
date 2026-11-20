use crate::services::AudioPreviewPlayer;
use gtk::prelude::*;
use gtk::{Box as GtkBox, Button, Frame, Label, Orientation, TreeView};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct AudioBarComponent {
    pub container: Frame,
    pub status_label: Label,
}

pub fn build_audio_bar(
    tree_view: &TreeView,
    project: &Rc<RefCell<crate::models::ProjectFile>>,
    audio_player: &Rc<RefCell<AudioPreviewPlayer>>,
) -> AudioBarComponent {
    let preview_frame = Frame::new(Some("Audio Preview Player"));
    let preview_box = GtkBox::new(Orientation::Horizontal, 8);
    preview_box.set_margin_start(8);
    preview_box.set_margin_end(8);
    preview_box.set_margin_top(4);
    preview_box.set_margin_bottom(4);

    let play_btn = Button::with_label("▶ Play");
    let pause_btn = Button::with_label("⏸ Pause");
    let stop_btn = Button::with_label("⏹ Stop");
    let audio_status_label = Label::new(Some("Select track to preview audio"));
    audio_status_label.set_xalign(0.0);

    preview_box.pack_start(&play_btn, false, false, 0);
    preview_box.pack_start(&pause_btn, false, false, 0);
    preview_box.pack_start(&stop_btn, false, false, 0);
    preview_box.pack_start(&audio_status_label, true, true, 8);

    // Play button
    {
        let tree_view = tree_view.clone();
        let project = Rc::clone(project);
        let audio_player = Rc::clone(audio_player);
        let audio_status_label = audio_status_label.clone();
        play_btn.connect_clicked(move |_| {
            let selection = tree_view.selection();
            if let Some((_, iter)) = selection.selected() {
                let path = tree_view.model().unwrap().path(&iter).unwrap();
                if let Some(indices) = path.indices().first() {
                    let idx = *indices as usize;
                    let p = project.borrow();
                    if let Some(track) = p.tracks.get(idx) {
                        let file_path = PathBuf::from(&track.file_path);
                        let title = track.title.clone();
                        drop(p);
                        let mut player = audio_player.borrow_mut();
                        match player.play_file(&file_path) {
                            Ok(_) => audio_status_label
                                .set_markup(&format!("▶ <b>Playing:</b> {}", title)),
                            Err(e) => {
                                audio_status_label.set_text(&format!("Playback error: {}", e))
                            }
                        }
                    }
                }
            } else {
                audio_status_label.set_text("Select a track in the table to play.");
            }
        });
    }

    // Pause button
    {
        let audio_player = Rc::clone(audio_player);
        let audio_status_label = audio_status_label.clone();
        pause_btn.connect_clicked(move |_| {
            audio_player.borrow().pause();
            audio_status_label.set_text("⏸ Paused");
        });
    }

    // Stop button
    {
        let audio_player = Rc::clone(audio_player);
        let audio_status_label = audio_status_label.clone();
        stop_btn.connect_clicked(move |_| {
            audio_player.borrow_mut().stop();
            audio_status_label.set_text("⏹ Stopped");
        });
    }

    preview_frame.add(&preview_box);

    AudioBarComponent {
        container: preview_frame,
        status_label: audio_status_label,
    }
}
