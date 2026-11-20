from fractions import Fraction as F
from itertools import product
import json
from pathlib import Path

eps=F(1,100)
worst=F(0); cases=0
for a,b,q,bias in product(range(-2,3),repeat=4):
    for ep,em in product([-eps, F(0),eps],repeat=2):
        plus=(F(a+b,2)+q+bias+ep)
        minus=(F(a+b,2)-q+bias+em)
        recovered=(plus-minus)/2
        error=abs(recovered-q)
        assert error<=eps
        worst=max(worst,error);cases+=1
assert worst==eps
# Non-simultaneous sampling: each direction has unit norm.
# A drift of d I between acquisitions attains an additional d/2 error.
delta=F(1,50); async_worst=F(0); async_cases=0
for ep,em,d in product([-eps,eps],[-eps,eps],[-delta,delta]):
    error=abs((ep-em-d)/2)
    assert error<=eps+delta/2
    async_worst=max(async_worst,error);async_cases+=1
assert async_worst==eps+delta/2
# Two orthogonal directions alone reveal only their sum of diagonal terms
# and coupling. Full tensor restrictions to two 3D hyperplanes still rank 9.
import sympy as s
from itertools import combinations_with_replacement
I=list(combinations_with_replacement(range(4),2)); J=list(combinations_with_replacement(range(3),2))
A=s.Matrix([[1,0,0,1],[0,1,0,0],[0,0,1,0]])
B=s.Matrix([[1,0,0,-1],[0,1,0,0],[0,0,1,0]])
C=s.eye(4)[:3,:]
basis=[]
for i,j in I:
    q=s.zeros(4);q[i,j]=q[j,i]=1;basis.append(q)
def obs(*ps):
    return s.Matrix.hstack(*[s.Matrix([(p*q*p.T)[i,j] for p in ps for i,j in J]) for q in basis])
T=obs(A,B); assert T.rank()==9
assert obs(A,B,C).rank()==10
hidden=s.diag(1,0,0,-1)
assert A*hidden*A.T==B*hidden*B.T==s.zeros(3)
out=dict(simultaneous_cases=cases,asynchronous_cases=async_cases,epsilon=str(eps),delta=str(delta),worst_simultaneous=str(worst),worst_asynchronous=str(async_worst),two_shadow_rank=9,historical_rank=10,hidden_mode='q_ww-q_zz')
print(json.dumps(out,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n')
