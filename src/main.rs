use musicbox_organizer::models::{PlaylistMetadata, ProjectFile};
use musicbox_organizer::services::{create_device_manager, ExportService};
use musicbox_organizer::ui::ExportDialog;
use std::env;
use std::path::Path;

const DEFAULT_PROJECT_NAME: &str = "playlist.txt";

fn print_help() {
    println!("MusicBox Organizer - Removable Media & Playlist Manager");
    println!("Usage:");
    println!("  musicbox status       - Show active project and connected removable devices");
    println!("  musicbox save [path]  - Save current project locally (does NOT write to USB/SD)");
    println!("  musicbox export       - Export playlist to connected USB/SD device");
    println!("\nWorkflows (Save vs Export):");
    println!("  - 'Save Project' persists project metadata (.txt) locally in current directory for editing.");
    println!("  - 'Export Playlist' writes audio files with [XXYY-filename.mp3] naming to removable media.");
}

fn auto_load_project() -> Option<ProjectFile> {
    let default_path = Path::new(DEFAULT_PROJECT_NAME);
    if default_path.exists() {
        match ProjectFile::load(default_path) {
            Ok(project) => {
                println!("Loaded project '{}' from {}", project.metadata.name, DEFAULT_PROJECT_NAME);
                Some(project)
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse {}: {}", DEFAULT_PROJECT_NAME, e);
                None
            }
        }
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("status");

    let project = auto_load_project().unwrap_or_else(|| {
        ProjectFile::new(PlaylistMetadata {
            name: "Default Playlist".to_string(),
            author: None,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
        })
    });

    match command {
        "help" | "--help" | "-h" => {
            print_help();
        }
        "save" => {
            let target_path = args.get(2).map(|s| s.as_str()).unwrap_or(DEFAULT_PROJECT_NAME);
            match project.save(target_path) {
                Ok(_) => println!("Successfully saved project locally to {}", target_path),
                Err(e) => eprintln!("Failed to save project: {}", e),
            }
        }
        "export" => {
            let manager = create_device_manager();
            let mut dialog = ExportDialog::new();
            if let Err(e) = dialog.refresh_devices(manager.as_ref()) {
                eprintln!("Failed to scan devices: {}", e);
                return;
            }

            if !dialog.can_export() {
                println!("{}", dialog.empty_state_message());
                return;
            }

            let dev = dialog.selected_device().unwrap();
            println!("Target device: {} [{}] ({:.2} GB capacity)", dev.name, dev.id, dev.capacity_bytes as f64 / 1e9);

            if project.tracks.is_empty() {
                println!("Playlist is empty (0 tracks). Add tracks before exporting.");
                return;
            }

            println!("Tracks to export: {}", project.tracks.len());

            let export_tracks: Vec<musicbox_organizer::models::Track> = project
                .tracks
                .iter()
                .filter_map(|pt| {
                    let path = std::path::PathBuf::from(&pt.file_path);
                    let filename = path
                        .file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or_else(|| format!("track_{}.mp3", pt.index));
                    let xxyy_name = musicbox_organizer::services::naming::format_export_filename(pt.index, &filename).ok()?;
                    let file_size_bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                    Some(musicbox_organizer::models::Track {
                        index: pt.index,
                        source_path: path,
                        filename,
                        xxyy_name,
                        file_size_bytes,
                    })
                })
                .collect();

            use std::io::{self, Write};
            print!("Reformat device before export? [y/N]: ");
            io::stdout().flush().ok();
            let mut input = String::new();
            io::stdin().read_line(&mut input).ok();
            let want_reformat = input.trim().eq_ignore_ascii_case("y");

            if want_reformat {
                dialog.reformat_enabled = true;
                if let Err(e) = dialog.trigger_export(export_tracks) {
                    eprintln!("Failed to initiate export: {}", e);
                    return;
                }

                if let Some(warning) = &dialog.warning_dialog {
                    println!("\n================== WARNING ==================");
                    println!("{}", warning.message());
                    println!("=============================================");
                    print!("Type 'CONFIRM' to wipe device and proceed: ");
                    io::stdout().flush().ok();
                    let mut confirm_input = String::new();
                    io::stdin().read_line(&mut confirm_input).ok();
                    if confirm_input.trim() == "CONFIRM" {
                        dialog.confirm_reformat_warning();
                        println!("Reformat confirmed.");
                    } else {
                        dialog.cancel_reformat_warning();
                        println!("Reformat cancelled. Export aborted.");
                        return;
                    }
                }
            } else {
                if let Err(e) = dialog.trigger_export(export_tracks) {
                    eprintln!("Failed to initiate export: {}", e);
                    return;
                }
            }

            let service = ExportService::new(manager);
            println!("Exporting tracks...");
            match dialog.execute_export(&service) {
                Ok(_) => {
                    println!("Export completed successfully!");
                    if let Some(job) = &dialog.current_job {
                        println!("Files written: {}/{}", job.files_written, job.tracks.len());
                    }
                    print!("Eject device safely? [Y/n]: ");
                    io::stdout().flush().ok();
                    let mut eject_in = String::new();
                    io::stdin().read_line(&mut eject_in).ok();
                    if !eject_in.trim().eq_ignore_ascii_case("n") {
                        match dialog.eject_selected(&service) {
                            Ok(_) => println!("Device safely unmounted/ejected."),
                            Err(e) => eprintln!("Failed to eject device: {}", e),
                        }
                    }
                }
                Err(e) => eprintln!("Export failed: {}", e),
            }
        }
        _ => {
            println!("MusicBox Organizer (active project: '{}')", project.metadata.name);
            println!("Tracks count: {}", project.tracks.len());
            let manager = create_device_manager();
            match manager.enumerate_devices() {
                Ok(devices) => {
                    if devices.is_empty() {
                        println!("Removable media: None detected.");
                    } else {
                        println!("Connected removable media ({}):", devices.len());
                        for d in devices {
                            println!("  - {} [{}] ({:.1} GB free)", d.name, d.id, d.available_bytes as f64 / 1e9);
                        }
                    }
                }
                Err(e) => eprintln!("Could not enumerate devices: {}", e),
            }
            println!("Type 'musicbox help' for available commands.");
        }
    }
}

