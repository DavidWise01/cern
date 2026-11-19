from fractions import Fraction as F
from random import Random
from math import lcm
import json
from pathlib import Path

def fixture(n,rng):
 return [(rng.randrange(6),rng.randrange(1,7),F(rng.randrange(1,9),100),rng.randrange(1,11)) for _ in range(n)]
def data(items):
 a=F(9,10); x=F(1,4)
 M=[[sum((a**(k-1-t)*f for t in range(k) if s<=t<s+l),F(0)) for s,l,f,w in items] for k in range(1,13)]
 b=[1-a**k*x for k in range(1,13)]
 scale=lcm(*(v.denominator for row in M for v in row),*(v.denominator for v in b))
 return [[int(v*scale) for v in row] for row in M],[int(v*scale) for v in b]
def solve(items,tight):
 M,b=data(items); w=[i[3] for i in items]; n=len(w); best=-1; winner=None; cert=[]; nodes=0
 def dfs(p,loads,v):
  nonlocal best,winner,nodes
  nodes+=1;j=len(p)
  for k in range(12):
   if loads[k]>b[k]:cert.append([p,'unsafe',k]);return
  if v+sum(w[j:])<=best:cert.append([p,'value',0]);return
  if tight and best>=0:
   for k,row in enumerate(M):
    for i in range(j,n):
     if not row[i]:continue
     y=F(w[i],row[i]); ub=v+y*(b[k]-loads[k])+sum(max(0,w[h]-y*row[h]) for h in range(j,n))
     if ub<best+1:
      cert.append([p,'dual',[k,str(y)]]);return
  if j==n:best=v;winner=p;cert.append([p,'leaf',0]);return
  dfs(p+[1],[u+row[j] for u,row in zip(loads,M)],v+w[j]);dfs(p+[0],loads,v)
 dfs([],[0]*12,0)
 return best,winner,cert,nodes

def verify(items,best,winner,cert):
 assert all(f>=0 and w>=0 for s,l,f,w in items)
 n=len(items);w=[v[3] for v in items];M,b=data(items)
 def trajectory(p):
  x=F(1,4);out=[]
  for t in range(12):
   x=F(9,10)*x+sum(f*z for (s,l,f,w),z in zip(items,p) if s<=t<s+l);out.append(x)
  return out
 assert len(winner)==n and set(winner)<={0,1} and max(trajectory(winner))<=1 and sum(z*v for z,v in zip(winner,w))==best
 records={tuple(p):(kind,k) for p,kind,k in cert};assert len(records)==len(cert);used=set()
 def walk(p):
  if p not in records:
   assert len(p)<n;walk(p+(0,));walk(p+(1,));return
  used.add(p);kind,k=records[p];v=sum(z*q for z,q in zip(p,w));j=len(p)
  if kind=='unsafe':assert 0<=k<12 and trajectory(p)[k]>1
  elif kind=='value':assert v+sum(w[j:])<=best
  elif kind=='leaf':assert j==n and v<=best
  elif kind=='dual':
   k,y=k;y=F(y);assert 0<=k<12 and y>=0
   row=M[k];remaining=b[k]-sum(z*q for z,q in zip(p,row))
   assert v+y*remaining+sum(max(0,w[h]-y*row[h]) for h in range(j,n))<best+1
  else:raise AssertionError(kind)
 walk(());assert used==set(records)
 assert sum(2**(n-len(p)) for p in records)==2**n

def main():
 rng=Random(115);small=[fixture(8,rng) for _ in range(32)];large=[(0,6,F(1,10),24-i) for i in range(24)]
 stats={'cases':33,'small_baseline_nodes':0,'small_tight_nodes':0,'dual_certificates':0,'tamper_rejected':0}
 for idx,p in enumerate(small+[large]):
  a=solve(p,False);b=solve(p,True);assert a[0]==b[0];verify(p,*a[:3]);verify(p,*b[:3]);stats['dual_certificates']+=sum(k=='dual' for _,k,_ in b[2])
  try:verify(p,b[0],b[1],b[2][:-1])
  except AssertionError:stats['tamper_rejected']+=1
  else:raise AssertionError('missing certificate accepted')
  if idx<32:stats['small_baseline_nodes']+=a[3];stats['small_tight_nodes']+=b[3]
  else:
   stats['large']={'baseline_nodes':a[3],'tight_nodes':b[3],'baseline_certificates':len(a[2]),'tight_certificates':len(b[2]),'value':b[0],'covered':2**24}
   Path(__file__).with_name('certificate.json').write_text(json.dumps({'items':[[s,l,str(f),w] for s,l,f,w in p],'best':b[0],'winner':b[1],'certificate':b[2]},indent=2))
 print(json.dumps(stats,indent=2));Path(__file__).with_name('results.json').write_text(json.dumps(stats,indent=2))
if __name__=='__main__':main()
