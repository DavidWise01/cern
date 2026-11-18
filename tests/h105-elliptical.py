import numpy as np
from itertools import combinations_with_replacement
rng=np.random.default_rng(105)
B=[]
for i,j in combinations_with_replacement(range(4),2):
 e=np.zeros((4,4));e[i,j]=e[j,i]=1 if i==j else 1/np.sqrt(2);B.append(e)
B=np.array(B);assert len(B)==10
assert np.allclose(np.einsum('aij,bij->ab',B,B),np.eye(10))
err=0
for _ in range(128):
 M=rng.normal(size=(4,4));Q=M.T@M+np.eye(4);R=np.linalg.qr(rng.normal(size=(4,4)))[0];x=rng.normal(size=4)
 c=np.einsum('aij,ij->a',B,Q); assert np.allclose(np.einsum('a,aij->ij',c,B),Q)
 T=np.einsum('aij,bij->ab',B,np.array([R@e@R.T for e in B]));assert np.allclose(T.T@T,np.eye(10))
 W=R@Q@R.T; assert np.allclose(T@c,np.einsum('aij,ij->a',B,W))
 err=max(err,abs(x@Q@x-(R@x)@W@(R@x)))
 assert np.allclose(np.linalg.eigvalsh(Q),np.linalg.eigvalsh(W))
print('PASS 128 configurations; max quadratic error',err)
