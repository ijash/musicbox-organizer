use id3::TagLike;
use musicbox_organizer::services::{AudioProcessor, NativeAudioProcessor};
use rodio::Source;
use std::fs;
use std::io::BufReader;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_real_mp3_normalization_and_trim_integration() {
    let sample_source = Path::new("mp3-sample/sample-3s.mp3");
    if !sample_source.exists() {
        eprintln!(
            "Skipping integration test: sample file missing at {}",
            sample_source.display()
        );
        return;
    }

    let dir = tempdir().expect("create temp dir");
    let test_mp3 = dir.path().join("test_track.mp3");
    fs::copy(sample_source, &test_mp3).expect("copy sample mp3");

    // 1. Tag with test metadata
    let mut initial_tag = id3::Tag::new();
    initial_tag.set_title("Integration Test Title");
    initial_tag.set_artist("Integration Test Artist");
    initial_tag
        .write_to_path(&test_mp3, id3::Version::Id3v24)
        .expect("write initial tags");

    // 2. Process audio with NativeAudioProcessor
    let processor = NativeAudioProcessor::new();
    assert!(processor.is_available());

    let modified = processor
        .process_track(&test_mp3, true, -45.0, true, -14.0)
        .expect("process track should succeed");

    assert!(modified, "Track should be modified by DSP pipeline");

    // 3. Verify playable output with rodio::Decoder
    let file = fs::File::open(&test_mp3).expect("open output mp3");
    let decoder =
        rodio::Decoder::new(BufReader::new(file)).expect("output must be valid decodable MP3");
    assert!(decoder.sample_rate() > 0);
    assert!(decoder.channels() > 0);

    // 4. Verify ID3 tags preserved verbatim
    let read_back_tag = id3::Tag::read_from_path(&test_mp3).expect("read back tags");
    assert_eq!(read_back_tag.title(), Some("Integration Test Title"));
    assert_eq!(read_back_tag.artist(), Some("Integration Test Artist"));

    // 5. Verify zero leftover temporary files in directory
    let entries: Vec<_> = fs::read_dir(dir.path())
        .expect("read dir")
        .filter_map(|e| e.ok())
        .collect();

    assert_eq!(
        entries.len(),
        1,
        "Only the target MP3 file must remain, zero temporary files left behind"
    );
    assert_eq!(entries[0].path(), test_mp3);
}

#[test]
fn test_real_mp3_trait_normalize_and_trim_methods() {
    let sample_source = Path::new("mp3-sample/sample-3s.mp3");
    if !sample_source.exists() {
        return;
    }

    let dir = tempdir().expect("create temp dir");
    let test_mp3 = dir.path().join("trait_test.mp3");
    fs::copy(sample_source, &test_mp3).expect("copy sample mp3");

    let processor: Box<dyn AudioProcessor> = Box::new(NativeAudioProcessor::new());
    processor
        .normalize_volume(&test_mp3, -16.0)
        .expect("normalize_volume via trait");
    processor
        .trim_silence(&test_mp3, -40.0)
        .expect("trim_silence via trait");

    // Verify still decodable
    let file = fs::File::open(&test_mp3).expect("open output mp3");
    let _decoder = rodio::Decoder::new(BufReader::new(file)).expect("valid mp3");
}

#[test]
fn test_dsp_rejects_non_mp3_files() {
    let dir = tempdir().expect("create temp dir");
    let test_flac = dir.path().join("song.flac");
    fs::write(&test_flac, b"fake flac content").expect("write fake flac");

    let processor = NativeAudioProcessor::new();
    let err = processor
        .process_track(&test_flac, true, -45.0, true, -14.0)
        .unwrap_err();
    assert_eq!(
        err.to_string(),
        "Only MP3 audio files are supported for DSP editing"
    );

    let trait_proc: Box<dyn AudioProcessor> = Box::new(processor);
    let norm_err = trait_proc.normalize_volume(&test_flac, -14.0).unwrap_err();
    assert_eq!(
        norm_err.to_string(),
        "Only MP3 audio files are supported for DSP editing"
    );

    let trim_err = trait_proc.trim_silence(&test_flac, -45.0).unwrap_err();
    assert_eq!(
        trim_err.to_string(),
        "Only MP3 audio files are supported for DSP editing"
    );
}
