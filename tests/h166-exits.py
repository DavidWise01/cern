from fractions import Fraction as F
from pathlib import Path
import json
def step(r,out,mask):
    flow=[r/12 if mask & (1<<i) else F(0) for i in range(3)]
    return r-sum(flow),tuple(out[i]+flow[i] for i in range(3))
states=checks=0
for i in range(21):
 for mask in range(8):
  r,out=F(i,20),(F(0),)*3;total=r;states+=1
  for _ in range(12):
   r,out=step(r,out,mask)
   assert r>=0 and all(q>=0 for q in out)
   assert r+sum(out)==total
   if mask==0:assert r==total
   checks+=1
assert states==168 and checks==2016
report=dict(version='H166',configurations=states,exact_updates=checks,conservation_failures=0,geometry='I-shaped braced POD with 3 illustrative .-. exits',caveat='Exit count and layout are exploratory, not established by user')
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
