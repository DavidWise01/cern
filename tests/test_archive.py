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

    def test_h020_toroidal_layers(self):
        model = json.loads((ROOT / "models/h020-cern-toroid-24-2-4-3-1-1.json").read_text())
        counts = [layer["count"] for layer in model["radial_layers"]]
        self.assertEqual(counts, [24, 2, 4, 3, 1, 1])
        self.assertEqual(sum(counts), 35)
        from math import prod
        self.assertEqual(prod(counts), 576)
        self.assertEqual(model["counts"]["cartesian_address_combinations"], 576)
        self.assertEqual(model["counts"]["independent_physical_toroid_coils_at_atlas"], 24)
        for layer in model["radial_layers"]:
            self.assertEqual(layer["count"], len(layer["labels"]))
        self.assertEqual(model["radial_layers"][0]["labels"][:8],
                         [f"R{i}" for i in range(8)])
        self.assertEqual(model["radial_layers"][0]["labels"][-8:],
                         [f"G{i}" for i in range(8)])
        self.assertEqual(model["radial_layers"][2]["labels"],
                         ["ATLAS", "ALICE", "CMS", "LHCb"])

    def test_h021_temporal_coordinate_invariant(self):
        spec = importlib.util.spec_from_file_location(
            "mesh_h021", ROOT / "tools/temporal_mesh.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        for address in range(576):
            item = module.decode(address)
            self.assertEqual(
                module.encode(item["slot"], item["direction"],
                              item["detector"], item["tier"]),
                address)
        self.assertEqual(len(module.SLOTS), 24)
        self.assertEqual(module.decode(0)["slot"], "R0")
        self.assertEqual(module.decode(575)["tier"], "strong")
        self.assertEqual(module.summarize(module.demo()), [])
        self.assertEqual(module.timestamp("2026-11-14T20:00:00Z"),
                         "2026-11-14T20:00:00Z")

    def test_h021_witness_gate_and_shared_ancestry(self):
        spec = importlib.util.spec_from_file_location(
            "mesh_h021", ROOT / "tools/temporal_mesh.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        base = {
            "kind": "measurement", "synthetic": False,
            "address": 0, "observed_at": "2026-11-14T20:00:00Z",
            "ingested_at": "2026-11-14T20:01:00Z",
            "variable": "example", "value": 3, "unit": "arbitrary",
            "source_url": "https://example.org/independent-record",
            "record_id": "sample",
        }
        rows = [dict(base, root="same"), dict(base, root="same")]
        self.assertEqual(module.summarize(rows)[0]["decision"],
                         "INSUFFICIENT_ROOTS")
        rows += [dict(base, root="second"), dict(base, root="third")]
        result = module.summarize(rows)[0]
        self.assertEqual(result["decision"], "AGREEMENT_3_ROOTS_CLAIMED")
        self.assertFalse(result["trusted"])  # independent ancestry unverified
        rows.append(dict(base, root="fourth", value=4))
        self.assertEqual(module.summarize(rows)[0]["decision"],
                         "DISAGREEMENT")

    def test_h022_partial_source_bridge(self):
        spec = importlib.util.spec_from_file_location(
            "bridge_h022", ROOT / "tools/h022_bridge.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        raw = {"record_id": "1120", "source": "open_data",
               "public_record_url": "https://opendata.cern.ch/record/1120",
               "doi": "10.7483/OPENDATA.ALICE.Y62S.E7UR",
               "title": "Pb Pb sample", "retrieved_utc": "2026-11-14T22:00:00Z"}
        result = module.bridge(raw)
        self.assertEqual(result["mapped_detector"], "ALICE")
        self.assertEqual(result["mapping_evidence"], "doi_prefix")
        self.assertIsNone(result["mesh_address"])
        self.assertIsNone(result["observed_at_utc"])
        self.assertIsNone(result["measurement_value"])
        unproven = module.bridge(dict(raw, doi=None, title="ATLAS test"))
        self.assertIsNone(unproven["mapped_detector"])
        self.assertEqual(unproven["status"], "UNMAPPED_REFERENCE")

    def test_h022_referenced_fixtures(self):
        doc = json.loads((ROOT / "evidence/h022-public-detector-seeds.json").read_text())
        self.assertEqual(set(x["experiment"] for x in doc["records"]),
                         {"ATLAS", "CMS", "ALICE", "LHCb"})
        self.assertTrue(all(x["mesh_mapping"]["address"] is None
                            for x in doc["records"]))
        actual = json.loads((ROOT / "evidence/h022-harvest-bridge.json").read_text())
        self.assertEqual(actual["count"], 40)
        self.assertEqual(actual["verified_full_address_count"], 0)
        self.assertEqual(sum(actual["detector_counts"].values()), 40)

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
