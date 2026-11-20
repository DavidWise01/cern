from fractions import Fraction as F
from itertools import combinations
import json
S=[1,2,4,8];P=[i|j for i,j in combinations(S,2)]
def eigenvalues(f,r,a,b):
 w={g:(F(0) if g==f else a) for g in S}
 w.update({g:(r*b if g&f else b) for g in P})
 return [2*sum(v for g,v in w.items() if (g&m).bit_count()%2) for m in range(16)]
rows=[];checks=0;comparisons=0
for k in range(17):
 r=F(k,16);a=4*r/(6+4*r);b=F(4)/(6+4*r);target=24*r/(6+4*r)
 for f in S:
  eig=eigenvalues(f,r,a,b);assert min(eig[1:])==target
  assert eig.count(0)>=2 if r==0 else eig.count(0)==1
  w={g:(F(0) if g==f else a) for g in S}
  w.update({g:(r*b if g&f else b) for g in P})
  for m in range(16):
   for x in range(16):
    chi=lambda v:(-1)**((v&m).bit_count())
    assert sum(z*(chi(x)-chi(x^g)) for g,z in w.items())==eig[m]*chi(x);checks+=1
 for j in range(401):
  aa=F(j,100);bb=(4-4*aa)/6
  assert min(min(eigenvalues(f,r,aa,bb)[1:]) for f in S)<=target
  comparisons+=1
 rows.append(dict(r=str(r),primitive=str(a),compound=str(b),gap=str(target)))
assert min(eigenvalues(1,F(1,2),F(1,4),F(1,2))[1:])==F(3,2)
assert min(eigenvalues(1,F(1,2),F(2,5),F(2,5))[1:])==F(6,5)
assert min(eigenvalues(1,F(1),F(0),F(2,3))[1:])==0
print(json.dumps(dict(failure_domains=4,independence_fractions=17,walsh_checks=checks,brute_budget_allocations=comparisons,zero_violations=True,example=rows[8],counterexample='all compound / no primitive parity gap=0'),indent=2))
