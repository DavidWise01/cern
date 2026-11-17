from itertools import product, combinations
from collections import Counter
V=[(b,*s) for b in range(3) for s in product((-1,1),repeat=3)]
def embed(v):
 b,a,c,d=v
 if b==0: return (a,c,d,a*c*d)
 if b==1: return (a,c,d,-a*c*d)
 i=2*(c==1)+(d==1)
 return tuple(2*a if j==i else 0 for j in range(4))
P={v:embed(v) for v in V}
dot=lambda p,q:sum(a*b for a,b in zip(p,q))
assert len(set(P.values()))==24
assert all(dot(p,p)==4 for p in P.values())
def opposite(v):
 b,a,c,d=v
 return (b,-a,-c,-d) if b<2 else (b,-a,c,d)
assert all(P[opposite(v)]==tuple(-x for x in P[v]) for v in V)
E={frozenset((v,w)) for v,w in combinations(V,2) if dot(P[v],P[w])==2}
assert len(E)==96
assert all(sum(v in e for e in E)==8 for v in V)
F=[t for t in combinations(V,3) if all(frozenset(e) in E for e in combinations(t,2))]
assert len(F)==96
C=[]
for i,j in combinations(range(4),2):
 for a,b in product((-1,1),repeat=2):
  cell=[v for v,p in P.items() if a*p[i]+b*p[j]==2]
  assert all(a*p[i]+b*p[j]<=2 for p in P.values())
  assert len(cell)==6 and sum(e<=set(cell) for e in E)==12
  C.append(cell)
assert len({frozenset(c) for c in C})==24
assert all(sum(set(f)<=set(c) for c in C)==2 for f in F)
shadow=lambda p:p[:3]
assert len({shadow(p) for p in P.values()})==15
assert len({(shadow(p),(-p[3],p[1],p[2])) for p in P.values()})==24
print('PASS: 24 vertices, 96 edges, 96 triangles, 24 octahedra; 15 shadows, 24 paired shadows')
print('Edges by band:',dict(Counter(tuple(sorted(v[0] for v in e)) for e in E)))
