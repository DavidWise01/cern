from itertools import combinations, product
from pathlib import Path
import json
import sympy as s
P=list(combinations(range(4),2)); E=s.eye(6)
# Hodge star in oriented Euclidean R4: wx <-> yz, wy <-> -xz, wz <-> xy.
S=s.zeros(6)
for i,j,k in [(0,5,1),(1,4,-1),(2,3,1)]: S[i,j]=S[j,i]=k
assert S*S==E
plus=(E+S)/2; minus=(E-S)/2
assert plus.rank()==minus.rank()==3 and plus*minus==s.zeros(6)
assert plus+minus==E
checks=0
for v in product((-1,0,1),repeat=6):
 v=s.Matrix(v); a=plus*v;b=minus*v
 assert a+b==v and S*a==a and S*b==-b and a.dot(b)==0
 checks+=1
# Nonrelativistic complete subshell has 2(2l+1) states.
assert all(2*(2*l+1)!=12 for l in range(100))
assert 2*(2*0+1)+2*(2*2+1)==12
assert 2*(2*1+1)*2==12
assert 2*(s.Rational(11,2))+1==12
out=dict(planes=['wx','wy','wz','xy','xz','yz'],hodge_ranks=[3,3],vector_tests=checks,added_electrons=12,target=172+12,physical_closure_established=False)
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
