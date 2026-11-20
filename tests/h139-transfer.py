import numpy as np, json
from scipy.optimize import brentq
from scipy.integrate import solve_ivp
from pathlib import Path
mu=.01
m=np.array([[-mu,0],[1-mu,0]])
def g(r):
 d=r-m;return r-sum(a*v/np.linalg.norm(v)**3 for a,v in zip([1-mu,mu],d))
J=np.array([[0,-1],[1,0]])
f=lambda x:g(np.array([x,0]))[0]
P=np.array([[brentq(f,a,b),0] for a,b in [(-mu+1e-6,1-mu-1e-6),(1-mu+1e-6,3),(-3,-mu-1e-6)]]+[[.5-mu,np.sqrt(3)/2],[.5-mu,-np.sqrt(3)/2]])
def route(i,j):
 a,b=P[i],P[j]; y=1.5 if a[1]+b[1]>=0 else -1.5
 return np.array([a,[a[0],y],[b[0],y],b])
def reference(t,W):
 k=min(int(t//4),2);s=np.clip((t-4*k)/4,0,1);D=W[k+1]-W[k]
 return W[k]+D*(10*s**3-15*s**4+6*s**5),D*(30*s*s-60*s**3+30*s**4)/4,D*(60*s-180*s*s+120*s**3)/16
rows=[]
for i in range(5):
 for j in range(5):
  if i==j:continue
  W=route(i,j)
  clear=min(np.linalg.norm(a+np.clip(np.dot(p-a,b-a)/np.dot(b-a,b-a),0,1)*(b-a)-p) if np.any(b!=a) else np.linalg.norm(a-p) for a,b in zip(W,W[1:]) for p in m)
  assert clear>0.01
  def rhs(t,z):
   r,v=z[:2],z[2:];R,V,A=reference(t,W)
   control=A-g(r)+2*J@v-2*(v-V)-(r-R)
   return np.r_[v,g(r)-2*J@v+control]
  initial=np.r_[P[i]+[.001,-.001],[.002,0]]
  sol=solve_ivp(rhs,[0,12],initial,rtol=1e-9,atol=1e-11,max_step=.05)
  assert sol.success
  err=np.linalg.norm(sol.y[:2,-1]-P[j]);assert err<1e-6
  tube=np.linalg.norm(initial[:2]-P[i])+np.linalg.norm(initial[2:]+initial[:2]-P[i])/np.e
  assert clear>tube
  rows.append(dict(source=i+1,target=j+1,clearance=float(clear),tracking_tube=float(tube),terminal_error=float(err),waypoints=W.tolist()))
out=dict(transfers=len(rows),max_terminal_error=max(r['terminal_error'] for r in rows),min_reference_clearance=min(r['clearance'] for r in rows),tracking_tube=rows[0]['tracking_tube'],points=P.tolist(),routes=rows)
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2));print({k:v for k,v in out.items() if k not in ('routes','points')})
