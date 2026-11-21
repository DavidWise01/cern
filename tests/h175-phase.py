from fractions import Fraction as Q
from itertools import product
import json
from pathlib import Path
J=Q(1,20);I=Q(2);IH=Q(6);EXHAUST=Q(1,100)
POLICIES=('aligned','opposed','staggered','alternating','feedback')
PHASE=(1,0,-1,0)
def kin(ls,h):return sum(x*x/(2*I) for x in ls)+h*h/(2*IH)
def phase(policy,t,i,rot):
 if policy=='aligned':return 0
 if policy=='opposed':return 2
 if policy=='staggered':return (t+i)%4
 if policy=='alternating':return 0 if t%2==0 else 2
 return 0 if rot>=0 else 2
def simulate(seed,policy,rate,load,mask):
 fuel=Q(seed,4);supplied=fuel
 lr=[Q(0)]*3;h=Q(0);external=Q(0);work=Q(0);heat=Q(0);ejecta=Q(0)
 fired=aligned=opposed=neutral=blocked=0
 for t in range(16):
  if t%2==0:fuel+=rate;supplied+=rate
  i=t%3;p=phase(policy,t,i,lr[i]);sgn=PHASE[p]
  if not (mask&(1<<i)) or sgn==0:neutral+=1
  else:
   deltaL=Q(sgn)*J
   old=kin(lr,h);nl=lr.copy();nl[i]+=deltaL;nh=h-deltaL
   dE=kin(nl,nh)-old;cost=EXHAUST+max(Q(0),dE)
   if fuel<cost:blocked+=1
   else:
    fuel-=cost;lr=nl;h=nh;ejecta+=EXHAUST
    if dE<0:heat-=dE
    fired+=1;aligned+=int(sgn>0);opposed+=int(sgn<0)
  if load:
   for a in range(3):
    before=lr[a];after=before*(1-load);removed=before-after
    if removed:
     gain=(before*before-after*after)/(2*I)
     lr[a]=after;external+=removed
     work+=gain*Q(3,4);heat+=gain*Q(1,4)
  assert sum(lr)+h+external==0
  assert fuel+kin(lr,h)+work+heat+ejecta==supplied
  assert all(x>=0 for x in (fuel,heat,work,ejecta))
 return dict(seed=seed,policy=policy,rate=str(rate),load=str(load),mask=mask,
  supplied=str(supplied),fuel=str(fuel),work=str(work),heat=str(heat),
  ejecta=str(ejecta),fired=fired,neutral=neutral,blocked=blocked,
  efficiency=str(work/supplied if supplied else Q(0)))
rows=[simulate(seed,p,rate,load,mask) for seed,p,rate,load,mask in
 product(range(5),POLICIES,(Q(0),Q(1,16),Q(1,8)),(Q(0),Q(1,8)),(1,3,7))]
assert len(rows)==450
matched={}
for r in rows:
 key=(r['seed'],r['rate'],r['load'],r['mask'])
 matched.setdefault(key,{})[r['policy']]=r
wins=sum(Q(z['aligned']['work'])>Q(z['opposed']['work']) for z in matched.values())
ties=sum(Q(z['aligned']['work'])==Q(z['opposed']['work']) for z in matched.values())
feedback=sum(Q(z['feedback']['work'])>Q(z['staggered']['work']) for z in matched.values())
best=max(rows,key=lambda r:Q(r['efficiency']))
report=dict(configurations=len(rows),exact_updates=len(rows)*16,
 attempted_events=sum(r['fired']+r['blocked'] for r in rows),
 powered_ejections=sum(r['fired'] for r in rows),
 rejected_events=sum(r['blocked'] for r in rows),
 neutral_events=sum(r['neutral'] for r in rows),
 conservation_failures=0,aligned_beats_opposed=wins,
 aligned_ties_opposed=ties,feedback_beats_staggered=feedback,
 matched_comparisons=len(matched),best=best)
assert (report['powered_ejections'],report['rejected_events'],
 report['neutral_events'],ties,feedback)==(4197,303,2700,90,42)
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
