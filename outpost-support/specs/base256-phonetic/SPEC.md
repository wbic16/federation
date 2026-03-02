# Base256 Phonetic Encoding Specification

**Version:** 0.1.0  
**Wave:** R23W23  
**Author:** Lumen ✴️ (with Will Bickford)  
**Status:** Draft

---

## 1. Overview

Base256 Phonetic Encoding maps each byte value (0x00–0xFF) to a unique, pronounceable one-syllable word. This enables humans to verbally communicate binary data without ambiguity.

**Use cases:**
- Reading checksums/hashes aloud
- Verbal debugging of byte streams
- Teaching binary/hex concepts
- Accessibility (screen readers, audio interfaces)
- Human-verifiable data transmission

---

## 2. Syllable Structure

Each syllable follows the CVC (Consonant-Vowel-Consonant) pattern:

```
syllable = onset + nucleus + coda
         = C₁ (4 bits) + V (2 bits) + C₂ (2 bits)
         = 8 bits = 1 byte
```

| Component | Bits | Position | Count | Purpose |
|-----------|------|----------|-------|---------|
| Onset (C₁) | 4 | high nibble | 16 | Primary consonant |
| Nucleus (V) | 2 | bits 2-3 | 4 | Vowel |
| Coda (C₂) | 2 | bits 0-1 | 4 | Terminal consonant |

---

## 3. Alphabet

### 3.1 Onset Consonants (16)

| Index | Hex | Consonant | IPA |
|-------|-----|-----------|-----|
| 0 | 0x0_ | b | /b/ |
| 1 | 0x1_ | c | /k/ |
| 2 | 0x2_ | d | /d/ |
| 3 | 0x3_ | f | /f/ |
| 4 | 0x4_ | g | /g/ |
| 5 | 0x5_ | h | /h/ |
| 6 | 0x6_ | j | /dʒ/ |
| 7 | 0x7_ | k | /k/ |
| 8 | 0x8_ | l | /l/ |
| 9 | 0x9_ | m | /m/ |
| 10 | 0xA_ | n | /n/ |
| 11 | 0xB_ | p | /p/ |
| 12 | 0xC_ | r | /r/ |
| 13 | 0xD_ | s | /s/ |
| 14 | 0xE_ | t | /t/ |
| 15 | 0xF_ | w | /w/ |

### 3.2 Nucleus Vowels (4)

| Index | Bits | Vowel | IPA | As in |
|-------|------|-------|-----|-------|
| 0 | 00 | a | /æ/ | "cat" |
| 1 | 01 | e | /ɛ/ | "bed" |
| 2 | 10 | i | /ɪ/ | "bit" |
| 3 | 11 | o | /ɒ/ | "hot" |

### 3.3 Coda Consonants (4)

| Index | Bits | Consonant | IPA |
|-------|------|-----------|-----|
| 0 | 00 | c | /k/ |
| 1 | 01 | d | /d/ |
| 2 | 10 | f | /f/ |
| 3 | 11 | m | /m/ |

---

## 4. Encoding Algorithm

### 4.1 Byte to Syllable

```
encode(byte) -> syllable:
    onset_idx  = (byte >> 4) & 0x0F      // high nibble
    vowel_idx  = (byte >> 2) & 0x03      // bits 2-3
    coda_idx   = byte & 0x03             // bits 0-1
    
    return ONSETS[onset_idx] + VOWELS[vowel_idx] + CODAS[coda_idx]
```

### 4.2 Syllable to Byte

```
decode(syllable) -> byte:
    onset_idx  = index_of(ONSETS, syllable[0])
    vowel_idx  = index_of(VOWELS, syllable[1])
    coda_idx   = index_of(CODAS, syllable[2])
    
    return (onset_idx << 4) | (vowel_idx << 2) | coda_idx
```

---

## 5. Formatting

### 5.1 Written Form

- Syllables are separated by spaces when written
- No capitalization (all lowercase)
- Example: `0xDEADBEEF` → `sod nic pid tof`

### 5.2 Spoken Form

- Each syllable pronounced as a single beat
- Natural rhythm: ~3 syllables per second
- Word boundaries at 4-syllable groups (32 bits)

### 5.3 Grouping Convention

| Data Size | Syllables | Example |
|-----------|-----------|---------|
| 1 byte | 1 | `bac` |
| 2 bytes (u16) | 2 | `bac bad` |
| 4 bytes (u32) | 4 | `bac bad baf bam` |
| 8 bytes (u64) | 8 | two 4-syllable groups |

---

## 6. Reference Values

### 6.1 Boundary Values

| Byte | Hex | Syllable |
|------|-----|----------|
| 0 | 0x00 | bac |
| 255 | 0xFF | wom |
| 128 | 0x80 | lac |
| 127 | 0x7F | kom |

### 6.2 ASCII Subset

| Char | Byte | Hex | Syllable |
|------|------|-----|----------|
| 'A' | 65 | 0x41 | ged |
| 'Z' | 90 | 0x5A | hif |
| 'a' | 97 | 0x61 | jed |
| 'z' | 122 | 0x7A | kif |
| '0' | 48 | 0x30 | fac |
| '9' | 57 | 0x39 | fed |
| ' ' | 32 | 0x20 | dac |
| '\n' | 10 | 0x0A | bif |

### 6.3 Common Patterns

| Pattern | Hex | Syllables |
|---------|-----|-----------|
| NULL | 0x00000000 | bac bac bac bac |
| DEADBEEF | 0xDEADBEEF | sof nod pof tom |
| CAFEBABE | 0xCAFEBABE | rif wof pif pof |
| "Hello" | 0x48656C6C6F | gic jed jom jom jom |

---

## 7. Design Rationale

### 7.1 Why CVC Structure?

- Universally pronounceable across languages
- Each syllable is phonetically distinct
- No consonant clusters (easy articulation)
- Natural stress patterns

### 7.2 Why These Consonants?

**Onsets (16):** Selected for:
- Unambiguous sounds (no 'v' vs 'w' confusion)
- Distinct phonemes (no 'c' vs 'k' when both present—'c' is /k/)
- Cross-linguistic compatibility

**Codas (4):** Selected for:
- Clear termination sounds
- No voicing confusion (c/d/f/m are distinct)
- Easy to hear in noise

### 7.3 Why These Vowels?

- Short vowels only (no diphthongs)
- Maximum acoustic distance
- No 'u' (too close to 'o' in many accents)

---

## 8. Future Extensions

### 8.1 Base65536 (Unicode)

Extend to 16-bit values using CCVCC or CVCCVC patterns.

### 8.2 Error Detection

Add optional checksum syllable (modular sum).

### 8.3 Compression

Common byte sequences → multi-syllable words.

---

## Changelog

- **0.1.0** (R23W23): Initial draft specification
