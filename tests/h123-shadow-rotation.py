import sympy as s
from random import Random
from itertools import combinations_with_replacement
import json
from pathlib import Path
I=list(combinations_with_replacement(range(4),2)); J=list(combinations_with_replacement(range(3),2))
B=[]
for i,j in I:
 q=s.zeros(4);q[i,j]=q[j,i]=1;B.append(q)
def obs(*ps):
 return s.Matrix.hstack(*[s.Matrix([(p*q*p.T)[i,j] for p in ps for i,j in J]) for q in B])
rng=Random(123)
A=s.eye(4)[:3,:]; C=s.eye(4)[1:,:]
T=obs(A,C)
assert T.rank()==9
stats=dict(rotations=0,full_rank=0,reconstructions=0,unchanged_rank9=0,swap_rank9=0)
for _ in range(64):
 t=s.Rational(rng.randint(1,10),rng.randint(1,10)); c=(1-t*t)/(1+t*t); d=2*t/(1+t*t)
 # avoid quarter-turn: it makes rotated hidden direction coincide with fixed observer
 if c==0: t=s.Rational(1,2);c=s.Rational(3,5);d=s.Rational(4,5)
 R=s.eye(4); R[0,0]=R[3,3]=c;R[0,3]=-d;R[3,0]=d
 assert R.T*R==s.eye(4) and R.det()==1
 U=obs(A,C*R);assert U.rank()==9
 V=T.col_join(U);assert V.rank()==10
 q=s.Matrix([rng.randint(-20,20) for _ in I]); recovered=V.gauss_jordan_solve(V*q)[0];assert recovered==q
 stats['rotations']+=1;stats['full_rank']+=1;stats['reconstructions']+=1
 assert T.col_join(T).rank()==9;stats['unchanged_rank9']+=1
 assert T.col_join(obs(C,A)).rank()==9;stats['swap_rank9']+=1
print(json.dumps(stats,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(stats,indent=2)+'\n')
