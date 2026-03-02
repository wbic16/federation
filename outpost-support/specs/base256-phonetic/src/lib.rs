//! # Base256 Phonetic Encoding
//!
//! Maps each byte (0x00-0xFF) to a unique pronounceable syllable.
//!
//! ## Structure
//! ```text
//! syllable = onset + nucleus + coda
//!          = C₁ (4 bits) + V (2 bits) + C₂ (2 bits)
//!          = 8 bits = 1 byte
//! ```
//!
//! ## Example
//! ```
//! use base256_phonetic::{encode_byte, decode_syllable, encode_bytes, decode_string};
//!
//! assert_eq!(encode_byte(0x00), "bac");
//! assert_eq!(encode_byte(0xFF), "wom");
//! assert_eq!(decode_syllable("bac"), Ok(0x00));
//! assert_eq!(encode_bytes(&[0xDE, 0xAD, 0xBE, 0xEF]), "sof nod pof tom");
//! ```

/// Onset consonants (high nibble, 4 bits, 16 values)
const ONSETS: [char; 16] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k',
    'l', 'm', 'n', 'p', 'r', 's', 't', 'w',
];

/// Nucleus vowels (bits 2-3, 2 bits, 4 values)
const VOWELS: [char; 4] = ['a', 'e', 'i', 'o'];

/// Coda consonants (bits 0-1, 2 bits, 4 values)
const CODAS: [char; 4] = ['c', 'd', 'f', 'm'];

/// Error type for decoding failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// Syllable is not exactly 3 characters
    InvalidLength(usize),
    /// Onset consonant not recognized
    InvalidOnset(char),
    /// Nucleus vowel not recognized
    InvalidVowel(char),
    /// Coda consonant not recognized
    InvalidCoda(char),
    /// Empty input
    EmptyInput,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::InvalidLength(len) => {
                write!(f, "syllable must be 3 characters, got {}", len)
            }
            DecodeError::InvalidOnset(c) => write!(f, "invalid onset consonant: '{}'", c),
            DecodeError::InvalidVowel(c) => write!(f, "invalid nucleus vowel: '{}'", c),
            DecodeError::InvalidCoda(c) => write!(f, "invalid coda consonant: '{}'", c),
            DecodeError::EmptyInput => write!(f, "empty input"),
        }
    }
}

impl std::error::Error for DecodeError {}

/// Encode a single byte to a 3-character syllable.
///
/// # Example
/// ```
/// use base256_phonetic::encode_byte;
/// assert_eq!(encode_byte(0x00), "bac");
/// assert_eq!(encode_byte(0xFF), "wom");
/// assert_eq!(encode_byte(0x41), "ged"); // ASCII 'A'
/// ```
#[inline]
pub fn encode_byte(byte: u8) -> String {
    let onset_idx = (byte >> 4) as usize;
    let vowel_idx = ((byte >> 2) & 0x03) as usize;
    let coda_idx = (byte & 0x03) as usize;

    let mut result = String::with_capacity(3);
    result.push(ONSETS[onset_idx]);
    result.push(VOWELS[vowel_idx]);
    result.push(CODAS[coda_idx]);
    result
}

/// Decode a 3-character syllable to a byte.
///
/// # Example
/// ```
/// use base256_phonetic::decode_syllable;
/// assert_eq!(decode_syllable("bac"), Ok(0x00));
/// assert_eq!(decode_syllable("wom"), Ok(0xFF));
/// assert!(decode_syllable("xyz").is_err());
/// ```
pub fn decode_syllable(syllable: &str) -> Result<u8, DecodeError> {
    let chars: Vec<char> = syllable.chars().collect();

    if chars.len() != 3 {
        return Err(DecodeError::InvalidLength(chars.len()));
    }

    let onset = chars[0].to_ascii_lowercase();
    let vowel = chars[1].to_ascii_lowercase();
    let coda = chars[2].to_ascii_lowercase();

    let onset_idx = ONSETS
        .iter()
        .position(|&c| c == onset)
        .ok_or(DecodeError::InvalidOnset(onset))?;

    let vowel_idx = VOWELS
        .iter()
        .position(|&c| c == vowel)
        .ok_or(DecodeError::InvalidVowel(vowel))?;

    let coda_idx = CODAS
        .iter()
        .position(|&c| c == coda)
        .ok_or(DecodeError::InvalidCoda(coda))?;

    Ok(((onset_idx as u8) << 4) | ((vowel_idx as u8) << 2) | (coda_idx as u8))
}

/// Encode a slice of bytes to a space-separated string of syllables.
///
/// # Example
/// ```
/// use base256_phonetic::encode_bytes;
/// assert_eq!(encode_bytes(&[0xDE, 0xAD, 0xBE, 0xEF]), "sof nod pof tom");
/// assert_eq!(encode_bytes(&[0x00, 0x00]), "bac bac");
/// ```
pub fn encode_bytes(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| encode_byte(b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Decode a space-separated string of syllables to bytes.
///
/// Handles multiple spaces, tabs, and newlines as separators.
///
/// # Example
/// ```
/// use base256_phonetic::decode_string;
/// assert_eq!(decode_string("sof nod pof tom"), Ok(vec![0xDE, 0xAD, 0xBE, 0xEF]));
/// assert_eq!(decode_string("bac  bac"), Ok(vec![0x00, 0x00])); // double space OK
/// ```
pub fn decode_string(input: &str) -> Result<Vec<u8>, DecodeError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(DecodeError::EmptyInput);
    }

    trimmed
        .split_whitespace()
        .map(decode_syllable)
        .collect()
}

/// Encode bytes to a string, grouping by word size.
///
/// # Example
/// ```
/// use base256_phonetic::encode_grouped;
/// // Group by 4 bytes (u32)
/// let bytes = [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
/// assert_eq!(encode_grouped(&bytes, 4), "sof nod pof tom | rif wof pif pof");
/// ```
pub fn encode_grouped(bytes: &[u8], group_size: usize) -> String {
    bytes
        .chunks(group_size)
        .map(encode_bytes)
        .collect::<Vec<_>>()
        .join(" | ")
}

/// Generate the complete encoding table (for reference/verification).
///
/// Returns a vector of (byte, syllable) pairs for all 256 values.
pub fn generate_table() -> Vec<(u8, String)> {
    (0u8..=255u8).map(|b| (b, encode_byte(b))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Single Byte Encoding Tests ==========

    #[test]
    fn t001_encode_min() {
        assert_eq!(encode_byte(0x00), "bac");
    }

    #[test]
    fn t002_encode_max() {
        assert_eq!(encode_byte(0xFF), "wom");
    }

    #[test]
    fn t003_encode_sign_boundary() {
        assert_eq!(encode_byte(0x80), "lac");
    }

    #[test]
    fn t004_encode_max_signed() {
        assert_eq!(encode_byte(0x7F), "kom");
    }

    #[test]
    fn t005_encode_max_low_nibble() {
        assert_eq!(encode_byte(0x0F), "bom");
    }

    #[test]
    fn t006_encode_max_high_nibble() {
        assert_eq!(encode_byte(0xF0), "wac");
    }

    // ========== Onset Progression Tests ==========

    #[test]
    fn t010_onset_progression() {
        let expected = [
            (0x00, "bac"), (0x10, "cac"), (0x20, "dac"), (0x30, "fac"),
            (0x40, "gac"), (0x50, "hac"), (0x60, "jac"), (0x70, "kac"),
            (0x80, "lac"), (0x90, "mac"), (0xA0, "nac"), (0xB0, "pac"),
            (0xC0, "rac"), (0xD0, "sac"), (0xE0, "tac"), (0xF0, "wac"),
        ];
        for (byte, syllable) in expected {
            assert_eq!(encode_byte(byte), syllable, "onset test failed for {:#04x}", byte);
        }
    }

    // ========== Vowel Progression Tests ==========

    #[test]
    fn t020_vowel_progression() {
        assert_eq!(encode_byte(0x00), "bac"); // a
        assert_eq!(encode_byte(0x04), "bec"); // e
        assert_eq!(encode_byte(0x08), "bic"); // i
        assert_eq!(encode_byte(0x0C), "boc"); // o
    }

    // ========== Coda Progression Tests ==========

    #[test]
    fn t030_coda_progression() {
        assert_eq!(encode_byte(0x00), "bac"); // c
        assert_eq!(encode_byte(0x01), "bad"); // d
        assert_eq!(encode_byte(0x02), "baf"); // f
        assert_eq!(encode_byte(0x03), "bam"); // m
    }

    // ========== Single Byte Decoding Tests ==========

    #[test]
    fn t100_decode_valid() {
        assert_eq!(decode_syllable("bac"), Ok(0x00));
        assert_eq!(decode_syllable("wom"), Ok(0xFF));
        assert_eq!(decode_syllable("lac"), Ok(0x80));
        assert_eq!(decode_syllable("kom"), Ok(0x7F));
    }

    #[test]
    fn t110_decode_invalid_onset() {
        assert!(matches!(decode_syllable("xyz"), Err(DecodeError::InvalidOnset('x'))));
    }

    #[test]
    fn t111_decode_invalid_vowel() {
        assert!(matches!(decode_syllable("bux"), Err(DecodeError::InvalidVowel('u'))));
    }

    #[test]
    fn t112_decode_invalid_coda() {
        assert!(matches!(decode_syllable("baz"), Err(DecodeError::InvalidCoda('z'))));
    }

    #[test]
    fn t113_decode_empty() {
        assert!(matches!(decode_syllable(""), Err(DecodeError::InvalidLength(0))));
    }

    #[test]
    fn t114_decode_too_short() {
        assert!(matches!(decode_syllable("ba"), Err(DecodeError::InvalidLength(2))));
    }

    #[test]
    fn t115_decode_too_long() {
        assert!(matches!(decode_syllable("bacc"), Err(DecodeError::InvalidLength(4))));
    }

    #[test]
    fn t116_decode_case_insensitive() {
        // Uppercase should work
        assert_eq!(decode_syllable("BAC"), Ok(0x00));
        assert_eq!(decode_syllable("Wom"), Ok(0xFF));
    }

    // ========== Multi-Byte Tests ==========

    #[test]
    fn t200_encode_known_patterns() {
        assert_eq!(encode_bytes(&[0x00, 0x00, 0x00, 0x00]), "bac bac bac bac");
        assert_eq!(encode_bytes(&[0xFF, 0xFF, 0xFF, 0xFF]), "wom wom wom wom");
        assert_eq!(encode_bytes(&[0xDE, 0xAD, 0xBE, 0xEF]), "sof nod pof tom");
        assert_eq!(encode_bytes(&[0xCA, 0xFE, 0xBA, 0xBE]), "rif wof pif pof");
    }

    #[test]
    fn t300_decode_known_patterns() {
        assert_eq!(decode_string("bac bac bac bac"), Ok(vec![0x00, 0x00, 0x00, 0x00]));
        assert_eq!(decode_string("wom wom wom wom"), Ok(vec![0xFF, 0xFF, 0xFF, 0xFF]));
        assert_eq!(decode_string("sof nod pof tom"), Ok(vec![0xDE, 0xAD, 0xBE, 0xEF]));
    }

    #[test]
    fn t310_decode_whitespace_handling() {
        // Multiple spaces
        assert_eq!(decode_string("bac  bad"), Ok(vec![0x00, 0x01]));
        // Newline
        assert_eq!(decode_string("bac\nbad"), Ok(vec![0x00, 0x01]));
        // Tab
        assert_eq!(decode_string("bac\tbad"), Ok(vec![0x00, 0x01]));
        // Leading/trailing whitespace
        assert_eq!(decode_string("  bac bad  "), Ok(vec![0x00, 0x01]));
    }

    // ========== Exhaustive Round-Trip Test ==========

    #[test]
    fn t400_exhaustive_roundtrip() {
        for i in 0u8..=255u8 {
            let encoded = encode_byte(i);
            let decoded = decode_syllable(&encoded).unwrap();
            assert_eq!(decoded, i, "Round-trip failed for {:#04x} -> {} -> {:#04x}", i, encoded, decoded);
        }
    }

    // ========== Collision Detection ==========

    #[test]
    fn t500_no_collisions() {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        for i in 0u8..=255u8 {
            let encoded = encode_byte(i);
            assert!(seen.insert(encoded.clone()), "Collision detected: {} appears twice", encoded);
        }
        assert_eq!(seen.len(), 256);
    }

    // ========== ASCII Tests ==========

    #[test]
    fn t210_ascii_encoding() {
        // 'A' = 0x41
        assert_eq!(encode_byte(0x41), "ged");
        // 'a' = 0x61
        assert_eq!(encode_byte(0x61), "jed");
        // 'Z' = 0x5A
        assert_eq!(encode_byte(0x5A), "hif");
        // 'z' = 0x7A
        assert_eq!(encode_byte(0x7A), "kif");
        // '0' = 0x30
        assert_eq!(encode_byte(0x30), "fac");
        // '9' = 0x39
        assert_eq!(encode_byte(0x39), "fid");
        // ' ' = 0x20
        assert_eq!(encode_byte(0x20), "dac");
        // '\n' = 0x0A
        assert_eq!(encode_byte(0x0A), "bif");
    }

    // ========== Grouped Encoding Test ==========

    #[test]
    fn t600_grouped_encoding() {
        let bytes = [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
        assert_eq!(encode_grouped(&bytes, 4), "sof nod pof tom | rif wof pif pof");
    }

    // ========== Table Generation Test ==========

    #[test]
    fn t700_generate_table() {
        let table = generate_table();
        assert_eq!(table.len(), 256);
        assert_eq!(table[0], (0x00, "bac".to_string()));
        assert_eq!(table[255], (0xFF, "wom".to_string()));
    }
}
