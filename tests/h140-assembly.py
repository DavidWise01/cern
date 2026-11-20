from fractions import Fraction as F
from itertools import product
from random import Random
from pathlib import Path
import json
h=(F(31,36),F(1),F(11,18)); eps=F(1,18); gamma=eps/2
counts=[1,2,3,5,8,13,21,34,55,89,144]
def A(x):
 a,b,c=x;return (a/2+3*b/8,a/4+b/2+3*c/8,b/4+c/2)
# Ring exchange: bounded neighbor signal, normalized by its own coordinate cap.
def step(X,D):
 N=len(X)
 return [tuple(v+gamma*(X[(i-1)%N][k]+X[(i+1)%N][k])/(2*h[k])+D[i][k] for k,v in enumerate(A(x))) for i,x in enumerate(X)]
def safe(X):return all(abs(v)<=b for x in X for v,b in zip(x,h))
rng=Random(140); checks=0; max_ratio=F(0)
for N in counts:
 X=[tuple(v*rng.choice((-1,1)) for v in h) for _ in range(N)]
 for t in range(64):
  D=[tuple(gamma*rng.choice((-1,0,1)) for _ in h) for _ in X]
  X=step(X,D);assert safe(X);checks+=N
  max_ratio=max(max_ratio,max(abs(v)/b for x in X for v,b in zip(x,h)))
for N in counts:
 X=[h]*N; D=[(gamma,)*3]*N
 assert step(X,D)==X
assert gamma+2*gamma>eps
out=dict(module_counts=counts,largest_modules=max(counts),largest_bodies=3*max(counts),steps_per_count=64,module_updates=checks,sharp_fixed_points=len(counts),external_budget=str(gamma),total_exchange_budget=str(gamma),maximum_observed_ratio=float(max_ratio),violations=0)
print(json.dumps(out,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2))
