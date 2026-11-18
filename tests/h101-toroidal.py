import numpy as np
from itertools import product
n=24
R=np.roll(np.eye(n,dtype=int),1,axis=0)
I=np.eye(n,dtype=int)
L=2*I-R-R.T
X=np.kron(R,I); Y=np.kron(I,R)
assert np.array_equal(X@Y,Y@X)
assert np.array_equal(np.linalg.matrix_power(R,24),I)
assert all(not np.array_equal(np.linalg.matrix_power(R,k),I) for k in range(1,24))
p=np.array([(-1)**(a+b+1) for a,b in product(range(n),repeat=2)])
assert sum(p)==0 and np.array_equal(X@p,-p) and np.array_equal(Y@p,-p)
for a,b in [(1,1),(1,.5),(1,.1),(1,0)]:
 K=a*np.kron(L,I)+b*np.kron(I,L)
 ev=np.linalg.eigvalsh(K)
 exact=np.sort([2*a*(1-np.cos(2*np.pi*j/n))+2*b*(1-np.cos(2*np.pi*k/n)) for j,k in product(range(n),repeat=2)])
 assert np.allclose(ev,exact,atol=1e-10)
 assert np.allclose(K@p,4*(a+b)*p)
 print(a,b,'neuron gap',max(0,ev[1]),'nullity',sum(abs(ev)<1e-9))
print('PASS: 576 addresses, commuting shifts, balanced polarity, four full spectra')
