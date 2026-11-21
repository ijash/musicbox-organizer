use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub struct AudioPreviewPlayer {
    _stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
    sink: Option<Sink>,
    pub current_file: Option<PathBuf>,
}

impl AudioPreviewPlayer {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default()
            .ok()
            .map(|(s, h)| (Some(s), Some(h)))
            .unwrap_or((None, None));

        Self {
            _stream: stream,
            stream_handle: handle,
            sink: None,
            current_file: None,
        }
    }

    /// Load and immediately play audio file for preview
    pub fn play_file(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        if let Some(ref handle) = self.stream_handle {
            let file = File::open(path)?;
            let source = Decoder::new(BufReader::new(file))?;
            let sink = Sink::try_new(handle)?;
            sink.append(source);
            sink.play();
            self.sink = Some(sink);
            self.current_file = Some(path.to_path_buf());
            Ok(())
        } else {
            Err("No audio output device available".into())
        }
    }

    pub fn pause(&self) {
        if let Some(ref sink) = self.sink {
            sink.pause();
        }
    }

    pub fn resume(&self) {
        if let Some(ref sink) = self.sink {
            sink.play();
        }
    }

    pub fn stop(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.stop();
        }
        self.sink = None;
        self.current_file = None;
    }

    pub fn is_playing(&self) -> bool {
        self.sink
            .as_ref()
            .map(|s| !s.is_paused() && !s.empty())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_instantiation() {
        let player = AudioPreviewPlayer::new();
        assert!(!player.is_playing());
    }
}
