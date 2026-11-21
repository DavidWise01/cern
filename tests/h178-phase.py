from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json
INITIAL=(F(-1,4),F(0),F(1,4))
DRIFTS=(F(0),F(1,64),F(1,32),F(-1,64))
DELAYS=(0,1,2,3)
DIFFS=(F(-1,64),F(0),F(1,64))
CAPS=(F(1,64),F(1,16),F(1,4))
POLICIES=('relative','delayed','unbounded_predictor','bounded_predictor')
def run(common,delay,diff,cap,policy):
 phase=list(INITIAL);observed=[F(0)];used=F(0);energy=F(0)
 maximum=F(0);clips=0
 for t in range(24):
  avg=sum(phase)/3
  phase=[(v+avg)/2+common+diff*(i-1) for i,v in enumerate(phase)]
  j=max(0,len(observed)-1-delay)
  old=observed[j];previous=observed[max(0,j-1)]
  if policy=='relative':correction=F(0)
  elif policy=='delayed':correction=old/2
  else:
   proposed=(old+(delay+1)*(old-previous))/2
   correction=proposed
   if policy=='bounded_predictor':
    correction=min(cap,max(-cap,proposed))
    clips+=int(correction!=proposed)
  phase=[v-correction for v in phase]
  observed.append(sum(phase)/3)
  used+=correction;energy+=correction**2
  maximum=max(maximum,abs(correction))
  assert observed[-1]+used==(t+1)*common
 return dict(error=abs(observed[-1]),spread=max(phase)-min(phase),
             effort=energy,max_command=maximum,clips=clips)
cases={(d,l,s,c,p):run(d,l,s,c,p) for d,l,s,c,p in
 product(DRIFTS,DELAYS,DIFFS,CAPS,POLICIES)}
assert len(cases)==576
counts=dict(bounded_beats_unbounded=0,bounded_ties_unbounded=0,bounded_worse_unbounded=0,
 bounded_beats_delayed=0,bounded_ties_delayed=0,bounded_worse_delayed=0,
 cap_respected=0,unbounded_exceeds_cap=0,bounded_uses_clip=0)
for d,l,s,c in product(DRIFTS,DELAYS,DIFFS,CAPS):
 b=cases[d,l,s,c,'bounded_predictor'];u=cases[d,l,s,c,'unbounded_predictor']
 f=cases[d,l,s,c,'delayed']
 counts['cap_respected']+=b['max_command']<=c
 counts['unbounded_exceeds_cap']+=u['max_command']>c
 counts['bounded_uses_clip']+=b['clips']>0
 for rival,name in ((u,'unbounded'),(f,'delayed')):
  prefix='bounded_'
  relation='beats' if b['error']<rival['error'] else 'ties' if b['error']==rival['error'] else 'worse'
  counts[prefix+relation+'_'+name]+=1
assert counts==dict(bounded_beats_unbounded=81,bounded_ties_unbounded=60,bounded_worse_unbounded=3,
 bounded_beats_delayed=30,bounded_ties_delayed=36,bounded_worse_delayed=78,
 cap_respected=144,unbounded_exceeds_cap=84,bounded_uses_clip=84)
result=dict(configurations=576,exact_updates=13824,matched_comparisons=144,
 verification_failures=0,counts=counts,
 representative={p:str(cases[F(1,32),2,F(0),F(1,16),p]['error']) for p in POLICIES})
Path(__file__).with_name('results.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
