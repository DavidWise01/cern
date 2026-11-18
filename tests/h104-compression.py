import numpy as np
from scipy.linalg import eigh_tridiagonal
n=24; N=n*n; k=1/N
I=np.eye(n); R=np.roll(I,1,axis=0); J=2*I-R-R.T
L=np.zeros((N+1,N+1)); L[:N,:N]=np.kron(J,I)+np.kron(I,J)+k*np.eye(N);L[:N,N]=L[N,:N]=-k;L[N,N]=N*k
D=2-2*np.cos(np.pi/12)+k
rng=np.random.default_rng(104); q=rng.normal(size=N+1);q-=q.mean();q/=np.linalg.norm(q)
Q=[];a=[];b=[]
for j in range(24):
 Q.append(q.copy());v=L@q; a.append(q@v);v-=a[-1]*q
 for _ in range(2):
  for u in Q:v-=u@v*u
 if j<23:b.append(np.linalg.norm(v));q=v/b[-1]
for m in (3,6,12,24):
 val=eigh_tridiagonal(a[:m],b[:m-1],eigvals_only=True)[0];assert val>=D-1e-12
 print(m,format(val,'.12g'),format(val-D,'.12g'))
# Exact invariant subspace: constant, lowest cosine mode, hub contrast.
u=np.ones(N+1)/np.sqrt(N+1);v=np.r_[np.repeat(np.cos(2*np.pi*np.arange(n)/n),n),0.];v/=np.linalg.norm(v);w=np.r_[np.ones(N),-N];w/=np.linalg.norm(w)
P=np.column_stack((u,v,w));T=P.T@L@P
assert np.allclose(P.T@P,np.eye(3));assert np.allclose(L@P,P@T);assert np.allclose(T,np.diag([0,D,(N+1)*k]));print('targeted',np.diag(T));print('PASS')
