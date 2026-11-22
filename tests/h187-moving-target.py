"""H187 multiple delay changes and delayed diagnostic returns (exact rational)."""
from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json
C=F(9,4096);N=24
PATHS=((0,1,2,0),(2,1,0,2),(1,2,1,0))
CHANGES=((6,12,18),(5,13,19),(4,9,16))
LATENCIES=(1,2,3);RISKS=(F(0),F(1,1024),F(1,256),F(1,64))
POLICIES=('never','always','every3','risk','timestamp')
def current(ds,ts,t):return ds[sum(t>=x for x in ts)]
def run(ds,ts,lat,q,policy):
 known=ds[0];last=-100;pending=[];cost=F(0);pulses=stale=accepted_stale=discarded=0
 for t in range(N):
  actual=current(ds,ts,t)
  for due,at,measured in pending[:]:
   if due!=t:continue
   pending.remove((due,at,measured))
   if policy=='timestamp' and any(at<x<=t for x in ts):discarded+=1
   else:
    known=measured
    accepted_stale+=int(measured!=actual)
  age=t-last if last>=0 else t+1
  risk=q*F(min(age,6),6)
  probe=(policy=='always' or policy=='every3' and t%3==0 or
    policy=='risk' and risk>C and not pending or
    policy=='timestamp' and (t in ts or risk>C) and not pending)
  if probe:pending.append((t+lat,t,actual));last=t;pulses+=1
  stale+=int(known!=actual)
  cost=pulses*C+stale*q
  assert cost>=0
 return dict(cost=cost,pulses=pulses,stale=stale,stale_accepted=accepted_stale,discarded=discarded)
groups={}
for ds,ts,lat,q,policy in product(PATHS,CHANGES,LATENCIES,RISKS,POLICIES):
 groups[(ds,ts,lat,q,policy)]=run(ds,ts,lat,q,policy)
assert len(groups)==540
summary={}
for p in POLICIES:
 rows=[v for k,v in groups.items() if k[-1]==p]
 summary[p]={'cases':len(rows),'pulses':sum(x['pulses'] for x in rows),
 'stale_steps':sum(x['stale'] for x in rows),
 'known_stale_accepted':sum(x['stale_accepted'] for x in rows),
 'known_stale_discarded':sum(x['discarded'] for x in rows),
 'aggregate_cost':str(sum((x['cost'] for x in rows),F(0)))}
assert summary['timestamp']['known_stale_accepted']==0
assert summary['timestamp']['known_stale_discarded']==126
assert summary['timestamp']['pulses']==735
comparisons={}
for p in POLICIES[:-1]:
 scores=[(groups[(ds,ts,lat,q,'timestamp')]['cost'],
          groups[(ds,ts,lat,q,p)]['cost'])
   for ds,ts,lat,q in product(PATHS,CHANGES,LATENCIES,RISKS)]
 comparisons[p]=dict(wins=sum(a<b for a,b in scores),ties=sum(a==b for a,b in scores),
                     losses=sum(a>b for a,b in scores))
assert comparisons['risk']==dict(wins=36,ties=31,losses=41)
assert all(sum(x.values())==108 for x in comparisons.values())
report=dict(configurations=540,exact_updates=12960,policies=summary,comparisons=comparisons,
 model_not_physical=True,known_changes_required_for_timestamp_gate=True)
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
