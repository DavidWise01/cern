from fractions import Fraction as F
from itertools import product
import json
from pathlib import Path
# Scalar observation errors bounded by eps; spectral norm drift bounded by delta.
# Extremal drift E=+/- delta I attains the mixed-direction error.
eps=F(1,100); delta=F(1,50)
results=[]
for t in [F(1,10),F(1,4),F(1,2),F(1,3),F(2,3),F(3,4),F(9,10),F(1,5)]:
 c=(1-t*t)/(1+t*t); s=2*t/(1+t*t); g=2*s*c
 bound=(2*eps+delta)/abs(g)
 worst=F(0)
 for a,b,q in product([-2,0,2],repeat=3):
  for na,nb,nm,drift in product([-1,1],repeat=4):
   m=s*s*a+g*q+c*c*b+nm*eps+drift*delta
   estimate=(m-s*s*(a+na*eps)-c*c*(b+nb*eps))/g
   worst=max(worst,abs(estimate-q))
   assert abs(estimate-q)<=bound
 assert worst==bound
 results.append(dict(t=str(t),gain=str(g),bound=str(bound),worst=str(worst)))
# At 45 degrees: coefficients are exactly 1/2, 1, 1/2 even though s,c irrational.
worst=max(abs(nm*eps+drift*delta-F(1,2)*(na+nb)*eps) for na,nb,nm,drift in product([-1,1],repeat=4))
assert worst==2*eps+delta
out=dict(epsilon=str(eps),delta=str(delta),rational_angles=8,cases=8*27*16+16,optimal_bound=str(worst),results=results)
print(json.dumps(out,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2))
