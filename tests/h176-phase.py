from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json
PHASES=(F(-1,4),F(0),F(1,4))
GAINS=(F(0),F(1,2),F(3,4))
DRIFTS=('zero','common','differential')
MISSES=(0,3,5)
J=F(1,64); Q=F(1,4); IR=F(2); IH=F(6); LOAD=F(1,16); DRAG=F(1,32)
def kinetic(ls,h):return sum(x*x/(2*IR) for x in ls)+h*h/(2*IH)
def drift(kind,t,i):
 if kind=='zero':return F(0)
 if kind=='common':return F(1,64)
 return F((-1,0,1)[i]*(1 if t%2==0 else -1),64)
def simulate(initial,gain,disturbance,miss_period):
 e=list(initial);L=[F(0)]*3;house=F(0);external=F(0)
 fuel=work=heat=ejecta=supplied=F(0);shots=missed=0
 for t in range(20):
  mu=sum(e)/3
  e=[(1-gain)*x+gain*mu+drift(disturbance,t,i) for i,x in enumerate(e)]
  mean=sum(e)/3
  supplied+=Q;fuel+=Q
  for i in range(3):
   if miss_period and (t+i+1)%miss_period==0:missed+=1;continue
   if abs(e[i]-mean)>F(1,8):continue
   old=kinetic(L,house);trial=L.copy();trial[i]+=J;new_house=house-J
   cost=kinetic(trial,new_house)-old
   if cost<0 or fuel<cost:continue
   fuel-=cost;L=trial;house=new_house;shots+=1
  for i in range(3):
   before=L[i];after=before*(1-LOAD)
   extracted=(before*before-after*after)/(2*IR)
   L[i]=after;external+=before-after
   work+=F(3,4)*extracted;heat+=F(1,4)*extracted
  for i in range(3):
   before=kinetic(L,house);delta=L[i]*DRAG
   L[i]-=delta;house+=delta
   loss=before-kinetic(L,house)
   assert loss>=0
   heat+=loss
  assert sum(L)+house+external==0
  assert supplied==fuel+kinetic(L,house)+work+heat+ejecta
  assert fuel>=0 and heat>=0
 spread=max(e)-min(e)
 return dict(spread=spread,locked=spread<=F(1,8),shots=shots,missed=missed,work=work)
records={}
for initial,gain,disturbance,miss_period in product(product(PHASES,repeat=3),GAINS,DRIFTS,MISSES):
 records[(initial,gain,disturbance,miss_period)]=simulate(initial,gain,disturbance,miss_period)
assert len(records)==729
sums={str(g):{'locked':sum(r['locked'] for (i,k,d,m),r in records.items() if k==g),
 'cases':243,'mean_final_spread':str(sum(r['spread'] for (i,k,d,m),r in records.items() if k==g)/243)}
 for g in GAINS}
better=ties=0
for initial,drift_kind,miss in product(product(PHASES,repeat=3),DRIFTS,MISSES):
 a=records[(initial,F(0),drift_kind,miss)]['spread']
 b=records[(initial,F(1,2),drift_kind,miss)]['spread']
 better+=b<a;ties+=b==a
assert sums['0']['locked']==27
assert sums['1/2']['locked']==sums['3/4']['locked']==243
assert (better,ties)==(216,18)
out=dict(configurations=729,exact_updates=14580,conservation_failures=0,
 phase_gains=sums,feedback_half_beats_open_on_spread=better,
 ties=ties,common_mode_caveat='Relative-phase feedback cannot detect drift common to all three toroids')
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps(out,indent=2))
