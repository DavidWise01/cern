import itertools as it
import numpy as np
from scipy.optimize import linprog
V=list(it.product((-1,1),repeat=4))+[tuple(2*s if i==j else 0 for i in range(4)) for j in range(4) for s in (-1,1)]
A=np.array([[int(np.dot(a,b)==2) for b in V] for a in V]); E=list(zip(*np.where(np.triu(A,1))))
F=[f for f in it.combinations(range(24),3) if all(A[a,b] for a,b in it.combinations(f,2))]
C=np.array([[int(a in f and b in f) for f in F] for a,b in E]); B=np.array([[int(v in f) for f in F] for v in range(24)])
assert len(E)==len(F)==96 and np.all(C.sum(axis=1)==3)
L=8*np.eye(24)-A; I=np.eye(2,dtype=complex)
Q=[I,1j*np.array([[0,1],[1,0]]),np.array([[0,1],[-1,0]],complex),1j*np.diag([1,-1])];Q += [-q for q in Q]
def check(U):
 K=8*np.eye(48,dtype=complex); links={}
 for (a,b),u in zip(E,U):
  links[a,b]=u;links[b,a]=u.conj().T;K[2*a:2*a+2,2*b:2*b+2]=-u;K[2*b:2*b+2,2*a:2*a+2]=-u.conj().T
 d=np.array([max(0.,2-np.trace(links[a,b]@links[b,c]@links[c,a]).real) for a,b,c in F]); S=d.sum()
 ts=np.linspace(0,1,101)
 def cert(alpha):
  w=B@(alpha*d)/9
  return max(float(np.linalg.eigvalsh(t*L+(1-t)*np.diag(w))[0]) for t in ts)
 uniform=cert(np.full(96,1/3))
 # LP maximizes total weighted curvature; not the spectral objective.
 opt=linprog(-d,A_ub=C,b_ub=np.ones(96),bounds=(0,1),method='highs');assert opt.success
 alpha=np.maximum(opt.x,0);alpha/=max(1.,float((C@alpha).max()))
 assert np.max(C@alpha)<=1+1e-12
 adaptive=cert(alpha); best=max(uniform,adaptive,max(d/(72+30*d)),S/(600+S/4)); m=float(np.linalg.eigvalsh(K)[0]);assert best<=m+1e-10
 return m,uniform,adaptive,best,float(d@alpha),float(S/3)
rng=np.random.default_rng(90);R=[check([Q[k] for k in rng.integers(8,size=96)]) for _ in range(128)]
print('random ranges',[(min(x[i] for x in R),max(x[i] for x in R)) for i in range(6)])
print('improved',sum(x[2]>x[1]+1e-10 for x in R),'worse',sum(x[2]<x[1]-1e-10 for x in R))
for theta in [1,.1,.01,.001]:
 U=[I]*96;U[0]=np.diag([np.exp(1j*theta),np.exp(-1j*theta)]);print(theta,check(U))
print('flat',check([I]*96));print('PASS 133')
