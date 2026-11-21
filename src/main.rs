#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use musicbox_organizer::models::{PlaylistMetadata, ProjectFile};
use musicbox_organizer::services::{create_device_manager, ExportService};
use musicbox_organizer::ui::ExportDialog;
use std::env;
use std::path::Path;

const DEFAULT_PROJECT_NAME: &str = "playlist.txt";

fn print_help() {
    println!("MusicBox Organizer - Removable Media & Playlist Manager");
    println!("Usage:");
    println!("  musicbox                     - Launch desktop GUI window");
    println!("  musicbox gui                 - Launch desktop GUI window");
    println!("  musicbox scan [dir]          - Scan directory for MP3 files and display ID3 tags");
    println!("  musicbox tag <file> [t] [a]  - View or update ID3 tag (title, artist)");
    println!("  musicbox status              - Show active project and connected removable devices");
    println!("  musicbox save [path]         - Save current project locally (does NOT write to USB/SD)");
    println!("  musicbox export              - Export playlist to connected USB/SD device");
    println!("  musicbox export-text [dir]   - Export txt & rtf to playlist-text/ folder");
    println!("  musicbox check-compat <dir>  - Audit directory for primitive device compatibility");
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

    #[cfg(windows)]
    {
        let is_cli = args.len() > 1 && args[1] != "gui";
        if is_cli {
            unsafe {
                use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
                let _ = AttachConsole(ATTACH_PARENT_PROCESS);
            }
        }
    }

    let command = if args.len() > 1 {
        args[1].as_str()
    } else {
        "gui"
    };

    if command == "gui" {
        musicbox_organizer::ui::run_gui();
        return;
    }

    let project = auto_load_project().unwrap_or_else(|| {
        ProjectFile::new(PlaylistMetadata {
            name: "Default Playlist".to_string(),
            author: None,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
        })
    });

    match command {
        "scan" => {
            let scan_dir = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            let scanner = musicbox_organizer::services::DirectoryScanner::new();
            match scanner.scan_directory(Path::new(scan_dir)) {
                Ok(tracks) => {
                    if tracks.is_empty() {
                        println!("No MP3 files found in '{}'", scan_dir);
                    } else {
                        println!("Found {} MP3 files in '{}':", tracks.len(), scan_dir);
                        let id3_svc = musicbox_organizer::services::Id3Service::new();
                        for t in tracks {
                            let meta = id3_svc.read_tags(&t.file_path).unwrap_or(t.metadata);
                            println!("  [{}] {} | Title: '{}' | Artist: '{}'", t.xxyy_prefix, t.original_filename, meta.title, meta.artist);
                        }
                    }
                }
                Err(e) => eprintln!("Scan error: {}", e),
            }
        }
        "tag" => {
            if let Some(file_str) = args.get(2) {
                let path = Path::new(file_str);
                let id3_svc = musicbox_organizer::services::Id3Service::new();
                match id3_svc.read_tags(path) {
                    Ok(mut meta) => {
                        if let Some(new_title) = args.get(3) {
                            meta.title = new_title.clone();
                            if let Some(new_artist) = args.get(4) {
                                meta.artist = new_artist.clone();
                            }
                            match id3_svc.write_tags(path, &meta) {
                                Ok(_) => println!("Updated tags for {}: Title='{}', Artist='{}'", file_str, meta.title, meta.artist),
                                Err(e) => eprintln!("Failed to write tags: {}", e),
                            }
                        } else {
                            println!("Tags for {}:", file_str);
                            println!("  Title:  {}", meta.title);
                            println!("  Artist: {}", meta.artist);
                            println!("  Album:  {}", meta.album);
                            println!("  Track:  {:?}", meta.track_number);
                        }
                    }
                    Err(e) => eprintln!("Failed to read tags: {}", e),
                }
            } else {
                eprintln!("Usage: musicbox tag <file.mp3> [new_title] [new_artist]");
            }
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        "export-text" => {
            let base_dir = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            let service = musicbox_organizer::services::ExportConfigService::new();
            let selection = musicbox_organizer::models::ExportFormatSelection {
                txt: true,
                rtf: true,
            };
            match service.export_playlist(Path::new(base_dir), &selection, &project) {
                Ok(files) => {
                    println!("Exported {} playlist files to '{}/playlist-text/':", files.len(), base_dir);
                    for f in files {
                        println!("  - {}", f.display());
                    }
                }
                Err(e) => eprintln!("Export failed: {}", e),
            }
        }
        "check-compat" => {
            let target_dir = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            let path = Path::new(target_dir);
            let service = musicbox_organizer::services::CompatibilityService::new();
            match service.validate_directory(path) {
                Ok(report) => {
                    println!("============================================================");
                    println!("Device Compatibility Audit Report for '{}'", target_dir);
                    println!("============================================================");
                    println!("Total audio files:     {}", report.total_files);
                    println!("ASCII byte-order sort: {}", if report.is_sorted { "PASS" } else { "FAIL (out of order)" });
                    println!("ASCII character check: {}", if report.ascii_violations.is_empty() { "PASS" } else { "FAIL" });
                    println!("FAT32 path length:     {}", if report.length_violations.is_empty() { "PASS" } else { "FAIL (>255 bytes)" });
                    println!("Naming format [XXYY-]: {}", if report.format_violations.is_empty() { "PASS" } else { "WARN (non-standard)" });
                    println!("------------------------------------------------------------");
                    println!("Overall status:        {}", if report.overall_compatible { "100% COMPATIBLE" } else { "INCOMPATIBLE" });
                    if !report.ascii_violations.is_empty() {
                        println!("Non-ASCII filenames detected: {:?}", report.ascii_violations);
                    }
                    if !report.length_violations.is_empty() {
                        println!("Path length violations detected: {:?}", report.length_violations);
                    }
                    println!("============================================================");
                }
                Err(e) => eprintln!("Audit error: {}", e),
            }
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

