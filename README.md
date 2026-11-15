# CERN — Seed to Tree (1947–2026)

**Independent public research archive** tracing the evolution of information substrates and scientific computation from the **1947 germanium point-contact transistor** to CERN's current public software and open data. This repository is not operated by CERN.

## Start here
- [Historical seed timeline](history/timeline.json): dated and sourced milestones from 1947 onward, with explicit continuity limits.
- [Public-source catalogue](catalog/sources.json): historical reports, APIs, open-source code and institutional documentation.
- [Metadata harvester](tools/harvest.py): bounded public CERN API collector; default metadata only. Optional strictly limited, explicitly CC0-labelled file downloads.
- [Open questions](research/OPEN_QUESTIONS.md): original REAP source, Mercury magnetic media and code-provenance gaps.
- [Tests](tests/test_archive.py) and [on-demand GitHub Action](.github/workflows/public-harvest.yml).

## Roots and hardware transitions
1947 Bell Labs **germanium transistor** → 1953 MIT **ferrite-core memory** → 1958 CERN **Ferranti Mercury** (vacuum-tube processor, ferrite memory, 40-bit words, magnetic drums) → 1961 IBM 709 → 1963 **IBM 7090** (transistors) → 1965 CDC 6600 → modern distributed computing and open scientific software.

This is an analytical lineage of technologies, not evidence of uninterrupted original hardware or byte-for-byte code transmission. The Mercury was transferred to Poland in 1966; surviving original magnetic tape images remain unverified.

## Run the collector
```bash
python3 tools/harvest.py --pages 2 --size 10 --max-records 40
```
Produces `harvest/records.jsonl`, `manifest.json`, `errors.json` and `sha256.json` when run in an environment with public Internet access. Opt-in binary download:
```bash
python3 tools/harvest.py --pages 1 --size 10 --max-records 10 --download-cc0 --max-file-mb 2 --max-total-mb 10
```

## Acquisition and evidence principles
1. Public sources only: no credentials, controlled networks, unpublished repositories or access circumvention.
2. Preserve original URLs, retrieval timestamps, record IDs, rights labels, and hashes of actual downloaded bytes.
3. Don't rehost third-party software, reports or data without checking their specific licenses. CERN Open Data *metadata* is CC0; binary datasets and other repositories can have different rights.
4. Do not attempt to push multi-petabyte scientific datasets into GitHub. Store bounded manifests and references; use archival storage for bulk data.
5. Treat CERN's GitLab, GitHub and public mirrors as potentially the same **source ancestry**, not independent corroboration.
6. Public modern systems (ROOT, CMSSW, Gaudi/Allen) are functional analogues or successors; no proved original REAP executable lineage to them.

## Starting references
[Computer History Museum: 1947 transistor](https://www.computerhistory.org/siliconengine/invention-of-the-point-contact-transistor/) · [CERN Mercury history](https://drupal-timeline.web.cern.ch/cern-installs-its-first-electronic-computer-ferranti-mercury) · [CERN computing timeline](https://drupal-timeline.web.cern.ch/timeline-header/109) · [Open Data](https://opendata.cern.ch/) · [ROOT](https://github.com/root-project/root).

## H020 — CERN toroidal coordinate mesh
- [Interactive six-ring Canvas2D model](docs/cern-24-2-4-3-1-1.html)
- [Machine-readable six-layer specification](models/h020-cern-toroid-24-2-4-3-1-1.json)
- Coordinate layers: **24 / 2 / 4 / 3 / 1 / 1**; 35 rendered nodes; 576 abstract combinations.
- 24 physical toroid-coil referents exist **only at ATLAS**. The other layers are an analytical mapping across the LHC and CERN, not magnets replicated at all four detectors.

## H021 — Time-indexed toroidal observation mesh
- [Interactive temporal 576-address viewer](docs/cern-h021-temporal-mesh.html)
- [Six-layer temporal/witness specification](models/h021-temporal-witness-mesh.json)
- [Deterministic address encoder and evidence evaluator](tools/temporal_mesh.py)
- 24×2×4×3×1×1 = **576 logical addresses**, indexed by UTC observation timestamps.
- Three separately rooted reports must agree exactly before a shared checkpoint may be *considered*; ancestry labels are not automatically verified, so the evaluator conservatively withholds a final trust claim.
- The viewer displays **NO MEASUREMENT ATTACHED** until real measurements with provenance are added; the sample ledger is synthetic.

## H022 — Real CERN public records and 576-address mesh bridge
- [Evidence dashboard](docs/h022-public-evidence.html) — 40 harvested records; 5 attributable by Open Data DOI (2 ALICE, 2 CMS, 1 LHCb); 35 intentionally unmapped.
- [Four sourced experiment datasets/services](evidence/h022-public-detector-seeds.json) — one ATLAS, CMS, ALICE and LHCb example with original DOI/URL and publication/time-granularity.
- [First-harvest bridge](evidence/h022-harvest-bridge.json) and [reproducible generator](tools/h022_bridge.py).
- **0 fully mapped 576-address measurements:** the sources identify detectors and dataset properties but do not establish physical ATLAS coil slot + a direction + abstract tier + observation UTC.
- Published event counts are **dataset metadata**, not measured values for the 576 toroidal coordinates. No collision event binary files have been downloaded by H022.

## H023 — First real public CMS event rows
- [Interactive Canvas2D view of genuine dimuon measurements](docs/h023-cms-real-events.html)
- [Four verbatim public CSV example rows](data/h023-cms-dimuon-excerpt.csv)
- [CERN/DOI, CC0 and observation-time provenance](data/h023-cms-dimuon-provenance.json)
- [Python CSV parser, validation and four-vector mass check](tools/h023_cms_events.py)
- [Canonical CERN record (100,000 derived dimuon events)](https://opendata.cern.ch/record/303) — DOI `10.7483/OPENDATA.CMS.4M97.3SQ9`. The repo presently contains **four authentic example rows**, not the full 100,000-event file.
- Reproducible local calculation: `python3 tools/h023_cms_events.py` outputs `data/h023-cms-derived-summary.json` with hashes, event IDs, mass checks, charge-pair counts and bins.
- Original measured-data timestamps remain unknown (year 2010 only). These CMS events have no verified assignment to the ATLAS-specific 24 toroid coil slots, rotational directions, or conceptual tiers. Do **not** interpret the H023 animation as physical collision geometry or a time series.

## H024 — CERN experiments 1969–2026 and particle scale atlas
- [Interactive CERN particle/experiment atlas](docs/h024-particle-atlas.html): historical milestones plus logarithmic spatial-scale diagram.
- [Research JSON with 17 milestones and 23 particle/scale entries](physics/h024-particles-experiments-1969-2026.json): all **17 Standard Model particle types** (counting W± as one type) plus six composite/atomic scale references.
- 1971 ISR, 1973 neutral currents, 1983 W/Z, 1989 LEP's three light active neutrino families, 2012 ATLAS/CMS Higgs, and 2026 LHCb rare-decay *evidence*.
- **Crucial geometry:** no geometric radius is measured for electron/quarks, neutrinos, photon/gluon, W/Z or Higgs. CERN's representative `<10^-19 m` electron/quark spatial-resolution statement is an experimental bound/context, not a literal measured sphere; the `~1.6×10^-18 m` 125-GeV Higgs reduced Compton wavelength is **not** a Higgs radius.
- Composite examples: proton charge rms radius ≈`0.84×10^-15 m`; pions/kaons `~10^-15 m`; atom `~10^-10 m`.
- CERN did not discover every Standard Model particle; the atlas notes non-CERN discovery origins for relevant particles.

## H025 — Higgs five-by-five reverse scale
- [Interactive logarithmic 25-step viewer](docs/h025-higgs-fivefold-descent.html)
- [26 calculated step values and comparison to Planck/3](physics/h025-higgs-5pow25-backward.json)
- Higgs reduced Compton length ~1.57735×10⁻¹⁸ m divided by 5²⁵ gives ~5.29272×10⁻³⁶ m, about 1.76% smaller than one-third of the Planck length. This is **mathematical proximity, not a demonstrated particle or spacetime structure**.

## H026 — 366-state toroid, 120×3 + 6
- [Interactive 366-state toroidal visualization](docs/h026-toroid-366.html)
- [Machine-readable 360 samples + 6 signed-axis anchors](models/h026-toroid-120x3-plus6.json)
- Three color bands of 120 samples each, plus ±X, ±Y, ±Z. Optional generational factorization 120 = 24×5. Squared Cartesian coordinates are used in the standard torus equation; they are not extra physical dimensions or measured CERN magnet values.

## H027 — 5³ cube × 5! permutation test / white-Higgs symbol
- [Interactive 125-address cube](docs/h027-white-permutation-cube.html): 120 colored permutation cells plus five white main-diagonal reserved cells.
- [Reversible Python permutation mapper](tools/h027_permutation_cube.py) and [frozen complete mapping](models/h027-permutation-cube-map.json).
- 6 quark + 6 lepton + 4 gauge-boson particle types = 16; Higgs is the 17th common Standard Model type and was the last to be discovered in 2012. **White is a symbolic completion marker, not a measured particle color or literal light emitted by the Higgs.**
- Formula: `5! = 120`, `5³ = 125`, five reserved addresses `0,31,62,93,124`. These counts are a constructed exact bijection, not an experimentally discovered correspondence.

## H028 — Closed-loop permutation cube across R/B/G toroidal stages
- [Interactive 360-step RGB orbit](docs/h028-closed-rgb-orbit.html)
- [Source-grounded 120-permutation rotation algorithm](tools/h028_closed_orbit.py), using the H027 125-cell cube and five fixed reserved white-axis positions.
- [Exhaustive orbit audit (361 checkpoints)](models/h028-closed-orbit-results.json): 120 distinct active occupants, 5 fixed reservations, 0 collisions, reversibility; **payload period 120**, **RGB labeled protocol period 360**.
- The three stages reuse the same 120 active cube positions. This is a deliberately designed permutation schedule, **not** a measurable Higgs/optical emission or a physical 360-coil assembly.

## H029 — Independent three-speed toroidal projections
- [Interactive three-speed Canvas2D model](docs/h029-three-speed-orbit.html)
- [Executable full-orbit verifier](tools/h029_multirate_orbit.py) and [721-step frozen results](models/h029-multirate-results.json).
- Red advances 1 slot/tick, Blue 1 slot/2 ticks, Green 1 slot/3 ticks. Individual periods: 120, 240, 360 ticks. Exact **first simultaneous return = lcm(120,240,360) = 720 ticks**.
- Same H027 120 permutations are projected in each band (not 360 novel permutations), with five fixed reserved diagonal cube addresses. All 721 states have injective, reversible maps; no collisions.
- Rate choices are artificial and not measured CERN magnet frequencies or particle dynamics. H026's six signed Cartesian axes are separate from H027's five reserved cube cells.
