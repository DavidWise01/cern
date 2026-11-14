#!/usr/bin/env python3
"""H017: bounded public HEPData publication-results metadata index.

HEPData serves published plot/table data; this script records search hits,
DOIs, collaboration, titles and record links without copying research papers
or bulk experimental datasets. No account or private CERN access is needed.
"""
import argparse
import datetime as dt
import hashlib
import json
import pathlib
import time
import urllib.parse
import urllib.request

EXPERIMENTS = ("ATLAS", "CMS", "ALICE", "LHCb")
API = "https://www.hepdata.net/search/"


def url_for(exp, page, size):
    return API + "?" + urllib.parse.urlencode(
        {"q": "", "collaboration": exp, "page": page, "size": size, "format": "json"}
    )


def fetch(url):
    with urllib.request.urlopen(
        urllib.request.Request(url, headers={
            "User-Agent": "DavidWise01-cern-H017/1.0",
            "Accept": "application/json",
        }), timeout=35
    ) as response:
        return json.load(response)


def hits(data):
    return data.get("results", []) if isinstance(data, dict) else []


def normalize(hit, exp, url, timestamp):
    record_id = hit.get("id") or hit.get("recid") or hit.get("inspire_id")
    doi = hit.get("doi") or hit.get("doi_url") or hit.get("hepdata_doi")
    return {
        "search_collaboration": exp,
        "metadata_verified_as_same_collaboration": False,
        "id": record_id,
        "title": hit.get("title") or hit.get("data_abstract"),
        "doi": doi,
        "record_url": hit.get("url") or (
            "https://www.hepdata.net/record/" + str(record_id) if record_id else None
        ),
        "collaboration_field": hit.get("collaboration"),
        "source_query_url": url,
        "retrieved_utc": timestamp,
    }


def run(output, pages, size, max_results):
    timestamp = dt.datetime.now(dt.timezone.utc).isoformat()
    results = {}
    for exp in EXPERIMENTS:
        folder = output / exp.lower()
        folder.mkdir(parents=True, exist_ok=True)
        records, seen, errors = [], set(), []
        for page in range(1, pages + 1):
            if len(records) >= max_results:
                break
            url = url_for(exp, page, size)
            try:
                data = fetch(url)
                batch = hits(data)
                if not batch:
                    break
                for item in batch:
                    if len(records) >= max_results:
                        break
                    rec = normalize(item, exp, url, timestamp)
                    key = str(rec["id"] or rec["doi"] or rec["title"])
                    if key not in seen:
                        seen.add(key)
                        records.append(rec)
            except Exception as exc:
                errors.append({"url": url, "error": str(exc)})
                break
            time.sleep(0.2)
        output_path = folder / "records.jsonl"
        output_path.write_text("".join(json.dumps(r, sort_keys=True, ensure_ascii=False)
                                       + "\n" for r in records), encoding="utf-8")
        digest = hashlib.sha256(output_path.read_bytes()).hexdigest()
        manifest = {
            "source": "HEPData public search JSON",
            "collaboration": exp,
            "records": len(records),
            "errors": errors,
            "sha256": digest,
            "retrieved_utc": timestamp,
            "complete_corpus": False,
            "results_are_metadata_only": True,
            "note": "Search match is not independently verified experiment ancestry.",
        }
        (folder / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
        results[exp] = {"records": len(records), "errors": len(errors)}
    print(json.dumps(results, indent=2))
    return 0 if any(x["records"] for x in results.values()) else 2


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--pages", type=int, default=10)
    p.add_argument("--size", type=int, default=25)
    p.add_argument("--max-per-experiment", type=int, default=250)
    p.add_argument("--output", default="hepdata")
    a = p.parse_args()
    if not (1 <= a.pages <= 50 and 1 <= a.size <= 200 and 1 <= a.max_per_experiment <= 2500):
        p.error("Invalid page, size, or result limit")
    return run(pathlib.Path(a.output), a.pages, a.size, a.max_per_experiment)


if __name__ == "__main__":
    raise SystemExit(main())
