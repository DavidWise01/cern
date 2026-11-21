from itertools import combinations
from fractions import Fraction as F
from pathlib import Path
import json
R={'P_M':(1,),'P_I':(2,),'P_U':(4,),'P_B':(8,),'A':(3,12),'B':(5,10),'C':(9,6)}
W=F(2,5)
def rank(gs):
 p={}
 for a in gs:
  while a:
   k=a.bit_length()-1
   if k in p:a^=p[k]
   else:p[k]=a;break
 return len(p)
def eigen(gs,g=None,w=F(0)):
 return [2*W*sum((m&a).bit_count()%2 for a in gs)+(2*w if g and (m&g).bit_count()%2 else 0) for m in range(16)]
weights=[F(0),F(1,100),F(1,40),F(1,20),F(1,10),F(1,5),F(2,5),F(1,2),F(1)]
checks=0; repaired=0; details=[]
for failed in combinations(R,4):
 gs=[x for name,group in R.items() if name not in failed for x in group]
 if rank(gs)!=3:continue
 mask=next(m for m in range(1,16) if all((m&x).bit_count()%2==0 for x in gs))
 baseline=eigen(gs); residual=min(x for x in baseline if x>0)
 gates=[g for g in range(1,16) if (mask&g).bit_count()%2]
 assert len(gates)==8
 for g in gates:
  for w in weights:
   vals=eigen(gs,g,w); actual=sorted(vals)[1]
   assert actual==min(2*w,residual)
   assert (actual>0)==(w>0)
   for mode in range(16):
    chi=lambda x:-1 if (x&mode).bit_count()%2 else 1
    for x in range(16):
     lhs=sum(W*(chi(x)-chi(x^a)) for a in gs)+w*(chi(x)-chi(x^g))
     assert lhs==vals[mode]*chi(x)
     checks+=1
  repaired+=1
 details.append(dict(failed=list(failed),parity_mask=format(mask,'04b'),eligible_gates=gates,residual_gap=str(residual),full_residual_weight=str(residual/2)))
assert len(details)==11 and repaired==88 and checks==202752
assert {d['residual_gap'] for d in details}=={'4/5','12/5'}
out=dict(schema='cern.h156.variable-repair.v1',fractures=11,eligible_repair_channels=88,cases=792,walsh_checks=checks,weights_tested=list(map(str,weights)),common_target_gap='4/5',common_target_minimum_weight='2/5',full_residual_weights=['2/5','6/5'],minimum_connectivity_weight='infimum 0, not attained',fracture_certificates=details)
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:v for k,v in out.items() if k!='fracture_certificates'},indent=2))
