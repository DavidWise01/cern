from itertools import product, combinations
from pathlib import Path
from collections import Counter
import json
seed=(1,3,6,8,9)
def path(v):
 q=seed
 if v&1:q=tuple((x-1)//3*3+3-(x-1)%3 for x in q)
 if v&2:q=tuple(-x for x in q)
 if v&4:q=tuple((1 if x>0 else -1)*(10-abs(x)) for x in q)
 if v&8:q=q[::-1]
 return q
V=list(range(16)); E=[(v,v^(1<<i)) for v in V for i in range(4) if not v&(1<<i)]
F=[(v,v^(1<<i),v^(1<<i)^(1<<j),v^(1<<j)) for i,j in combinations(range(4),2) for v in V if not v&((1<<i)|(1<<j))]
assert len(set(map(path,V)))==16 and len(E)==32 and len(F)==24
edges={frozenset(e) for e in E}; incid=Counter()
for f in F:
 assert len(set(f))==4
 for a,b in zip(f,f[1:]+f[:1]):
  e=frozenset((a,b));assert e in edges;incid[e]+=1
 assert f[0]^f[1]^f[2]^f[3]==0
assert set(incid.values())=={3} and len(incid)==32
from itertools import permutations
for v in V:
 for order in permutations(range(4)):
  q=v
  for i in order:q^=1<<i
  assert q==v^15
spectrum=Counter()
for mask in V:
 f=lambda v:(-1)**((v&mask).bit_count())
 lam=2*mask.bit_count()
 for v in V:assert 4*f(v)-sum(f(v^(1<<i)) for i in range(4))==lam*f(v)
 spectrum[lam]+=1
out=dict(vertices=16,edges=32,square_faces=24,cubic_cells=8,edge_face_incidence=3,antipodal_routes=384,walsh_checks=256,spectrum=dict(sorted(spectrum.items())),paths={str(v):path(v) for v in V})
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2));print(json.dumps(out))
