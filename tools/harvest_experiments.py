#!/usr/bin/env python3
"""H017 — discover public CERN records for the four main LHC experiments.

This uses CERN's public Open Data search API and records queries exactly.
It does NOT claim a query match proves an experiment attribution; false
positives and duplicate records should be independently checked.
"""
import argparse
import datetime as dt
import hashlib
import json
import pathlib
import time
import urllib.error
import urllib.parse
import urllib.request

EXPERIMENTS = ("ATLAS", "CMS", "ALICE", "LHCb")
BASE = "https://opendata.cern.ch/api/records/"
HEADERS = {
    "Accept": "application/json",
    "User-Agent": "DavidWise01-cern-H017-public-metadata/1.0",
}


def search_url(experiment, page, size):
    # Search is intentionally explicit and logged; NOT an exact experiment filter.
    query = urllib.parse.urlencode({"q": experiment, "page": page, "size": size})
    return BASE + "?" + query


def read_json(url):
    with urllib.request.urlopen(
        urllib.request.Request(url, headers=HEADERS), timeout=35
    ) as response:
        if response.status != 200:
            raise RuntimeError(f"HTTP {response.status} at {url}")
        return json.load(response)


def hits_from(payload):
    hits = payload.get("hits", {})
    if isinstance(hits, dict):
        return hits.get("hits", [])
    if isinstance(hits, list):
        return hits
    return []


def flatten(item, experiment, url, timestamp):
    meta = item.get("metadata", {})
    if not isinstance(meta, dict):
        meta = {}
    identifier = str(item.get("id", item.get("recid", "")))
    files = item.get("files", {})
    entries = files.get("entries", {}) if isinstance(files, dict) else {}
    if isinstance(entries, dict):
        file_count = len(entries)
    elif isinstance(entries, list):
        file_count = len(entries)
    else:
        file_count = 0
    exp = meta.get("experiment") or meta.get("experiments") or meta.get("collaboration")
    links = item.get("links", {})
    return {
        "search_label_not_verified_attribution": experiment,
        "experiment_field_as_returned": exp,
        "record_id": identifier,
        "title": meta.get("title", item.get("title")),
        "doi": meta.get("doi"),
        "record_url": "https://opendata.cern.ch/record/" + identifier,
        "source_url": url,
        "retrieved_utc": timestamp,
        "file_count_in_search_response": file_count,
        "license_field": meta.get("license") or meta.get("rights"),
        "description_excerpt": str(meta.get("description", ""))[:500],
    }


def run(experiments, out, pages, size, limit):
    timestamp = dt.datetime.now(dt.timezone.utc).isoformat()
    totals = {}
    for exp in experiments:
        folder = out / exp.lower()
        folder.mkdir(parents=True, exist_ok=True)
        found, errors, visited, source_pages = [], [], set(), []
        for page in range(1, pages + 1):
            if len(found) >= limit:
                break
            url = search_url(exp, page, size)
            source_pages.append(url)
            try:
                data = read_json(url)
                page_hits = hits_from(data)
                if not page_hits:
                    break
                for item in page_hits:
                    if len(found) >= limit:
                        break
                    record = flatten(item, exp, url, timestamp)
                    key = record["record_id"]
                    if key and key not in visited:
                        visited.add(key)
                        found.append(record)
            except (urllib.error.URLError, TimeoutError, OSError, RuntimeError,
                    ValueError, KeyError, TypeError) as err:
                errors.append({"url": url, "error": str(err)})
                break
            time.sleep(0.25)
        with (folder / "records.jsonl").open("w", encoding="utf-8") as f:
            for rec in found:
                f.write(json.dumps(rec, ensure_ascii=False, sort_keys=True) + "\n")
        (folder / "errors.json").write_text(
            json.dumps(errors, indent=2) + "\n", encoding="utf-8"
        )
        digest = hashlib.sha256((folder / "records.jsonl").read_bytes()).hexdigest()
        manifest = {
            "schema": "cern.h017.experiment-query-snapshot.v1",
            "experiment_search_term": exp,
            "attribution_verified": False,
            "records": len(found),
            "errors": len(errors),
            "retrieved_utc": timestamp,
            "source_pages_attempted": source_pages,
            "records_jsonl_sha256": digest,
            "metadata_only": True,
            "exhaustive": False,
            "limitations": ("Search results may contain false positives. "
                            "Does not include all CERN CDS publications, HEPData tables, "
                            "or binary collision datasets."),
        }
        (folder / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
        totals[exp] = {"records": len(found), "errors": len(errors), "sha256": digest}
    (out / "summary.json").write_text(
        json.dumps({"retrieved_utc": timestamp, "results": totals,
                    "data_type": "public catalogue metadata, not collision files",
                    "completeness": "bounded sample, not all published results"}, indent=2)
        + "\n", encoding="utf-8"
    )
    print(json.dumps(totals, indent=2))
    return 0 if any(x["records"] for x in totals.values()) else 2


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--experiment", default="ALL",
                   choices=("ALL",) + EXPERIMENTS)
    p.add_argument("--pages", type=int, default=10)
    p.add_argument("--size", type=int, default=25)
    p.add_argument("--max-per-experiment", type=int, default=250)
    p.add_argument("--output", default="experiments")
    a = p.parse_args()
    if not (1 <= a.pages <= 50 and 1 <= a.size <= 50
            and 1 <= a.max_per_experiment <= 2500):
        p.error("Bounded options: pages 1..50, size 1..50, per experiment 1..2500")
    experiments = EXPERIMENTS if a.experiment == "ALL" else (a.experiment,)
    return run(experiments, pathlib.Path(a.output), a.pages, a.size,
               a.max_per_experiment)


if __name__ == "__main__":
    raise SystemExit(main())
