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

    def test_h023_real_cms_dimuon_rows(self):
        spec = importlib.util.spec_from_file_location(
            "h023_cms", ROOT / "tools/h023_cms_events.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        rows = module.load_rows(ROOT / "data/h023-cms-dimuon-excerpt.csv")
        self.assertEqual(len(rows), 4)
        self.assertEqual(rows[0]["Run"], 146436)
        self.assertEqual(rows[0]["Event"], 90830792)
        self.assertAlmostEqual(rows[0]["M"], 2.73205)
        self.assertEqual([row["Event"] for row in rows],
                         [90830792, 90862225, 90644850, 90678594])
        for row in rows:
            mass = module.invariant_mass_from_four_vectors(row)
            self.assertAlmostEqual(mass, row["M"], delta=0.05)

    def test_h023_provenance_and_no_fabricated_times(self):
        spec = importlib.util.spec_from_file_location(
            "h023_cms", ROOT / "tools/h023_cms_events.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        import hashlib
        source = ROOT / "data/h023-cms-dimuon-excerpt.csv"
        provenance = json.loads(
            (ROOT / "data/h023-cms-dimuon-provenance.json").read_text())
        report = module.summarize(
            module.load_rows(source),
            hashlib.sha256(source.read_bytes()).hexdigest(),
            provenance)
        self.assertEqual(report["rows"], 4)
        self.assertEqual(report["doi"], "10.7483/OPENDATA.CMS.4M97.3SQ9")
        self.assertEqual(sum(report["mass_histogram_10GeV_bins"]), 4)
        self.assertTrue(all(x["source_event_timestamp"] is None
                            and x["mesh_coordinate"] is None
                            for x in report["event_rows"]))
        self.assertEqual(report["four_vector_checks"]["mismatches"], [])

    def test_h024_particle_atlas(self):
        data = json.loads((ROOT / "physics/h024-particles-experiments-1969-2026.json").read_text())
        history = data["timeline"]
        self.assertEqual(history[0]["year"], 1969)
        self.assertEqual(history[-1]["year"], 2026)
        self.assertEqual([x["year"] for x in history], sorted(x["year"] for x in history))
        self.assertTrue(any(x["year"] == 2012 and "Higgs" in x["experiment"] for x in history))
        self.assertTrue(all(x["source"].startswith("https://") for x in history))
        particles = data["particles"]
        self.assertEqual(len(particles), 23)
        self.assertEqual(len([p for p in particles if p["nature"] == "pointlike"]), 17)
        self.assertTrue(all(p["characteristic_radius_m"] is None for p in particles
                            if p["nature"] == "pointlike"))
        higgs = next(x for x in particles if x["name"] == "Higgs boson")
        self.assertIsNone(higgs["characteristic_radius_m"])
        proton = next(x for x in particles if x["name"] == "proton")
        self.assertAlmostEqual(proton["characteristic_radius_m"], 0.84e-15)
        self.assertTrue(proton["physical_radius_measured"])

    def test_h026_366_toroid_mapping(self):
        import math
        model=json.loads((ROOT / "models/h026-toroid-120x3-plus6.json").read_text())
        self.assertEqual(model["counts"]["ring_positions"], 360)
        self.assertEqual(model["counts"]["total"], 366)
        self.assertEqual(len(model["anchors"]), 6)
        self.assertEqual(len({tuple(a["unit_vector"]) for a in model["anchors"]}), 6)
        self.assertEqual(120*3+6, 366)
        self.assertEqual(set(range(360)) | set(range(360,366)), set(range(366)))
        R,r=model["geometric_equations"]["R"],model["geometric_equations"]["r"]
        self.assertGreater(R,r)
        for k in range(3):
            for j in range(120):
                phi=2*math.pi*k/3
                theta=2*math.pi*j/120
                x=(R+r*math.cos(phi))*math.cos(theta)
                y=(R+r*math.cos(phi))*math.sin(theta)
                z=r*math.sin(phi)
                self.assertAlmostEqual((math.hypot(x,y)-R)**2+z*z,r*r)

    def test_h027_permutation_cube_bijection(self):
        import importlib.util
        spec = importlib.util.spec_from_file_location(
            "permutation_h027", ROOT / "tools/h027_permutation_cube.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        self.assertEqual(len(module.PERMUTATIONS), 120)
        self.assertEqual(len(module.ACTIVE), 120)
        self.assertEqual(len(module.RESERVED), 5)
        self.assertEqual({module.address(c) for c in module.RESERVED},
                         {0, 31, 62, 93, 124})
        for i, perm in enumerate(module.PERMUTATIONS):
            cell = module.encode_permutation(perm)
            self.assertEqual(module.decode_cell(cell), perm)
            self.assertNotIn(cell, module.RESERVED)
        for cell in module.RESERVED:
            self.assertIsNone(module.decode_cell(cell))
        doc = json.loads((ROOT / "models/h027-permutation-cube-map.json").read_text())
        self.assertEqual(doc["active_count"], 120)
        self.assertEqual(doc["reserved_count"], 5)
        self.assertEqual({x["id"] for x in doc["active_cells"]} |
                         {x["id"] for x in doc["reserved_cells"]},
                         set(range(125)))

    def test_h028_closed_orbit_full_cycle(self):
        spec = importlib.util.spec_from_file_location(
            "orbit_h028", ROOT / "tools/h028_closed_orbit.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        cube = json.loads((ROOT / "models/h027-permutation-cube-map.json").read_text())
        result = module.check(cube)
        self.assertEqual(result["collisions"], 0)
        self.assertEqual(result["payload_minimal_return_period"], 120)
        self.assertEqual(result["full_band_labeled_protocol_period"], 360)
        self.assertEqual(result["reversible_steps_checked"], 361)
        self.assertEqual(result["reserved_fixed"], [0,31,62,93,124])
        for step in range(361):
            self.assertEqual(len({module.transport(rank,step)
                                  for rank in range(120)}),120)
            self.assertEqual(module.transport(0,step),step%120)
        for step in (0,120,240,360):
            self.assertTrue(all(module.transport(i,step)==i for i in range(120)))
        for step in (1,119,121,359):
            self.assertFalse(any(module.transport(i,step)==i for i in range(120)))
        self.assertEqual(module.phase(120)["band"],"B")
        self.assertEqual(module.phase(240)["band"],"G")
        self.assertEqual(module.phase(360)["band"],"R")
        frozen = json.loads((ROOT / "models/h028-closed-orbit-results.json").read_text())
        self.assertEqual(frozen["payload_minimal_return_period"],120)
        self.assertEqual(frozen["collisions"],0)

    def test_h029_independent_multirate_first_joint_return(self):
        spec = importlib.util.spec_from_file_location(
            "multirate_h029", ROOT / "tools/h029_multirate_orbit.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        self.assertEqual(module.PERIODS, {"R":120, "B":240, "G":360})
        self.assertEqual(module.JOINT_PERIOD, 720)
        cube = json.loads((ROOT / "models/h027-permutation-cube-map.json").read_text())
        result = module.evaluate(cube)
        self.assertEqual(result["joint_minimal_period_ticks"], 720)
        self.assertEqual(result["ticks_exhaustively_checked"], 721)
        self.assertEqual(result["per_tick_active_mapping_collisions"], 0)
        self.assertEqual(result["unique_joint_returns_in_positive_interval"], [720])
        self.assertEqual(module.states(719), {"R":119,"B":119,"G":119})
        self.assertEqual(module.states(720), {"R":0,"B":0,"G":0})
        self.assertEqual(module.states(360), {"R":0,"B":60,"G":0})
        for tick in range(721):
            for band in ("R","B","G"):
                mapping = [module.move(rank, tick, band)
                           for rank in range(120)]
                self.assertEqual(set(mapping), set(range(120)))
                self.assertTrue(all(module.inverse(mapping[rank],tick,band)==rank
                                    for rank in range(120)))
        frozen=json.loads((ROOT / "models/h029-multirate-results.json").read_text())
        self.assertEqual(frozen["joint_minimal_period_ticks"],720)
        self.assertEqual(frozen["per_tick_active_mapping_collisions"],0)

    def test_h030_coupled_carry_period(self):
        spec = importlib.util.spec_from_file_location(
            "carry_h030", ROOT / "tools/h030_coupled_orbit.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        self.assertEqual(module.TOTAL, 1728000)
        for t in (0,1,119,120,14399,14400,1727999):
            self.assertEqual(module.encode(*module.decode(t)), t)
            self.assertEqual(module.backward(module.forward(module.decode(t))),
                             module.decode(t))
        self.assertEqual(module.forward((119,0,0)), (0,1,0))
        self.assertEqual(module.forward((119,119,0)), (0,0,1))
        self.assertEqual(module.forward((119,119,119)), (0,0,0))
        self.assertEqual(module.backward((0,0,0)), (119,119,119))
        frozen = json.loads((ROOT / "models/h030-carry-coupled-results.json").read_text())
        self.assertEqual(frozen["distinct_states_exhaustively_checked"], 1728000)
        self.assertEqual(frozen["duplicate_states_before_closure"], 0)
        self.assertEqual(frozen["fixed_white_cube_addresses"], [0,31,62,93,124])
        cube = json.loads((ROOT / "models/h027-permutation-cube-map.json").read_text())
        ids = module.active_ids(cube)
        self.assertEqual(len(ids),120)
        for band, values in module.transported_ids(cube, (0,1,119)).items():
            self.assertEqual(len(set(values)),120)
            self.assertEqual(set(values),set(ids))

    def test_h031_disturbance_persistence_and_repair(self):
        spec = importlib.util.spec_from_file_location(
            "h031", ROOT / "tools/h031_disturbance.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        base = module.decode(14399)
        self.assertEqual(base, (119,119,0))
        expected = {"R":-119, "B":-14280, "G":14400}
        for band, offset in expected.items():
            faulty = module.inject(base,band)
            self.assertEqual(module.signed_minimal_offset(faulty,base), offset)
            with self.assertRaises(PermissionError):
                module.repair(faulty,base)
            restored, correction = module.repair(faulty,base,verified=True)
            self.assertEqual(restored,base)
            for t in (0,1,120,240,1728000):
                healthy=module.advance(base,t)
                disturbed=module.advance(faulty,t)
                self.assertEqual(
                    (module.encode(disturbed)-module.encode(healthy))%module.TOTAL,
                    offset%module.TOTAL)
                self.assertNotEqual(healthy,disturbed)
                self.assertEqual(module.advance(restored,t),healthy)
        results=module.experiment()
        self.assertEqual(len(results["cases"]),3)
        self.assertTrue(all(not row["spontaneous_recovery"] for row in results["cases"]))
        self.assertEqual(results["white_reservations_fixed"],[0,31,62,93,124])
        frozen=json.loads((ROOT/"models/h031-disturbance-results.json").read_text())
        self.assertEqual([r["integer_address_offset_signed"] for r in frozen["cases"]],
                         [-119,-14280,14400])
        self.assertFalse(frozen["cern_measurement"])

    def test_h032_three_root_witness_gate(self):
        spec = importlib.util.spec_from_file_location(
            "h032", ROOT / "tools/h032_witness_repair.py")
        mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(mod)
        doc = mod.run()
        self.assertEqual(len(doc["cases"]), 4)
        self.assertEqual([x["decision"] for x in doc["cases"]],
                         ["CONSENSUS_IN_SIMULATOR", "INSUFFICIENT",
                          "CONTRADICTION", "UNVERIFIED_ROOTS"])
        self.assertEqual([x["repaired"] for x in doc["cases"]],
                         [True,False,False,False])
        for case in doc["cases"]:
            self.assertTrue(mod.verify_chain(case["events"]))
            self.assertEqual(case["white_reserved"], [0,31,62,93,124])
            self.assertEqual(case["events"][0]["kind"],"FAULT")
            self.assertEqual(case["events"][-1]["kind"],
                             "CORRECTION" if case["repaired"] else "REPAIR_WITHHELD")
        self.assertEqual(doc["cases"][0]["final_state"], [119,119,0])

    def test_h032_provenance_double_count_and_chain_tampering(self):
        spec = importlib.util.spec_from_file_location(
            "h032", ROOT / "tools/h032_witness_repair.py")
        mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(mod)
        ref=[119,119,0]
        witnesses=[mod.observation(14399,ref,"A",str(i),"shared",True)
                   for i in range(3)]
        result=mod.evaluate(witnesses)
        self.assertEqual(result["independent_roots"],1)
        self.assertEqual(result["decision"],"INSUFFICIENT")
        good=mod.trial("agreement")
        records=json.loads(json.dumps(good["events"]))
        self.assertTrue(mod.verify_chain(records))
        records[0]["payload"]["fault_band"]="G"
        self.assertFalse(mod.verify_chain(records))
        untrusted=mod.trial("unverified")
        self.assertFalse(untrusted["repaired"])

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
