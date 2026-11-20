import sympy as s
from itertools import product, combinations
import json
from pathlib import Path
I=s.I
E=lambda a,b:s.eye(3)[:,a]*s.eye(3)[b,:]
S=[]
for a,b in combinations(range(3),2):
 S.extend([E(a,b)+E(b,a),-I*(E(a,b)-E(b,a))])
H=[E(0,0)-E(1,1), E(0,0)+E(1,1)-2*E(2,2)]
B=S+H
V=lambda M:s.Matrix(list(M))
M=s.Matrix.hstack(*map(V,B))
assert M.rank()==8 and s.Matrix.hstack(M,V(s.eye(3))).rank()==9
br=lambda a,b:s.simplify((a*b-b*a)/(2*I))
C=[br(a,b) for a,b in combinations(S,2)]
assert s.Matrix.hstack(*map(V,S+C)).rank()==8
assert all(s.trace(a)==0 and a.H==a for a in B)
for a,b in product(B,repeat=2):
 c=br(a,b); coeff=M.gauss_jordan_solve(V(c))[0];assert M*coeff==V(c) and all(x.is_real for x in coeff)
assert br(S[0],S[1])==H[0]
assert br(S[2],S[3])==(H[0]+H[1])/2
assert br(S[4],S[5])==(-H[0]+H[1])/2
a=S[0]+S[1];b=S[0]-S[1]
assert br(a,b)==-2*H[0]
checks=0
for v in product((-1,0,1),repeat=6):
 Q=sum((x*y for x,y in zip(v,S)),s.zeros(3)); assert Q.H==Q and s.trace(Q)==0
 assert M.gauss_jordan_solve(V(Q))[0]==s.Matrix([*v,0,0]);checks+=1
out=dict(ternary_vectors=checks,bracket_checks=64,seed_rank=6,generated_rank=8,identity_extended_rank=9,cross_shadow_bracket_nonzero=True,physical_closure=False)
print(json.dumps(out,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2))
