from fractions import Fraction as F
from itertools import product
from random import Random
import json
from pathlib import Path
H=(F(31,36),F(1),F(11,18)); E=F(1,18); B=F(3,4)
def parts(x):
 a,b,c=x
 return (a/2,a/4+b/2,b/4+c/2),(b,c,F(0))
def interval(x,eps=E):
 lo,hi=F(0),B
 for p,q,h in zip(*parts(x),H):
  r=h-eps
  if r<0:return None
  if q==0:
   if abs(p)>r:return None
  else:
   a,b=sorted(((-r-p)/q,(r-p)/q));lo=max(lo,a);hi=min(hi,b)
 if lo>hi:return None
 return lo,hi
def step(x,b,d):return tuple(p+b*q+e for p,q,e in zip(*parts(x),d))
def safe(x,b):return all(abs(p+b*q)+E<=h for p,q,h in zip(*parts(x),H))
def main():
 grid=above=checks=0
 for v in product(range(-4,5),repeat=3):
  x=tuple(h*F(k,4) for h,k in zip(H,v));lo,hi=interval(x);assert lo==0 and hi>=F(3,8)
  above+=hi>F(3,8);grid+=1
  for d in product((-E,E),repeat=3):
   assert all(abs(y)<=h for y,h in zip(step(x,hi,d),H));checks+=1
  if hi<B:assert not safe(x,hi+min(F(1,10000),B-hi))
 rng=Random(135); total=F(0);high=0;steps=0
 for trial in range(40):
  x=tuple(h*rng.choice((-1,1)) for h in H)
  for k in range(50):
   hi=interval(x)[1];b=F((hi*1024).__floor__(),1024)
   assert b>=F(3,8) and safe(x,b)
   x=step(x,b,tuple(E*rng.choice((-1,0,1)) for _ in H));assert all(abs(y)<=h for y,h in zip(x,H))
   total+=b;high+=b>F(3,8);steps+=1
 out=dict(grid_states=grid,above_fixed_grid=above,corner_checks=checks,trajectory_steps=steps,above_fixed_steps=high,mean_feedback=str(total/steps),mean_feedback_decimal=float(total/steps),hardware_cap=str(B),fixed_limit='3/8',violations=0,zero_state_feedback=str(interval((F(0),)*3)[1]),positive_corner_feedback=str(interval(H)[1]))
 print(json.dumps(out,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n')
if __name__=='__main__':main()
