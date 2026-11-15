#!/usr/bin/env python3
"""H029: independent R/B/G rates on 120-position toroidal permutation rings.

At time t, R advances floor(t/1), B floor(t/2), G floor(t/3).
Each channel has its own cyclic projection of the SAME 120 H027 permutations;
there are not 360 distinct permutations. Five H027 white-axis cells remain
fixed on each cube projection. Six H026 Cartesian axis anchors are a separate
geometric construct and are not included in the 125-cell cube count.
"""
import argparse
import hashlib
import json
import math
from pathlib import Path

RING = 120
RATES = {"R": 1, "B": 2, "G": 3}
WHITE = (0, 31, 62, 93, 124)
PERIODS = {name: RING * ticks for name, ticks in RATES.items()}
JOINT_PERIOD = math.lcm(*PERIODS.values())


def phase(tick, band):
    if not isinstance(tick, int) or isinstance(tick, bool) or tick < 0:
        raise ValueError("tick must be nonnegative integer")
    if band not in RATES:
        raise ValueError("band must be R, B, or G")
    return (tick // RATES[band]) % RING


def move(rank, tick, band):
    if not isinstance(rank, int) or isinstance(rank, bool) or not 0 <= rank < RING:
        raise ValueError("rank must be 0..119")
    return (rank + phase(tick, band)) % RING


def inverse(position, tick, band):
    if not isinstance(position, int) or isinstance(position, bool) or not 0 <= position < RING:
        raise ValueError("position must be 0..119")
    return (position - phase(tick, band)) % RING


def states(tick):
    return {band: phase(tick, band) for band in RATES}


def validate_cube(cube):
    active = cube["active_cells"]
    assert len(active) == RING
    assert [x["rank"] for x in active] == list(range(RING))
    assert len({x["permutation"] for x in active}) == RING
    assert len({x["id"] for x in active}) == RING
    assert tuple(x["id"] for x in cube["reserved_cells"]) == WHITE
    assert set(x["id"] for x in active) | set(WHITE) == set(range(125))


def evaluate(cube):
    validate_cube(cube)
    for tick in range(JOINT_PERIOD + 1):
        for band in RATES:
            positions = [move(rank, tick, band) for rank in range(RING)]
            assert len(set(positions)) == RING
            assert all(inverse(positions[rank], tick, band) == rank
                       for rank in range(RING))
            assert set(positions) == set(range(RING))
    returns = [t for t in range(1, JOINT_PERIOD + 1)
               if all(offset == 0 for offset in states(t).values())]
    assert returns == [JOINT_PERIOD], returns
    milestones = [0, 1, 2, 3, 5, 6, 60, 120, 240, 360, 480, 600, 719, 720]
    return {
        "schema": "cern.h029.multirate-orbit.v1",
        "model": "simultaneous three projections of one 120-permutation payload",
        "ticks_per_position_shift": RATES,
        "individual_minimal_periods": PERIODS,
        "joint_minimal_period_ticks": JOINT_PERIOD,
        "ring_positions_per_band": RING,
        "white_cube_ids_fixed": list(WHITE),
        "h026_six_axes_separate": True,
        "unique_joint_returns_in_positive_interval": returns,
        "ticks_exhaustively_checked": JOINT_PERIOD + 1,
        "per_tick_active_mapping_collisions": 0,
        "reversibility_check": "all 120 ranks, all 3 bands, all 721 ticks",
        "synthetic": True,
        "claim_boundary": (
            "Rates, units and phases are mathematical choices. "
            "720 is not a CERN field, machine, beam or Higgs recurrence."
        ),
        "milestones": [
            {"tick": t, "positions": states(t),
             "joint_return": all(p == 0 for p in states(t).values())}
            for t in milestones
        ],
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--cube", default="models/h027-permutation-cube-map.json")
    parser.add_argument("--output", default="models/h029-multirate-results.json")
    args = parser.parse_args()
    path = Path(args.cube)
    cube = json.loads(path.read_text(encoding="utf-8"))
    result = evaluate(cube)
    result["source_sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
    out = Path(args.output)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(f"PASS: unique joint recurrence {JOINT_PERIOD} ticks, "
          f"721 checkpoints, 0 collisions, five fixed white markers")


if __name__ == "__main__":
    main()
