from itertools import product, combinations
from collections import Counter
import random
V=[(a,b,c,d) for a,b,c,d in product((-1,1),repeat=4)]+[tuple(s*2 if j==i else 0 for j in range(4)) for i in range(4) for s in (-1,1)]
band=lambda v: 2 if 0 in v else int(v[0]*v[1]*v[2]*v[3]<0)
E=[(i,j) for i,j in combinations(range(24),2) if sum(a*b for a,b in zip(V[i],V[j]))==2]
F=[(i,j,k) for i,j,k in combinations(range(24),3) if all(e in E for e in [(i,j),(j,k),(i,k)])]
assert (len(E),len(F))==(96,96)
assert all(len({band(V[i]) for i in f})==3 for f in F)
Q=(-4,-3,-2,-1,1,2,3,4)
T=((1,2,3,4),(2,-1,4,-3),(3,-4,-1,2),(4,3,-2,-1))
def mul(a,b):return (1 if a*b>0 else -1)*T[abs(a)-1][abs(b)-1]
def inv(a):return a if abs(a)==1 else -a
def chain(*xs):
 r=1
 for x in xs:r=mul(r,x)
 return r
def link(U,i,j):return U[i,j] if i<j else inv(U[j,i])
def hol(U,i,j,k):return chain(link(U,i,j),link(U,j,k),link(U,k,i))
def action(q):return 1-q if abs(q)==1 else 1
rng=random.Random(83); histogram=Counter(); checks=0
for _ in range(128):
 U={e:rng.choice(Q) for e in E};g=[rng.choice(Q) for v in V]
 W={(i,j):chain(g[i],u,inv(g[j])) for (i,j),u in U.items()}
 for i,j,k in F:
  p=hol(U,i,j,k);q=hol(W,i,j,k)
  assert q==chain(g[i],p,inv(g[i]))
  assert action(p)==action(q)
  histogram[action(p)]+=1;checks+=1
print('PASS',checks,dict(sorted(histogram.items())))
