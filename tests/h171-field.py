from fractions import Fraction as F
from pathlib import Path
import json
def step(st,feed,w,h):
    store,work,heat,ejecta,total=st
    assert 0<=w and 0<=h and w+h<=1
    return (store,work+feed*w,heat+feed*h,ejecta+feed*(1-w-h),total+feed)
cases=checks=0
for i in range(9):
 for w in (F(0),F(1,4),F(1,2),F(3,4)):
  for h in (F(0),F(1,8),F(1,4)):
   if w+h>1:continue
   s=(F(0),)*5
   for n in range(64):
    s=step(s,F(i,16),w,h)
    assert s[0]+s[1]+s[2]+s[3]==s[4]
    checks+=1
   cases+=1
assert (cases,checks)==(108,6912)
zero=(F(0),)*5
for _ in range(64):zero=step(zero,F(0),F(1,2),F(1,4))
assert zero[1]==0
out=dict(cases=cases,checks=checks,failures=0,zero_flux_work='0',interpretation='infinite event-index does not establish positive available energy flux')
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n')
print(out)
