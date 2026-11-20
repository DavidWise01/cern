from fractions import Fraction as F
from itertools import product
import json
from pathlib import Path
V=range(16); checks=0; fixtures=0; best=F(0); opt=[]
for a in product(range(17),repeat=4):
 if sum(a)!=16: continue
 w=tuple(F(x,4) for x in a); fixtures+=1
 gap=2*min(w)
 for m in V:
  lam=2*sum(w[j] for j in range(4) if m>>j&1)
  for v in V:
   f=lambda x: (-1)**((x&m).bit_count())
   assert sum(w[j]*(f(v)-f(v^(1<<j))) for j in range(4))==lam*f(v)
   checks+=1
 if gap>best:best=gap;opt=[]
 if gap==best:opt.append(a)
assert best==2 and opt==[(4,4,4,4)]
fault=(F(0),F(4,3),F(4,3),F(4,3)); assert sum(fault)==4
assert sum(1 for m in V if sum(fault[j] for j in range(4) if m>>j&1)==0)==2
out=dict(allocations=fixtures,walsh_checks=checks,budget=4,optimal_gap=str(best),optimal_weights=[1]*4,fault_nullity=2,violations=0)
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n');print(out)
