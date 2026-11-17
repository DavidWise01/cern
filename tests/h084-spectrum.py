from itertools import product
import sympy as s
V=list(product((-1,1),repeat=4))+[tuple(2*a if i==j else 0 for i in range(4)) for j in range(4) for a in (-1,1)]
A=s.Matrix(24,24,lambda i,j:int(sum(a*b for a,b in zip(V[i],V[j]))==2))
L=8*s.eye(24)-A
assert A* s.ones(24,1)==8*s.ones(24,1)
print('spectrum',L.eigenvals())
print('charpoly',s.factor(L.charpoly().as_expr()))
B=s.Matrix(24,3,lambda i,j:int(j==(2 if 0 in V[i] else int(s.prod(V[i])<0))))
Q=s.Matrix([[8,-4,-4],[-4,8,-4],[-4,-4,8]])
assert L*B==B*Q
assert L.eigenvals()=={0:1,4:4,8:9,10:8,12:2}
print('PASS; band spectrum',Q.eigenvals())
