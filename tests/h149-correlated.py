from fractions import Fraction as F
from itertools import combinations
from pathlib import Path
import json
S=[1,2,4,8]; P=[a|b for a,b in combinations(S,2)]
def weights(f,r):
 return {g:(F(0) if g==f else F(2,5)*r if g&f else F(2,5)) for g in S+P}
def spectrum(w):return [2*sum(a for g,a in w.items() if (g&m).bit_count()%2) for m in range(16)]
checks=0; rows=[]
for k in range(17):
 r=F(k,16); gaps=[]
 for f in S:
  w=weights(f,r); eig=spectrum(w); gap=min(eig[1:]); assert gap==F(12,5)*r; gaps.append(gap)
  for m in range(16):
   for x in range(16):
    chi=lambda y:(-1)**((y&m).bit_count())
    assert sum(a*(chi(x)-chi(x^g)) for g,a in w.items())==eig[m]*chi(x);checks+=1
  assert sum(v==0 for v in eig)==(2 if r==0 else 1)
 rows.append({'independent_fraction':str(r),'gap':str(min(gaps))})
out=dict(failure_domains=4,fractions=17,spectral_checks=checks,rows=rows,violations=0)
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2));print(json.dumps(out))
