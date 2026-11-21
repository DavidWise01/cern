from fractions import Fraction as Q
from pathlib import Path
import json
J=Q(1,50)
def kinetic(L,P):return L*L+P*P
def advance(state,mask):
 E,L,P,n=state
 dL=J*(int(bool(mask&1))+int(bool(mask&2)))
 dP=J*(int(bool(mask&1))-int(bool(mask&2)))
 need=kinetic(L+dL,P+dP)-kinetic(L,P)
 if not mask or need>E:return state,False
 return (E-need,L+dL,P+dP,n+1),True
counts=dict(states=0,updates=0,fired_steps=0,conservation_failures=0,two_open_zero_net_force=0,one_open_thrust=0,closed_no_rotation=0)
for i in range(13):
 for mask in range(4):
  state=(Q(i,10),Q(0),Q(0),0);initial=state[0];counts['states']+=1
  for _ in range(12):
   state,fired=advance(state,mask)
   E,L,P,_=state
   assert E>=0 and E+kinetic(L,P)==initial
   assert L+(-L)==0 and P+(-P)==0
   if mask==3:
    assert P==0
    counts['two_open_zero_net_force']+=int(fired)
   if mask in (1,2):counts['one_open_thrust']+=int(fired)
   if mask==0:
    assert L==P==0
    counts['closed_no_rotation']+=1
   counts['updates']+=1;counts['fired_steps']+=int(fired)
assert counts==dict(states=52,updates=624,fired_steps=424,conservation_failures=0,two_open_zero_net_force=138,one_open_thrust=286,closed_no_rotation=156)
report={'model':'H167 Rupert toroid impulse model','counts':counts,'normalizations':{'R':'1','J':'1/50','inertia rotor and housing':'1','mass rotor and housing':'1'},'conservation':['fuel+L²+P²','rotor L+housing L=0','rotor P+housing P=0'],'limitations':['toy energy and impulse ledger','no measured material fragmentation or nozzle mechanism','not a self-powering device','not a Big Bang derivation']}
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
