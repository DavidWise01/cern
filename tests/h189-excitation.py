from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json
N=30; EPS=F(1,64); A=F(3,64)
C=A*A; STALE=F(1,128)
DELAYS=(0,1,2)
BASES=('flat','slow_ramp','natural_pulses')
NOISES=(-EPS,F(0),EPS)
CHANGES=(8,15,22)
POLICIES=('passive','periodic6','self_exciting')
def signal(t,probes,base):
 t=max(0,t)
 natural=sum(F(1,16) for s in (5,12,19,26) if s<t) if base=='natural_pulses' else F(0)
 ramp=F(t,128) if base=='slow_ramp' else F(0)
 return natural+ramp+sum(A for s in probes if s<t)
def predicted(t,d,probes,base):
 return signal(max(0,t-d),probes,base)
def feasible(t,y,probes,base):
 return [d for d in DELAYS if abs(y-predicted(t,d,probes,base))<=EPS]
def informative(t,probes,base):
 v=[predicted(t,d,probes,base) for d in DELAYS]
 return all(abs(v[i]-v[j])>2*EPS for i in range(3) for j in range(i+1,3))
def run(d0,d1,change,base,noise,policy):
 known=d0;probes=[];stale=0;alarms=0;false_alarms=0;first=None;false_id=0
 for t in range(N):
  actual=d0 if t<change else d1
  y=predicted(t,actual,probes,base)+noise
  residual=abs(y-predicted(t,known,probes,base))
  feasible_set=feasible(t,y,probes,base)
  assert actual in feasible_set
  is_informative=informative(t,probes,base)
  if residual>EPS:
   alarms+=1;false_alarms+=known==actual
   if len(feasible_set)==1:
    known=feasible_set[0]
    false_id+=known!=actual
    if first is None and t>=change and known==d1:first=t
  stale+=known!=actual
  if policy=='periodic6':send=t%6==0
  elif policy=='self_exciting':send=not is_informative and (not probes or t-probes[-1]>=5)
  else:send=False
  if send and t+1<N:probes.append(t)
 assert false_alarms==false_id==0
 return dict(pulses=len(probes),stale_steps=stale,detected=first is not None,
             latency=None if first is None else first-change,
             cost=len(probes)*C+stale*STALE,false_alarms=false_alarms)
def main():
 results={}
 for d0,d1,change,base,noise,policy in product(DELAYS,DELAYS,CHANGES,BASES,NOISES,POLICIES):
  if d0==d1:continue
  results[(d0,d1,change,base,noise,policy)]=run(d0,d1,change,base,noise,policy)
 assert len(results)==486
 total={}
 for p in POLICIES:
  records=[v for k,v in results.items() if k[-1]==p]
  total[p]=dict(cases=len(records),detected=sum(v['detected'] for v in records),
    pulses=sum(v['pulses'] for v in records),stale=sum(v['stale_steps'] for v in records),
    false_alarms=sum(v['false_alarms'] for v in records),
    aggregate_cost=str(sum((v['cost'] for v in records),F(0))))
 assert (total['passive']['detected'],total['periodic6']['detected'],total['self_exciting']['detected'])==(48,132,138)
 assert (total['passive']['pulses'],total['periodic6']['pulses'],total['self_exciting']['pulses'])==(0,810,972)
 assert (total['passive']['stale'],total['periodic6']['stale'],total['self_exciting']['stale'])==(1908,888,756)
 matches=[]
 for d0,d1,change,base,noise in product(DELAYS,DELAYS,CHANGES,BASES,NOISES):
  if d0==d1:continue
  a=results[d0,d1,change,base,noise,'self_exciting']['cost']
  b=results[d0,d1,change,base,noise,'periodic6']['cost']
  matches.append((a,b))
 assert (sum(a<b for a,b in matches),sum(a>b for a,b in matches))==(78,84)
 output=dict(schema='cern.h189.self-exciting.v1',configurations=486,updates=14580,
              policies=total,self_vs_periodic={'wins':78,'losses':84,'ties':0},
              probe_amplitude='3/64',noise_bound='1/64',stale_cost='1/128',
              physical_demonstration=False)
 Path(__file__).with_name('results.json').write_text(json.dumps(output,indent=2)+'\n')
 print(json.dumps(output,indent=2))
if __name__=='__main__':main()
