from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json
A=F(3,64); C=A*A; HORIZON=12
DELAYS=(0,1,2)
PENALTIES=(F(0),F(1,1024),F(1,256),F(1,64),F(1,16))
POLICIES=('never','always','every3','risk')
def should_probe(policy,age,q,t):
 if policy=='never':return False
 if policy=='always':return True
 if policy=='every3':return t%3==0
 return q*F(min(age,6),6)>C
def run(d0,d1,change,q,policy):
 assert d0!=d1
 known=d0;age=0;probing=F(0);stale=F(0);count=wrong=0
 for t in range(HORIZON):
  actual=d0 if t<change else d1
  age+=1
  if should_probe(policy,age,q,t):
   known=actual;age=0;probing+=C;count+=1
  if actual!=known:
   stale+=q;wrong+=1
  assert probing>=0 and stale>=0
 return dict(total=probing+stale,probe=probing,stale=stale,pulses=count,stale_steps=wrong)
records=[(d0,d1,change,q,p,run(d0,d1,change,q,p))
 for d0 in DELAYS for d1 in DELAYS if d0!=d1
 for change,q,p in product(range(HORIZON),PENALTIES,POLICIES)]
assert len(records)==1440
summary={}
for p in POLICIES:
 group=[r for *_,policy,r in records if policy==p]
 summary[p]=dict(cases=len(group),pulses=sum(r['pulses'] for r in group),
   stale_steps=sum(r['stale_steps'] for r in group),
   total_cost=str(sum((r['total'] for r in group),F(0))))
assert {p:(v['pulses'],v['stale_steps']) for p,v in summary.items()}=={
 'never':(0,2340),'always':(4320,0),'every3':(1440,360),'risk':(1944,1044)}
wins={}
for p in ('never','always','every3'):
 w=t=l=0
 for d0 in DELAYS:
  for d1 in DELAYS:
   if d0==d1:continue
   for change,q in product(range(HORIZON),PENALTIES):
    a=run(d0,d1,change,q,'risk')['total']
    b=run(d0,d1,change,q,p)['total']
    w+=a<b;t+=a==b;l+=a>b
 wins[p]=(w,t,l)
assert wins=={'never':(186,144,30),'always':(216,144,0),'every3':(228,18,114)}
print(json.dumps(dict(configurations=1440,exact_updates=17280,totals=summary,comparisons=wins),indent=2))
