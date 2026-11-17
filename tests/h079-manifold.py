from itertools import product, combinations
P=list(product(range(-2,3),repeat=4))
def rot(p,i,j):
 q=list(p); q[i],q[j]=-p[j],p[i]; return tuple(q)
for i,j in combinations(range(4),2):
 for p in P:
  q=p
  for _ in range(4): q=rot(q,i,j)
  assert q==p and sum(x*x for x in rot(p,i,j))==sum(x*x for x in p)
 count=len({(p[:3],rot(p,i,j)[:3]) for p in P})
 assert count==(625 if j==3 else 125)
 print(i,j,count)
print('PASS',len(P)*6)
