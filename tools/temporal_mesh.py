#!/usr/bin/env python3
"""H021 deterministic temporal / witness ledger for a 576-address conceptual mesh.

No CERN measurements are produced, retrieved or simulated as real evidence.
The optional --demo creates clearly marked synthetic observations only.
"""
import argparse
import datetime as dt
import json
import pathlib
from collections import defaultdict

SLOTS = tuple(channel + str(i) for channel in "RBG" for i in range(8))
DIRECTIONS = ("CW", "CCW")
DETECTORS = ("ATLAS", "ALICE", "CMS", "LHCb")
TIERS = ("weak", "medium", "strong")
FIELDS = ("address", "observed_at", "variable", "unit", "value")


def encode(slot, direction, detector, tier):
    return (((SLOTS.index(slot) * 2 + DIRECTIONS.index(direction)) * 4
             + DETECTORS.index(detector)) * 3 + TIERS.index(tier))


def decode(address):
    if not isinstance(address, int) or isinstance(address, bool) or not 0 <= address < 576:
        raise ValueError("Address must be an integer between 0 and 575")
    return {
        "address": address,
        "slot": SLOTS[address // 24],
        "direction": DIRECTIONS[(address // 12) % 2],
        "detector": DETECTORS[(address // 3) % 4],
        "tier": TIERS[address % 3],
        "accelerator": "LHC",
        "institution": "CERN",
    }


def timestamp(value):
    if not isinstance(value, str):
        raise ValueError("timestamp must be ISO 8601 UTC text")
    parsed = dt.datetime.fromisoformat(value.replace("Z", "+00:00"))
    if parsed.tzinfo is None or parsed.utcoffset() != dt.timedelta(0):
        raise ValueError("timestamp must be explicitly UTC")
    return parsed.isoformat(timespec="seconds").replace("+00:00", "Z")


def validate(row):
    if row.get("kind") != "measurement" or row.get("synthetic") is True:
        raise ValueError("Real measurement records require kind=measurement and synthetic=false")
    for key in (*FIELDS, "root", "source_url", "record_id", "ingested_at"):
        if key not in row:
            raise ValueError("Missing " + key)
    decode(row["address"])
    timestamp(row["observed_at"])
    timestamp(row["ingested_at"])
    if row["ingested_at"] < row["observed_at"]:
        raise ValueError("Ingest time precedes observation")
    if not all(isinstance(row.get(key), str) and row[key] for key in
               ("root", "source_url", "record_id", "variable", "unit")):
        raise ValueError("Empty provenance or variable field")
    if not row["source_url"].startswith("https://"):
        raise ValueError("Public source URL must use HTTPS")
    if not isinstance(row["value"], (int, float, str)) or isinstance(row["value"], bool):
        raise ValueError("Value must be scalar and non-boolean")
    return row


def summarize(rows):
    """Strict, conservative gate: same event and variable, 3 unique source roots."""
    grouped = defaultdict(list)
    for row in rows:
        if row.get("synthetic") is True or row.get("kind") != "measurement":
            continue
        try:
            validate(row)
        except (ValueError, KeyError):
            continue
        event = (row["address"], timestamp(row["observed_at"]), row["variable"], row["unit"])
        grouped[event].append(row)
    results = []
    for event, items in sorted(grouped.items()):
        # Collapse repeated sightings of one source ancestry; disputes within one root block trust.
        roots = defaultdict(set)
        for x in items:
            roots[x["root"]].add(json.dumps(x["value"], sort_keys=True))
        distinct_values = set().union(*roots.values())
        trusted = (len(roots) >= 3 and len(distinct_values) == 1
                   and all(len(values) == 1 for values in roots.values()))
        results.append({
            "address": event[0], "observed_at": event[1],
            "variable": event[2], "unit": event[3],
            "independent_roots_claimed": len(roots),
            "values_observed": sorted(distinct_values),
            "decision": "AGREEMENT_3_ROOTS_CLAIMED" if trusted
                        else ("DISAGREEMENT" if len(distinct_values) > 1
                              else "INSUFFICIENT_ROOTS"),
            "verified_independence": False,
            "trusted": False,
            "limitation": "Ancestry labels alone do not prove independent roots; inspect citation chains before trust."
        })
    return results


def demo():
    """Synthetic layout demonstrations are not measurements, even with 3 labels."""
    when = "2026-11-14T20:00:00Z"
    return [{
        "kind": "synthetic_fixture", "synthetic": True,
        "address": encode("R0", "CW", "ATLAS", "weak"),
        "observed_at": when, "ingested_at": when,
        "variable": "illustrative_state", "value": 1, "unit": "arbitrary",
        "root": "fictional_" + root, "record_id": "demo-" + root,
        "source_url": "https://example.org/fictional/" + root
    } for root in ("a", "b", "c")]


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--input", help="Newline-delimited JSON, with original citations")
    p.add_argument("--demo", action="store_true", help="Make synthetic fixture ONLY")
    p.add_argument("--output", default="models/h021-example-ledger.json")
    a = p.parse_args()
    if not a.input and not a.demo:
        p.error("Provide --input or --demo")
    if a.demo and a.input:
        p.error("Use only one of --demo or --input")
    rows = demo() if a.demo else [json.loads(line) for line in
                 pathlib.Path(a.input).read_text(encoding="utf-8").splitlines() if line.strip()]
    result = {
        "schema": "cern.h021.ledger.v1",
        "addresses": 576,
        "records": rows,
        "assessments": summarize(rows),
        "note": "No trust decision without verified independent source ancestry; demonstration is synthetic."
    }
    out = pathlib.Path(a.output)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(f"Wrote {out}: {len(rows)} records; {len(result['assessments'])} assessments")


if __name__ == "__main__":
    main()
