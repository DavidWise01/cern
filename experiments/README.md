# H017 — Four experiments around one LHC

The **LHC is one shared accelerator**, with four major experiment stations:
ATLAS, CMS, ALICE, LHCb. These are not four separate colliders. CERN currently
lists **nine** LHC experiments including five smaller collaborations: TOTEM,
LHCf, MoEDAL-MAPP, FASER and SND@LHC.

## Four-around-one analytical topology

```
               CMS
                ○
        ╭─────────────╮
   ALICE ○     LHC     ○ LHCb
        ╰─────────────╯
                ○
              ATLAS
```

Diagram is topology **not a scale map or their actual compass locations**.

| Experiment | CERN role | CERN Open Data | Public publication/results archive |
| --- | --- | --- | --- |
| ATLAS | General-purpose detector; Higgs, precision, new phenomena | https://opendata.cern.ch/search?experiment=ATLAS | https://cds.cern.ch/collection/ATLAS |
| CMS | Independent general-purpose detector; Higgs, precision, new phenomena | https://opendata.cern.ch/search?experiment=CMS | https://cds.cern.ch/collection/CMS |
| ALICE | Heavy-ion and quark–gluon plasma | https://opendata.cern.ch/search?experiment=ALICE | https://cds.cern.ch/collection/ALICE |
| LHCb | Heavy-flavour, CP violation and matter/antimatter asymmetries | https://opendata.cern.ch/search?experiment=LHCb | https://cds.cern.ch/collection/LHCb |

## Published CERN Document Server counts (observed web page, 2026-11-14)
CERN Experiments collection page: https://cds.cern.ch/collection/CERN%20Experiments

- ATLAS — 73,989
- CMS — 16,489
- ALICE — 4,970
- LHCb — 14,290
- Combined four subcollections = 109,738. **These are CDS catalogued records, not 109,738 independent physics findings, datasets, or files downloaded.**
- LHC Experiments heading: 109,940; includes small-experiment subcollections and other headings.
- Counts may shift with catalogue updates and may include cross-indexed records.

The four detector collaborations have separate analysis chains, and the two independent general-purpose detectors (ATLAS & CMS) can corroborate discoveries, but mirrored content or shared collision conditions must not be called four fully independent evidentiary roots.

## Current and reusable public results
* CERN Open Data experiments: https://opendata.cern.ch/ (choose ALICE, ATLAS, CMS, or LHCb).
* CERN Repository public metadata API: https://repository.cern/api/records
* CERN Document Server papers: https://cds.cern.ch/collection/CERN%20Experiments
* Machine-readable published figure/table results: https://www.hepdata.net/
* LHCb Run II public access service: https://opendata.cern.ch/ (announced 2026-02-22).
* ALICE educational LHC15o datasets: https://opendata.cern.ch/ (announced 2026-03-10).
* ATLAS 2025 education/outreach beta open data: https://opendata.cern.ch/ (announced 2025-07-03).

## Running per-experiment harvest
```bash
python3 tools/harvest_experiments.py --experiment ALL --pages 10 --size 25 --max-per-experiment 250
python3 -m unittest discover -s tests -v
```

The output is one **bounded metadata sample per experiment** in
`experiments/<name>/records.jsonl`, `manifest.json`, and `errors.json`.
An explicit experiment query is sent to the public API; matching is **search-based
discovery**, not a guaranteed exhaustive experiment-tag filter. Check record
metadata before attributing a result to a detector. Full public results sets may
be vastly larger, paginated and published through other catalogues.

**No original collision datasets are downloaded by this experiment metadata workflow**.
Open data files vary from small tutorials to petabyte-scale scientific collections.
Individual rights must be checked before redistribution.

## Next evidence test
Capture a public DOI and exact data file for each detector, plus a linked analysis
paper, and verify checksums and machine-readable outputs. Keep the four
collaboration ancestries distinct and record shared beam-level conditions.

## Sources
- https://home.cern/science/experiments/
- https://home.cern/science/accelerators/large-hadron-collider/
- https://cds.cern.ch/collection/CERN%20Experiments
- https://openscience.web.cern.ch/research-data-management-open-data/
- https://opendata.cern.ch/
