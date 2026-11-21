from itertools import combinations, product
from fractions import Fraction as F
from pathlib import Path
import json
ROOTS={'P_M':(1,),'P_I':(2,),'P_U':(4,),'P_B':(8,),'A':(3,12),'B':(5,10),'C':(9,6)}
W=F(2,5)
def rank(gs):
 p={}
 for x in gs:
  while x:
   k=x.bit_length()-1
   if k in p:x^=p[k]
   else:p[k]=x;break
 return len(p)
def spec(gs,bridges=()):
 return [2*W*sum((m&g).bit_count()%2 for g in gs)+2*W*sum((m&g).bit_count()%2 for g in bridges) for m in range(16)]
def canonical(a):
 mp={};out=[]
 for x in a:
  if x not in mp:mp[x]=len(mp)
  out.append(mp[x])
 return tuple(out)
partitions=sorted({canonical(a) for a in product(range(3),repeat=3)})
assert len(partitions)==5
cuts=[];checks=0;scenarios=0
for failed in combinations(ROOTS,4):
 gs=[g for root,group in ROOTS.items() if root not in failed for g in group]
 if rank(gs)!=3:continue
 mask=next(m for m in range(1,16) if all((m&g).bit_count()%2==0 for g in gs))
 choices=[g for g in range(1,16) if (mask&g).bit_count()%2]
 price=lambda g:F(3+g.bit_count(),4)
 chosen=min(combinations(choices,3),key=lambda q:(sum(price(v) for v in q),q))
 entries=[]
 for part in partitions:
  unique=sorted(set(part));worst=F(10);safe=True;failed_cases=[]
  for k in range(min(2,len(unique))+1):
   for dead in combinations(unique,k):
    live=tuple(g for g,root in zip(chosen,part) if root not in dead)
    eig=spec(gs,live);gap=sorted(eig)[1];scenarios+=1
    worst=min(worst,gap)
    if gap<2*W:safe=False;failed_cases.append(dict(failed_roots=dead,gap=str(gap)))
    for mode in range(16):
     lam=eig[mode]
     for v in range(16):
      chi=lambda x:1 if (mode&x).bit_count()%2==0 else -1
      assert sum(W*(chi(v)-chi(v^g)) for g in gs+list(live))==lam*chi(v)
      checks+=1
  assert safe==(len(unique)==3)
  entries.append(dict(root_partition=part,unique_roots=len(unique),survives_two_root_failures=safe,worst_gap=str(worst),failure_examples=failed_cases[:2]))
 cuts.append(dict(failed_original_roots=failed,parity_mask=format(mask,'04b'),bridges=chosen,partitions=entries))
assert len(cuts)==11
counts={str(n):sum(a['unique_roots']==n for c in cuts for a in c['partitions']) for n in (1,2,3)}
assert counts=={'1':11,'2':33,'3':11}
assert scenarios==231 and checks==59136
assert all(sum(x['survives_two_root_failures'] for x in c['partitions'])==1 for c in cuts)
report=dict(schema='cern.h159.ancestry-audit.v1',fractures=11,root_partition_types=5,configurations=55,root_fail_scenarios=scenarios,walsh_checks=checks,unique_root_counts=counts,passing_configurations=11,failed_configurations=44,common_ancestor_counterexamples=11,minimum_sources_for_two_failures=3,guaranteed_surviving_gap='4/5',limitations=['Logical source ancestry is asserted, not externally attested','Failure of a source removes all dependent bridges','Not a hardware independence audit'],certificates=cuts)
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:v for k,v in report.items() if k!='certificates'},indent=2))
