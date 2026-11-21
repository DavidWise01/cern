from fractions import Fraction as F
from pathlib import Path
import json
J=F(1,20);R=F(1);M=F(1);I=F(2);EM=F(1)
def kinetic(L,P):return L*L/(2*I)+P*P/(2*M)
def advance(st,mask):
 fuel,L,P,Le,Pe,Ke,shots=st
 n=int(bool(mask&1))+int(bool(mask&2))
 dL=-R*J*n;dP=-J*(int(bool(mask&1))-int(bool(mask&2)))
 extra=F(n)*J*J/(2*EM)
 cost=kinetic(L+dL,P+dP)-kinetic(L,P)+extra
 if not n or fuel<cost:return st,False
 return fuel-cost,L+dL,P+dP,Le-dL,Pe-dP,Ke+extra,shots+1,True
counts=dict(states=0,updates=0,fired=0,two_port=0,one_port=0,closed=0,energy_failures=0,momentum_failures=0)
for k in range(13):
 for mask in range(4):
  initial=F(k,10);state=(initial,F(0),F(0),F(0),F(0),F(0),0)
  counts['states']+=1
  for t in range(12):
   state,fired=advance(state,mask)
   fuel,L,P,Le,Pe,Ke,_=state
   assert fuel>=0 and fuel+kinetic(L,P)+Ke==initial
   assert L+Le==0 and P+Pe==0
   if mask==3:assert P==0
   if mask==0:assert L==P==0
   counts['updates']+=1;counts['fired']+=int(fired)
   if fired:counts['two_port' if mask==3 else 'one_port']+=1
   if mask==0:counts['closed']+=1
assert counts==dict(states=52,updates=624,fired=403,two_port=131,one_port=272,closed=156,energy_failures=0,momentum_failures=0)
print(json.dumps(counts,indent=2))
