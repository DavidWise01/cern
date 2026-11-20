from fractions import Fraction as F
from itertools import product
from random import Random
import json
from pathlib import Path
H=(F(31,36),F(1),F(11,18)); E=F(1,18); B=F(3,4)
def parts(x):
 a,b,c=x
 return (a/2,a/4+b/2,b/4+c/2),(b,c,F(0))
def step(x,b,d):return tuple(p+b*q+e for p,q,e in zip(*parts(x),d))
def box(y,r):return tuple((max(-h,v-r*h),min(h,v+r*h)) for v,h in zip(y,H))
def limit(U):
 if any(l>r for l,r in U):return None
 lo,hi=F(0),B
 for x in (tuple(l for l,r in U),tuple(r for l,r in U)):
  for p,q,h in zip(*parts(x),H):
   c=h-E
   if q==0:
    if abs(p)>c:return None
   else:
    a,b=sorted(((-c-p)/q,(c-p)/q));lo=max(lo,a);hi=min(hi,b)
 return hi if lo<=hi else None

def main():
 checks=0;rows=[]
 for r in map(F,['0','1/16','1/8','1/4','1/2','1']):
  total=F(0);above=0
  for v in product(range(-4,5),repeat=3):
   y=tuple(h*F(k,4) for h,k in zip(H,v));U=box(y,r);b=limit(U)
   assert b>=F(3,8)
   total+=b;above+=b>F(3,8)
   for x in product(*U):
    for d in product((-E,E),repeat=3):
     assert all(abs(z)<=h for z,h in zip(step(x,b,d),H));checks+=1
   if b<B:
    bp=b+min(F(1,10000),B-b)
    assert any(abs(p+bp*q)+E>h for x in (tuple(l for l,u in U),tuple(u for l,u in U)) for p,q,h in zip(*parts(x),H))
  rows.append(dict(radius=str(r),mean=float(total/729),above_fixed=above))
 rng=Random(136); n=0;total=F(0);above=0;r=F(1,8)
 for trial in range(20):
  x=tuple(h*rng.choice((-1,1)) for h in H)
  for k in range(50):
   y=tuple(v+r*h*rng.choice((-1,0,1)) for v,h in zip(x,H));U=box(y,r);assert all(l<=v<=u for v,(l,u) in zip(x,U))
   b=F((limit(U)*1024).__floor__(),1024);assert b>=F(3,8)
   x=step(x,b,tuple(E*rng.choice((-1,0,1)) for h in H));assert all(abs(v)<=h for v,h in zip(x,H));n+=1;total+=b;above+=b>F(3,8)
 # unsafe point-only decision at center of uncertainty box near upper boundary
 y=tuple(h*F(7,8) for h in H);U=box(y,F(1,8));naive=limit(box(y,F(0)));robust=limit(U)
 assert naive>robust and any(z>h for z,h in zip(step(H,naive,(E,)*3),H))
 out=dict(grid_states=729,radii=rows,corner_checks=checks,trajectory_steps=n,trajectory_mean=float(total/n),trajectory_above_fixed=above,naive_counterexample=dict(naive=str(naive),robust=str(robust)),violations=0)
 print(json.dumps(out,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n')
if __name__=='__main__':main()
