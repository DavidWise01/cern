"""H185 — The Cost of Asking Again; exact rational synthetic decision model."""
from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json
N=24; EPS=F(1,64); A=F(3,64); COST=A*A
STARTS=(-F(1,4),F(0),F(1,4))
DRIFTS=(-F(1,64),F(0),F(1,64))
DELAYS=(0,1,2); NOISES=(-EPS,F(0),EPS)
PENALTIES=(F(0),F(1,256),F(1,64),F(1,16))
POLICIES=('never','always','adaptive','repeat')
def readings(start,drift,delay,noise,amp):
    ph=[start]
    for t in range(N):ph.append(ph[-1]+drift-(amp if t==3 else F(0)))
    return [ph[max(0,t-delay)]+noise for t in range(N)]
def candidates(s,v,d,n,amp):
    obs=readings(s,v,d,n,amp)
    valid=[]
    for guess in DELAYS:
        expected=readings(s,v,guess,F(0),amp)
        if max(abs(x-y) for x,y in zip(obs,expected))<=EPS:valid.append(guess)
    assert d in valid
    return valid
def choose(k,penalty,policy):
    risk=penalty*F(k-1,k)
    num={'never':0,'always':1,'repeat':2}.get(policy,int(risk>COST))
    loss=risk if num==0 else F(0)
    return num, num*COST, loss+num*COST
rows=[];unique_passive=unique_active=0
for s,v,d,n in product(STARTS,DRIFTS,DELAYS,NOISES):
    passive=candidates(s,v,d,n,F(0));active=candidates(s,v,d,n,A)
    unique_passive+=len(passive)==1;unique_active+=len(active)==1
    assert len(active)==1
    for q,p in product(PENALTIES,POLICIES):
        pulses,energy,total=choose(len(passive),q,p)
        rows.append((s,v,d,n,q,p,pulses,energy,total))
assert len(rows)==1296 and (unique_passive,unique_active)==(12,81)
metrics={}
for policy in POLICIES:
    r=[x for x in rows if x[5]==policy]
    metrics[policy]={'cases':len(r),'pulses':sum(x[6] for x in r),
      'energy':str(sum((x[7] for x in r),F(0))),
      'total':str(sum((x[8] for x in r),F(0)))}
comparison={}
for rival in ('never','always','repeat'):
    wins=ties=losses=0
    for s,v,d,n,q in product(STARTS,DRIFTS,DELAYS,NOISES,PENALTIES):
        a=choose(len(candidates(s,v,d,n,F(0))),q,'adaptive')[2]
        b=choose(len(candidates(s,v,d,n,F(0))),q,rival)[2]
        wins+=a<b; ties+=a==b; losses+=a>b
    comparison['adaptive_vs_'+rival]={'wins':wins,'ties':ties,'losses':losses}
assert (metrics['adaptive']['pulses'],metrics['always']['pulses'])==(183,324)
assert comparison['adaptive_vs_never']=={'wins':183,'ties':141,'losses':0}
assert comparison['adaptive_vs_always']=={'wins':141,'ties':183,'losses':0}
assert comparison['adaptive_vs_repeat']=={'wins':324,'ties':0,'losses':0}
report={'cases':1296,'sensor_cases':81,'pulse_cost':str(COST),
  'passive_identified':unique_passive,'active_identified':unique_active,
  'metrics':metrics,'pairwise':comparison,'true_delay_exclusions':0,
  'scope':'Synthetic one-step expected decision cost; assumed uniform delay prior'}
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
