# H019 — 8R | 8B | 8G: a 24-position conceptual toroid register

Recorded: 2026-11-14. Classification: **user-defined analytical model; not an ATLAS operating specification.**

## The encoding
Three toroid assemblies of eight coils each are assigned color-channel names and ordinal labels:

| Register | Toroid | Positions | Assigned conceptual tier |
| --- | --- | --- | --- |
| 8R | Barrel | R0..R7 | weak |
| 8B | End cap A | B0..B7 | medium |
| 8G | End cap C | G0..G7 | strong |

`{{ 8R | 8B | 8G }} -> 24 binary positions -> 2^24 = 16,777,216 patterns`.

The three tier names and color assignments are **not measured comparative ATLAS magnet-field strengths**. Individual toroidal superconducting coils are not memory bits or generally independently switchable logical register cells.

## Geometric invariant
- Eightfold rotational symmetry within each toroid, angular pitch `360°/8 = 45°`.
- The end-cap and barrel windings are angularly staggered by about 22.5°.
- `Z` denotes the conceptual central axis; the detector contains a separate central solenoid that must not be counted in the 24 toroid coils.
- The LHC's four major detectors are at Points 1, 2, 5, 8; **only ATLAS** has the three giant superconducting toroids.

## Formal logical mapping
```
bits = [R0..R7, B0..B7, G0..G7]
state = sum(bit[i] * 2^i for i in range(24))
```
For all-zero state `0`, all-one state `0xFFFFFF`. The logical bit order is a choice; it does **not** reproduce ATLAS magnet hardware control, coil wiring, experimental telemetry, or physics-force strengths.

## Next evidence test
If using real magnet measurements, derive calibrated per-coil current/field samples from openly published ATLAS sources and preserve measurement time, units, uncertainty and original source provenance. Continuous measurements must not be discretized to bits without explicit thresholds and validation.

## References
- https://www.atlas.cern/Discover/Detector/Magnet-System
- https://github.com/DavidWise01/cern/blob/main/magnets/atlas-toroids.json
