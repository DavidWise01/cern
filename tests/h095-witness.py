from itertools import product
from collections import Counter
from fractions import Fraction as F
Q=(1,2,3,4,-1,-2,-3,-4)
T=((1,2,3,4),(2,-1,4,-3),(3,-4,-1,2),(4,3,-2,-1))
def mul(a,b):return (1 if a*b>0 else -1)*T[abs(a)-1][abs(b)-1]
def vec(q):return tuple((1 if q>0 else -1) if j==abs(q)-1 else 0 for j in range(4))
def hopf(p):
 a,b,c,d=p;return (2*(a*c+b*d),2*(b*c-a*d),a*a+b*b-c*c-d*d)
def phase(p):
 a,b,c,d=p;return (0,a,b) if a*a+b*b>=c*c+d*d else (1,c,d)
def reconstruct(n,w):
 x,y,z=n;j,a,b=w;r=a*a+b*b
 if not r or j not in (0,1):return None
 if j==0:return (a,b,F(a*x+b*y,2*r),F(b*x-a*y,2*r))
 return (F(a*x-b*y,2*r),F(b*x+a*y,2*r),a,b)
ROOT={'G':'rG','P':'rP','L':'rL','PM':'rG'}
BASE=vec(1); FRAME='frame-A'; ID='occupant-1'
def packets(p,k):
 ctx=(ID,FRAME,k,0)
 return [('G',ctx,hopf(p)),('P',ctx,phase(p)),('L',ctx,ID)]
def gate(ps,k):
 ctx=(ID,FRAME,k,0)
 if len(ps)!=3 or any(p[1]!=ctx for p in ps):return 'CONTEXT'
 if len({ROOT.get(p[0]) for p in ps})!=3:return 'ROOTS'
 data={p[0]:p[2] for p in ps}
 if set(data)!={'G','P','L'} or data['L']!=ID:return 'LINEAGE'
 p=reconstruct(data['G'],data['P'])
 if p is None or sum(x*x for x in p)!=1 or hopf(p)!=data['G'] or phase(p)!=data['P']:return 'INCONSISTENT'
 return 'RETURN' if p==BASE else 'NOT_RETURN'
C=Counter();checks=0
for a,b,c in product(Q,repeat=3):
 h=mul(c,mul(b,a));q=1
 for k in range(1,5):
  q=mul(h,q);p=vec(q);ps=packets(p,k)
  v=gate(ps,k);assert (v=='RETURN')==(q==1);C[v]+=1;checks+=1
  bad=ps.copy();bad[1]=('PM',ps[1][1],ps[1][2]);assert gate(bad,k)=='ROOTS'
  assert gate(ps,k+1)=='CONTEXT'
  bad=ps.copy();bad[2]=('L',ps[2][1],'other');assert gate(bad,k)=='LINEAGE'
  bad=ps.copy();bad[1]=('P',ps[1][1],(0,0,0));assert gate(bad,k)=='INCONSISTENT'
  checks+=4
# An internally consistent sign forgery defeats the three typed witnesses.
p=packets(vec(-1),1);p[1]=('P',p[1][1],phase(BASE));assert gate(p,1)=='RETURN'
print(dict(observations=sum(C.values()),returns=C['RETURN'],not_returns=C['NOT_RETURN'],checks=checks,consistent_phase_forgery='FALSE_ACCEPT'))
