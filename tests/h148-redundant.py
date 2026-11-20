from itertools import combinations
from fractions import Fraction as F
import json
from pathlib import Path
V=range(16); singles=[1<<i for i in range(4)]; pairs=[a|b for a,b in combinations(singles,2)]
def spectrum(w):
 return [2*sum(a for g,a in w.items() if (g&m).bit_count()%2) for m in V]
def components(w):
 unseen=set(V); sizes=[]
 while unseen:
  q=[unseen.pop()]; n=0
  while q:
   x=q.pop();n+=1
   for g,a in w.items():
    if a and x^g in unseen:unseen.remove(x^g);q.append(x^g)
  sizes.append(n)
 return sorted(sizes)
checks=0; rows=[]
for f in singles:
 base={g:F(1) for g in singles if g!=f}
 assert components(base)==[8,8]
 for g in pairs:
  w=base|{g:F(1)}; eig=spectrum(w)
  for m in V:
   for x in V:
    chi=lambda y:(-1)**((y&m).bit_count())
    assert sum(a*(chi(x)-chi(x^k)) for k,a in w.items())==eig[m]*chi(x);checks+=1
  restored=bool(g&f);assert components(w)==([16] if restored else [8,8]);assert min(eig[1:])==(2 if restored else 0)
  rows.append({'failed':f,'compound':g,'restored':restored,'gap':int(min(eig[1:]))})
frontier=[]
for k in range(25):
 a=F(k,24);b=(4-4*a)/6
 gaps=[]
 for f in singles:
  w={g:a for g in singles if g!=f}|{g:b for g in pairs}
  gap=min(spectrum(w)[1:]);assert gap==2*min(3*b,a+3*b,a+4*b,2*a+4*b,2*a+3*b,3*a+3*b,3*a)
  gaps.append(gap)
 frontier.append((min(gaps),a,b))
best,a,b=max(frontier);assert best<=F(12,5)
assert 4*F(2,5)+6*F(2,5)==4
for f in singles:
 w={g:F(2,5) for g in singles if g!=f}|{g:F(2,5) for g in pairs}
 assert min(spectrum(w)[1:])==F(12,5)
out={'single_failures':4,'compound_trials':len(rows),'restored':sum(r['restored'] for r in rows),'walsh_checks':checks,'rows':rows,'robust_weight':'2/5','robust_gap':'12/5','healthy_gap':str(min(spectrum({g:F(2,5) for g in singles+pairs})[1:]))}
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k!='rows'},indent=2))
