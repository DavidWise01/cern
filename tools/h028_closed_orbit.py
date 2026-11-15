#!/usr/bin/env python3
"""H028: three-stage toroidal transport of the H027 120-permutation cube.

One stage has 120 one-slot permutation translations. Three stages (R/B/G)
give a 360-step *labeled protocol* cycle. The cube payload actually returns
after EVERY 120 steps: 360 is not the minimal physical permutation period.
Five diagonal reserved cells remain fixed at all steps.
"""
import argparse
import hashlib
import json
from pathlib import Path

WHITE = (0, 31, 62, 93, 124)
BANDS = ("R", "B", "G")
STEPS_PER_BAND = 120
TOTAL_STEPS = len(BANDS) * STEPS_PER_BAND


def permutation_rank_map(cube):
    active = cube["active_cells"]
    assert len(active) == 120
    by_rank = [None] * 120
    for entry in active:
        rank = entry["rank"]
        assert isinstance(rank, int) and 0 <= rank < 120
        assert by_rank[rank] is None
        by_rank[rank] = entry["id"]
    assert len(set(by_rank)) == 120
    assert set(by_rank).isdisjoint(WHITE)
    assert set(by_rank) | set(WHITE) == set(range(125))
    return tuple(by_rank)


def phase(step):
    if not isinstance(step, int) or isinstance(step, bool) or step < 0:
        raise ValueError("step must be a nonnegative integer")
    stage = (step // STEPS_PER_BAND) % 3
    return {"step": step, "cycle": step // TOTAL_STEPS,
            "band": BANDS[stage], "band_index": stage,
            "step_in_band": step % STEPS_PER_BAND}


def transport(original_rank, step):
    """Position rank of a payload after 'step' cyclic rank translations."""
    if not isinstance(original_rank, int) or not 0 <= original_rank < 120:
        raise ValueError("rank must be 0..119")
    return (original_rank + step) % 120


def snapshot(cube, step):
    ids = permutation_rank_map(cube)
    p = phase(step)
    states = [
        {"permutation": cube["active_cells"][rank]["permutation"],
         "origin_cube_id": ids[rank],
         "current_cube_id": ids[transport(rank, step)],
         "source_rank": rank, "destination_rank": transport(rank, step)}
        for rank in range(120)
    ]
    return {
        **p,
        "moved_count": sum(x["origin_cube_id"] != x["current_cube_id"] for x in states),
        "reserved_ids": list(WHITE),
        "reserved_moved": 0,
        "positions": states,
    }


def check(cube):
    ids = permutation_rank_map(cube)
    assert tuple(x["id"] for x in cube["reserved_cells"]) == WHITE
    assert len({x["permutation"] for x in cube["active_cells"]}) == 120
    outcomes = []
    for step in range(TOTAL_STEPS + 1):
        positions = [transport(i, step) for i in range(120)]
        assert len(set(positions)) == 120
        assert set(ids[i] for i in positions) | set(WHITE) == set(range(125))
        for i in range(120):
            reverse = (positions[i] - step) % 120
            assert reverse == i
        outcomes.append({"step": step, "phase": phase(step)["band"],
                         "returned_positions": sum(transport(i, step) == i for i in range(120)),
                         "white_positions_fixed": 5})
    assert all(n["returned_positions"] == 120 for n in outcomes if n["step"] % 120 == 0)
    assert all(n["returned_positions"] == 0 for n in outcomes if n["step"] % 120 != 0)
    return {
        "schema": "cern.h028.closed-orbit-test.v1",
        "source": "models/h027-permutation-cube-map.json",
        "model": "3 labeled consecutive 120-translation stages",
        "steps": TOTAL_STEPS, "active_permutations": 120, "reserved_fixed": list(WHITE),
        "distinct_active_ids": len(ids), "collisions": 0, "lost_permutations": 0,
        "reversible_steps_checked": TOTAL_STEPS + 1,
        "payload_minimal_return_period": 120,
        "full_band_labeled_protocol_period": 360,
        "roundtrip_at_120": True,
        "roundtrip_at_240": True,
        "roundtrip_at_360": True,
        "meaning": "360 is a chosen three-band protocol length. The payload itself returns each 120 steps.",
        "synthetic_geometry_only": True,
        "milestones": [outcomes[s] for s in (0,1,5,24,60,119,120,121,239,240,241,359,360)]
    }


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--cube", default="models/h027-permutation-cube-map.json")
    p.add_argument("--output", default="models/h028-closed-orbit-results.json")
    args = p.parse_args()
    source = Path(args.cube)
    cube = json.loads(source.read_text(encoding="utf-8"))
    doc = check(cube)
    doc["cube_sha256"] = hashlib.sha256(source.read_bytes()).hexdigest()
    dest = Path(args.output)
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
    print("H028 PASS: 120 unique mappings, 5 white fixed, 0 collisions.")
    print("Payload period 120; labeled RGB orbit protocol period 360.")


if __name__ == "__main__":
    main()
