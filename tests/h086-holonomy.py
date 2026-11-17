import itertools as it
import numpy as np
V=list(it.product((-1,1),repeat=4))+[tuple(2*s if j==i else 0 for j in range(4)) for i in range(4) for s in (-1,1)]
E=[(i,j) for i,j in it.combinations(range(24),2) if np.dot(V[i],V[j])==2]
I=np.eye(2,dtype=complex)
Q=[I,1j*np.array([[0,1],[1,0]]),np.array([[0,1],[-1,0]],complex),1j*np.diag([1,-1])]; Q += [-q for q in Q]
def analyze(U):
 L=8*np.eye(48,dtype=complex); adj=[[] for _ in V]
 for (a,b),u in zip(E,U):
  L[2*a:2*a+2,2*b:2*b+2]=-u; L[2*b:2*b+2,2*a:2*a+2]=-u.conj().T
  adj[a].append((b,u)); adj[b].append((a,u.conj().T))
 T={0:I}; queue=[0]
 for a in queue:
  for b,u in adj[a]:
   if b not in T:T[b]=u.conj().T@T[a];queue.append(b)
 assert len(T)==24
 H=[T[a].conj().T@u@T[b] for (a,b),u in zip(E,U)]
 flat=all(np.array_equal(h,I) for h in H)
 ev=np.linalg.eigvalsh(L); nullity=int(np.count_nonzero(abs(ev)<1e-9))
 assert nullity==(2 if flat else 0) and ev[0]>=-1e-9
 return flat,nullity,float(ev[0])
rng=np.random.default_rng(86)
cases=[]
for _ in range(64):
 g=[Q[k] for k in rng.integers(8,size=24)]
 cases.append([g[a]@g[b].conj().T for a,b in E])
for _ in range(64): cases.append([Q[k] for k in rng.integers(8,size=96)])
res=[analyze(u) for u in cases];assert all(x[0] for x in res[:64]) and not any(x[0] for x in res[64:])
print('Q8 cases',len(res),'flat',sum(x[0] for x in res),'curved',sum(not x[0] for x in res))
print('curved minimum range',min(x[2] for x in res[64:]),max(x[2] for x in res[64:]))
# Continuous SU(2) counterfamily: exact flatness follows from a triangle containing the defect.
assert any((0,k) in E and (1,k) in E for k in range(2,24)) if (0,1) in E else True
for theta in [1.,.1,.01,.001]:
 U=[I]*96;U[0]=np.diag([np.exp(1j*theta),np.exp(-1j*theta)])
 # Numerical spectrum only for this continuous family, not exact holonomy classification.
 L=8*np.eye(48,dtype=complex)
 for (a,b),u in zip(E,U):L[2*a:2*a+2,2*b:2*b+2]=-u;L[2*b:2*b+2,2*a:2*a+2]=-u.conj().T
 m=np.linalg.eigvalsh(L)[0];bound=(1-np.cos(theta))/12
 assert 0<m<=bound+1e-12
 print('theta',theta,'minimum',m,'Rayleigh bound',bound)
print('PASS')
