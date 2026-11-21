from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json

N=24
STARTS=(-F(1,4),F(0),F(1,4))
DRIFTS=(-F(1,32),F(0),F(1,32))
DELAYS=(0,1,2)
NOISE=(-F(1,64),F(0),F(1,64))
REVERSALS=(False,True)
POLICIES=('single','history','naive','coast')
CAP=F(1,16); NB=F(1,64); VM=F(1,32)

def clamp(x):return min(CAP,max(-CAP,x))

def envelope(y,k,t,commands):
    out=[]
    for d in DELAYS:
        source=max(0,k-d); span=t-source
        u=sum(commands[source:t],F(0))
        out.append((y-NB-span*VM-u,y+NB+span*VM-u))
    return min(a for a,b in out),max(b for a,b in out)

def simulate(start,drift,delay,noise,reversal,policy):
    states=[start];commands=[];observed=[];cover=unsafe=coast=actions=0
    width=F(0)
    for t in range(N):
        observed.append(states[max(0,t-delay)]+noise)
        single=envelope(observed[-1],t,t,commands)
        if policy=='history':
            candidates=[envelope(observed[k],k,t,commands) for k in range(t+1)]
            lo=max(x[0] for x in candidates);hi=min(x[1] for x in candidates)
        else:lo,hi=single
        assert lo<=states[t]<=hi
        cover+=1;width+=hi-lo
        if policy in ('single','history'):
            u=clamp(lo/2) if lo>0 else clamp(hi/2) if hi<0 else F(0)
        elif policy=='naive':u=clamp(observed[-1]/2)
        else:u=F(0)
        assert abs(u)<=CAP
        unsafe+=abs(states[t]-u)>abs(states[t])
        actions+=u!=0;coast+=u==0
        commands.append(u)
        actual_drift=drift if not reversal or t<12 else -drift
        states.append(states[-1]+actual_drift-u)
    return dict(final=abs(states[-1]),avgwidth=width/N,covered=cover,unsafe=unsafe,coast=coast,actions=actions)

records={}
for params in product(STARTS,DRIFTS,DELAYS,NOISE,REVERSALS):
    for policy in POLICIES:records[params+(policy,)]=simulate(*params,policy)
assert len(records)==648

metrics={p:{'coverage':0,'unsafe':0,'coast':0,'actions':0,'mean_width':F(0)} for p in POLICIES}
comparisons={'history_vs_single':{'wins':0,'ties':0,'losses':0},'history_vs_naive':{'wins':0,'ties':0,'losses':0}}
width_better=width_tie=0
for params in product(STARTS,DRIFTS,DELAYS,NOISE,REVERSALS):
    for p in POLICIES:
        r=records[params+(p,)];m=metrics[p]
        m['coverage']+=r['covered'];m['unsafe']+=r['unsafe']
        m['coast']+=r['coast'];m['actions']+=r['actions']
        m['mean_width']+=r['avgwidth']
    a=records[params+('history',)];b=records[params+('single',)]
    width_better+=a['avgwidth']<b['avgwidth']
    width_tie+=a['avgwidth']==b['avgwidth']
    for rival in ('single','naive'):
        other=records[params+(rival,)]['final'];key='history_vs_'+rival
        label='wins' if a['final']<other else 'ties' if a['final']==other else 'losses'
        comparisons[key][label]+=1

assert all(m['coverage']==3888 for m in metrics.values())
assert metrics['history']['unsafe']==0
assert metrics['single']['unsafe']==0
assert width_better+width_tie==162

summary={'schema':'cern.h182.informative-interval.v1','configurations':len(records),
 'exact_updates':len(records)*N,'matched_scenarios':162,
 'metrics':{p:{**{k:v for k,v in m.items() if k!='mean_width'},
   'mean_interval_width':str(m['mean_width']/162)} for p,m in metrics.items()},
 'history_width_better':width_better,'history_width_tied':width_tie,
 'comparisons':comparisons,
 'limits':['Unknown delay fixed per run: candidate delays 0,1,2',
           'Drift bound 1/32 and noise bound 1/64',
           'All prior commands exactly recorded',
           'No validated physical Rupert POD or CMB power generation']}
Path(__file__).with_name('results.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
