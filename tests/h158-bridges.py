from itertools import combinations
from fractions import Fraction as F
import json
from pathlib import Path
ROOTS={'P_M':(1,),'P_I':(2,),'P_U':(4,),'P_B':(8,),'A':(3,12),'B':(5,10),'C':(9,6)}
W=F(2,5);TARGET=F(4,5)
def rank(gs):
    p={}
    for x in gs:
        while x:
            k=x.bit_length()-1
            if k in p:x^=p[k]
            else:p[k]=x;break
    return len(p)
def eigen(gs,bridges=()):
    return [2*W*sum((m&g).bit_count()%2 for g in gs)+2*sum(w for g,w in bridges if (m&g).bit_count()%2) for m in range(16)]
def price(g):return F(3+g.bit_count(),4)
rows=[];trials=checks=scenarios=0
for failed in combinations(ROOTS,4):
    gs=[g for root,group in ROOTS.items() if root not in failed for g in group]
    if rank(gs)!=3:continue
    m=next(m for m in range(1,16) if all((m&g).bit_count()%2==0 for g in gs))
    valid=[g for g in range(1,16) if (m&g).bit_count()%2]
    assert len(valid)==8
    options=[]
    for chosen in combinations(valid,3):
        trials+=1
        for surviving in (chosen, *((g,) for g in chosen)):
            bridges=tuple((g,W) for g in surviving)
            vals=eigen(gs,bridges)
            assert sorted(vals)[1]>=TARGET
            assert vals.count(0)==1
            for mode in range(16):
                chi=lambda x:1 if (mode&x).bit_count()%2==0 else -1
                for x in range(16):
                    lhs=sum(W*(chi(x)-chi(x^g)) for g in gs)+sum(w*(chi(x)-chi(x^g)) for g,w in bridges)
                    assert lhs==vals[mode]*chi(x)
                    checks+=1
            scenarios+=1
        options.append((W*sum(price(g) for g in chosen),chosen))
    cost,chosen=min(options)
    rows.append(dict(failed_roots=list(failed),mask=format(m,'04b'),selected_gates=list(chosen),price=str(cost)))
assert len(rows)==11 and trials==616 and scenarios==2464 and checks==630784
assert sorted([r['price'] for r in rows]).count('6/5')==1
assert sorted([r['price'] for r in rows]).count('13/10')==6
assert sorted([r['price'] for r in rows]).count('7/5')==4
report=dict(fractures=11,triples_tested=trials,root_failure_scenarios=scenarios,walsh_checks=checks,rows=rows)
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:v for k,v in report.items() if k!='rows'},indent=2))
