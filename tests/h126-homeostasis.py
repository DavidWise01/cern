from fractions import Fraction as F
from itertools import product
import json
from pathlib import Path
# u=(qww-qzz)/2, v=qwz. Both drift by at most delta per step.
eps=F(1,100); delta=F(1,50)
worst=[F(0),F(0)]; cases=0
for u,v in product(range(-2,3),repeat=2):
 for du,dv in product([-delta,0,delta],repeat=2):
  # Test each component separately; arbitrary joint extremes need not satisfy spectral bound.
  if du*du+dv*dv>delta*delta: continue
  for n0,n1,p in product([-eps,0,eps],[-eps,0,eps],[0,1]):
   old=[F(u),F(v)]; current=[old[0]+du,old[1]+dv]
   observed=p; hidden=1-p
   estimate=current.copy(); estimate[observed]+=n1; estimate[hidden]=old[hidden]+n0
   e0=abs(estimate[observed]-current[observed]);e1=abs(estimate[hidden]-current[hidden])
   assert e0<=eps and e1<=eps+delta
   worst[0]=max(worst[0],e0);worst[1]=max(worst[1],e1);cases+=1
assert worst==[eps,eps+delta]
# indistinguishable histories: previous samples have error +/- eps,
# followed by hidden drift +/- delta; current visible component is equal.
for sign in [-1,1]:
 old=sign*eps; current=old+sign*delta
 assert old-sign*eps==0 and abs(current-old)==delta
 assert abs(current)==eps+delta
out=dict(cases=cases,epsilon=str(eps),delta=str(delta),visible_bound=str(worst[0]),hidden_bound=str(worst[1]),minimax_witnesses=2)
print(json.dumps(out,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n')
