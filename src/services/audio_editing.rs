use ebur128::{EbuR128, Mode};
use mp3lame_encoder::{Bitrate, Builder, FlushNoGap, InterleavedPcm, MonoPcm, Quality};
use rodio::Source;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

/// Extensibility trait for audio DSP editing features
/// cite: spec.md FR-001, FR-002, FR-003
pub trait AudioProcessor: Send + Sync {
    fn normalize_volume(&self, path: &Path, target_db: f32) -> Result<(), Box<dyn Error>>;
    fn trim_silence(&self, path: &Path, threshold_db: f32) -> Result<(), Box<dyn Error>>;
    fn is_available(&self) -> bool;
}

/// Fallback / placeholder processor for testing and initial compatibility
pub struct PlaceholderAudioProcessor;

impl AudioProcessor for PlaceholderAudioProcessor {
    fn normalize_volume(&self, _path: &Path, _target_db: f32) -> Result<(), Box<dyn Error>> {
        Err("Audio editing features are coming in v2".into())
    }

    fn trim_silence(&self, _path: &Path, _threshold_db: f32) -> Result<(), Box<dyn Error>> {
        Err("Audio editing features are coming in v2".into())
    }

    fn is_available(&self) -> bool {
        false
    }
}

/// Batch DSP configuration parameters
/// cite: data-model.md DspJobConfig
#[derive(Debug, Clone)]
pub struct DspJobConfig {
    pub normalize: bool,
    pub target_db: f32,
    pub trim_silence: bool,
    pub silence_threshold_db: f32,
    pub target_paths: Vec<PathBuf>,
}

/// Progress update message sent across thread boundaries
/// cite: data-model.md DspProgress
#[derive(Debug, Clone)]
pub struct DspProgress {
    pub current_index: usize,
    pub total_tracks: usize,
    pub current_file: String,
    pub status_message: String,
}

/// Summary report after processing a batch
/// cite: data-model.md DspBatchReport
#[derive(Debug, Clone, Default)]
pub struct DspBatchReport {
    pub succeeded: usize,
    pub skipped: usize,
    pub failed: Vec<(PathBuf, String)>,
}

/// Production native pure-Rust / LAME audio processor
pub struct NativeAudioProcessor;

impl NativeAudioProcessor {
    pub fn new() -> Self {
        Self
    }

    /// Process a single audio file with optional silence trim and loudness normalization.
    /// Order of execution: Trim first (so silence does not bias LUFS), Normalize second.
    /// Returns Ok(true) if modified, Ok(false) if skipped (e.g. silent or non-MP3).
    pub fn process_track(
        &self,
        path: &Path,
        trim: bool,
        trim_threshold_db: f32,
        normalize: bool,
        target_db: f32,
    ) -> Result<bool, Box<dyn Error>> {
        if !trim && !normalize {
            return Ok(false);
        }

        // Verify MP3 extension
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if !ext.eq_ignore_ascii_case("mp3") {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }

        // 1. Read existing ID3 tags verbatim before reading/writing audio
        let original_tag = id3::Tag::read_from_path(path).ok();

        // 2. Decode MP3 to PCM using rodio::Decoder
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let decoder = match rodio::Decoder::new(reader) {
            Ok(d) => d,
            Err(e) => return Err(format!("Failed to decode audio {}: {}", path.display(), e).into()),
        };

        let sample_rate = decoder.sample_rate();
        let channels = decoder.channels();
        if channels == 0 || sample_rate == 0 {
            return Ok(false);
        }

        // Convert decoded samples to interleaved f32 in [-1.0, 1.0]
        let mut samples: Vec<f32> = decoder.convert_samples::<f32>().collect();
        if samples.is_empty() {
            return Ok(false);
        }

        let mut modified = false;

        // 3. Trim silence first if requested
        if trim {
            let trimmed = trim_silence_buffer(&samples, sample_rate, channels, trim_threshold_db);
            if trimmed.len() != samples.len() {
                samples = trimmed;
                modified = true;
            }
        }

        // If entire track became empty or is silent, skip further processing
        if samples.is_empty() {
            return Ok(false);
        }

        // 4. Normalize volume second if requested
        if normalize {
            let gain = calculate_lufs_gain(&samples, sample_rate, channels, target_db);
            if (gain - 1.0).abs() > 0.005 {
                for sample in &mut samples {
                    *sample = (*sample * gain).clamp(-1.0, 1.0);
                }
                modified = true;
            }
        }

        if !modified {
            return Ok(false);
        }

        // 5. Encode modified PCM samples to standard MP3
        let mp3_bytes = encode_mp3(&samples, sample_rate, channels, 192)?;

        // 6. Write atomically in parent directory and re-apply ID3 tags
        write_atomic_mp3(path, &mp3_bytes, original_tag.as_ref())?;

        Ok(true)
    }
}

impl Default for NativeAudioProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioProcessor for NativeAudioProcessor {
    fn normalize_volume(&self, path: &Path, target_db: f32) -> Result<(), Box<dyn Error>> {
        self.process_track(path, false, -45.0, true, target_db)?;
        Ok(())
    }

    fn trim_silence(&self, path: &Path, threshold_db: f32) -> Result<(), Box<dyn Error>> {
        self.process_track(path, true, threshold_db, false, -14.0)?;
        Ok(())
    }

    fn is_available(&self) -> bool {
        true
    }
}

/// Calculate EBU R128 loudness and peak-safe linear gain.
/// Default target: target_db LUFS, true-peak ceiling: -1.0 dBTP (linear ~0.89125).
pub fn calculate_lufs_gain(
    samples: &[f32],
    sample_rate: u32,
    channels: u16,
    target_db: f32,
) -> f32 {
    let mode = Mode::I | Mode::TRUE_PEAK;
    let mut ebur = match EbuR128::new(channels as u32, sample_rate, mode) {
        Ok(e) => e,
        Err(_) => return 1.0,
    };

    if ebur.add_frames_f32(samples).is_err() {
        return 1.0;
    }

    let current_lufs = match ebur.loudness_global() {
        Ok(l) => l,
        Err(_) => return 1.0,
    };

    // If audio is practically silent, do not apply runaway gain
    if !current_lufs.is_finite() || current_lufs < -70.0 {
        return 1.0;
    }

    // Desired linear gain = 10^((target - current) / 20)
    let desired_gain = 10.0f64.powf((target_db as f64 - current_lufs) / 20.0) as f32;

    // True-peak limiter: true peak ceiling -1.0 dBTP (~0.89125 linear)
    let ceiling_linear = 0.89125f32;
    let mut max_true_peak = 0.0f32;
    for ch in 0..channels as u32 {
        if let Ok(tp) = ebur.true_peak(ch) {
            max_true_peak = max_true_peak.max(tp as f32);
        }
    }

    // Fallback: check sample peaks if true peak is zero
    if max_true_peak <= 0.0 {
        for &s in samples {
            max_true_peak = max_true_peak.max(s.abs());
        }
    }

    if max_true_peak > 0.0 && (desired_gain * max_true_peak) > ceiling_linear {
        (ceiling_linear / max_true_peak).min(desired_gain)
    } else {
        desired_gain
    }
}

/// Trim leading and trailing silence from interleaved PCM samples.
/// Window: ~20 ms RMS window.
/// Padding: ~50 ms safety margin preserved on both sides.
/// Alignment: preserves channel frame boundaries.
pub fn trim_silence_buffer(
    samples: &[f32],
    sample_rate: u32,
    channels: u16,
    threshold_db: f32,
) -> Vec<f32> {
    if samples.is_empty() || channels == 0 || sample_rate == 0 {
        return samples.to_vec();
    }

    let channels_usize = channels as usize;
    let total_frames = samples.len() / channels_usize;
    if total_frames == 0 {
        return samples.to_vec();
    }

    // 20 ms window in frames
    let window_frames = ((sample_rate as usize * 20) / 1000).max(1);
    let step_frames = (window_frames / 2).max(1);
    // 50 ms safety padding in frames
    let padding_frames = (sample_rate as usize * 50) / 1000;

    let threshold_linear = 10.0f32.powf(threshold_db / 20.0);
    let threshold_power = threshold_linear * threshold_linear;

    // Find first frame where window RMS exceeds threshold
    let mut first_active_frame = None;
    let mut frame_idx = 0;
    while frame_idx < total_frames {
        let end_frame = (frame_idx + window_frames).min(total_frames);
        let count = end_frame - frame_idx;
        let mut sum_sq = 0.0f32;
        for f in frame_idx..end_frame {
            for ch in 0..channels_usize {
                let s = samples[f * channels_usize + ch];
                sum_sq += s * s;
            }
        }
        let mean_sq = sum_sq / (count * channels_usize) as f32;
        if mean_sq >= threshold_power {
            first_active_frame = Some(frame_idx);
            break;
        }
        frame_idx += step_frames; // 50% overlap for accuracy
    }

    let first_frame = match first_active_frame {
        Some(f) => f.saturating_sub(padding_frames),
        None => return samples.to_vec(), // Entire audio is below threshold: no-op
    };

    // Find last frame where window RMS exceeds threshold
    let mut last_active_frame = total_frames;
    let mut r_frame = total_frames;
    while r_frame > 0 {
        let start_frame = r_frame.saturating_sub(window_frames);
        let count = r_frame - start_frame;
        let mut sum_sq = 0.0f32;
        for f in start_frame..r_frame {
            for ch in 0..channels_usize {
                let s = samples[f * channels_usize + ch];
                sum_sq += s * s;
            }
        }
        let mean_sq = sum_sq / (count * channels_usize) as f32;
        if mean_sq >= threshold_power {
            last_active_frame = r_frame;
            break;
        }
        if start_frame == 0 {
            break;
        }
        r_frame = r_frame.saturating_sub(step_frames);
    }

    let last_frame = (last_active_frame + padding_frames).min(total_frames);

    if first_frame >= last_frame {
        return samples.to_vec();
    }

    let start_sample = first_frame * channels_usize;
    let end_sample = last_frame * channels_usize;
    samples[start_sample..end_sample].to_vec()
}

/// Encode interleaved f32 PCM samples into standard MP3 bytes.
pub fn encode_mp3(
    samples: &[f32],
    sample_rate: u32,
    channels: u16,
    bitrate_kbps: u16,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut builder = Builder::new().ok_or("Failed to initialize LAME builder")?;
    builder
        .set_num_channels(channels as u8)
        .map_err(|e| format!("Failed to set channels: {:?}", e))?;
    builder
        .set_sample_rate(sample_rate)
        .map_err(|e| format!("Failed to set sample rate: {:?}", e))?;

    let brate = match bitrate_kbps {
        320 => Bitrate::Kbps320,
        256 => Bitrate::Kbps256,
        192 => Bitrate::Kbps192,
        160 => Bitrate::Kbps160,
        128 => Bitrate::Kbps128,
        _ => Bitrate::Kbps192,
    };
    builder
        .set_brate(brate)
        .map_err(|e| format!("Failed to set bitrate: {:?}", e))?;
    builder
        .set_quality(Quality::NearBest)
        .map_err(|e| format!("Failed to set quality: {:?}", e))?;

    let mut encoder = builder
        .build()
        .map_err(|e| format!("Failed to build LAME encoder: {:?}", e))?;

    let frames_count = samples.len() / (channels as usize);
    let mut mp3_buffer = Vec::new();
    mp3_buffer.reserve(mp3lame_encoder::max_required_buffer_size(frames_count));

    let encoded_size = if channels == 1 {
        let input = MonoPcm(samples);
        encoder
            .encode(input, mp3_buffer.spare_capacity_mut())
            .map_err(|e| format!("LAME encoding error: {:?}", e))?
    } else {
        let input = InterleavedPcm(samples);
        encoder
            .encode(input, mp3_buffer.spare_capacity_mut())
            .map_err(|e| format!("LAME encoding error: {:?}", e))?
    };

    unsafe {
        mp3_buffer.set_len(mp3_buffer.len() + encoded_size);
    }

    let flush_size = encoder
        .flush::<FlushNoGap>(mp3_buffer.spare_capacity_mut())
        .map_err(|e| format!("LAME flush error: {:?}", e))?;

    unsafe {
        mp3_buffer.set_len(mp3_buffer.len() + flush_size);
    }

    Ok(mp3_buffer)
}

/// Atomically overwrite audio file on disk, preserving ID3 tags and cleaning up on failure.
/// cite: Constitution Principle IV (zero temp files left behind)
pub fn write_atomic_mp3(
    target_path: &Path,
    mp3_data: &[u8],
    original_tag: Option<&id3::Tag>,
) -> Result<(), Box<dyn Error>> {
    let parent_dir = target_path
        .parent()
        .unwrap_or_else(|| Path::new("."));

    // Write to a named temp file in the SAME directory to guarantee same-filesystem atomic rename
    let mut temp_file = tempfile::NamedTempFile::new_in(parent_dir)?;
    temp_file.write_all(mp3_data)?;
    temp_file.flush()?;

    // Re-apply original ID3 tags if present
    if let Some(tag) = original_tag {
        tag.write_to_path(temp_file.path(), id3::Version::Id3v24)?;
    }

    // Atomically persist/rename over target file
    temp_file.persist(target_path).map_err(|e| e.error)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn generate_sine_wave(sample_rate: u32, duration_secs: f32, freq: f32, amplitude: f32) -> Vec<f32> {
        let num_samples = (sample_rate as f32 * duration_secs) as usize;
        let mut samples = Vec::with_capacity(num_samples * 2);
        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let val = (2.0 * PI * freq * t).sin() * amplitude;
            samples.push(val); // Left
            samples.push(val); // Right
        }
        samples
    }

    #[test]
    fn test_silence_trimming_synthetic_pcm() {
        let sample_rate = 44100;
        let channels = 2;
        // 0.2s silence + 0.5s audio + 0.2s silence
        let mut buffer = Vec::new();
        buffer.extend(vec![0.0f32; (sample_rate as f32 * 0.2) as usize * channels]);
        buffer.extend(generate_sine_wave(sample_rate, 0.5, 440.0, 0.5));
        buffer.extend(vec![0.0f32; (sample_rate as f32 * 0.2) as usize * channels]);

        let original_len = buffer.len();
        let trimmed = trim_silence_buffer(&buffer, sample_rate, channels as u16, -45.0);

        assert!(trimmed.len() < original_len, "Silence should be trimmed");
        assert_eq!(trimmed.len() % channels, 0, "Channel alignment must be preserved");
    }

    #[test]
    fn test_all_silence_no_op() {
        let sample_rate = 44100;
        let channels = 2;
        let buffer = vec![0.0f32; 44100 * channels];
        let trimmed = trim_silence_buffer(&buffer, sample_rate, channels as u16, -45.0);
        assert_eq!(trimmed.len(), buffer.len(), "All-silence track should no-op");
    }

    #[test]
    fn test_lufs_gain_calculation_and_peak_ceiling() {
        let sample_rate = 44100;
        let channels = 2;
        // Quiet 440Hz sine wave at amplitude 0.05 (~-26 dBFS)
        let samples = generate_sine_wave(sample_rate, 1.0, 440.0, 0.05);
        let gain = calculate_lufs_gain(&samples, sample_rate, channels as u16, -14.0);
        assert!(gain > 1.0, "Gain should boost quiet audio toward -14 LUFS");

        // Already loud audio at amplitude 0.95: gain should be clamped by -1.0 dBTP ceiling
        let loud_samples = generate_sine_wave(sample_rate, 1.0, 440.0, 0.95);
        let loud_gain = calculate_lufs_gain(&loud_samples, sample_rate, channels as u16, -14.0);
        let max_peak_after = 0.95 * loud_gain;
        assert!(max_peak_after <= 0.90, "Loud audio should not exceed -1.0 dBTP ceiling");
    }

    #[test]
    fn test_silent_audio_gain_safe() {
        let sample_rate = 44100;
        let channels = 2;
        let samples = vec![0.0f32; 44100 * channels];
        let gain = calculate_lufs_gain(&samples, sample_rate, channels as u16, -14.0);
        assert_eq!(gain, 1.0, "Silent audio should produce 1.0 gain without panicking");
    }
}
