import numpy as np
n=24; N=n*n; k=1/N
x,y=np.meshgrid(2*np.pi*np.arange(n)/n,2*np.pi*np.arange(n)/n,indexing='ij')
cols=[np.ones(N+1)]
for z in (x,y,x+y,x-y):
 for f in (np.cos,np.sin): cols.append(np.r_[f(z).ravel(),0])
cols.append(np.r_[np.ones(N),-N])
P=np.column_stack([v/np.linalg.norm(v) for v in cols]); I=np.eye(n); R=np.roll(I,1,axis=0); J=2*I-R-R.T
L=np.zeros((N+1,N+1)); L[:N,:N]=np.kron(J,I)+np.kron(I,J)+k*np.eye(N); L[:N,N]=L[N,:N]=-k;L[N,N]=1
D=P.T@L@P; delta=2-2*np.cos(2*np.pi/n)+k
assert np.allclose(P.T@P,np.eye(10));assert np.allclose(L@P,P@D)
assert np.allclose(np.linalg.eigvalsh(D),[0]+[delta]*4+[2*(delta-k)+k]*4+[577*k])
# Work on mean-zero modes only; equilibrium amplitude is conserved.
D=D[1:,1:]; rng=np.random.default_rng(106); worst=0
for _ in range(128):
 M=rng.normal(size=(9,9)); Q=M.T@M+np.eye(9); C=np.linalg.cholesky(Q).T
 W=rng.normal(size=(9,9)); S=W-W.T
 A=np.linalg.solve(C,(-D+S)@C)
 residual=A.T@Q+Q@A+2*delta*Q
 worst=max(worst,np.linalg.eigvalsh(residual)[-1]); assert np.linalg.eigvalsh(residual)[-1]<1e-9
print('spectrum',np.linalg.eigvalsh(P.T@L@P)); print('PASS 128; max positive residual',worst)
