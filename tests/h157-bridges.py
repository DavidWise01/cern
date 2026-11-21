from itertools import combinations
from fractions import Fraction as F
from pathlib import Path
import json
ROOTS={'P_M':(1,),'P_I':(2,),'P_U':(4,),'P_B':(8,),'A':(3,12),'B':(5,10),'C':(9,6)}
W=F(2,5); TARGET=F(4,5)
def rank(gs):
 p={}
 for x in gs:
  while x:
   i=x.bit_length()-1
   if i in p:x^=p[i]
   else:p[i]=x;break
 return len(p)
def eig(gs, bridges=()):
 return [2*W*sum((m&g).bit_count()%2 for g in gs)+2*sum(w for g,w in bridges if (m&g).bit_count()%2) for m in range(16)]
def price(g):return F(3+g.bit_count(),4)
def fine(gs,bridges):return sorted(eig(gs,bridges))[1]>=TARGET
rows=[]; pairs=0; checks=0; naive_failures=0
for failed in combinations(ROOTS,4):
 gs=[g for name,vs in ROOTS.items() if name not in failed for g in vs]
 if rank(gs)!=3:continue
 m=next(m for m in range(1,16) if all((m&g).bit_count()%2==0 for g in gs))
 candidates=[g for g in range(1,16) if (m&g).bit_count()%2]
 assert len(candidates)==8
 feasible=[]
 for a,b in combinations(candidates,2):
  bridges=((a,W),(b,W));pairs+=1
  for scenario in [(),((a,W),),((b,W),)]:
   active=bridges if not scenario else tuple(x for x in bridges if x not in scenario)
   assert fine(gs,active)
   vals=eig(gs,active)
   for mode in range(16):
    lam=vals[mode]
    for vertex in range(16):
     chi=lambda x:1 if (mode&x).bit_count()%2==0 else -1
     lhs=sum(W*(chi(vertex)-chi(vertex^g)) for g in gs)+sum(w*(chi(vertex)-chi(vertex^g)) for g,w in active)
     assert lhs==lam*chi(vertex)
     checks+=1
  feasible.append((W*(price(a)+price(b)),a,b))
 best=min(feasible)
 a,b=best[1:]
 assert best[0]==min(F(2,5)*(price(x)+price(y)) for x,y in combinations(candidates,2))
 assert not fine(gs,())
 naive_failures+=1
 rows.append({'failed_roots':list(failed),'mask':format(m,'04b'),'selection':[a,b],'unit_costs':[str(price(a)),str(price(b))],'total_cost':str(best[0]),'gap_both':str(sorted(eig(gs,((a,W),(b,W))))[1]),'gap_after_either_failure':[str(sorted(eig(gs,((a,W),)))[1]),str(sorted(eig(gs,((b,W),)))[1])],'eligible':candidates})
assert len(rows)==11 and pairs==308 and checks==236544
out={'schema':'cern.h157.independent-bridge-pricing.v1','fractures':11,'pairs_tested':pairs,'source_failure_scenarios_per_pair':3,'walsh_checks':checks,'shared_root_counterexamples':naive_failures,'target':'4/5','minimum_bridge_weight_per_source':'2/5','cost_rule':'(3+popcount(g))/4 per unit weight','rows':rows,'scope':'One failure of either new repair root after original fracture; independent roots assumed.'}
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:v for k,v in out.items() if k!='rows'},indent=2))
