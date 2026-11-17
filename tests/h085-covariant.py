import itertools as it
import numpy as np
V=list(it.product((-1,1),repeat=4))+[tuple(2*s if j==i else 0 for j in range(4)) for i in range(4) for s in (-1,1)]
E=[(i,j) for i,j in it.combinations(range(24),2) if np.dot(V[i],V[j])==2]
I=np.eye(2,dtype=complex)
Q=[I,1j*np.array([[0,1],[1,0]]),np.array([[0,1],[-1,0]],complex),1j*np.diag([1,-1])]; Q=Q+[-q for q in Q]
def lap(U):
 L=8*np.eye(48,dtype=complex)
 for (i,j),u in zip(E,U): L[2*i:2*i+2,2*j:2*j+2]=-u; L[2*j:2*j+2,2*i:2*i+2]=-u.conj().T
 return L
def transform(U,g):return [g[i]@u@g[j].conj().T for (i,j),u in zip(E,U)]
rng=np.random.default_rng(85); flat=[I]*96; defect=flat.copy();defect[0]=-I
f=np.linalg.eigvalsh(lap(flat)); assert np.allclose(f,np.repeat([0,4,8,10,12],[2,8,18,16,4]))
print('flat',np.round(f,8)); print('defect',np.linalg.eigvalsh(lap(defect))[:6]); mins=[];err=0
for U in [flat,defect]+[[Q[k] for k in rng.integers(8,size=96)] for _ in range(128)]:
 g=[Q[k] for k in rng.integers(8,size=24)]; L=lap(U); W=lap(transform(U,g)); G=np.zeros((48,48),complex)
 for i,a in enumerate(g):G[2*i:2*i+2,2*i:2*i+2]=a
 assert np.array_equal(W,G@L@G.conj().T)
 a=np.linalg.eigvalsh(L);b=np.linalg.eigvalsh(W);err=max(err,np.max(abs(a-b)));assert a[0]>=-1e-10 and np.allclose(a,b)
 mins.append(float(a[0]))
print('random range',min(mins[2:]),max(mins[2:]),'max error',err,'PASS 130')
