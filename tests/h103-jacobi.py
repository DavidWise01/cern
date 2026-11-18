import numpy as np
from scipy.linalg import eigh
n=24; N=n*n
I=np.eye(n); R=np.roll(I,1,axis=0); J=2*I-R-R.T
X=np.kron(J,I); Y=np.kron(I,J)
d=2-2*np.cos(np.pi/12)
for a,b,B in [(1,1,1),(1,0,1),(0,0,1),(1,1,0.01)]:
 k=B/N; M=np.zeros((N+1,N+1)); M[:N,:N]=a*X+b*Y+k*np.eye(N); M[:N,N]=M[N,:N]=-k; M[N,N]=B
 gap=eigh(M,subset_by_index=[1,1],eigvals_only=True)[0]
 expected=min((N+1)*k,k+d*min(a,b)); assert abs(gap-expected)<1e-10
 print(a,b,B,format(gap,'.12g'))
# Jacobi frame cycle and Lie bracket identity
D=np.array([[-1,-np.sqrt(3)],[np.sqrt(3),-1]])/2
assert np.allclose(D@D@D,np.eye(2)) and np.allclose(D.T@D,np.eye(2))
A=np.array([[0,1],[-1,0]]); C=np.diag([1,-1]); E=np.array([[0,1],[1,0]])
br=lambda x,y:x@y-y@x
assert np.array_equal(br(A,br(C,E))+br(C,br(E,A))+br(E,br(A,C)),np.zeros((2,2)))
print('PASS: 4 gap cases, shell cycle, bracket identity')
