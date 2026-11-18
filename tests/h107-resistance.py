import numpy as np
from scipy.linalg import expm
n=24; k=1/n**2; d=2-2*np.cos(2*np.pi/n)+k
D=np.diag([d]*4+[2*d-k]*4+[577*k]); rng=np.random.default_rng(107)
ratios=[0,.5,1,1.01]; worst=-np.inf; count=0
for _ in range(128):
 M=rng.normal(size=(9,9)); C=np.linalg.cholesky(M.T@M+np.eye(9)).T
 W=rng.normal(size=(9,9)); A=-D+W-W.T
 x=rng.normal(size=9); x/=np.linalg.norm(x)
 for ratio in ratios[:3]:
  u=ratio*d*x
  y=np.linalg.solve(C,x); f=np.linalg.solve(C,u)
  derivative=2*(C@y)@(A@(C@y)+C@f)
  worst=max(worst,derivative); assert derivative<=1e-12; count+=1
# Sharp example: slowest eigenmode, no circulation, constant aligned force.
times=np.linspace(0,50,101)
for ratio in ratios:
 r=ratio+(1-ratio)*np.exp(-d*times)
 assert np.all(r<=1+1e-12) if ratio<=1 else np.all(r[1:]>1)
 print(f'ratio={ratio:g} radius_at_50={r[-1]:.12f}')
print(f'PASS {count} boundary checks; max derivative={worst:.12g}; delta={d:.12g}')
