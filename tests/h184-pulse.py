from fractions import Fraction as F
from itertools import product
from pathlib import Path
import json
N=24
EPS=F(1,64)
STARTS=(-F(1,4),F(0),F(1,4))
DRIFTS=(-F(1,64),F(0),F(1,64))
DELAYS=(0,1,2)
NOISES=(-EPS,F(0),EPS)
AMPS=tuple(F(k,64) for k in range(9))
def run(amp,start,drift,delay,noise):
 theta=[start]
 for t in range(N):theta.append(theta[-1]+drift-(amp if t==3 else F(0)))
 observations=[theta[max(0,t-delay)]+noise for t in range(N)]
 residuals=[max(abs(observations[t]-theta[max(0,t-d)]) for t in range(N)) for d in DELAYS]
 feasible=[d for d,r in zip(DELAYS,residuals) if r<=EPS]
 assert delay in feasible
 return len(feasible)==1
def curves(amp,start=F(0),drift=F(0)):
 theta=[start]
 for t in range(N):theta.append(theta[-1]+drift-(amp if t==3 else F(0)))
 return [[theta[max(0,t-d)] for t in range(N)] for d in DELAYS]
def min_sep(amp):
 x=curves(amp)
 return min(max(abs(a-b) for a,b in zip(x[i],x[j])) for i in DELAYS for j in DELAYS if i<j)
rows=[];experiments=checks=0
for amp in AMPS:
 unique=0
 for start,drift,delay,noise in product(STARTS,DRIFTS,DELAYS,NOISES):
  unique+=run(amp,start,drift,delay,noise);experiments+=1;checks+=N*3
 sep=min_sep(amp);assert sep==amp
 rows.append(dict(amplitude=str(amp),quadratic_cost=str(amp*amp),
  pairwise_separation=str(sep),identified_constant_noise=unique,total=81,
  guaranteed_arbitrary_bounded_noise=sep>2*EPS))
assert (experiments,checks)==(729,52488)
assert next(row['amplitude'] for row in rows if row['guaranteed_arbitrary_bounded_noise'])=='3/64'
for amp in (F(1,64),F(1,32)):
 a,b,_=curves(amp);mid=[(x+y)/2 for x,y in zip(a,b)]
 assert all(abs(m-x)<=EPS and abs(m-y)<=EPS for m,x,y in zip(mid,a,b))
assert all(max(abs(x-y) for x,y in zip(curves(F(3,64))[i],curves(F(3,64))[j]))>2*EPS for i in range(3) for j in range(i+1,3))
report=dict(schema='cern.h184.minimum-diagnostic-pulse.v1',noise_bound='1/64',
 experiments=experiments,residual_checks=checks,minimum_guaranteed_grid_amplitude='3/64',
 quadratic_cost='9/4096',threshold='A>1/32',counterexample='At A=1/32 the midpoint fits two possible delays',rows=rows)
Path(__file__).with_name('results.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
