from itertools import product, permutations
from pathlib import Path
import json
import sympy as s
seed=(1,3,6,8,9)
def mirror(q):return tuple((1 if x>0 else -1)*((abs(x)-1)//3*3+3-(abs(x)-1)%3) for x in q)
def upside(q):return tuple((1 if x>0 else -1)*(10-abs(x)) for x in q)
def invert(q):return tuple(-x for x in q)
def backward(q):return q[::-1]
ops=(mirror,invert,upside,backward)
variants={}
for bits in product((0,1),repeat=4):
 q=seed
 for b,f in zip(bits,ops):
  if b:q=f(q)
 variants[''.join(map(str,bits))]=q
assert len(set(variants.values()))==16
for q in variants.values():
 for f in ops:
  assert f(f(q))==q
  for g in ops:assert f(g(q))==g(f(q))
Q=s.Matrix([[1,1+s.I,2],[1-s.I,-2,s.I],[2,-s.I,1]])
checks=0
for p in permutations(range(3)):
 P=s.eye(3)[:,list(p)]
 for signs in product((-1,1),repeat=3):
  U=P*s.diag(*signs)
  if U.det()!=1:continue
  R=U*Q*U.H
  assert U.H*U==s.eye(3) and R.H==R and s.trace(R)==s.trace(Q) and R.charpoly().as_expr()==Q.charpoly().as_expr()
  checks+=1
out=dict(seed=seed,variants=variants,distinct=len(set(variants.values())),involution_checks=64,commutation_checks=256,su3_transport_checks=checks)
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2));print(json.dumps(out))
