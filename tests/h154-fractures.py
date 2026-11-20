"""H154: exact cut certificates and Walsh graph-spectrum verification."""
from itertools import combinations
from collections import Counter
from fractions import Fraction
from pathlib import Path
import json
ROOTS={"P_M":(1,),"P_I":(2,),"P_U":(4,),"P_B":(8,),"A":(3,12),"B":(5,10),"C":(9,6)}
NAMES=tuple(ROOTS)
W=Fraction(2,5)
def rank_gf2(gs):
    pivots={}
    for v in gs:
        while v:
            p=v.bit_length()-1
            if p in pivots:v^=pivots[p]
            else:pivots[p]=v;break
    return len(pivots)
def orthogonal(gs):
    return [m for m in range(1,16) if all((m&g).bit_count()%2==0 for g in gs)]
def spectrum(gs):
    return [2*W*sum((g&m).bit_count()%2 for g in gs) for m in range(16)]
def components(gs):
    unseen=set(range(16));sizes=[]
    while unseen:
        todo=[unseen.pop()];n=0
        while todo:
            x=todo.pop();n+=1
            for g in gs:
                y=x^g
                if y in unseen:unseen.remove(y);todo.append(y)
        sizes.append(n)
    return sorted(sizes)
families=Counter();cuts=[];census=[];checks=0
for k in range(8):
    cases=[]
    for failed in combinations(NAMES,k):
        gs=[g for root,gens in ROOTS.items() if root not in failed for g in gens]
        rk=rank_gf2(gs);witness=orthogonal(gs);eig=spectrum(gs);sizes=components(gs)
        assert len(witness)==2**(4-rk)-1
        assert sizes==[2**rk]*(2**(4-rk))
        assert len(sizes)==sum(v==0 for v in eig)
        for mask in range(16):
            for x in range(16):
                chi=lambda y:-1 if (mask&y).bit_count()%2 else 1
                assert sum(W*(chi(x)-chi(x^g)) for g in gs)==eig[mask]*chi(x)
                checks+=1
        if k==4 and rk<4:
            pc=sum(name.startswith("P_") for name in failed)
            family={4:"all_four_primitive",2:"two_primitive_two_compound",1:"one_primitive_all_three_compound"}[pc]
            families[family]+=1
            assert rk==3 and sizes==[8,8] and len(witness)==1
            cuts.append(dict(roots=list(failed),family=family,parity_mask=format(witness[0],"04b"),surviving_generators=gs,components=sizes))
        cases.append((rk,str(sorted(eig)[1])))
    census.append(dict(failed_count=k,cases=len(cases),connected=sum(r==4 for r,g in cases),disconnected=sum(r<4 for r,g in cases),min_gap=str(min(Fraction(g) for r,g in cases))))
assert checks==32768 and len(cuts)==11
assert families=={"all_four_primitive":1,"two_primitive_two_compound":6,"one_primitive_all_three_compound":4}
assert all(v["disconnected"]==0 for v in census[:4])
assert census[4]["connected"]==24 and census[4]["disconnected"]==11
out=dict(schema="cern.h154.cut-certificates.v1",roots={k:list(v) for k,v in ROOTS.items()},gate_weight="2/5",subsets=128,walsh_checks=checks,min_root_cut=4,families=dict(families),census=census,cutsets=cuts)
Path(__file__).with_name("results.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps({k:out[k] for k in ("subsets","walsh_checks","min_root_cut","families","census")},indent=2))
