from fractions import Fraction as Q
from pathlib import Path
import json
J=Q(1,20);IR=IH=Q(2)
LOADS=(Q(0),Q(1,16),Q(1,8),Q(1,4))
DRAGS=(Q(0),Q(1,32),Q(1,16),Q(1,8))
def kr(l): return l*l/(2*IR)
def kh(l): return l*l/(2*IH)
def run(initial,load,drag,recharge):
 fuel,lr,lh,le,work,heat,ejecta,supplied=initial,Q(0),Q(0),Q(0),Q(0),Q(0),Q(0),initial
 charges=work_steps=heat_steps=0
 for n in range(16):
  if recharge and n==8:
   fuel+=Q(1,4);supplied+=Q(1,4)
  if n<10:
   increment=kr(lr+J)+kh(lh-J)-kr(lr)-kh(lh)
   exhaust=increment/4
   if fuel>=increment+exhaust:
    fuel-=increment+exhaust;ejecta+=exhaust;lr+=J;lh-=J;charges+=1
  if load:
   old=kr(lr);removed=lr*load;lr-=removed;le+=removed
   delivered=old-kr(lr)
   assert delivered>=0
   work+=Q(3,4)*delivered;heat+=Q(1,4)*delivered
   work_steps+=int(delivered>0)
  if drag:
   old=kr(lr)+kh(lh)
   transfer=lr*drag
   lr-=transfer;lh+=transfer
   dissipation=old-kr(lr)-kh(lh)
   assert dissipation>=0
   heat+=dissipation;heat_steps+=int(dissipation>0)
  assert fuel>=0 and lr+lh+le==0
  assert fuel+kr(lr)+kh(lh)+work+heat+ejecta==supplied
  assert work<=supplied
 return dict(fuel=fuel,rotor=kr(lr),housing=kh(lh),work=work,heat=heat,ejecta=ejecta,supplied=supplied,charges=charges,work_steps=work_steps,heat_steps=heat_steps,eff=work/supplied if supplied else Q(0))
records=[]
for k in range(11):
 for load in LOADS:
  for drag in DRAGS:
   for recharge in (False,True):
    x=run(Q(k,10),load,drag,recharge)
    records.append((k,load,drag,recharge,x))
assert len(records)==352
assert all(x['work']==0 for k,l,d,r,x in records if not l)
assert all(x['work']==0 for k,l,d,r,x in records if k==0 and not r)
assert all(x['eff']<=1 for _,_,_,_,x in records)
counts=dict(initial_configurations=len(records),exact_updates=len(records)*16,charging_impulses=sum(x['charges'] for *_,x in records),work_events=sum(x['work_steps'] for *_,x in records),drag_events=sum(x['heat_steps'] for *_,x in records),recharged_runs=sum(int(r) for _,_,_,r,_ in records),conservation_failures=0)
assert counts==dict(initial_configurations=352,exact_updates=5632,charging_impulses=3212,work_events=3936,drag_events=3936,recharged_runs=176,conservation_failures=0)
points=[]
for load in LOADS:
 for drag in DRAGS:
  x=run(Q(1),load,drag,False)
  points.append(dict(load=str(load),drag=str(drag),net_efficiency=str(x['eff']),net_efficiency_float=round(float(x['eff']),6),work=str(x['work']),heat=str(x['heat']),exhaust=str(x['ejecta']),remaining_rotor_energy=str(x['rotor'])))
best=max(points,key=lambda x:x['net_efficiency_float'])
assert best['load']=='1/8' and best['drag']=='0'
report=dict(version='H170',model='torque delivery gate: exact finite energy and angular momentum ledger',counts=counts,operating_points=points,best_tested=best,scope='synthetic model only; no real Rupert turbine')
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(dict(counts=counts,best_tested=best),indent=2))
