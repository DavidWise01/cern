from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json,random

def A(x,b):
 a,c,e=x
 return (a/2+b*c,a/4+c/2+b*e,c/4+e/2)
def weights(b):
 m=(3+4*b)/(1-2*b);g=(2+2*b*m,m,2+m/2);e=1/max(g)
 return tuple(e*v for v in g),e
B=F(3,8);h,e=weights(B);checks=0
for n in range(-36,37):
 s=F(80829,50000)*F(10)**n
 for b in (F(0),F(1,8),F(1,4),B):
  for sign in product((-1,1),repeat=3):
   x=tuple(s*v*z for v,z in zip(h,sign))
   for ds in product((-1,1),repeat=3):
    y=tuple(v+s*e*z for v,z in zip(A(x,b),ds))
    assert all(abs(v)<=s*r for v,r in zip(y,h));checks+=1
rng=random.Random(134);steps=0
for trial in range(100):
 x=h
 for k in range(100):
  b=F(rng.randrange(25),64);d=tuple(e*rng.choice((-1,0,1)) for _ in h)
  x=tuple(v+w for v,w in zip(A(x,b),d));assert all(abs(v)<=r for v,r in zip(x,h));steps+=1
h0,e0=weights(F(0)); y=tuple(v+e for v in A(h0,B));assert any(v>r for v,r in zip(y,h))
assert max((e+F(1,1000))*r/e for r in h)>1
out=dict(beta_max=str(B),weights=list(map(str,h)),reserve=str(e),corner_checks=checks,switching_steps=steps,violations=0,local_box_switch=dict(source=list(map(str,h0)),result=list(map(str,y)),target=list(map(str,h))),above_bound_equilibrium=True)
print(json.dumps(out,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n')
