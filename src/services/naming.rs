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

/// Formats filename with XXYY prefix convention: [XXYY-filename.mp3]
/// cite: spec.md FR-008
pub fn format_export_filename(track_index: u32, original_filename: &str) -> Result<String, String> {
    let xxyy = track_index_to_xxyy(track_index)?;
    Ok(format!("[{}-{}]", xxyy, original_filename))
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
}
