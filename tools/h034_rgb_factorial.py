#!/usr/bin/env python3
"""H034: lazily index 3 spectral copies of RGB 8! × 6 × 8 + one singlet.

Eight distinct symbols per primary bank are permuted (8! = 40,320).
UV and IR reuse this indexing scheme symbolically, not as optical RGB spectra.
"""
import itertools
import json
import math
from pathlib import Path

SPECTRA=("visible","ultraviolet","infrared")
PRIMARY=("R","G","B")
SECONDARY=6
TETRARY=8
PERM=math.factorial(8)
ACTIVE_PER=3*PERM*SECONDARY*TETRARY
ACTIVE_TOTAL=len(SPECTRA)*ACTIVE_PER
TOTAL=ACTIVE_TOTAL+len(SPECTRA)


def rank_to_perm(rank):
    if type(rank) is not int or not 0<=rank<PERM:raise ValueError("rank 0..40319")
    remaining=list(range(8));result=[]
    for n in range(7,-1,-1):
        f=math.factorial(n);i,rank=divmod(rank,f)
        result.append(remaining.pop(i))
    return tuple(result)


def perm_to_rank(p):
    p=tuple(p)
    if sorted(p)!=list(range(8)):raise ValueError("must be a permutation of 0..7")
    remaining=list(range(8));rank=0
    for i,v in enumerate(p):
        j=remaining.index(v)
        rank+=j*math.factorial(7-i)
        remaining.pop(j)
    return rank


def encode(spectrum,primary,perm_rank,secondary,tetrary):
    if spectrum not in SPECTRA or primary not in PRIMARY:
        raise ValueError("unknown spectral or primary channel")
    if type(perm_rank) is not int or not 0<=perm_rank<PERM:
        raise ValueError("invalid rank")
    if type(secondary) is not int or not 0<=secondary<SECONDARY:
        raise ValueError("invalid secondary address")
    if type(tetrary) is not int or not 0<=tetrary<TETRARY:
        raise ValueError("invalid tetrary address")
    return (((((SPECTRA.index(spectrum)*3 + PRIMARY.index(primary))
            *PERM+perm_rank)*SECONDARY+secondary)*TETRARY)+tetrary)


def decode(address):
    if type(address) is not int or not 0<=address<TOTAL:
        raise ValueError("invalid total address")
    if address>=ACTIVE_TOTAL:
        return {"kind":"SINGLET","spectrum":SPECTRA[address-ACTIVE_TOTAL]}
    address,tetrary=divmod(address,TETRARY)
    address,secondary=divmod(address,SECONDARY)
    address,perm_rank=divmod(address,PERM)
    spectrum,primary=divmod(address,3)
    return {"kind":"ACTIVE","spectrum":SPECTRA[spectrum],
            "primary":PRIMARY[primary],"perm_rank":perm_rank,
            "permutation":rank_to_perm(perm_rank),
            "secondary":secondary,"tetrary":tetrary}


def verify():
    assert PERM==40320
    assert ACTIVE_PER==5806080
    assert TOTAL==17418243
    sample_ranks=[0,1,2,40,5039,5040,20160,40318,40319]
    for rank in sample_ranks:
        assert perm_to_rank(rank_to_perm(rank))==rank
    for spectrum in SPECTRA:
        for primary in PRIMARY:
            for rank in sample_ranks:
                for sec in range(SECONDARY):
                    for tet in range(TETRARY):
                        address=encode(spectrum,primary,rank,sec,tet)
                        d=decode(address)
                        assert (d["spectrum"],d["primary"],d["perm_rank"],d["secondary"],d["tetrary"])==(spectrum,primary,rank,sec,tet)
    assert [decode(ACTIVE_TOTAL+i) for i in range(3)]==[
        {"kind":"SINGLET","spectrum":s} for s in SPECTRA]
    # Boundary injectivity and global cardinality proven by mixed-radix coding.
    assert len({encode(s,p,r,0,0) for s in SPECTRA for p in PRIMARY for r in range(PERM)})==3*3*PERM
    return {"permutation_count_each":PERM,"active_per_spectrum":ACTIVE_PER,
            "singlets_per_spectrum":1,"total_per_spectrum":ACTIVE_PER+1,
            "all_spectra_total":TOTAL,"checked":"boundary/sample roundtrips and unique perm-bank seed addresses",
            "cardinality_proof":"mixed-radix decomposition 3 × 3 × 40320 × 6 × 8 plus three special singlet IDs",
            "interpretation":"mathematical indexing only; not physically observed UV/IR RGB primaries"}


if __name__=="__main__":
    print(json.dumps(verify(),indent=2))
