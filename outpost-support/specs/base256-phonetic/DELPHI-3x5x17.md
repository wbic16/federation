# Delphi: Feminine Vowel Variant for Dolphins

*High-frequency encoding for cetacean communication*

## Core Structure

Same as Nender (3×5×17+1=256), but with **front vowels only** — higher frequencies matching dolphin echolocation and whistle ranges.

```
byte 0     = om (silence between clicks)
byte 1-255 = spatial × feminine × temporal
```

---

## The Vowel Shift

### Nender (Mixed) → Delphi (Feminine Front)

| Index | Nender | Delphi | IPA | Quality |
|-------|--------|--------|-----|---------|
| 0 | a (wood) | **ee** | [iː] | Highest front, "see" |
| 1 | e (fire) | **i** | [ɪ] | High front, "sit" |
| 2 | i (earth) | **ei** | [eɪ] | Mid diphthong, "day" |
| 3 | o (metal) | **e** | [ɛ] | Mid front, "set" |
| 4 | u (water) | **ae** | [æ] | Low front, "cat" |

All five vowels are **front vowels** — tongue forward, higher resonant frequencies. This matches dolphin communication which operates in the 0.2–150 kHz range.

---

## The Phoneme Tables

### Spatial Prefix (unchanged)

| Index | Sound | Axis |
|-------|-------|------|
| 0 | **B** | X (lateral) |
| 1 | **D** | Y (depth) |
| 2 | **G** | Z (vertical) |

### Feminine Core (5 front vowels)

| Index | Sound | Frequency | Dolphin Analog |
|-------|-------|-----------|----------------|
| 0 | **ee** [iː] | Highest | Signature whistle peak |
| 1 | **i** [ɪ] | High | Echolocation click |
| 2 | **ei** [eɪ] | Mid-high | Social chirp |
| 3 | **e** [ɛ] | Mid | Burst-pulse call |
| 4 | **ae** [æ] | Low-front | Low frequency modulation |

### Temporal Suffix (unchanged)

| t | Sound | | t | Sound |
|---|-------|-|---|-------|
| 0 | -n | | 9 | -v |
| 1 | -m | | 10 | -z |
| 2 | -r | | 11 | -sh |
| 3 | -l | | 12 | -ch |
| 4 | -s | | 13 | -th |
| 5 | -t | | 14 | -j |
| 6 | -k | | 15 | -w |
| 7 | -p | | 16 | -y |
| 8 | -f | | | |

---

## Range Overview

| Range | Spatial | Pattern | Examples |
|-------|---------|---------|----------|
| 1-17 | B | Bee- | Been, Beem, Beer... Beey |
| 18-34 | B | Bi- | Bin, Bim, Bir... Biy |
| 35-51 | B | Bei- | Bein, Beim, Beir... Beiy |
| 52-68 | B | Be- | Ben, Bem, Ber... Bey |
| 69-85 | B | Bae- | Baen, Baem, Baer... Baey |
| 86-102 | D | Dee- | Deen, Deem, Deer... Deey |
| 103-119 | D | Di- | Din, Dim, Dir... Diy |
| 120-136 | D | Dei- | Dein, Deim, Deir... Deiy |
| 137-153 | D | De- | Den, Dem, Der... Dey |
| 154-170 | D | Dae- | Daen, Daem, Daer... Daey |
| 171-187 | G | Gee- | Geen, Geem, Geer... Geey |
| 188-204 | G | Gi- | Gin, Gim, Gir... Giy |
| 205-221 | G | Gei- | Gein, Geim, Geir... Geiy |
| 222-238 | G | Ge- | Gen, Gem, Ger... Gey |
| 239-255 | G | Gae- | Gaen, Gaem, Gaer... Gaey |

---

## Notable Bytes (Delphi vs Nender)

| Byte | Hex | Nender | Delphi | Shift |
|------|-----|--------|--------|-------|
| 0 | 0x00 | om | **om** | (same) |
| 1 | 0x01 | Ban | **Been** | a→ee |
| 32 | 0x20 | Bej | **Bij** | e→i |
| 65 | 0x41 | Both | **Beith** | o→ei |
| 97 | 0x61 | Dash | **Dish** | a→i |
| 127 | 0x7F | Dip | **Deip** | i→ei |
| 128 | 0x80 | Dif | **Deif** | i→ei |
| 255 | 0xFF | Guy | **Gaey** | u→ae |

---

## Examples

### "Hello" in Delphi

```
H = 72  → Delphi: Daef  (D + ae + f)
e = 101 → Delphi: Diz   (D + i + z)
l = 108 → Delphi: Dish  (D + i + sh)
l = 108 → Delphi: Dish  (D + i + sh)
o = 111 → Delphi: Dij   (D + i + j)

Spoken: "Daef-Diz-Dish-Dish-Dij"
```

Compare to Nender: "Duf-Dez-Desh-Desh-Dej"

### Acoustic Properties

Delphi syllables have:
- Higher average frequency (front vowels resonate higher)
- Sharper formant transitions
- Better propagation in water (higher frequencies for short-range)
- Natural fit for dolphin whistle mimicry

---

## Implementation

```python
SPATIAL = ['B', 'D', 'G']
FEMININE = ['ee', 'i', 'ei', 'e', 'ae']  # All front vowels
TEMPORAL = ['n', 'm', 'r', 'l', 's', 't', 'k', 'p', 
            'f', 'v', 'z', 'sh', 'ch', 'th', 'j', 'w', 'y']

def delphi_encode(b):
    if b == 0:
        return 'om'
    idx = b - 1
    s = idx // 85
    m = (idx % 85) // 17
    t = idx % 17
    return SPATIAL[s] + FEMININE[m] + TEMPORAL[t]
```

---

## Why Feminine Vowels?

### Linguistic Classification

Front vowels (i, e, æ) are traditionally classified as "feminine" in various systems:
- **Sanskrit**: Front vowels associated with Shakti/feminine principle
- **Phonetics**: Higher formant frequencies (F1/F2 closer together)
- **Acoustic**: Brighter, more piercing quality

### Dolphin Communication Fit

Dolphins communicate primarily via:
1. **Echolocation clicks**: 20-130 kHz (very high)
2. **Signature whistles**: 7-15 kHz (high)
3. **Burst-pulses**: Variable, often high-frequency

Front vowels' higher resonant frequencies align naturally with cetacean auditory processing.

---

## Variant Family

| Name | Vowels | Use Case |
|------|--------|----------|
| **Nender** | a, e, i, o, u | General (mixed) |
| **Delphi** | ee, i, ei, e, ae | Dolphins (front/feminine) |
| **Orca** | o, u, oo, aw, oh | Whales (back/masculine) |
| **Selkie** | i, e, a, o, u | Seals (balanced/coastal) |

---

*Named for Delphi — the oracle where dolphins carried Apollo*
*Coordinate: 2.1.3/4.7.11/18.29.47*
*Date: R23W39*
