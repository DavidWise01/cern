from itertools import product
from collections import Counter
import sympy as s
Q=(1,2,3,4,-1,-2,-3,-4)
T=((1,2,3,4),(2,-1,4,-3),(3,-4,-1,2),(4,3,-2,-1))
def mul(a,b):return (1 if a*b>0 else -1)*T[abs(a)-1][abs(b)-1]
def vec(q):return tuple((1 if q>0 else -1) if j==abs(q)-1 else 0 for j in range(4))
def hopf(p):
 a,b,c,d=p;return (2*(a*c+b*d),2*(b*c-a*d),a*a+b*b-c*c-d*d)
def witness(p):
 a,b,c,d=p;return (0,a,b) if a*a+b*b>=c*c+d*d else (1,c,d)
def reconstruct(n,w):
 x,y,z=n;j,a,b=w
 if j==0:
  r=a*a+b*b;assert r>0
  return a,b,s.Rational(a*x+b*y,2*r),s.Rational(b*x-a*y,2*r)
 r=a*a+b*b;assert r>0
 return s.Rational(a*x-b*y,2*r),s.Rational(b*x+a*y,2*r),a,b
counts=Counter();checks=0
for a,b,c in product(Q,repeat=3):
 h=mul(c,mul(b,a));p=1;visible=full=None
 for k in range(1,5):
  p=mul(h,p);v=vec(p);n=hopf(v);w=witness(v)
  assert reconstruct(n,w)==v;checks+=1
  if visible is None and n==hopf(vec(1)):visible=3*k
  if full is None and p==1:full=3*k
 counts[visible,full]+=1
assert counts=={(3,3):64,(3,6):64,(3,12):128,(6,12):256}
# Verify both reconstruction charts symbolically, including arbitrary real coordinates.
a,b,c,d=s.symbols('a b c d',real=True);p=(a,b,c,d);x,y,z=hopf(p)
for r,expressions in [(a*a+b*b,((a*x+b*y)/2,(b*x-a*y)/2)),(c*c+d*d,((c*x-d*y)/2,(d*x+c*y)/2))]:
 target=(c,d) if r==a*a+b*b else (a,b)
 assert all(s.expand(e-r*t)==0 for e,t in zip(expressions,target))
print('PASS',checks,'reconstructions',sorted(counts.items()),'two symbolic charts')
