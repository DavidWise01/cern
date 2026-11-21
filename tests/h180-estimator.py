from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json

DRIFTS=(F(-1,32),F(-1,64),F(0),F(1,64),F(1,32))
DELAYS=range(4)
CAPS=(F(1,64),F(1,16),F(1,4))
STARTS=(F(-1,4),F(0),F(1,4))
POLICIES=('delayed','naive','observer','coast')
N=24

def clip(x,cap):return min(cap,max(-cap,x))

def simulation(v,d,cap,start,policy,changing=False):
    theta=[start]; controls=[]; estimates=[]; errors=[]
    for t in range(N):
        k=max(0,t-d)
        y=theta[k]
        if policy=='coast':raw=F(0)
        elif policy=='delayed':raw=y/F(2)
        elif policy=='naive':
            previous=theta[max(0,k-1)]
            raw=(y+F(d+1)*(y-previous))/2
        else:
            if k>=1:
                vhat=theta[k]-theta[k-1]+controls[k-1]
                estimated=y+F(t-k)*vhat-sum(controls[k:t],F(0))
            else:
                vhat=F(0)
                estimated=y-sum(controls[k:t],F(0))
            raw=estimated/F(2)
            estimates.append(vhat)
        u=clip(raw,cap)
        assert abs(u)<=cap
        controls.append(u)
        vnow=v if not changing or t<12 else -v
        theta.append(theta[t]+vnow-u)
        errors.append(abs(theta[-1]))
        assert theta[-1]==start+sum((v if not changing or j<12 else -v) for j in range(t+1))-sum(controls)
    return {'final':abs(theta[-1]),'mean':sum(errors)/N,'peak':max(errors),
            'last':theta[-1],'controls':controls,'theta':theta,'estimates':estimates}

records={}
for change,v,d,c,s,p in product((False,True),DRIFTS,DELAYS,CAPS,STARTS,POLICIES):
    records[(change,v,d,c,s,p)]=simulation(v,d,c,s,p,change)

results={}
for change in (False,True):
    triples=[]
    for v,d,c,s in product(DRIFTS,DELAYS,CAPS,STARTS):
        r={p:records[(change,v,d,c,s,p)] for p in POLICIES}
        triples.append(r)
    def compare(a,b):
        return {'wins':sum(x[a]['final']<x[b]['final'] for x in triples),
                'ties':sum(x[a]['final']==x[b]['final'] for x in triples),
                'losses':sum(x[a]['final']>x[b]['final'] for x in triples)}
    results['constant' if not change else 'reversal']={
        'observer_vs_delayed':compare('observer','delayed'),
        'observer_vs_naive':compare('observer','naive'),
        'observer_vs_coast':compare('observer','coast'),
        'mean_absolute_final_error':{p:str(sum(x[p]['final'] for x in triples)/len(triples)) for p in POLICIES},
        'mean_absolute_error_all_steps':{p:str(sum(x[p]['mean'] for x in triples)/len(triples)) for p in POLICIES}}

reconstruction_checks=0
for v,d,c,s in product(DRIFTS,DELAYS,CAPS,STARTS):
    r=records[(False,v,d,c,s,'observer')]
    for t in range(N):
        k=max(0,t-d)
        if k>=1:
            assert r['estimates'][t]==v
            reconstruction_checks+=1

summary={'version':'H180','configurations':len(records),'exact_updates':len(records)*N,
         'constant_drift_reconstruction_checks':reconstruction_checks,
         'hard_cap_violations':0,'phase_dynamics_failures':0,'results':results,
         'assumptions':['exact delay and observation for this experiment',
         'known issued control history','constant or one unannounced drift reversal at step 12',
         'unwrapped scalar common phase, not a physical toroid engine']}
Path(__file__).with_name('results.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
