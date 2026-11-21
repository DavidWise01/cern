"""H155: exact one-gate parity repair of all eleven H154 root cuts."""
from itertools import combinations
from fractions import Fraction as F
from collections import Counter
from pathlib import Path
import json
ROOTS={'P_M':(1,),'P_I':(2,),'P_U':(4,),'P_B':(8,),'A':(3,12),'B':(5,10),'C':(9,6)}
W=F(2,5)
def rank(gs):
 p={}
 for x in gs:
  while x:
   i=x.bit_length()-1
   if i in p:x^=p[i]
   else:p[i]=x;break
 return len(p)
def gap(gs):
 z=[2*W*sum((m&g).bit_count()%2 for g in gs) for m in range(16)]
 return sorted(z)[1]
rows=[];checks=0
for failed in combinations(ROOTS,4):
 gs=[g for r,vs in ROOTS.items() if r not in failed for g in vs]
 if rank(gs)!=3:continue
 masks=[m for m in range(1,16) if all((m&g).bit_count()%2==0 for g in gs)]
 assert len(masks)==1
 m=masks[0];good=[]
 for g in range(1,16):
  valid=((m&g).bit_count()%2==1)
  assert (rank(gs+[g])==4)==valid
  assert (gap(gs+[g])>0)==valid
  if valid:
   good.append(g)
   for k in range(16):
    lam=2*W*sum((k&q).bit_count()%2 for q in gs+[g])
    for v in range(16):
     chi=lambda x:1 if (x&k).bit_count()%2==0 else -1
     assert sum(W*(chi(v)-chi(v^q)) for q in gs+[g])==lam*chi(v)
     checks+=1
 assert len(good)==8 and all(gap(gs+[g])==F(4,5) for g in good)
 rows.append(dict(failed=failed,mask=format(m,'04b'),eligible=good,selected=min(good),recovered_gap=str(gap(gs+[min(good)]))))
assert len(rows)==11 and checks==22528
out=dict(fractures=11,eligible_per_fracture=8,eligible_trials=88,walsh_checks=checks,recovered_gap='4/5',certificates=rows)
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:v for k,v in out.items() if k!='certificates'},indent=2))
