import itertools as it
import numpy as np
V=list(it.product((-1,1),repeat=4))+[tuple(2*s if i==j else 0 for i in range(4)) for j in range(4) for s in (-1,1)]
A=np.array([[int(np.dot(a,b)==2) for b in V] for a in V]); E=list(zip(*np.where(np.triu(A,1))))
F=[f for f in it.combinations(range(24),3) if all(A[a,b] for a,b in it.combinations(f,2))]
D=np.where(A,1,99);np.fill_diagonal(D,0)
for k in range(24):D=np.minimum(D,D[:,k,None]+D[None,k,:])
assert len(E)==len(F)==96
assert all(sum(a in f and b in f for f in F)==3 for a,b in E)
# At each vertex, 12 faces meet it. For SU(2), delta_f squared times vertex norm squared <= 3 face energy.
# Sum over all vertex-face incidences, then use scalar norm Poincare to distribute mass.
assert all(sum(v in f for f in F)==12 for v in range(24))
I=np.eye(2,dtype=complex); Q=[I,1j*np.array([[0,1],[1,0]]),np.array([[0,1],[-1,0]],complex),1j*np.diag([1,-1])];Q += [-q for q in Q]
def check(U):
 L=8*np.eye(48,dtype=complex);links={}
 for (a,b),u in zip(E,U):
  links[a,b]=u;links[b,a]=u.conj().T;L[2*a:2*a+2,2*b:2*b+2]=-u;L[2*b:2*b+2,2*a:2*a+2]=-u.conj().T
 d=np.array([max(0.,2-np.trace(links[a,b]@links[b,c]@links[c,a]).real) for a,b,c in F]); S=d.sum(); m=np.linalg.eigvalsh(L)[0]
 old=max(d/(72+30*d)); new=S/(600+S/4) if S else 0
 # Derived via weighted face incidence, scalar Poincare, and Cauchy-Schwarz (see text).
 assert m>=max(old,new)-1e-10
 return float(m),float(old),float(new),float(S)
rng=np.random.default_rng(88); R=[check([Q[k] for k in rng.integers(8,size=96)]) for _ in range(128)]
print('random min/max',min(x[0] for x in R),max(x[0] for x in R),'new bound range',min(x[2] for x in R),max(x[2] for x in R),'improved',sum(x[2]>x[1] for x in R))
for theta in [1,.1,.01,.001]:
 U=[I]*96;U[0]=np.diag([np.exp(1j*theta),np.exp(-1j*theta)]);print(theta,check(U))
print('flat',check([I]*96));print('PASS 133')
