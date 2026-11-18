from itertools import product
from collections import Counter
import sympy as s
Q=(1,2,3,4,-1,-2,-3,-4)
T=((1,2,3,4),(2,-1,4,-3),(3,-4,-1,2),(4,3,-2,-1))
def m(a,b): return (1 if a*b>0 else -1)*T[abs(a)-1][abs(b)-1]
def inv(a): return a if abs(a)==1 else -a
def order(h):
 p=1
 for n in range(1,5):
  p=m(h,p)
  if p==1:return n
 raise AssertionError(h)
R=s.Matrix([[-1,-s.sqrt(3)],[s.sqrt(3),-1]])/2
assert s.simplify(R**3)==s.eye(2)
assert s.simplify(R.T*R)==s.eye(2)
assert R!=s.eye(2) and s.simplify(R**2)!=s.eye(2)
counts=Counter(); checks=0
for a,b,c in product(Q,repeat=3):
 h=m(c,m(b,a)); n=order(h); counts[3*n]+=1
 for g in Q:
  assert order(m(g,m(h,inv(g))))==n;checks+=1
assert counts=={3:64,6:64,12:384}
# Unit spinor Hopf projection: opposite spinors have identical shadows.
u,v,x,y=s.symbols('u v x y',real=True)
def hopf(q):
 a,b,c,d=q;return s.Matrix([2*(a*c+b*d),2*(b*c-a*d),a*a+b*b-c*c-d*d])
p=(u,v,x,y); P=hopf(p)
assert s.simplify(hopf(tuple(-z for z in p))-P)==s.zeros(3,1)
assert s.expand(P.dot(P)-(u*u+v*v+x*x+y*y)**2)==0
print(dict(periods=sorted(counts.items()),conjugation_checks=checks,hopf_antipodes=True,hopf_norm=True))
