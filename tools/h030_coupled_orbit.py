#!/usr/bin/env python3
"""H030 — reversible carry-coupled triple toroid, base-120 odometer.

R steps every tick; B steps at R wrap; G steps at B wrap.
The joint state is a bijection with all 120^3 states. The 5 H027 white-axis
cube reservations stay fixed; no CERN field data or Higgs physics is implied.
"""
import argparse
import hashlib
import json
from pathlib import Path

BASE = 120
CUBE_SIZE = 125
TOTAL = BASE**3
WHITE = (0,31,62,93,124)
BANDS = ("R","B","G")


def encode(r, b, g):
    for v in (r,b,g):
        if type(v) is not int or not 0 <= v < BASE:
            raise ValueError("R, B, G must be integers 0..119")
    return r + BASE*b + BASE*BASE*g


def decode(n):
    if type(n) is not int or not 0 <= n < TOTAL:
        raise ValueError("address must be in [0, 120^3)")
    return (n % BASE, (n // BASE) % BASE, n // (BASE*BASE))


def forward(state):
    r,b,g = state
    code = encode(r,b,g)
    return decode((code + 1) % TOTAL)


def backward(state):
    r,b,g = state
    code = encode(r,b,g)
    return decode((code - 1) % TOTAL)


def active_ids(cube):
    active = sorted(cube["active_cells"], key=lambda x:x["rank"])
    ids = tuple(a["id"] for a in active)
    assert len(set(ids)) == BASE
    assert set(ids) | set(WHITE) == set(range(CUBE_SIZE))
    assert tuple(a["id"] for a in cube["reserved_cells"]) == WHITE
    return ids


def transported_ids(cube, state):
    ids = active_ids(cube)
    return {
        band: [ids[(j+phase)%BASE] for j in range(BASE)]
        for band,phase in zip(BANDS,state)
    }


def analyze(cube):
    ids = active_ids(cube)
    # Exhaustive verification of all 1,728,000 states, not a sampled orbit.
    visited = bytearray(TOTAL)
    state=(0,0,0)
    visits=0
    for tick in range(TOTAL):
        address=encode(*state)
        assert not visited[address], f"Repeated state before closure at {tick}"
        visited[address]=1
        assert decode(address)==state
        next_state=forward(state)
        assert backward(next_state)==state
        # Cyclic translations are bijections, so all 120 occupants remain
        # unique for each band at every state; the proof uses modular arithmetic
        # rather than materializing 360 items at every one of 1.728m ticks.
        assert all(0<=phase<BASE for phase in state)
        visits+=1
        state=next_state
    assert state==(0,0,0)
    assert visits==TOTAL and all(visited)
    checkpoints=(0,1,119,120,121,14399,14400,14401,720,1727999,1728000)
    return {
        "schema":"cern.h030.carry-coupled-orbit.v1",
        "status":"exact, synthetic, reversible finite-state mapping",
        "base":BASE,
        "bands":list(BANDS),
        "rule":"R increments every tick; B on R wrap; G on simultaneous R+B wrap",
        "encoding":"t mod 120^3 = R + 120*B + 14400*G",
        "joint_minimal_period":TOTAL,
        "three_band_active_positions_per_state":360,
        "distinct_permutations_per_cube_projection":120,
        "physical_particle_count_claim":None,
        "fixed_white_cube_addresses":list(WHITE),
        "distinct_states_exhaustively_checked":visits,
        "duplicate_states_before_closure":0,
        "invertible":True,
        "all_active_mappings_bijective":True,
        "full_joint_return_exactly_at":TOTAL,
        "source_geometry":"models/h027-permutation-cube-map.json",
        "relation_to_H029":"H029 independent shift periods yield 720 ticks; H030 nested carry yields 1,728,000 ticks",
        "checkpoints":[{"tick":t,"RGB":list(decode(t%TOTAL))}
                       for t in checkpoints],
        "limits":["This is a base-120 odometer defined by us, not an empirical property of CERN magnets.","White cells remain fixed in the H027 cube and are symbolic, not Higgs emissions.","The ring transport is a cyclic rank translation and does not model particle forces or QCD."],
    }


def main():
    p=argparse.ArgumentParser()
    p.add_argument("--cube",default="models/h027-permutation-cube-map.json")
    p.add_argument("--output",default="models/h030-carry-coupled-results.json")
    args=p.parse_args()
    cube_path=Path(args.cube)
    cube=json.loads(cube_path.read_text(encoding="utf-8"))
    result=analyze(cube)
    result["cube_sha256"]=hashlib.sha256(cube_path.read_bytes()).hexdigest()
    output=Path(args.output)
    output.parent.mkdir(parents=True,exist_ok=True)
    output.write_text(json.dumps(result,indent=2)+"\n",encoding="utf-8")
    print("PASS: 1,728,000 unique states, 0 early collisions, reversible; 5 white fixed")


if __name__=="__main__":
    main()
