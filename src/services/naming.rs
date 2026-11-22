//! XXYY Naming Algorithm: Base-26 sequence (AAAA → ZZZZ)
//! Supports max 17,576 tracks (26^4)
//!
//! cite: data-model.md Naming Sequence

const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const MAX_TRACKS: u32 = 17_576; // 26^4

pub fn track_index_to_xxyy(index: u32) -> Result<String, String> {
    if index >= MAX_TRACKS {
        return Err(format!(
            "Track index {} exceeds max {}",
            index,
            MAX_TRACKS - 1
        ));
    }

    // Scheme: index = c0*676 + c1*26 + c3, with c2 = c0
    // (matches spec sequence AAAA..AAAZ, ABAA..ABAZ, ..., ZZZZ)
    let c0 = (index / 676) as usize;
    let r = (index % 676) as usize;
    let c1 = r / 26;
    let c3 = r % 26;
    let c2 = c0;

    let mut result = String::new();
    result.push(ALPHABET[c0] as char);
    result.push(ALPHABET[c1] as char);
    result.push(ALPHABET[c2] as char);
    result.push(ALPHABET[c3] as char);

    Ok(result)
}

pub fn xxyy_to_track_index(xxyy: &str) -> Result<u32, String> {
    if xxyy.len() != 4 {
        return Err(format!(
            "XXYY string must be exactly 4 characters, got {}",
            xxyy.len()
        ));
    }

    let chars: Vec<char> = xxyy.chars().collect();
    if chars.len() != 4 {
        return Err(format!(
            "XXYY string must be exactly 4 characters, got {}",
            chars.len()
        ));
    }

    // Validate all chars first
    for ch in &chars {
        if !ALPHABET.contains(&(*ch as u8)) {
            return Err(format!("Invalid character '{}' in XXYY string", ch));
        }
    }

    // Scheme: index = c0*676 + c1*26 + c3, with c2 expected == c0
    let c0 = (chars[0] as u8 - b'A') as u32;
    let c1 = (chars[1] as u8 - b'A') as u32;
    let c3 = (chars[3] as u8 - b'A') as u32;

    let index = c0 * 676 + c1 * 26 + c3;

    if index >= MAX_TRACKS {
        return Err(format!(
            "XXYY '{}' maps to index {} which exceeds max",
            xxyy, index
        ));
    }

    Ok(index)
}

/// Sanitizes a string into a clean lowercase slug for filenames.
/// ASCII lowercase, non-alphanumeric chars (except `_` and `-`) replaced with `_`,
/// consecutive `_` collapsed, leading/trailing `_` trimmed.
pub fn sanitize_slug(input: &str) -> String {
    let mut slug = String::with_capacity(input.len());
    let mut prev_underscore = false;

    for c in input.chars() {
        if c.is_ascii_alphanumeric() || c == '-' {
            slug.push(c.to_ascii_lowercase());
            prev_underscore = false;
        } else if (c == '_' || c.is_whitespace() || !c.is_ascii() || !c.is_alphanumeric())
            && !prev_underscore
        {
            slug.push('_');
            prev_underscore = true;
        }
    }

    let trimmed = slug.trim_matches('_');
    if trimmed.is_empty() {
        "track".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Formats filename with XXYY prefix convention: XXYY-filename.mp3
/// cite: Constitution v1.9.0 Principle IX
pub fn format_export_filename(track_index: u32, original_filename: &str) -> Result<String, String> {
    let xxyy = track_index_to_xxyy(track_index)?;
    Ok(format!("{}-{}", xxyy, original_filename))
}

/// Formats export filename prioritizing ID3 title: XXYY-<slug>.mp3
/// Falls back to sanitized original filename stem if id3_title is empty after trimming.
/// cite: Constitution v1.9.0 Principle IX
pub fn format_export_filename_from_title(
    track_index: u32,
    id3_title: &str,
    original_filename: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let xxyy = track_index_to_xxyy(track_index).map_err(Box::<dyn std::error::Error>::from)?;
    let path = std::path::Path::new(original_filename);

    let stem = if !id3_title.trim().is_empty() {
        id3_title.trim()
    } else {
        path.file_stem()
            .and_then(|s| s.to_str())
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("track")
    };

    let slug = sanitize_slug(stem);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .filter(|e| !e.is_empty())
        .unwrap_or_else(|| "mp3".to_string());

    Ok(format!("{}-{}.{}", xxyy, slug, ext))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_track() {
        assert_eq!(track_index_to_xxyy(0).unwrap(), "AAAA");
    }

    #[test]
    fn test_second_track() {
        assert_eq!(track_index_to_xxyy(1).unwrap(), "AAAB");
    }

    #[test]
    fn test_26th_track() {
        assert_eq!(track_index_to_xxyy(25).unwrap(), "AAAZ");
    }

    #[test]
    fn test_27th_track() {
        assert_eq!(track_index_to_xxyy(26).unwrap(), "ABAA");
    }

    #[test]
    fn test_last_track() {
        assert_eq!(track_index_to_xxyy(17575).unwrap(), "ZZZZ");
    }

    #[test]
    fn test_inverse_aaaa() {
        assert_eq!(xxyy_to_track_index("AAAA").unwrap(), 0);
    }

    #[test]
    fn test_inverse_aaab() {
        assert_eq!(xxyy_to_track_index("AAAB").unwrap(), 1);
    }

    #[test]
    fn test_inverse_zzzz() {
        assert_eq!(xxyy_to_track_index("ZZZZ").unwrap(), 17575);
    }

    #[test]
    fn test_round_trip() {
        for i in (0..17576).step_by(100) {
            let xxyy = track_index_to_xxyy(i).unwrap();
            let index = xxyy_to_track_index(&xxyy).unwrap();
            assert_eq!(index, i, "Round-trip failed for index {}", i);
        }
    }

    #[test]
    fn test_out_of_bounds() {
        assert!(track_index_to_xxyy(17576).is_err());
    }

    #[test]
    fn test_invalid_char() {
        assert!(xxyy_to_track_index("AAA1").is_err());
    }

    #[test]
    fn test_format_export_filename_from_title_musang_pandan() {
        // Track 26 gives ABAA in XXYY scheme
        let res = format_export_filename_from_title(26, "Musang Pandan", "song.mp3").unwrap();
        assert_eq!(res, "ABAA-musang_pandan.mp3");
    }

    #[test]
    fn test_format_export_filename_from_title_empty_title_fallback() {
        let res = format_export_filename_from_title(0, "   ", "My Song (Remix).MP3").unwrap();
        assert_eq!(res, "AAAA-my_song_remix.mp3");
    }

    #[test]
    fn test_sanitize_slug_varieties() {
        assert_eq!(sanitize_slug("Hello World!"), "hello_world");
        assert_eq!(sanitize_slug("___a___b___"), "a_b");
        assert_eq!(sanitize_slug("Track-01 (Live)"), "track-01_live");
        assert_eq!(sanitize_slug(""), "track");
    }
}
