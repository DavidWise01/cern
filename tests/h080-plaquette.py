from itertools import product
Q=tuple(range(1,5))+tuple(range(-1,-5,-1))
T=((1,2,3,4),(2,-1,4,-3),(3,-4,-1,2),(4,3,-2,-1))
def mul(a,b): return (1 if a*b>0 else -1)*T[abs(a)-1][abs(b)-1]
def inv(a): return a if abs(a)==1 else -a
def chain(xs):
 r=1
 for x in xs:r=mul(r,x)
 return r
def trace(a):return 2*a if abs(a)==1 else 0
def check(links,g):
 p=chain(links)
 q=chain([chain((g[i],links[i],inv(g[(i+1)%4]))) for i in range(4)])
 assert q==chain((g[0],p,inv(g[0]))) and trace(q)==trace(p)
for a,b,c in product(Q,repeat=3):assert mul(mul(a,b),c)==mul(a,mul(b,c))
for a in Q: assert mul(a,inv(a))==1
n=0
for links in product(Q,repeat=4):
 for a,b in product(Q,repeat=2):
  check(links,(a,b,1,1));n+=1
for g in product(Q,repeat=4):
 check((2,3,-2,-3),g);n+=1
print({'status':'PASS','group':'Q8','checks':n,'nontrivial_loop':chain((2,3,-2,-3))})
