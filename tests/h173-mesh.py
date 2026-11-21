from fractions import Fraction as Q
from itertools import product
from pathlib import Path
import json
RATES=[Q(i,8) for i in range(8)]
ALLOC=[(Q(a,3),Q(b,3),Q(3-a-b,3)) for a in range(4) for b in range(4-a)]
KAPPAS=[Q(0),Q(1,4),Q(1,2),Q(3,4)]
TRANSFERS=[Q(0),Q(1,4),Q(1,2),Q(1)]
LOADS=(Q(1,2),Q(1,4),Q(0))
DRAG=Q(1,16)
def advance(st,q,allocation,k,t):
    source,rot,work,heat,exhaust=st
    source+=q
    cap=q*k
    exhaust+=q-cap
    rot=[rot[i]+cap*allocation[i] for i in range(3)]
    mean=sum(rot)/3
    rot=[(1-t)*z+t*mean for z in rot]
    for i in range(3):
        delivered=LOADS[i]*rot[i]
        rot[i]-=delivered
        work+=delivered
        loss=DRAG*rot[i]
        rot[i]-=loss
        heat+=loss
    assert source==sum(rot)+work+heat+exhaust
    assert all(x>=0 for x in rot)
    return source,tuple(rot),work,heat,exhaust
runs=[];steps=0;positive=0
for rate,allocation,k,t in product(RATES,ALLOC,KAPPAS,TRANSFERS):
    state=(Q(0),(Q(0),)*3,Q(0),Q(0),Q(0))
    for tick in range(16):
        q=rate if tick%2==0 else Q(0)
        state=advance(state,q,allocation,k,t)
        steps+=1
        positive+=q>0
    source,rot,work,heat,exhaust=state
    runs.append(dict(rate=str(rate),allocation=list(map(str,allocation)),
       coupling=str(k),transfer=str(t),input=str(source),work=str(work),
       heat=str(heat),exhaust=str(exhaust),retained=list(map(str,rot)),
       efficiency=str(work/source if source else Q(0))))
assert len(runs)==1280 and steps==20480 and positive==8960
best=max(runs,key=lambda x:Q(x['efficiency']))
assert best['allocation']==['1','0','0'] and best['coupling']=='3/4' and best['transfer']=='0'
assert all(Q(row['work'])==0 for row in runs if row['rate']=='0' or row['coupling']=='0')
report=dict(configurations=len(runs),exact_updates=steps,positive_input_updates=int(positive),conservation_failures=0,best=best)
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
