from fractions import Fraction as F
from itertools import product
import json
from pathlib import Path

def step(r,d,e,i):
    p=[r[j]+d[j] for j in range(2)]
    p[i]=min(p[i],e[i]);return tuple(p)
def adaptive(r,d,e):
    return min(range(2),key=lambda i:(max(step(r,d,e,i)),sum(step(r,d,e,i)),i))
# Exact box uncertainty, independent component drift and bounded scalar noise.
values=[F(i,100) for i in range(4)]
cases=0
for ru,rv,du,dv,eu,ev in product(values,repeat=6):
    r=(ru,rv);d=(du,dv);e=(eu,ev);i=adaptive(r,d,e)
    assert max(step(r,d,e,i))==min(max(step(r,d,e,j)) for j in range(2))
    cases+=1
# Anisotropic drift fixture; calibrated eps=.01, drift .03 along u, .005 along v.
d=(F(3,100),F(1,200));e=(F(1,100),)*2
r0=e
out={}
for name in ('fixed','adaptive'):
    r=r0;peaks=[];schedule=[]
    for k in range(60):
        i=k%2 if name=='fixed' else adaptive(r,d,e)
        r=step(r,d,e,i);peaks.append(max(r));schedule.append('uv'[i])
    out[name]={'peak':str(max(peaks)),'mean':str(sum(peaks)/len(peaks)),'schedule':''.join(schedule),'last':list(map(str,r))}
# Sharpness: centered observations permit every endpoint of the propagated box.
# For each prior, drift and noise budget, endpoints of each posterior interval are reachable.
sharp=0
for rr,dd,ee in product(values,repeat=3):
    p=min(rr+dd,ee)
    for sign in (-1,1):
        x=sign*min(rr,p);y=sign*p
        assert abs(x)<=rr and abs(y-x)<=dd and abs(y)<=ee
        sharp+=1
result={'local_optimality_cases':cases,'endpoint_checks':sharp,'fixture':out,'horizon':60,'drift':['3/100','1/200'],'noise':'1/100'}
Path(__file__).with_name('results.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
# Finite-horizon optimum for cumulative max radius, with no switching penalty.
from functools import lru_cache
@lru_cache(None)
def plan(k,r):
    if k==0:return F(0),''
    options=[]
    for i in range(2):
        nxt=step(r,d,e,i);cost,suffix=plan(k-1,nxt)
        options.append((max(nxt)+cost,'uv'[i]+suffix))
    return min(options)
cost,schedule=plan(60,r0);r=r0;peaks=[]
for label in schedule:
    r=step(r,d,e,'uv'.index(label));peaks.append(max(r))
assert sum(peaks)==cost
out['lookahead']={'peak':str(max(peaks)),'mean':str(cost/60),'schedule':schedule,'last':list(map(str,r))}
result['dp_states']=plan.cache_info().currsize
Path(__file__).with_name('results.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
