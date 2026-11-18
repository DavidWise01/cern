from itertools import product
import sympy as s
R=s.Matrix([[-1,-s.sqrt(3)],[s.sqrt(3),-1]])/2
assert s.simplify(R.T*R)==s.eye(2) and s.simplify(R**3)==s.eye(2)
Q=tuple(range(1,5))+tuple(range(-1,-5,-1))
T=((1,2,3,4),(2,-1,4,-3),(3,-4,-1,2),(4,3,-2,-1))
def m(a,b):return (1 if a*b>0 else -1)*T[abs(a)-1][abs(b)-1]
def inv(a):return a if abs(a)==1 else -a
def loop(a,b,c):return m(c,m(b,a))
flat=curved=checks=0
for a,b,c in product(Q,repeat=3):
 h=loop(a,b,c)
 flat+=h==1;curved+=h!=1
 for x,y,z in product(Q,repeat=3):
  A=m(y,m(a,inv(x)));B=m(z,m(b,inv(y)));C=m(x,m(c,inv(z)))
  assert loop(A,B,C)==m(x,m(h,inv(x)))
  checks+=1
print(dict(flat=flat,curved=curved,covariance_checks=checks))
