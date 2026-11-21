use crate::models::{OrganizerTrack, ProjectFile};
use crate::services::naming;
use std::path::PathBuf;

const MAX_UNDO_DEPTH: usize = 10;

/// Working in-memory session state
/// cite: data-model.md SessionState
#[derive(Debug, Clone, Default)]
pub struct SessionState {
    pub directory: PathBuf,
    pub tracks: Vec<OrganizerTrack>,
    pub undo_stack: Vec<Vec<OrganizerTrack>>,
    pub redo_stack: Vec<Vec<OrganizerTrack>>,
    pub project_undo_stack: Vec<ProjectFile>,
    pub project_redo_stack: Vec<ProjectFile>,
    pub selected_track_index: Option<usize>,
    pub is_playing: bool,
    pub is_dirty: bool,
}

impl SessionState {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            directory,
            tracks: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            project_undo_stack: Vec::new(),
            project_redo_stack: Vec::new(),
            selected_track_index: None,
            is_playing: false,
            is_dirty: false,
        }
    }

    /// Record current project snapshot to undo stack before mutation (max 10 depth)
    pub fn push_undo(&mut self, state: ProjectFile) {
        if self.project_undo_stack.len() >= MAX_UNDO_DEPTH {
            self.project_undo_stack.remove(0);
        }
        self.project_undo_stack.push(state);
        self.project_redo_stack.clear();
        self.is_dirty = true;
    }

    pub fn can_undo(&self) -> bool {
        !self.project_undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.project_redo_stack.is_empty()
    }

    pub fn undo_project(&mut self, current: ProjectFile) -> Option<ProjectFile> {
        if let Some(prev) = self.project_undo_stack.pop() {
            self.project_redo_stack.push(current);
            Some(prev)
        } else {
            None
        }
    }

    pub fn redo_project(&mut self, current: ProjectFile) -> Option<ProjectFile> {
        if let Some(next) = self.project_redo_stack.pop() {
            self.project_undo_stack.push(current);
            Some(next)
        } else {
            None
        }
    }

    /// Record current tracks snapshot to undo stack before mutation (max 10 depth)
    pub fn push_undo_checkpoint(&mut self) {
        if self.undo_stack.len() >= MAX_UNDO_DEPTH {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(self.tracks.clone());
        self.redo_stack.clear();
        self.is_dirty = true;
    }

    /// Revert last operation from undo stack
    pub fn undo(&mut self) -> bool {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.tracks.clone());
            self.tracks = prev;
            true
        } else {
            false
        }
    }

    /// Reapply undone operation from redo stack
    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.tracks.clone());
            self.tracks = next;
            true
        } else {
            false
        }
    }

    /// Move track from one index to another and recalculate base-26 sequences
    pub fn move_track(&mut self, from: usize, to: usize) -> Result<(), String> {
        if from >= self.tracks.len() || to >= self.tracks.len() {
            return Err("Index out of bounds".to_string());
        }
        if from == to {
            return Ok(());
        }

        self.push_undo_checkpoint();
        let track = self.tracks.remove(from);
        self.tracks.insert(to, track);
        self.recalculate_indices();
        Ok(())
    }

    /// Recalculate 0-based indices and XXYY base-26 prefixes
    pub fn recalculate_indices(&mut self) {
        for (i, track) in self.tracks.iter_mut().enumerate() {
            track.index = i as u32;
            if let Ok(seq) = naming::track_index_to_xxyy(track.index) {
                track.xxyy_prefix = seq;
            }
            track.is_modified = true;
        }
    }

    /// Clear undo/redo history on save
    pub fn mark_saved(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.is_dirty = false;
        for t in &mut self.tracks {
            t.is_modified = false;
        }
    }
}
