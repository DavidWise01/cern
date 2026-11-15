#!/usr/bin/env python3
"""H022 source-grounded bridge from public CERN catalogue to conceptual H021 mesh.

Only a documented experiment can be mapped. Slot, beam direction, and strength
tier remain null until an actual observation supports those fields.
A year-only date must NEVER be promoted to a fabricated UTC timestamp.
"""
import argparse
import datetime as dt
import hashlib
import json
import pathlib
from collections import Counter

ORDER = ("ATLAS", "ALICE", "CMS", "LHCb")
DOI_PREFIXES = {
    "ATLAS": "10.7483/OPENDATA.ATLAS.",
    "ALICE": "10.7483/OPENDATA.ALICE.",
    "CMS": "10.7483/OPENDATA.CMS.",
    "LHCb": "10.7483/OPENDATA.LHCB.",
}


def experiment_from_source(record):
    """Conservative authoritative hints. No attribution from title alone."""
    doi = str(record.get("doi") or "").upper()
    matches = [name for name, prefix in DOI_PREFIXES.items()
               if doi.startswith(prefix.upper())]
    if len(matches) == 1:
        return matches[0], "doi_prefix"
    # CERN record response can include an explicit experiment field.
    explicit = record.get("experiment_field_as_returned")
    if isinstance(explicit, str) and explicit.casefold() in {
            exp.casefold() for exp in ORDER}:
        return next(exp for exp in ORDER if explicit.casefold() == exp.casefold()), "explicit_field"
    return None, "unverified"


def bridge(record):
    detector, proof = experiment_from_source(record)
    year = None
    date = record.get("publication_date")
    if isinstance(date, str) and len(date) >= 4 and date[:4].isdigit():
        year = int(date[:4])
    return {
        "source_record_id": record.get("record_id"),
        "source_url": record.get("public_record_url"),
        "source_api_url": record.get("source_api_url"),
        "source_record_title": record.get("title"),
        "doi": record.get("doi"),
        "source_catalogue": record.get("source"),
        "original_harvest_timestamp": record.get("retrieved_utc"),
        "source_publication_year": year,
        "source_root": "cern-record:" + str(record.get("source")) + ":" +
                       str(record.get("record_id")),
        "mapped_detector": detector,
        "mapping_evidence": proof,
        "mesh_address": None,
        "slot": None,
        "direction": None,
        "tier": None,
        "observed_at_utc": None,
        "measurement_value": None,
        "measurement_unit": None,
        "status": "PARTIAL_DETECTOR_REFERENCE" if detector else "UNMAPPED_REFERENCE",
        "prohibition": "Do not treat this catalogue hit as a measured 576-address physics state.",
    }


def convert(rows):
    results = [bridge(item) for item in rows]
    return {
        "schema": "cern.h022.metadata-bridge.v1",
        "count": len(results),
        "detector_counts": dict(Counter(x["mapped_detector"] or "UNMAPPED"
                                         for x in results)),
        "verified_full_address_count": sum(x["mesh_address"] is not None for x in results),
        "source_ancestry_note": (
            "Shared API publications are one source root, even if mirrored."
        ),
        "records": results,
    }


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--input", default="harvest/records.jsonl")
    p.add_argument("--output", default="evidence/h022-harvest-bridge.json")
    a = p.parse_args()
    src = pathlib.Path(a.input)
    rows = [json.loads(line) for line in src.read_text(encoding="utf-8").splitlines()
            if line.strip()]
    doc = convert(rows)
    doc["source_sha256"] = hashlib.sha256(src.read_bytes()).hexdigest()
    doc["generated_utc"] = dt.datetime.now(dt.timezone.utc).isoformat()
    out = pathlib.Path(a.output)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps({"records": doc["count"],
                      "detectors": doc["detector_counts"],
                      "fully_addressed": doc["verified_full_address_count"]}, indent=2))


if __name__ == "__main__":
    main()
