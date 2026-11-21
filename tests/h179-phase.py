from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json
INITIAL=(-F(1,4),F(0),F(1,4))
DRIFTS=(F(0),F(1,64),F(1,32),-F(1,64))
DELAYS=range(4)
DIFFS=(-F(1,64),F(0),F(1,64))
POLICIES=('delayed','fixed_cap','adaptive','unbounded')
CAPMAX=F(1,4)
def clip(x,cap):return min(cap,max(-cap,x))
def run(common,delay,diff,policy):
 p=list(INITIAL);history=[F(0)];effort=F(0);clips=0;max_command=F(0)
 for n in range(24):
  mean=sum(p)/3
  p=[(v+mean)/2+common+diff*(i-1) for i,v in enumerate(p)]
  ix=max(0,len(history)-1-delay)
  obs=history[ix];prev=history[max(0,ix-1)]
  delta=obs-prev;raw=(obs+F(delay+1)*delta)/2
  if policy=='delayed':cmd=obs/2;cap=CAPMAX
  elif policy=='unbounded':cmd=raw;cap=CAPMAX
  elif policy=='fixed_cap':cap=F(1,16);cmd=clip(raw,cap)
  else:
   cap=min(CAPMAX,max(F(1,64),F(1,64)+abs(delta)/F(delay+1)))
   cmd=clip(raw,cap)
  if policy in ('fixed_cap','adaptive'):clips+=cmd!=raw
  max_command=max(max_command,abs(cmd))
  p=[v-cmd for v in p];history.append(sum(p)/3)
  effort+=cmd*cmd
  if policy=='adaptive':assert abs(cmd)<=cap<=CAPMAX
  if policy=='fixed_cap':assert abs(cmd)<=F(1,16)
 return dict(error=abs(history[-1]),effort=effort,clips=clips,max_command=max_command)
cases={(d,t,s,p):run(d,t,s,p) for d,t,s,p in product(DRIFTS,DELAYS,DIFFS,POLICIES)}
assert len(cases)==192
counts={f'adaptive_{word}_{name}':0 for name in ('fixed','delayed','unbounded') for word in ('beats','ties','loses')}
counts.update(adaptive_clipped_cases=0,adaptive_cap_verified_cases=0)
for d,t,s in product(DRIFTS,DELAYS,DIFFS):
 a=cases[d,t,s,'adaptive']
 counts['adaptive_clipped_cases']+=a['clips']>0
 counts['adaptive_cap_verified_cases']+=a['max_command']<=CAPMAX
 for name,policy in (('fixed','fixed_cap'),('delayed','delayed'),('unbounded','unbounded')):
  b=cases[d,t,s,policy]
  word='beats' if a['error']<b['error'] else 'ties' if a['error']==b['error'] else 'loses'
  counts[f'adaptive_{word}_{name}']+=1
assert counts=={
 'adaptive_beats_fixed':18,'adaptive_ties_fixed':18,'adaptive_loses_fixed':12,
 'adaptive_beats_delayed':6,'adaptive_ties_delayed':12,'adaptive_loses_delayed':30,
 'adaptive_beats_unbounded':27,'adaptive_ties_unbounded':18,'adaptive_loses_unbounded':3,
 'adaptive_clipped_cases':30,'adaptive_cap_verified_cases':48}
report=dict(version='H179',configurations=192,updates=4608,matched=48,violations=0,counts=counts,
 example={policy:{'error':str(cases[F(1,32),2,F(0),policy]['error']),
 'effort':str(cases[F(1,32),2,F(0),policy]['effort'])} for policy in POLICIES},
 note='Synthetic unwrapped phase reference, not a demonstrated kinetic engine')
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
