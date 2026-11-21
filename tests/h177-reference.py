from fractions import Fraction as F
from itertools import product
import json
from pathlib import Path
INITIAL=(F(-1,4),F(0),F(1,4))
DRIFTS=(F(0),F(1,64),F(1,32),F(-1,64))
DELAYS=(0,1,2,3)
DIFFERENTIALS=(F(-1,64),F(0),F(1,64))
POLICIES=('relative','delayed','predictive')
def run(common,delay,diff,policy):
 p=list(INITIAL);hist=[F(0)];budget=F(1000);spent=F(0)
 for t in range(24):
  mean=sum(p)/3
  p=[(v+mean)/2+common+diff*(i-1) for i,v in enumerate(p)]
  obs=hist[max(0,len(hist)-1-delay)]
  if policy=='relative':correction=F(0)
  elif policy=='delayed':correction=obs/2
  else:
   previous=hist[max(0,len(hist)-2-delay)]
   correction=(obs+F(delay+1)*(obs-previous))/2
  p=[v-correction for v in p]
  hist.append(sum(p)/3)
  cost=correction**2
  assert budget>=cost
  budget-=cost;spent+=cost
  assert spent+budget==1000
 return abs(sum(p)/3),max(p)-min(p)
cases={}
for common,delay,diff,policy in product(DRIFTS,DELAYS,DIFFERENTIALS,POLICIES):
 cases[(common,delay,diff,policy)]=run(common,delay,diff,policy)
assert len(cases)==144
counts=dict(predictive_beats_delayed=0,predictive_worse_delayed=0,
 predictive_ties_delayed=0,predictive_beats_relative=0,delayed_beats_relative=0)
for common,delay,diff in product(DRIFTS,DELAYS,DIFFERENTIALS):
 r=cases[(common,delay,diff,'relative')][0]
 d=cases[(common,delay,diff,'delayed')][0]
 p=cases[(common,delay,diff,'predictive')][0]
 counts['predictive_beats_delayed']+=p<d
 counts['predictive_worse_delayed']+=p>d
 counts['predictive_ties_delayed']+=p==d
 counts['predictive_beats_relative']+=p<r
 counts['delayed_beats_relative']+=d<r
assert counts==dict(predictive_beats_delayed=9,predictive_worse_delayed=27,
 predictive_ties_delayed=12,predictive_beats_relative=18,delayed_beats_relative=36)
assert cases[(F(1,32),0,F(0),'relative')]==(F(3,4),F(1,33554432))
out={'configurations':144,'exact_updates':3456,'controller_ledger_checks':3456,
 'conservation_failures':0,'policy_comparisons':counts,
 'reference_counterexample':{'mean_error':'3/4','relative_spread':'1/33554432'},
 'representative':{p:str(cases[(F(1,32),2,F(0),p)][0]) for p in POLICIES},
 'limits':'Synthetic unwrapped phase control; no physical Rupert engine or independently validated clock'}
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps(out,indent=2))
