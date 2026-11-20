import sympy as s
from random import Random
from itertools import combinations_with_replacement
import json
from pathlib import Path
I=list(combinations_with_replacement(range(4),2))
J=list(combinations_with_replacement(range(3),2))
B=[]
for i,j in I:
    q=s.zeros(4);q[i,j]=q[j,i]=1;B.append(q)
def observation(A,C):
    return s.Matrix.hstack(*[s.Matrix([(P*q*P.T)[i,j] for P in (A,C) for i,j in J]) for q in B])
def coords(q):return s.Matrix([q[i,j] for i,j in I])
rng=Random(122)
def projection():
    while True:
        p=s.Matrix(3,4,[rng.randint(-4,4) for _ in range(12)])
        if p.rank()==3:return p
stats={'distinct_pairs':0,'equal_plane_pairs':0,'rank9':0,'rank6':0,'mixed_rank10':0,'kernel_checks':0}
for _ in range(64):
    A=projection();a=A.nullspace()[0]
    while True:
        C=projection();b=C.nullspace()[0]
        if s.Matrix.hstack(a,b).rank()==2:break
    T=observation(A,C);q=a*b.T+b*a.T
    assert T.rank()==9 and T*coords(q)==s.zeros(12,1) and q!=s.zeros(4)
    v=next(v for t in range(7) if (v:=s.Matrix([1,t,t*t,t**3])).dot(a)!=0 and v.dot(b)!=0)
    row=s.Matrix([[ (v.T*r*v)[0] for r in B]])
    assert T.col_join(row).rank()==10
    stats['distinct_pairs']+=1;stats['rank9']+=1;stats['mixed_rank10']+=1;stats['kernel_checks']+=1
for _ in range(16):
    A=projection();C=s.Matrix([[1,2,0],[0,1,1],[0,0,1]])*A
    assert observation(A,C).rank()==6
    stats['equal_plane_pairs']+=1;stats['rank6']+=1
print(json.dumps(stats,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(stats,indent=2)+'\n')
