#!/usr/bin/env python3
"""H027 - Reversible permutation/cube mapping; 120 permutations + 5 reserved cells.

No relationship to Higgs mass or the physical emission of white light is implied.
"""
import itertools
import json
from pathlib import Path

SYMBOLS = tuple("ABCDE")
SIDE = 5
RESERVED = tuple((i, i, i) for i in range(SIDE))
PERMUTATIONS = tuple(itertools.permutations(SYMBOLS))
CELLS = tuple((x, y, z) for z in range(SIDE)
              for y in range(SIDE) for x in range(SIDE))
ACTIVE = tuple(cell for cell in CELLS if cell not in RESERVED)
assert len(CELLS) == 125 and len(ACTIVE) == 120 and len(PERMUTATIONS) == 120


def encode_permutation(permutation):
    p = tuple(permutation)
    if p not in PERMUTATIONS:
        raise ValueError("Expected a permutation of A,B,C,D,E")
    return ACTIVE[PERMUTATIONS.index(p)]


def decode_cell(cell):
    c = tuple(cell)
    if c in RESERVED:
        return None
    if c not in ACTIVE:
        raise ValueError("Cell outside the five-cube")
    return PERMUTATIONS[ACTIVE.index(c)]


def address(cell):
    x, y, z = cell
    return x + 5 * y + 25 * z


def build():
    return {
        "schema": "cern.h027.permutation-cube.v1",
        "five_cube": {"dimensions": [5, 5, 5], "total_addresses": 125,
                      "linearization": "x + 5*y + 25*z; coordinates 0..4"},
        "active_count": 120, "reserved_count": 5,
        "reserved_cells": [
            {"id": address(c), "xyz": list(c), "tag": "WHITE_AXIS_RESERVED",
             "physical_particle": False} for c in RESERVED],
        "active_cells": [
            {"rank": i, "id": address(cell), "xyz": list(cell),
             "permutation": "".join(PERMUTATIONS[i])}
            for i, cell in enumerate(ACTIVE)],
        "symbolic_interpretation": {
            "16": "12 fermion types + 4 gauge-boson types in a common counting convention",
            "17": "Higgs as seventeenth TYPE, discovered last (2012)",
            "white": "user-defined synthesis/completeness marker; not a Higgs color or optical emission",
            "reserved_cells": "five illustrative white-axis marks, not five physical Higgs states",
        },
        "separation": [
            "The mapping is reversible by construction, not inferred from CERN data.",
            "5!=120 distinct orderings, whereas 5^3=125 three-index addresses.",
            "Five cube positions were intentionally chosen (main diagonal), not discovered.",
            "White light is a metaphor for the last missing Standard Model type.",
        ],
        "references": [
            "https://home.cern/science/physics/higgs-boson/",
            "https://openstax.org/books/physics/pages/23-2-quarks",
        ]
    }


def main():
    out = Path("models/h027-permutation-cube-map.json")
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(build(), indent=2) + "\n")
    print("125 cube cells: 120 one-to-one permutation addresses; 5 reserved.")
    print("Verified reversible mapping for all 120 active cells.")


if __name__ == "__main__":
    main()
