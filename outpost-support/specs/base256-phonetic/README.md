# Base256 Phonetic Encoding

**Version:** 0.1.0  
**Wave:** R23W23  
**Status:** Implementation complete, awaiting manual verification

Encode bytes as pronounceable one-syllable words.

## Quick Start

```bash
# Build
cargo build --release

# Run tests
cargo test

# Use CLI
cargo run --bin b256 -- encode DEADBEEF
# → sof nod pof tom

cargo run --bin b256 -- decode "sof nod pof tom"
# → 0xDEADBEEF

# Auto-detect
cargo run --bin b256 -- CAFEBABE
# → rif wof pif pof

# Print table
cargo run --bin b256 -- table
```

## Encoding Scheme

Each byte (0x00-0xFF) maps to a unique CVC syllable:

```
byte = (onset << 4) | (vowel << 2) | coda

onset (4 bits): b c d f g h j k l m n p r s t w
vowel (2 bits): a e i o
coda  (2 bits): c d f m
```

### Quick Reference

| Hex | Syllable | | Hex | Syllable |
|-----|----------|-|-----|----------|
| 0x00 | bac | | 0x80 | lac |
| 0xFF | wom | | 0x41 | ged ('A') |
| 0xDE | sof | | 0xAD | nod |
| 0xBE | pof | | 0xEF | tom |

### Magic Numbers

| Pattern | Syllables |
|---------|-----------|
| DEADBEEF | sof nod pof tom |
| CAFEBABE | rif wof pif pof |
| 00000000 | bac bac bac bac |
| FFFFFFFF | wom wom wom wom |

## Files

```
specs/base256-phonetic/
├── SPEC.md           # Full specification
├── TABLE.md          # Complete 256-entry reference table
├── TESTS.md          # Test case documentation
├── README.md         # This file
├── Cargo.toml        # Rust package config
└── src/
    ├── lib.rs        # Library implementation
    └── bin/
        └── b256.rs   # CLI tool
```

## API

```rust
use base256_phonetic::{encode_byte, decode_syllable, encode_bytes, decode_string};

// Single byte
let syllable = encode_byte(0xDE);  // "sof"
let byte = decode_syllable("sof"); // Ok(0xDE)

// Multiple bytes
let text = encode_bytes(&[0xDE, 0xAD, 0xBE, 0xEF]); // "sof nod pof tom"
let data = decode_string("sof nod pof tom");        // Ok([0xDE, 0xAD, 0xBE, 0xEF])
```

## Design Rationale

- **CVC structure**: Universally pronounceable, no consonant clusters
- **One syllable per byte**: 16×4×4 = 256 unique combinations
- **Short vowels only**: Maximum acoustic distance (no diphthongs)
- **Distinct consonants**: Selected to avoid confusion (no v/w, no similar sounds)

## Verbal Bandwidth

At ~3 syllables/second speaking rate:
- 3 bytes/second
- 24 bits/second
- 180 bytes/minute

## License

MIT
