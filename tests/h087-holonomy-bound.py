import itertools as it
import numpy as np
V=list(it.product((-1,1),repeat=4))+[tuple(2*s if i==j else 0 for i in range(4)) for j in range(4) for s in (-1,1)]
A=np.array([[int(np.dot(a,b)==2) for b in V] for a in V]); E=list(zip(*np.where(np.triu(A,1))))
F=[(a,b,c) for a,b,c in it.combinations(range(24),3) if A[a,b] and A[b,c] and A[c,a]]
D=np.where(A,1,99);np.fill_diagonal(D,0)
for k in range(24): D=np.minimum(D,D[:,k,None]+D[None,k,:])
assert len(E)==len(F)==96
sums=[int(D[:,f].min(axis=1).sum()) for f in F];print('distance sums',set(sums)); print('distance histogram',np.bincount(D[:,F[0]].min(axis=1)))
I=np.eye(2,dtype=complex);Q=[I,1j*np.array([[0,1],[1,0]]),np.array([[0,1],[-1,0]],complex),1j*np.diag([1,-1])];Q += [-q for q in Q]
def check(U):
 L=8*np.eye(48,dtype=complex); links={}
 for (a,b),u in zip(E,U):
  links[a,b]=u;links[b,a]=u.conj().T;L[2*a:2*a+2,2*b:2*b+2]=-u;L[2*b:2*b+2,2*a:2*a+2]=-u.conj().T
 delta2=[max(0.,2-np.trace(links[a,b]@links[b,c]@links[c,a]).real) for a,b,c in F]
 bound=max(d/(72+d*s) for d,s in zip(delta2,sums));m=np.linalg.eigvalsh(L)[0];assert m>=bound-1e-10
 return float(m),float(bound)
rng=np.random.default_rng(87);results=[check([Q[k] for k in rng.integers(8,size=96)]) for _ in range(128)]
print('random min range',min(m for m,b in results),max(m for m,b in results),'bound range',min(b for m,b in results),max(b for m,b in results))
for t in [1,.1,.01,.001]:
 U=[I]*96;U[0]=np.diag([np.exp(1j*t),np.exp(-1j*t)]);print(t,*check(U))
print('flat',check([I]*96));print('PASS 133')
