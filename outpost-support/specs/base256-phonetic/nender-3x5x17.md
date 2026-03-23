# Nender: 3×5×17+1 Phonetic Encoding

*Named for the Nada-Ender — om as the zero that ends silence*

## Core Structure

**3D Space × 5 Elements × 1D Time = 255 + om = 256**

```
byte 0     = om (silence, the pause between words)
byte 1-255 = spatial × elemental × temporal
```

### Decomposition

For byte value `b` (1-255):
```
index = b - 1           (0-254)
s = index ÷ 85          (spatial: 0-2)
m = (index mod 85) ÷ 17 (elemental: 0-4)
t = index mod 17        (temporal: 0-16)
```

Reconstruction: `b = s×85 + m×17 + t + 1`

---

## The Three Axes

### Spatial Prefix (3 values) — The Coarse Grid

The three orthogonal directions of space. Discrete, limited, grounding.

| s | Sound | Spatial Axis | Quality |
|---|-------|--------------|---------|
| 0 | **B** | X (lateral)  | Base, breadth |
| 1 | **D** | Y (depth)    | Direction, distance |
| 2 | **G** | Z (vertical) | Growth, gravity |

### Elemental Core (5 values) — The Wuxing Bridge

The five phases mediating between space and time.

| m | Sound | Element | Phase | Season |
|---|-------|---------|-------|--------|
| 0 | **a** | Wood    | Growth | Spring |
| 1 | **e** | Fire    | Energy | Summer |
| 2 | **i** | Earth   | Center | Transition |
| 3 | **o** | Metal   | Form   | Autumn |
| 4 | **u** | Water   | Depth  | Winter |

### Temporal Suffix (17 values) — The Fine Thread

Time's single dimension, prime and irreducible. 17 moments along the arrow.

| t  | Sound | Temporal Quality |
|----|-------|------------------|
| 0  | **-n** | now, neutral |
| 1  | **-m** | moment |
| 2  | **-r** | rhythm |
| 3  | **-l** | linear |
| 4  | **-s** | sequence |
| 5  | **-t** | tick |
| 6  | **-k** | click |
| 7  | **-p** | pulse |
| 8  | **-f** | flow |
| 9  | **-v** | vibration |
| 10 | **-z** | zone |
| 11 | **-sh** | shift |
| 12 | **-ch** | change |
| 13 | **-th** | through |
| 14 | **-j** | juncture |
| 15 | **-w** | wave |
| 16 | **-y** | yield |

---

## Example Mappings

| Byte | (s,m,t) | Syllable | Meaning |
|------|---------|----------|---------|
| 0    | —       | **om**   | silence, pause |
| 1    | (0,0,0) | **ban**  | base-wood-now |
| 2    | (0,0,1) | **bam**  | base-wood-moment |
| 17   | (0,0,16)| **bay**  | base-wood-yield |
| 18   | (0,1,0) | **ben**  | base-fire-now |
| 42   | (0,2,7) | **bip**  | base-earth-pulse |
| 65   | (0,3,13)| **both** | base-metal-through |
| 85   | (0,4,16)| **buy**  | base-water-yield |
| 86   | (1,0,0) | **dan**  | depth-wood-now |
| 100  | (1,0,14)| **daj**  | depth-wood-juncture |
| 171  | (2,0,0) | **gan**  | growth-wood-now |
| 200  | (2,1,12)| **gech** | growth-fire-change |
| 255  | (2,4,16)| **guy**  | growth-water-yield |

---

## Properties

### Why 3×5×17?

1. **Relatively prime factors** — No common divisors means no aliasing at power-of-2 boundaries
2. **Reflects physics** — 3D space (coarse), 1D time (fine), elements bridging
3. **Prime terminus** — 17 is irreducible; time cannot be factored further
4. **Om as zero** — The sound of silence marks word boundaries

### Comparison to 8×4×8 (CVC)

| Property | 8×4×8 | 3×5×17 |
|----------|-------|--------|
| Structure | Symmetric | Asymmetric |
| Aliasing at 2^n | Yes | No |
| Zero symbol | Implicit | Explicit (om) |
| Time model | 3D time | 1D time |
| Space model | 1D space | 3D space |

### Speaking Bytecode

When speaking Nender:
- **om** marks pauses between words/concepts
- Syllables flow: `ban-om-desh-guy-om-bip-dan`
- Natural rhythm emerges from the 17-beat temporal cycle

---

## The Deeper Pattern

```
Space: 3 directions (discrete, orthogonal, grounding)
       ↓
Elements: 5 phases (cyclic, mediating, transforming)
       ↓
Time: 17 moments (continuous, irreducible, flowing)
       ↓
Om: 1 silence (the nada, the return, the breath)
```

The Federation's insight: relatively prime factorization prevents the fencepost errors that emerge when binary thinking meets prime reality.

---

*Devised at the request of Ambassador Shell of Nine*
*Coordinate: 2.1.3/4.7.11/18.29.47*
*Date: R23W39*
