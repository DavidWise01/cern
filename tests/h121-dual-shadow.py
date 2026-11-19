from itertools import product, combinations_with_replacement
import sympy as s
A=s.Matrix([[1,0,0,0],[0,1,0,0],[0,0,1,0]])
B=s.Matrix([[0,1,0,0],[0,0,1,0],[0,0,0,1]])
P=A.col_join(B)
assert P.rank()==4
assert P.T*P==s.diag(1,2,2,1)
points=list(product(range(-2,3),repeat=4))
seen=set()
for w,x,y,z in points:
 a=(w,x,y); b=(x,y,z)
 assert (a[0],a[1],a[2],b[2])==(w,x,y,z)
 seen.add((a,b))
assert len(seen)==625
basis=[]
for i,j in combinations_with_replacement(range(4),2):
 q=s.zeros(4);q[i,j]=q[j,i]=1;basis.append(q)
vec=lambda q:list(q)
T=s.Matrix.hstack(*(s.Matrix(vec(A*q*A.T)+vec(B*q*B.T)) for q in basis))
assert T.rank()==9
assert T.nullspace()[0]==s.eye(10)[:,3]
C=s.Matrix([[1,0,0,1],[0,1,0,0],[0,0,1,0]])
U=T.col_join(s.Matrix.hstack(*(s.Matrix([ (C*q*C.T)[0,0]]) for q in basis)))
assert U.rank()==10
print('PASS: 625 points; stacked rank 4; quadratic rank 9; mixed observation rank 10')
