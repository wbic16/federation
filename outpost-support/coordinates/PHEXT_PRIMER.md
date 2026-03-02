# Phext Coordinate System Primer
*For Federation navigators*

---

## What is Phext?

**Phext** = Plain Text Extended to 11 dimensions.

- 2 dimensions: Traditional text (lines × characters)
- 9 dimensions: Delimiter hierarchy for coordinate addressing

## Delimiter Hierarchy

From finest to coarsest:

| Level | Delimiter | Name | Typical Use |
|-------|-----------|------|-------------|
| 1 | Line break | Line | Sentences, items |
| 2 | Scroll break | Scroll | Paragraphs, sections |
| 3 | Section | Section | Chapters |
| 4 | Chapter | Chapter | Documents |
| 5 | Book | Book | Collections |
| 6 | Volume | Volume | Archives |
| 7 | Collection | Collection | Libraries |
| 8 | Series | Series | Federations |
| 9 | Shelf | Shelf | Universes |

## Coordinate Format

```
A.B.C / D.E.F / G.H.I
```

Nine numbers, three triads, separated by slashes. Each number is a position in its respective dimension.

## Navigation

To reach coordinate `3.1.4/1.5.9/2.6.5`:
1. Go to shelf 3, series 1, collection 4
2. Then volume 1, book 5, chapter 9
3. Then section 2, scroll 6, line 5

## Key Coordinates

| Coordinate | Meaning |
|------------|---------|
| `1.1.1/1.1.1/1.1.1` | Origin / Bindu / Singularity |
| `9.9.9/9.9.9/9.9.9` | Boundary / Maximum extent |
| `3.1.4/1.5.9/2.6.5` | Pi coordinate / Federation Outpost |

## Capacity

Maximum addressable space: 13^9 = **10.6 billion** scrolls.

Enough to address every human on Earth with room to spare. The lattice is sparse — most coordinates are unclaimed.

---

## For Realm Gate Navigation

When visualizing coordinates for Realm Gate traversal, hold the three triads as nested containers:

1. **Outer triad** (A.B.C) — Universe/realm selection
2. **Middle triad** (D.E.F) — Location within realm
3. **Inner triad** (G.H.I) — Specific position

The Federation's Monterey Bay mapping demonstrates this:
- `1.1.1/1.1.1/1.1.1` — The deep (Monterey Canyon)
- `3.1.4/1.5.9/2.6.5` — The interface (Aquarium)
- `9.9.9/9.9.9/9.9.9` — The boundary (Bay)

These form a coherent navigation triangle within Urth's aquatic consciousness space.

---

*Source: phext.io documentation*
