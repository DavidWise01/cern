from fractions import Fraction as Q
from pathlib import Path
import json,math
# Three-region phenomenological POD: conserved total penetration.
def advance(a,b,x):
 return ((3*a+b)/4,(a+3*b)/4,(x+a-b)/4)
def E(a,b,x):return (a-b)**2+x*x
checks=0; states=0; equilibrium=0
for ia in range(11):
 for ib in range(11-ia):
  for ix in range(-10,11):
   a,b,x=Q(ia,10),Q(ib,10),Q(ix,10)
   S=a+b;states+=1
   for _ in range(12):
    old=E(a,b,x);da=a-b
    aa,bb,xx=advance(a,b,x)
    assert aa+bb==S and aa-bb==da/2
    assert Q(0)<=aa<=1 and Q(0)<=bb<=1 and abs(xx)<=1
    assert E(aa,bb,xx)<=Q(3,8)*old
    a,b,x=aa,bb,xx;checks+=1
   if ia==ib and ix==0:equilibrium+=1
   assert abs(a-b)<=Q(1,2**12)
assert states==1386 and checks==16632 and equilibrium==6
def status(p_intact,d_intact):
 return 'contained' if p_intact and d_intact else 'ruptured'
assert status(True,True)=='contained'
assert all(status(*v)=='ruptured' for v in [(False,True),(True,False),(False,False)])
report=dict(exact_states=states,checks=checks,equilibrium_initial_states=equilibrium,contraction_bound='3/8',length_m_approx=1.616255e-35/(5*math.pi),tested_physical_model=False)
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
