import numpy as np
from itertools import product
n=24; N=n*n
I=np.eye(n); R=np.roll(I,1,axis=0); L=2*I-R-R.T
X=np.kron(L,I); Y=np.kron(I,L)
c=2-2*np.cos(np.pi/12)
for a,b,k in [(1,1,0),(1,0,0),(1,1,.01),(1,0,.01),(0,0,.01),(1,0,1/N),(1,1,1/N),(0,0,1/N),(1,1,1)]:
 M=np.zeros((N+1,N+1)); M[:N,:N]=a*X+b*Y+k*np.eye(N); M[:N,N]=M[N,:N]=-k; M[N,N]=N*k
 base=[2*a*(1-np.cos(2*np.pi*j/n))+2*b*(1-np.cos(2*np.pi*l/n)) for j,l in product(range(n),repeat=2) if (j,l)!=(0,0)]
 expected=np.sort([0,(N+1)*k]+[v+k for v in base]); ev=np.linalg.eigvalsh(M)
 assert np.allclose(ev,expected,atol=1e-9)
 gap=min((N+1)*k,k+c*min(a,b))
 assert abs(ev[1]-gap)<1e-9
 err=np.max(abs(ev-expected)); print(f'{a:g} {b:g} {k:.9g} gap={gap:.12g} nullity={sum(abs(ev)<1e-9)} error={err:.3g}')
print('PASS: 9 full spectra, 577 eigenvalues each')
