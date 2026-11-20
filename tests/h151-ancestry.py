"""H151: exact Walsh spectrum under certified source-root failures."""
from itertools import combinations, product
from fractions import Fraction
from pathlib import Path
import json
S=(1,2,4,8)
P=tuple(a|b for a,b in combinations(S,2))
W=Fraction(2,5)
def spectrum(active):
    return tuple(2*W*sum((g&mode).bit_count()%2 for g in active) for mode in range(16))
def gap(active):return min(spectrum(active)[1:])
def failures(mapping):
    return [gap([g for g in S+P if g!=f]) for f in S]+[
        gap(list(S)+[g for g,root in zip(P,mapping) if root!=f]) for f in range(3)]
all_cases=[(min(failures(a)),a,failures(a)) for a in product(range(3),repeat=6)]
best_value=max(x[0] for x in all_cases)
best=[x for x in all_cases if x[0]==best_value]
assert len(all_cases)==729 and len(best)==6 and best_value==Fraction(12,5)
assert best[0][1]==(0,1,2,2,1,0)
for _,a,gaps in best:
    assert a[0]==a[5] and a[1]==a[4] and a[2]==a[3]
    assert len({a[0],a[1],a[2]})==3 and gaps==[best_value]*7
assert min(failures((0,)*6))==Fraction(4,5)
checks=0
for _,a,_ in best:
    for fail in S:
        active=[g for g in S+P if g!=fail]
        for mask in range(16):
            lam=spectrum(active)[mask]
            for x in range(16):
                chi=lambda v:-1 if (v&mask).bit_count()%2 else 1
                assert sum(W*(chi(x)-chi(x^g)) for g in active)==lam*chi(x)
                checks+=1
    for fail in range(3):
        active=list(S)+[g for g,r in zip(P,a) if r!=fail]
        for mask in range(16):
            lam=spectrum(active)[mask]
            for x in range(16):
                chi=lambda v:-1 if (v&mask).bit_count()%2 else 1
                assert sum(W*(chi(x)-chi(x^g)) for g in active)==lam*chi(x)
                checks+=1
assert checks==10752
# Losing any primitive gate gives an unavoidable gap of 12/5.
assert all(v<=Fraction(12,5) for v,_,_ in all_cases)
report=dict(candidate_mappings=729,optimal_mappings=6,worst_case_gap="12/5",common_source_gap="4/5",
 certified_mapping={"MI":"A","MU":"B","MB":"C","IU":"C","IB":"B","UB":"A"},
 root_failure_gaps={k:"12/5" for k in ["P_M","P_I","P_U","P_B","A","B","C"]},
 walsh_eigenvector_checks=checks,is_independence_physically_verified=False)
Path(__file__).with_name("results.json").write_text(json.dumps(report,indent=2)+"\n")
print(json.dumps(report,indent=2))
