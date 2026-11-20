import itertools as it, json
from functools import lru_cache
from pathlib import Path
import sympy as s
planes=list(it.combinations(range(4),2)); names=['wx','wy','wz','xy','xz','yz']
I=list(it.combinations_with_replacement(range(4),2)); J=list(it.combinations_with_replacement(range(3),2))
basis=[]
for i,j in I:
 q=s.zeros(4);q[i,j]=q[j,i]=1;basis.append(q)
E=s.eye(4)
def obs(A,B):
 return s.Matrix.hstack(*[s.Matrix([(P*q*P.T)[i,j] for P in (A,B) for i,j in J]) for q in basis])
for a,b in planes:
 A=E[[i for i in range(4) if i!=a],:];B=E[[i for i in range(4) if i!=b],:]
 T=obs(A,B);assert T.rank()==9 and T.nullspace()[0]==s.eye(10)[:,I.index((a,b))]
 common=[i for i in range(4) if i not in (a,b)]
 C=s.Matrix.vstack(E[a,:]+E[b,:],*(E[i,:] for i in common));D=s.Matrix.vstack(E[a,:]-E[b,:],*(E[i,:] for i in common))
 U=obs(C,D);h=s.zeros(4);h[a,a]=1;h[b,b]=-1
 assert U.rank()==9 and U*s.Matrix([h[i,j] for i,j in I])==s.zeros(12,1)
 assert T.col_join(U).rank()==10
# Units of 0.001, coordinatewise drift; all nine visible coefficients reset to eps.
eps=10; drift=(30,18,12,8,5,2); H=24; switch=12; cap=40
r0=(eps,)*6
def step(r,i):return tuple(r[j]+drift[j] if j==i else eps for j in range(6))
def score(seq):
 r=r0;last=-1;cost=0;peak=0;total=0;changes=0
 for i in seq:
  r=step(r,i);m=max(r);peak=max(peak,m);total+=m;changes+=int(last>=0 and i!=last);last=i
  assert m<=cap
 return dict(peak=peak,mean=total/len(seq),switches=changes,cost=total+switch*changes,schedule=' '.join(names[i] for i in seq))
def greedy():
 r=r0;last=-1;seq=[]
 for _ in range(H):
  i=min((i for i in range(6) if max(step(r,i))<=cap),key=lambda i:(max(step(r,i))+switch*int(last>=0 and i!=last),i));seq.append(i);r=step(r,i);last=i
 return seq
@lru_cache(None)
def dp(k,r,last):
 if not k:return 0,()
 opts=[]
 for i in range(6):
  nxt=step(r,i);m=max(nxt)
  if m<=cap:
   v,seq=dp(k-1,nxt,i);opts.append((m+switch*int(last>=0 and i!=last)+v,(i,)+seq))
 return min(opts)
value,seq=dp(H,r0,-1)
results={'geometry_pairs':6,'rotated_pairs':6,'combined_rank':10,'horizon':H,'unit':0.001,'epsilon':eps,'drift':drift,'switch_cost':switch,'cap':cap,'fixed':score(tuple(i%6 for i in range(H))),'greedy':score(greedy()),'planned':score(seq),'dp_states':dp.cache_info().currsize}
assert results['planned']['cost']==value
# Independent exhaustive short-horizon oracle.
for k in range(1,6):
 def valid(seq):
  r=r0;last=-1;cost=0
  for i in seq:
   r=step(r,i);m=max(r)
   if m>cap:return None
   cost+=m+switch*int(last>=0 and i!=last);last=i
  return cost
 oracle=min(v for seq in it.product(range(6),repeat=k) if (v:=valid(seq)) is not None)
 assert dp(k,r0,-1)[0]==oracle
results['exhaustive_sequences']=sum(6**k for k in range(1,6))
print(json.dumps(results,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(results,indent=2))
