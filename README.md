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
