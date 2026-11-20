from fractions import Fraction as F
from itertools import product
import json
from pathlib import Path
# dimensionless nested layer model: retention and inward transfer
A=((F(1,2),0,0),(F(1,4),F(1,2),0),(0,F(1,4),F(1,2)))
eps=F(1,4)
def step(x,d): return tuple(sum(a*b for a,b in zip(row,x))+v for row,v in zip(A,d))
def scale(n): return F(80829,50000)*F(10)**n
checks=0
for n in range(-36,37):
 s=scale(n); assert scale(n+1)==10*s
 for x in product((-1,1),repeat=3):
  for d in product((-eps,eps),repeat=3):
   y=step(x,d); assert max(map(abs,y))<=1
   assert step(tuple(s*v for v in x),tuple(s*v for v in d))==tuple(s*v for v in y)
   checks+=1
# exact moving-scale transport, each generation uses its own scale
x=(F(1),F(-1),F(1)); transitions=0
for n in range(-36,36):
 s,t=scale(n),scale(n+1)
 d=(eps,-eps,eps); y=step(x,d)
 physical=step(tuple(s*v for v in x),tuple(s*v for v in d))
 assert tuple(t/s*v for v in physical)==tuple(t*v for v in y)
 assert max(map(abs,y))<=1
 x=y;transitions+=1
out=dict(scales=73,corner_checks=checks,adjacent_transitions=transitions,contraction='3/4',disturbance='1/4',violations=0)
print(json.dumps(out,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n')
