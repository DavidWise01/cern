from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json
N=24
STARTS=(-F(1,4),F(0),F(1,4))
DRIFTS=(-F(1,64),F(0),F(1,64))
DELAYS=(0,1,2)
NOISES=(-F(1,64),F(0),F(1,64))
BOUND=F(1,64)
def pulses(excited):
    return [F(1,8) if excited and t in (3,10,17) else -F(1,16) if excited and t in (6,14,20) else F(0) for t in range(N)]
def evaluate(start,drift,delay,noise,excited):
    u=pulses(excited)
    theta=[start]
    for t in range(N):theta.append(theta[-1]+drift-u[t])
    readings=[theta[max(0,t-delay)]+noise for t in range(N)]
    candidates=[]
    residuals=[]
    for d in DELAYS:
        res=max(abs(readings[t]-theta[max(0,t-d)]) for t in range(N))
        residuals.append(res)
        if res<=BOUND:candidates.append(d)
    assert delay in candidates
    return dict(true_delay=delay,feasible=candidates,identified=len(candidates)==1,
      ambiguous=len(candidates)>1,max_residuals=[str(x) for x in residuals])
rows=[]
for start,drift,delay,noise,excited in product(STARTS,DRIFTS,DELAYS,NOISES,(False,True)):
    r=evaluate(start,drift,delay,noise,excited)
    rows.append(dict(start=str(start),drift=str(drift),noise=str(noise),excited=excited,**r))
assert len(rows)==162
metrics={}
for excited in (False,True):
    subset=[r for r in rows if r['excited']==excited]
    metrics['pulses' if excited else 'no_pulses']=dict(cases=len(subset),
      unique=sum(r['identified'] for r in subset),
      ambiguous=sum(r['ambiguous'] for r in subset),
      false_eliminations=sum(r['true_delay'] not in r['feasible'] for r in subset))
assert metrics['pulses']==dict(cases=81,unique=81,ambiguous=0,false_eliminations=0)
assert metrics['no_pulses']==dict(cases=81,unique=12,ambiguous=69,false_eliminations=0)
report=dict(title="H183 — The Age of an Observation",cases=len(rows),
   measurements=len(rows)*N,observation_checks=len(rows)*N*len(DELAYS),noise_bound=str(BOUND),
   metrics=metrics,limitations=["Exact known initial phase, dynamics and pulse history",
   "Constant bounded noise; only delay uncertain","No energy cost of probing modeled"])
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
