from itertools import combinations
from fractions import Fraction as F
from collections import Counter
from pathlib import Path
import json

S=(1,2,4,8)
P=(3,5,9,6,10,12)
roots={"P_M":(1,),"P_I":(2,),"P_U":(4,),"P_B":(8,),"A":(3,12),"B":(5,10),"C":(9,6)}
labels=tuple(roots)
w=F(2,5)

def spectrum(active):
    return [2*w*sum((mask&g).bit_count()%2 for g in active) for mask in range(16)]

def components(active):
    remaining=set(range(16)); groups=[]
    while remaining:
        start=remaining.pop(); seen={start}; stack=[start]
        while stack:
            x=stack.pop()
            for g in active:
                y=x^g
                if y in remaining:
                    remaining.remove(y); seen.add(y); stack.append(y)
        groups.append(sorted(seen))
    return sorted(groups,key=lambda z:(-len(z),z[0]))

def evaluate(failed):
    active=tuple(g for name,gens in roots.items() if name not in failed for g in gens)
    eig=spectrum(active)
    comp=components(active)
    assert eig.count(0)==len(comp)
    return {'failed':list(failed),'remaining_generators':list(active),'gap':str(sorted(eig)[1]),'components':[len(c) for c in comp]}

hist={}; examples={}; cuts=[]; checks=0
for k in range(8):
    rows=[]
    for fail in combinations(labels,k):
        item=evaluate(fail);rows.append(item)
        active=item['remaining_generators']
        lam=spectrum(active)
        for mask in range(16):
            for v in range(16):
                chi=lambda x:1 if (x&mask).bit_count()%2==0 else -1
                assert sum(w*(chi(v)-chi(v^g)) for g in active)==lam[mask]*chi(v)
                checks+=1
        if len(item['components'])>1:
            cuts.append(item)
    minimum=min(F(r['gap']) for r in rows)
    hist[str(k)]={'cases':len(rows),'connected':sum(len(r['components'])==1 for r in rows),'disconnected':sum(len(r['components'])>1 for r in rows),'min_gap':str(minimum)}
    examples[str(k)]=min(rows,key=lambda x:(F(x['gap']),x['failed']))
assert sum(d['cases'] for d in hist.values())==128
assert all(hist[str(k)]['disconnected']==0 for k in range(4))
assert hist['4']['disconnected']>0
mincut=min(len(c['failed']) for c in cuts)
assert mincut==4
assert checks==32768
report={'roots':{k:list(v) for k,v in roots.items()},'weight':'2/5','subsets_checked':128,'walsh_eigenvector_checks':checks,'min_root_cut_size':mincut,'by_failed_root_count':hist,'first_minimum_cut':next(x for x in cuts if len(x['failed'])==4),'representative_cases':examples,'notes':['A failing root removes all gates with that ancestry.','Root independence is an assumed architecture, not independently observed hardware.','Spectral gap is second smallest eigenvalue including multiplicity, zero if disconnected.','The kernel is a Cayley graph of (Z/2Z)^4, not physical QCD.']}
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:report[k] for k in ['subsets_checked','walsh_eigenvector_checks','min_root_cut_size','by_failed_root_count','first_minimum_cut']},indent=2))
