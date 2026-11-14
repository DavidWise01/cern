"""Self-contained schema and provenance tests; do not require Internet access."""
import importlib.util
import json
import pathlib
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]


class ArchiveTests(unittest.TestCase):
    def test_seed_and_ordering(self):
        data = json.loads((ROOT / "history/timeline.json").read_text())
        events = data["events"]
        self.assertGreaterEqual(len(events), 10)
        self.assertEqual(events[0]["year"], 1947)
        self.assertIn("Germanium", events[0]["material"])
        self.assertEqual([x["year"] for x in events], sorted(x["year"] for x in events))
        self.assertTrue(all(x["source"].startswith("https://") for x in events))
        self.assertTrue(all(x["continuity"].strip() for x in events))

    def test_source_catalogue(self):
        data = json.loads((ROOT / "catalog/sources.json").read_text())
        keys = [x["key"] for x in data["sources"]]
        self.assertEqual(len(keys), len(set(keys)))
        self.assertGreaterEqual(len(keys), 15)
        self.assertTrue(all(x["url"].startswith("https://") for x in data["sources"]))
        self.assertTrue(all(x.get("rights") for x in data["sources"]))

    def test_four_experiment_harvest(self):
        spec = importlib.util.spec_from_file_location(
            "four_harvest", ROOT / "tools/harvest_experiments.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        self.assertEqual(set(module.EXPERIMENTS), {"ATLAS", "CMS", "ALICE", "LHCb"})
        self.assertIn("q=LHCb", module.search_url("LHCb", 1, 25))
        self.assertEqual(module.hits_from({"hits": {"hits": [{"id": "a"}]}}),
                         [{"id": "a"}])
        sample = module.flatten(
            {"id": "42", "metadata": {"title": "Example analysis"}},
            "ATLAS", module.search_url("ATLAS", 1, 25),
            "2026-11-14T00:00:00+00:00")
        self.assertFalse("attribution_verified" in sample)
        self.assertEqual(sample["search_label_not_verified_attribution"], "ATLAS")
        self.assertEqual(sample["record_url"], "https://opendata.cern.ch/record/42")

    def test_harvester_policy(self):
        spec = importlib.util.spec_from_file_location("cern_harvest", ROOT / "tools/harvest.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        self.assertTrue(module.is_cc0({"id": "CC0-1.0"}))
        self.assertFalse(module.is_cc0({"id": "CC-BY-4.0"}))
        self.assertFalse(module.allowed_url("http://opendata.cern.ch/data"))
        self.assertFalse(module.allowed_url("https://example.com/file"))
        self.assertTrue(module.allowed_url("https://opendata.cern.ch/api/records/"))
        self.assertEqual(module.record_list({"hits": {"hits": [{"id": 1}]}}), [{"id": 1}])
        self.assertEqual(module.record_list({"hits": []}), [])


if __name__ == "__main__":
    unittest.main()
