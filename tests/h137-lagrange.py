import numpy as np, json
from scipy.optimize import brentq
from fractions import Fraction as F
from pathlib import Path
mu=.01

def grad(x,y):
 a=np.array([x+mu,y]); b=np.array([x-1+mu,y]);return np.array([x,y])-(1-mu)*a/np.linalg.norm(a)**3-mu*b/np.linalg.norm(b)**3

def hessian(x,y):
 H=np.eye(2)
 for m,r in [(1-mu,np.array([x+mu,y])),(mu,np.array([x-1+mu,y]))]:
  R=np.linalg.norm(r);H+=m*(3*np.outer(r,r)/R**5-np.eye(2)/R**3)
 return H
p=[(brentq(lambda x:grad(x,0)[0],a,b),0) for a,b in [(-mu+1e-6,1-mu-1e-6),(1-mu+1e-6,3),(-3,-mu-1e-6)]]+[(.5-mu,np.sqrt(3)/2),(.5-mu,-np.sqrt(3)/2)]
J=np.array([[0,-1],[1,0]])
rows=[];checks=0
for i,(x,y) in enumerate(p):
 H=hessian(x,y);M=np.block([[np.zeros((2,2)),np.eye(2)],[H,-2*J]])
 eig=np.linalg.eigvals(M);res=float(np.linalg.norm(grad(x,y)));assert res<1e-9
 growth=float(max(eig.real));assert (growth>1e-6)==(i<3)
 for t in np.linspace(0,2*np.pi,73):
  R=np.array([[np.cos(t),-np.sin(t)],[np.sin(t),np.cos(t)]]);T=np.block([[R,np.zeros((2,2))],[np.zeros((2,2)),R]])
  assert np.allclose(R@J@R.T,J) and np.allclose(np.sort_complex(np.linalg.eigvals(T@M@T.T)).real,np.sort_complex(eig).real,atol=1e-7)
  checks+=1
 rows.append(dict(point=f'L{i+1}',x=x,y=y,residual=res,growth=growth))
assert 27*F(1,100)*(1-F(1,100))<1
assert 27*F(1,20)*(1-F(1,20))>1
out=dict(mu=mu,points=rows,orientation_checks=checks,max_residual=max(r['residual'] for r in rows),routh_threshold=(1-np.sqrt(23/27))/2)
Path(__file__).with_name('results.json').write_text(json.dumps(out,indent=2));print(json.dumps(out,indent=2))
