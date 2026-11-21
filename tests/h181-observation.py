from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json
N=24
DRIFTS=(-F(1,32),F(0),F(1,32)); NOISE=(-F(1,64),F(0),F(1,64))
DELAYS=(0,1,2); STARTS=(-F(1,4),F(0),F(1,4))
POLICIES=('naive','interval','coast','oracle')
CAP=F(1,16); NB=F(1,64); VMIN=-F(1,32); VMAX=F(1,32)
def clamp(x):return max(-CAP,min(CAP,x))
def envelope(y,t,u):
    estimates=[]
    for d in DELAYS:
        k=max(0,t-d);h=sum(u[k:t],F(0));dt=t-k
        estimates.append((y-NB+dt*VMIN-h,y+NB+dt*VMAX-h))
    return min(x for x,z in estimates),max(z for x,z in estimates)
def run(start,drift,reverse,delay,noise,policy):
    phase=[start];u=[];unsafe=coasts=covered=0
    for t in range(N):
        y=phase[max(0,t-delay)]+noise
        lo,hi=envelope(y,t,u);theta=phase[t]
        assert lo<=theta<=hi
        covered+=1
        if policy=='naive':cmd=clamp(y/2)
        elif policy=='oracle':cmd=clamp(theta/2)
        elif policy=='coast':cmd=F(0)
        else:cmd=clamp(lo/2) if lo>0 else clamp(hi/2) if hi<0 else F(0)
        assert abs(cmd)<=CAP
        unsafe+=abs(theta-cmd)>abs(theta)
        coasts+=cmd==0
        u.append(cmd)
        v=drift if not reverse or t<12 else -drift
        phase.append(theta+v-cmd)
    return dict(error=abs(phase[-1]),unsafe=unsafe,coasts=coasts,covered=covered)
data={(s,v,r,d,n,p):run(s,v,r,d,n,p) for s,v,r,d,n,p in product(STARTS,DRIFTS,(False,True),DELAYS,NOISE,POLICIES)}
assert len(data)==648
groups={(s,v,r,d,n):{p:data[(s,v,r,d,n,p)] for p in POLICIES} for s,v,r,d,n in product(STARTS,DRIFTS,(False,True),DELAYS,NOISE)}
comparison={}
for p in ('naive','coast','oracle'):
    comparison[p]={t:sum((g['interval']['error']<g[p]['error']) if t=='wins' else (g['interval']['error']==g[p]['error']) if t=='ties' else (g['interval']['error']>g[p]['error']) for g in groups.values()) for t in ('wins','ties','losses')}
counts=dict(configurations=648,exact_updates=15552,coverage_checks=sum(x['covered'] for x in data.values()),interval_unsafe_steps=sum(x['unsafe'] for k,x in data.items() if k[-1]=='interval'),naive_unsafe_steps=sum(x['unsafe'] for k,x in data.items() if k[-1]=='naive'),interval_coasts=sum(x['coasts'] for k,x in data.items() if k[-1]=='interval'),naive_coasts=sum(x['coasts'] for k,x in data.items() if k[-1]=='naive'),cap_violations=0)
assert counts==dict(configurations=648,exact_updates=15552,coverage_checks=15552,interval_unsafe_steps=0,naive_unsafe_steps=658,interval_coasts=1382,naive_coasts=232,cap_violations=0)
assert comparison['naive']==dict(wins=12,ties=6,losses=144)
report=dict(schema='cern.h181.uncertain-observation.v1',counts=counts,comparisons=comparison,limitations=['Conservative envelope coverage requires correct model bounds','Immediate safe actions do not maximize 24-step accuracy','Not physically calibrated'])
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
