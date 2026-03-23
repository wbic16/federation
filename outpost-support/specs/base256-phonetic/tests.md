# Base256 Phonetic Encoding — Test Cases

**Version:** 0.1.0 | **Wave:** R23W23

---

## 1. Single Byte Encoding Tests

### 1.1 Boundary Values

| Test ID | Input (hex) | Expected Output | Description |
|---------|-------------|-----------------|-------------|
| T001 | 0x00 | "bac" | Minimum value |
| T002 | 0xFF | "wom" | Maximum value |
| T003 | 0x80 | "lac" | Sign bit boundary |
| T004 | 0x7F | "kom" | Max signed positive |
| T005 | 0x0F | "bom" | Max low nibble |
| T006 | 0xF0 | "wac" | Max high nibble |

### 1.2 Onset Progression (varying high nibble)

| Test ID | Input (hex) | Expected Output | Onset |
|---------|-------------|-----------------|-------|
| T010 | 0x00 | "bac" | b |
| T011 | 0x10 | "cac" | c |
| T012 | 0x20 | "dac" | d |
| T013 | 0x30 | "fac" | f |
| T014 | 0x40 | "gac" | g |
| T015 | 0x50 | "hac" | h |
| T016 | 0x60 | "jac" | j |
| T017 | 0x70 | "kac" | k |
| T018 | 0x80 | "lac" | l |
| T019 | 0x90 | "mac" | m |
| T01A | 0xA0 | "nac" | n |
| T01B | 0xB0 | "pac" | p |
| T01C | 0xC0 | "rac" | r |
| T01D | 0xD0 | "sac" | s |
| T01E | 0xE0 | "tac" | t |
| T01F | 0xF0 | "wac" | w |

### 1.3 Vowel Progression (varying bits 2-3)

| Test ID | Input (hex) | Binary | Expected | Vowel |
|---------|-------------|--------|----------|-------|
| T020 | 0x00 | 0000_00_00 | "bac" | a |
| T021 | 0x04 | 0000_01_00 | "bec" | e |
| T022 | 0x08 | 0000_10_00 | "bic" | i |
| T023 | 0x0C | 0000_11_00 | "boc" | o |

### 1.4 Coda Progression (varying bits 0-1)

| Test ID | Input (hex) | Binary | Expected | Coda |
|---------|-------------|--------|----------|------|
| T030 | 0x00 | 000000_00 | "bac" | c |
| T031 | 0x01 | 000000_01 | "bad" | d |
| T032 | 0x02 | 000000_10 | "baf" | f |
| T033 | 0x03 | 000000_11 | "bam" | m |

---

## 2. Single Byte Decoding Tests

### 2.1 Valid Syllables

| Test ID | Input | Expected (hex) | Description |
|---------|-------|----------------|-------------|
| T100 | "bac" | 0x00 | Minimum value |
| T101 | "wom" | 0xFF | Maximum value |
| T102 | "lac" | 0x80 | Mid-range |
| T103 | "kom" | 0x7F | ASCII DEL |
| T104 | "ged" | 0x45 | ASCII 'E' |
| T105 | "jed" | 0x65 | ASCII 'e' |

### 2.2 Invalid Syllables (error cases)

| Test ID | Input | Expected | Description |
|---------|-------|----------|-------------|
| T110 | "xyz" | Error | Invalid onset 'x' |
| T111 | "bux" | Error | Invalid vowel 'u' |
| T112 | "baz" | Error | Invalid coda 'z' |
| T113 | "" | Error | Empty string |
| T114 | "ba" | Error | Too short |
| T115 | "bacc" | Error | Too long |
| T116 | "BAC" | Error or 0x00 | Case sensitivity (define behavior) |

---

## 3. Multi-Byte Encoding Tests

### 3.1 Known Patterns

| Test ID | Input (hex) | Expected Output |
|---------|-------------|-----------------|
| T200 | 0x00000000 | "bac bac bac bac" |
| T201 | 0xFFFFFFFF | "wom wom wom wom" |
| T202 | 0xDEADBEEF | "sof nod pof tom" |
| T203 | 0xCAFEBABE | "rif wof pif pof" |
| T204 | 0x12345678 | "caf fec hif kic" |

### 3.2 ASCII Strings

| Test ID | Input (ASCII) | Input (hex) | Expected Output |
|---------|---------------|-------------|-----------------|
| T210 | "A" | 0x41 | "ged" |
| T211 | "Hi" | 0x4869 | "gic jid" |
| T212 | "OK" | 0x4F4B | "gom gim" |
| T213 | "test" | 0x74657374 | "ked jed kef ked" |

---

## 4. Multi-Byte Decoding Tests

### 4.1 Valid Sequences

| Test ID | Input | Expected (hex) |
|---------|-------|----------------|
| T300 | "bac bac bac bac" | 0x00000000 |
| T301 | "wom wom wom wom" | 0xFFFFFFFF |
| T302 | "sof nod pof tom" | 0xDEADBEEF |
| T303 | "rif wof pif pof" | 0xCAFEBABE |

### 4.2 Whitespace Handling

| Test ID | Input | Expected (hex) | Description |
|---------|-------|----------------|-------------|
| T310 | "bac bad" | 0x0001 | Single space |
| T311 | "bac  bad" | 0x0001 | Double space |
| T312 | "bac\nbad" | 0x0001 | Newline separator |
| T313 | "bac\tbad" | 0x0001 | Tab separator |
| T314 | " bac bad " | 0x0001 | Leading/trailing space |

---

## 5. Round-Trip Tests

### 5.1 Exhaustive Single-Byte Round-Trip

```
For i in 0..256:
    encoded = encode(i)
    decoded = decode(encoded)
    assert decoded == i
```

| Test ID | Description |
|---------|-------------|
| T400 | All 256 bytes encode → decode correctly |

### 5.2 Random Multi-Byte Round-Trip

| Test ID | Input Length | Description |
|---------|--------------|-------------|
| T410 | 1 byte | Single byte round-trip |
| T411 | 4 bytes | u32 round-trip |
| T412 | 8 bytes | u64 round-trip |
| T413 | 32 bytes | SHA256 hash round-trip |
| T414 | 1024 bytes | 1KB data round-trip |

---

## 6. Edge Cases

### 6.1 Collision Detection

| Test ID | Description |
|---------|-------------|
| T500 | Verify no two bytes produce the same syllable |
| T501 | Verify no syllable decodes to multiple bytes |

### 6.2 Real Words (potential confusion)

Some valid syllables form English words. Verify correct handling:

| Test ID | Syllable | Byte | English Word |
|---------|----------|------|--------------|
| T510 | "bad" | 0x01 | "bad" |
| T511 | "bed" | 0x05 | "bed" |
| T512 | "bid" | 0x09 | "bid" |
| T513 | "cod" | 0x1D | "cod" |
| T514 | "dad" | 0x21 | "dad" |
| T515 | "did" | 0x29 | "did" |
| T516 | "dim" | 0x2B | "dim" |
| T517 | "fad" | 0x31 | "fad" |
| T518 | "fed" | 0x35 | "fed" |
| T519 | "god" | 0x4D | "god" |
| T51A | "had" | 0x51 | "had" |
| T51B | "ham" | 0x53 | "ham" |
| T51C | "hem" | 0x57 | "hem" |
| T51D | "hid" | 0x59 | "hid" |
| T51E | "him" | 0x5B | "him" |
| T51F | "jam" | 0x63 | "jam" |
| T520 | "kid" | 0x79 | "kid" |
| T521 | "lad" | 0x81 | "lad" |
| T522 | "led" | 0x85 | "led" |
| T523 | "lid" | 0x89 | "lid" |
| T524 | "mad" | 0x91 | "mad" |
| T525 | "mid" | 0x99 | "mid" |
| T526 | "mod" | 0x9D | "mod" |
| T527 | "mom" | 0x9F | "mom" |
| T528 | "nod" | 0xAD | "nod" |
| T529 | "pad" | 0xB1 | "pad" |
| T52A | "pod" | 0xBD | "pod" |
| T52B | "rad" | 0xC1 | "rad" |
| T52C | "red" | 0xC5 | "red" |
| T52D | "rid" | 0xC9 | "rid" |
| T52E | "rod" | 0xCD | "rod" |
| T52F | "sad" | 0xD1 | "sad" |
| T530 | "sod" | 0xDD | "sod" |
| T531 | "tad" | 0xE1 | "tad" |
| T532 | "wad" | 0xF1 | "wad" |
| T533 | "wed" | 0xF5 | "wed" |

---

## 7. Performance Tests

| Test ID | Description | Target |
|---------|-------------|--------|
| T600 | Encode 1MB | < 10ms |
| T601 | Decode 1MB | < 10ms |
| T602 | Round-trip 1MB | < 20ms |

---

## 8. Test Implementation Notes

### 8.1 Rust Test Module Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t001_encode_min() {
        assert_eq!(encode_byte(0x00), "bac");
    }

    #[test]
    fn t002_encode_max() {
        assert_eq!(encode_byte(0xFF), "wom");
    }

    #[test]
    fn t400_exhaustive_roundtrip() {
        for i in 0u8..=255u8 {
            let encoded = encode_byte(i);
            let decoded = decode_syllable(&encoded).unwrap();
            assert_eq!(decoded, i, "Round-trip failed for {:#04x}", i);
        }
    }

    // ... additional tests
}
```

### 8.2 Test Coverage Requirements

- [ ] All 256 single-byte values
- [ ] All 16 onsets
- [ ] All 4 vowels
- [ ] All 4 codas
- [ ] All error conditions
- [ ] Multi-byte sequences
- [ ] Whitespace normalization
- [ ] Case handling (if applicable)
