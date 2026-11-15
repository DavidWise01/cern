#!/usr/bin/env python3
"""H023: read authentic public CMS dimuon CSV rows without inventing timestamps.

Validates detector metadata and the source identifiers; computes simple
kinematical summaries. Optional --input accepts a larger *user-downloaded*
CERN CSV after the original file URL and rights are verified.
"""
import argparse
import csv
import datetime as dt
import hashlib
import json
import math
import pathlib
from collections import Counter

FIELDS = ["Run","Event","Type1","E1","px1","py1","pz1","pt1","eta1","phi1","Q1",
          "Type2","E2","px2","py2","pz2","pt2","eta2","phi2","Q2","M"]
NUMERIC = ["E1","px1","py1","pz1","pt1","eta1","phi1","E2","px2",
           "py2","pz2","pt2","eta2","phi2","M"]


def load_rows(path, limit=100000):
    with pathlib.Path(path).open(newline="", encoding="utf-8-sig") as stream:
        data = csv.DictReader(stream)
        fields = [x.strip() for x in (data.fieldnames or [])]
        if fields != FIELDS:
            raise ValueError("Unexpected CMS CSV fields, got " + repr(fields))
        records, seen = [], set()
        for raw in data:
            if len(records) >= limit:
                raise ValueError("row limit exceeded")
            if None in raw:
                raise ValueError("trailing extra CSV fields")
            row = {k: v.strip() for k,v in raw.items()}
            for field in NUMERIC:
                row[field] = float(row[field])
                if not math.isfinite(row[field]):
                    raise ValueError("non-finite numeric value: " + field)
            for field in ("Run","Event","Q1","Q2"):
                row[field] = int(row[field])
            if row["Run"] <= 0 or row["Event"] < 0:
                raise ValueError("invalid run/event identifier")
            if row["Q1"] not in (-1, 1) or row["Q2"] not in (-1, 1):
                raise ValueError("unexpected muon charge")
            if row["M"] < 0 or row["E1"] < 0 or row["E2"] < 0:
                raise ValueError("invalid energy or mass")
            if row["Type1"] not in ("G","T") or row["Type2"] not in ("G","T"):
                raise ValueError("unknown CMS muon type")
            identifier = (row["Run"],row["Event"])
            if identifier in seen:
                raise ValueError("duplicate run/event within input")
            seen.add(identifier)
            records.append(row)
    return records


def invariant_mass_from_four_vectors(row):
    energy = row["E1"]+row["E2"]
    p2 = sum((row[c+"1"]+row[c+"2"])**2 for c in ("px","py","pz"))
    return math.sqrt(max(0.0,energy**2-p2))


def summarize(rows, source_hash, provenance):
    masses=[r["M"] for r in rows]
    bins=[0]*12
    # 0 <= M < 120 GeV, width 10 GeV
    for mass in masses:
        index=min(int(mass//10),11)
        bins[index]+=1
    mismatches=[]
    events=[]
    for r in rows:
        recomputed=invariant_mass_from_four_vectors(r)
        delta=abs(recomputed-r["M"])
        if delta>0.05:
            mismatches.append({"run":r["Run"],"event":r["Event"],
                               "source_mass_GeV":r["M"],
                               "derived_mass_GeV":round(recomputed,6),
                               "absolute_difference_GeV":round(delta,6)})
        events.append({
            "run":r["Run"],"event":r["Event"],
            "mass_GeV":r["M"],"calculated_mass_GeV":round(recomputed,6),
            "charge_pair":[r["Q1"],r["Q2"]],
            "pt_pair_GeV":[r["pt1"],r["pt2"]],
            "phi_pair_rad":[r["phi1"],r["phi2"]],
            "source_event_timestamp":None,
            "observed_year":provenance["data_recorded_year"],
            "detector":"CMS",
            "mesh_coordinate":None,
        })
    return {
        "schema":"cern.h023.real-event-analysis.v1",
        "experiment":"CMS",
        "real_public_measurement_derivatives":True,
        "analysis_suitable_for_full_physics":False,
        "source_url":provenance["source_url"],
        "doi":provenance["doi"],
        "input_sha256":source_hash,
        "rows":len(rows),
        "mass_GeV":{"min":min(masses) if masses else None,
                    "max":max(masses) if masses else None,
                    "mean":sum(masses)/len(masses) if masses else None},
        "mass_histogram_10GeV_bins":bins,
        "charge_pair_counts":dict(Counter(str((r["Q1"],r["Q2"])) for r in rows)),
        "four_vector_checks":{"recalculated_for_all":True,
                              "tolerance_GeV":0.05,"mismatches":mismatches},
        "event_rows":events,
        "temporal_provenance":"2010 only; no per-event UTC timestamps supplied by source excerpt",
        "unmapped_fields":["slot","direction","tier","mesh_address"],
        "warning":"No physical toroid state or precise event time was inferred."
    }


def main():
    p=argparse.ArgumentParser()
    p.add_argument("--input",default="data/h023-cms-dimuon-excerpt.csv")
    p.add_argument("--source",default="data/h023-cms-dimuon-provenance.json")
    p.add_argument("--output",default="data/h023-cms-derived-summary.json")
    p.add_argument("--max-rows",type=int,default=100000)
    a=p.parse_args()
    if not 1<=a.max_rows<=1000000:
        p.error("max rows must be in 1..1000000")
    provenance=json.loads(pathlib.Path(a.source).read_text(encoding="utf-8"))
    if provenance.get("experiment")!="CMS":
        raise ValueError("Input not accompanied by explicit CMS provenance")
    raw=pathlib.Path(a.input).read_bytes()
    rows=load_rows(a.input,a.max_rows)
    output=summarize(rows,hashlib.sha256(raw).hexdigest(),provenance)
    out=pathlib.Path(a.output)
    out.parent.mkdir(parents=True,exist_ok=True)
    out.write_text(json.dumps(output,indent=2)+"\n",encoding="utf-8")
    print(f"CMS dimuon: {len(rows)} real published excerpt rows, "
          f"{len(output['four_vector_checks']['mismatches'])} recalculation differences")


if __name__=="__main__":
    main()
