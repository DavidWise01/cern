import sympy as s
from itertools import product
N=24
address=lambda a,b,c:12*a+6*b+c
states=[address(a,b,c) for a,b,c in product(range(2),range(2),range(6))]
assert states==list(range(N))
p=s.Matrix([(-1)**(k+1) for k in states]+[0])
R=s.zeros(25); R[24,24]=1
for k in states:R[(k+1)%N,k]=1
assert R**24==s.eye(25) and R*p==-p and R**6*p==p
assert all(R**j!=s.eye(25) for j in range(1,24))
assert sum(p)==0
L=s.zeros(25)
for a,b in [(k,(k+1)%N) for k in states]+[(k,24) for k in states]:
 L[a,a]+=1; L[b,b]+=1; L[a,b]-=1; L[b,a]-=1
assert L*p==5*p and R*L==L*R
x=s.Symbol('x')
poly=s.factor(L.charpoly(x).as_expr())
expected=x*(x-25)*(x-5)*(x-3)**2*(x-2)**2*(x-4)**2*(x*x-6*x+6)**2*(x*x-6*x+7)**2*(x**4-12*x**3+50*x*x-84*x+46)**2
# Factor pairs for cos(pi/12), cos(5pi/12), cos(7pi/12), cos(11pi/12).
assert s.expand(poly-expected)==0
assert L*s.ones(25,1)==s.zeros(25,1)
q=s.Matrix([1]*24+[-24]);assert L*q==25*q
assert len([k for k in states if k//6!=((k+1)%24)//6])==4
print('PASS: 24 addresses, 24-step cycle, 4 seams, 12 positive/12 negative, one center, polarity eigenvalue 5, center contrast 25')
print('charpoly:',poly)
print('gap:',3-2*s.cos(s.pi/12),'=',float(3-2*s.cos(s.pi/12)))
